//! Allocation-free Rust port of `REL::IDDB` (the Address Library loader).
//!
//! Supported on-disk formats:
//!
//! - format 1: SE `version-1-5-97-0.bin`
//! - format 2: AE `versionlib-1-6-xxxx-0.bin`
//! - format 5: AE 1.7.99+ `versionlib-1-7-xxx-0.bin` (dense, direct-indexed array)
//! - VR `version-1-4-15-0.csv`
//!
//! Formats 1/2 and the VR csv are unpacked into the same named shared mapping
//! (`CommonLibSSEOffsets-v2-<version>`) that C++ CommonLibSSE-NG uses, so every
//! plugin of the process shares one decoded table. Format 5 is memory-mapped
//! straight from disk.
//!
//! Nothing in this file allocates through the Rust global allocator: plugin
//! crates route that allocator through the engine heap, which itself needs the
//! Address Library to be resolved.

use core::ffi::c_void;
use core::sync::atomic::{AtomicU8, Ordering};

use windows_sys::Win32::Foundation::{CloseHandle, GENERIC_READ, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_READONLY, FILE_SHARE_READ, GetFileSizeEx, OPEN_EXISTING,
};
use windows_sys::Win32::System::Memory::{
    CreateFileMappingW, FILE_MAP_READ, FILE_MAP_WRITE, MEMORY_MAPPED_VIEW_ADDRESS, MapViewOfFile,
    OpenFileMappingW, PAGE_READONLY, PAGE_READWRITE, UnmapViewOfFile,
};

use super::module::{self, ModuleRuntime, WideBuf};
use crate::version::Version;

/// `REL::IDDB::mapping_t`
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Mapping {
    pub id: u64,
    pub offset: u64,
}

/// Which format was loaded (`REL::IDDB::Format`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    SSEv1,
    SSEv2,
    SSEv5,
    VR,
}

/// Why the Address Library could not be loaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadError {
    /// The executable version could not be determined.
    UnknownVersion,
    /// The expected `Data/SKSE/Plugins/version*.bin|csv` file is missing.
    MissingFile,
    /// The format byte of the file is not one of 1/2/5.
    UnsupportedFormat(i32),
    /// The file was made for a different game version.
    VersionMismatch,
    /// The file is truncated or otherwise malformed.
    Corrupt,
    /// The shared mapping could not be created.
    MappingFailed,
}

enum Table {
    Sparse(&'static [Mapping]),
    Dense(&'static [u32]),
}

struct Database {
    table: Table,
    format: Format,
}

const STATE_UNLOADED: u8 = 0;
const STATE_LOADING: u8 = 1;
const STATE_READY: u8 = 2;
const STATE_FAILED: u8 = 3;

static STATE: AtomicU8 = AtomicU8::new(STATE_UNLOADED);
static mut DATABASE: Option<Database> = None;
static mut LAST_ERROR: Option<LoadError> = None;

fn database() -> Option<&'static Database> {
    loop {
        match STATE.load(Ordering::Acquire) {
            STATE_READY => {
                // SAFETY: DATABASE is written once before STATE_READY is published.
                return unsafe { (*core::ptr::addr_of!(DATABASE)).as_ref() };
            }
            STATE_FAILED => return None,
            STATE_UNLOADED => {
                if STATE
                    .compare_exchange(
                        STATE_UNLOADED,
                        STATE_LOADING,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                    .is_ok()
                {
                    let result = load();
                    unsafe {
                        match result {
                            Ok(db) => {
                                *core::ptr::addr_of_mut!(DATABASE) = Some(db);
                                STATE.store(STATE_READY, Ordering::Release);
                            }
                            Err(error) => {
                                *core::ptr::addr_of_mut!(LAST_ERROR) = Some(error);
                                STATE.store(STATE_FAILED, Ordering::Release);
                            }
                        }
                    }
                }
            }
            _ => core::hint::spin_loop(),
        }
    }
}

/// Error recorded by the last failed load, if any.
pub fn last_error() -> Option<LoadError> {
    if STATE.load(Ordering::Acquire) == STATE_FAILED {
        unsafe { *core::ptr::addr_of!(LAST_ERROR) }
    } else {
        None
    }
}

/// Format of the loaded database, loading it on demand.
pub fn loaded_format() -> Option<Format> {
    database().map(|db| db.format)
}

/// `REL::IDDB::id2offset`. Returns `None` for unknown ids or when the database
/// could not be loaded.
pub fn id_to_offset(id: u64) -> Option<usize> {
    let db = database()?;
    match db.table {
        Table::Dense(table) => {
            let offset = *table.get(usize::try_from(id).ok()?)?;
            (offset != 0).then_some(offset as usize)
        }
        Table::Sparse(table) => {
            let index = table.binary_search_by(|m| m.id.cmp(&id)).ok()?;
            Some(table[index].offset as usize)
        }
    }
}

/// `REL::IDDB::offset2id` (linear for format 5, binary search otherwise
/// would require a second table; this is a debugging helper only).
pub fn offset_to_id(offset: usize) -> Option<u64> {
    let db = database()?;
    match db.table {
        Table::Dense(table) => table
            .iter()
            .position(|&o| o as usize == offset)
            .map(|id| id as u64),
        Table::Sparse(table) => table
            .iter()
            .find(|m| m.offset as usize == offset)
            .map(|m| m.id),
    }
}

fn load() -> Result<Database, LoadError> {
    let version = module::version()
        .or_else(|| {
            crate::runtime::CURRENT_VERSION
                .is_init()
                .then(|| *crate::runtime::CURRENT_VERSION)
        })
        .ok_or(LoadError::UnknownVersion)?;

    let runtime = ModuleRuntime::from_version(version);
    let mut path = WideBuf::<128>::new();
    match runtime {
        ModuleRuntime::VR => {
            path.push_str("Data/SKSE/Plugins/version-")
                .push_version(version, "-")
                .push_str(".csv");
            load_csv(&path, version)
        }
        ModuleRuntime::AE => {
            path.push_str("Data/SKSE/Plugins/versionlib-")
                .push_version(version, "-")
                .push_str(".bin");
            load_bin(&path, version)
        }
        ModuleRuntime::SE => {
            path.push_str("Data/SKSE/Plugins/version-")
                .push_version(version, "-")
                .push_str(".bin");
            load_bin(&path, version)
        }
    }
}

// ---------------------------------------------------------------------------
// Win32 helpers

/// Read-only view of a whole file. Dropping it unmaps the view.
struct FileView {
    mapping: HANDLE,
    view: *const u8,
    len: usize,
}

impl FileView {
    fn open<const N: usize>(path: &WideBuf<N>) -> Option<Self> {
        unsafe {
            let file = CreateFileW(
                path.as_ptr(),
                GENERIC_READ,
                FILE_SHARE_READ,
                core::ptr::null(),
                OPEN_EXISTING,
                FILE_ATTRIBUTE_READONLY,
                core::ptr::null_mut(),
            );
            if file == INVALID_HANDLE_VALUE {
                return None;
            }

            let mut size = 0i64;
            if GetFileSizeEx(file, &mut size) == 0 || size <= 0 {
                CloseHandle(file);
                return None;
            }

            let mapping = CreateFileMappingW(
                file,
                core::ptr::null(),
                PAGE_READONLY,
                0,
                0,
                core::ptr::null(),
            );
            CloseHandle(file);
            if mapping.is_null() {
                return None;
            }

            let view = MapViewOfFile(mapping, FILE_MAP_READ, 0, 0, 0);
            if view.Value.is_null() {
                CloseHandle(mapping);
                return None;
            }

            Some(Self {
                mapping,
                view: view.Value as *const u8,
                len: size as usize,
            })
        }
    }

    fn bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.view, self.len) }
    }

    /// Keeps the view mapped for the lifetime of the process.
    fn leak(self) -> &'static [u8] {
        let bytes = unsafe { core::slice::from_raw_parts(self.view, self.len) };
        core::mem::forget(self);
        bytes
    }
}

impl Drop for FileView {
    fn drop(&mut self) {
        unsafe {
            UnmapViewOfFile(MEMORY_MAPPED_VIEW_ADDRESS {
                Value: self.view as *mut c_void,
            });
            CloseHandle(self.mapping);
        }
    }
}

/// Opens or creates the named shared mapping used by every CommonLib-based
/// plugin. Returns `(table, created)`.
fn shared_table(
    version: Version,
    count: usize,
) -> Result<(&'static mut [Mapping], bool), LoadError> {
    let mut name = WideBuf::<64>::new();
    name.push_str("CommonLibSSEOffsets-v2-")
        .push_version(version, ".");

    let byte_size = count
        .checked_mul(core::mem::size_of::<Mapping>())
        .ok_or(LoadError::Corrupt)?;

    unsafe {
        let mut created = false;
        let mut mapping = OpenFileMappingW(FILE_MAP_READ | FILE_MAP_WRITE, 0, name.as_ptr());
        if mapping.is_null() {
            mapping = CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                core::ptr::null(),
                PAGE_READWRITE,
                (byte_size as u64 >> 32) as u32,
                byte_size as u32,
                name.as_ptr(),
            );
            if mapping.is_null() {
                return Err(LoadError::MappingFailed);
            }
            created = true;
        }

        let view = MapViewOfFile(mapping, FILE_MAP_READ | FILE_MAP_WRITE, 0, 0, byte_size);
        if view.Value.is_null() {
            CloseHandle(mapping);
            return Err(LoadError::MappingFailed);
        }

        let table = core::slice::from_raw_parts_mut(view.Value.cast::<Mapping>(), count);
        Ok((table, created))
    }
}

// ---------------------------------------------------------------------------
// Binary formats

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn take<const N: usize>(&mut self) -> Result<[u8; N], LoadError> {
        let end = self.pos.checked_add(N).ok_or(LoadError::Corrupt)?;
        let slice = self.bytes.get(self.pos..end).ok_or(LoadError::Corrupt)?;
        self.pos = end;
        let mut out = [0u8; N];
        out.copy_from_slice(slice);
        Ok(out)
    }

    fn skip(&mut self, n: usize) -> Result<(), LoadError> {
        let end = self.pos.checked_add(n).ok_or(LoadError::Corrupt)?;
        if end > self.bytes.len() {
            return Err(LoadError::Corrupt);
        }
        self.pos = end;
        Ok(())
    }

    fn u8(&mut self) -> Result<u8, LoadError> {
        Ok(self.take::<1>()?[0])
    }

    fn u16(&mut self) -> Result<u16, LoadError> {
        Ok(u16::from_le_bytes(self.take()?))
    }

    fn u32(&mut self) -> Result<u32, LoadError> {
        Ok(u32::from_le_bytes(self.take()?))
    }

    fn i32(&mut self) -> Result<i32, LoadError> {
        Ok(i32::from_le_bytes(self.take()?))
    }

    fn u64(&mut self) -> Result<u64, LoadError> {
        Ok(u64::from_le_bytes(self.take()?))
    }
}

fn header_version(raw: [u32; 4]) -> Version {
    Version::new(raw[0] as u16, raw[1] as u16, raw[2] as u16, raw[3] as u16)
}

fn load_bin<const N: usize>(path: &WideBuf<N>, version: Version) -> Result<Database, LoadError> {
    let file = FileView::open(path).ok_or(LoadError::MissingFile)?;
    let mut reader = Reader::new(file.bytes());
    let format = reader.i32()?;

    match format {
        1 | 2 => {
            // header_v2_t
            let mut raw = [0u32; 4];
            for v in &mut raw {
                *v = reader.i32()? as u32;
            }
            let name_len = reader.i32()?;
            reader.skip(usize::try_from(name_len).map_err(|_| LoadError::Corrupt)?)?;
            let pointer_size = reader.i32()?;
            let address_count = reader.i32()?;

            if header_version(raw) != version {
                return Err(LoadError::VersionMismatch);
            }
            let count = usize::try_from(address_count).map_err(|_| LoadError::Corrupt)?;
            let pointer_size = u64::try_from(pointer_size).map_err(|_| LoadError::Corrupt)?;

            let (table, created) = shared_table(version, count)?;
            if created {
                unpack_v2(&mut reader, table, pointer_size)?;
                table.sort_unstable_by_key(|m| m.id);
            }

            Ok(Database {
                table: Table::Sparse(table),
                format: if format == 1 {
                    Format::SSEv1
                } else {
                    Format::SSEv2
                },
            })
        }
        5 => {
            // header_v5_t
            let mut raw = [0u32; 4];
            for v in &mut raw {
                *v = reader.u32()?;
            }
            reader.skip(64)?; // name[64]
            let _pointer_size = reader.i32()?;
            let _data_format = reader.i32()?;
            let offset_count = reader.i32()?;

            if header_version(raw) != version {
                return Err(LoadError::VersionMismatch);
            }

            let count = usize::try_from(offset_count).map_err(|_| LoadError::Corrupt)?;
            let header_bytes = reader.pos;
            let needed = count
                .checked_mul(4)
                .and_then(|n| n.checked_add(header_bytes))
                .ok_or(LoadError::Corrupt)?;
            if file.len < needed {
                return Err(LoadError::Corrupt);
            }

            let bytes = file.leak();
            let data = unsafe { bytes.as_ptr().add(header_bytes) };
            // The header is 0x5C bytes, so the dense array is 4-byte aligned.
            if (data as usize) % core::mem::align_of::<u32>() != 0 {
                return Err(LoadError::Corrupt);
            }
            let table = unsafe { core::slice::from_raw_parts(data.cast::<u32>(), count) };

            Ok(Database {
                table: Table::Dense(table),
                format: Format::SSEv5,
            })
        }
        other => Err(LoadError::UnsupportedFormat(other)),
    }
}

/// `REL::IDDB::unpack_file`
fn unpack_v2(
    reader: &mut Reader<'_>,
    table: &mut [Mapping],
    pointer_size: u64,
) -> Result<(), LoadError> {
    let mut prev_id = 0u64;
    let mut prev_offset = 0u64;

    for mapping in table.iter_mut() {
        let ty = reader.u8()?;
        let lo = ty & 0xF;
        let hi = ty >> 4;

        let id = match lo {
            0 => reader.u64()?,
            1 => prev_id.wrapping_add(1),
            2 => prev_id.wrapping_add(reader.u8()? as u64),
            3 => prev_id.wrapping_sub(reader.u8()? as u64),
            4 => prev_id.wrapping_add(reader.u16()? as u64),
            5 => prev_id.wrapping_sub(reader.u16()? as u64),
            6 => reader.u16()? as u64,
            7 => reader.u32()? as u64,
            _ => return Err(LoadError::Corrupt),
        };

        let scaled = (hi & 8) != 0;
        if scaled && pointer_size == 0 {
            return Err(LoadError::Corrupt);
        }
        let tmp = if scaled {
            prev_offset / pointer_size
        } else {
            prev_offset
        };

        let mut offset = match hi & 7 {
            0 => reader.u64()?,
            1 => tmp.wrapping_add(1),
            2 => tmp.wrapping_add(reader.u8()? as u64),
            3 => tmp.wrapping_sub(reader.u8()? as u64),
            4 => tmp.wrapping_add(reader.u16()? as u64),
            5 => tmp.wrapping_sub(reader.u16()? as u64),
            6 => reader.u16()? as u64,
            7 => reader.u32()? as u64,
            _ => unreachable!(),
        };

        if scaled {
            offset = offset.wrapping_mul(pointer_size);
        }

        *mapping = Mapping { id, offset };
        prev_offset = offset;
        prev_id = id;
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// VR csv

fn csv_lines(bytes: &[u8]) -> impl Iterator<Item = &[u8]> {
    bytes
        .split(|&b| b == b'\n')
        .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
        .filter(|line| !line.is_empty())
}

fn parse_uint(field: &[u8], radix: u32) -> Option<u64> {
    let field = field.trim_ascii();
    let field = if radix == 16 {
        field
            .strip_prefix(b"0x")
            .or_else(|| field.strip_prefix(b"0X"))
            .unwrap_or(field)
    } else {
        field
    };
    if field.is_empty() {
        return None;
    }
    let mut value = 0u64;
    for &b in field {
        let digit = (b as char).to_digit(radix)? as u64;
        value = value.checked_mul(radix as u64)?.checked_add(digit)?;
    }
    Some(value)
}

fn load_csv<const N: usize>(path: &WideBuf<N>, version: Version) -> Result<Database, LoadError> {
    let file = FileView::open(path).ok_or(LoadError::MissingFile)?;
    let mut lines = csv_lines(file.bytes());

    // Row 0: "<address_count>,<address library version>"
    let header = lines.next().ok_or(LoadError::Corrupt)?;
    let mut header_fields = header.split(|&b| b == b',');
    let count = header_fields
        .next()
        .and_then(|f| parse_uint(f, 10))
        .ok_or(LoadError::Corrupt)? as usize;

    let (table, created) = shared_table(version, count)?;
    if created {
        let mut filled = 0usize;
        for line in lines {
            if filled >= count {
                return Err(LoadError::Corrupt);
            }
            let mut fields = line.split(|&b| b == b',');
            let id = fields
                .next()
                .and_then(|f| parse_uint(f, 10))
                .ok_or(LoadError::Corrupt)?;
            let offset = fields
                .next()
                .and_then(|f| parse_uint(f, 16))
                .ok_or(LoadError::Corrupt)?;
            table[filled] = Mapping { id, offset };
            filled += 1;
        }
        if filled != count {
            return Err(LoadError::Corrupt);
        }
        table.sort_unstable_by_key(|m| m.id);
    }

    Ok(Database {
        table: Table::Sparse(table),
        format: Format::VR,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpack_v2_decodes_delta_encoding() {
        // id=7 (type lo=7, u32), offset=0x10 (hi=6, u16)
        // id=prev+1 (lo=1), offset=prev+1 scaled by pointer size 8 (hi=8|1)
        let bytes = [
            0x67, 7, 0, 0, 0, 0x10, 0x00, //
            0x91,
        ];
        let mut table = [Mapping::default(); 2];
        let mut reader = Reader::new(&bytes);
        unpack_v2(&mut reader, &mut table, 8).unwrap();
        assert_eq!(
            table[0],
            Mapping {
                id: 7,
                offset: 0x10
            }
        );
        assert_eq!(
            table[1],
            Mapping {
                id: 8,
                offset: 0x18
            }
        );
    }

    #[test]
    fn csv_parsing_handles_hex_offsets() {
        assert_eq!(parse_uint(b" 0x1A2b ", 16), Some(0x1A2B));
        assert_eq!(parse_uint(b"1A2b", 16), Some(0x1A2B));
        assert_eq!(parse_uint(b"42", 10), Some(42));
        assert_eq!(parse_uint(b"", 10), None);
    }
}
