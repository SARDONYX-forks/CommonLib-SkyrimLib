//! Rust port of the parts of `REL::Module` that libskyrim needs: the loaded
//! executable's base address, its PE segments, and its file version.
//!
//! Everything in here is allocation free on purpose: the global allocator of
//! plugin crates routes through the engine `MemoryManager`, whose singleton is
//! resolved through the Address Library, which in turn needs this module.

use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

use windows_sys::Win32::System::Diagnostics::Debug::{IMAGE_NT_HEADERS64, IMAGE_SECTION_HEADER};
use windows_sys::Win32::System::LibraryLoader::{
    FindResourceW, GetModuleHandleW, LoadResource, LockResource, SizeofResource,
};
use windows_sys::Win32::System::SystemServices::{
    IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_NT_SIGNATURE,
};

use crate::version::Version;

const IMAGE_SCN_MEM_EXECUTE: u32 = 0x2000_0000;
const RT_VERSION: u16 = 16;
const VS_FIXEDFILEINFO_SIGNATURE: u32 = 0xFEEF_04BD;

/// Mirrors `REL::Segment::Name`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Segment {
    /// `.text` (executable)
    Textx,
    /// `.idata`
    Idata,
    /// `.rdata`
    Rdata,
    /// `.data`
    Data,
    /// `.pdata`
    Pdata,
    /// `.tls`
    Tls,
    /// `.text` (writable)
    Textw,
    /// `.gfids`
    Gfids,
}

impl Segment {
    const fn name(self) -> &'static [u8] {
        match self {
            Self::Textx | Self::Textw => b".text",
            Self::Idata => b".idata",
            Self::Rdata => b".rdata",
            Self::Data => b".data",
            Self::Pdata => b".pdata",
            Self::Tls => b".tls",
            Self::Gfids => b".gfids",
        }
    }
}

/// Runtime flavour of the loaded executable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleRuntime {
    SE,
    AE,
    VR,
}

impl ModuleRuntime {
    /// Classifies a runtime version the same way CommonLibSSE-NG does:
    /// `1.4.x` is VR, `1.6.x` and newer (including `1.7.x`) is AE.
    pub const fn from_version(version: Version) -> Self {
        if version.major() == 1 && version.minor() == 4 {
            Self::VR
        } else if version.major() > 1 || version.minor() >= 6 {
            Self::AE
        } else {
            Self::SE
        }
    }
}

const STATE_UNKNOWN: u8 = 0;
const STATE_GAME: u8 = 1;
const STATE_NOT_GAME: u8 = 2;

static GAME_STATE: AtomicU8 = AtomicU8::new(STATE_UNKNOWN);
static VERSION: AtomicUsize = AtomicUsize::new(0);

/// `true` when the current process is `SkyrimSE.exe` or `SkyrimVR.exe`.
pub fn is_game_process() -> bool {
    match GAME_STATE.load(Ordering::Acquire) {
        STATE_GAME => true,
        STATE_NOT_GAME => false,
        _ => {
            let se: [u16; 13] = wide_lit(b"SkyrimSE.exe");
            let vr: [u16; 13] = wide_lit(b"SkyrimVR.exe");
            let found = unsafe {
                !GetModuleHandleW(se.as_ptr()).is_null() || !GetModuleHandleW(vr.as_ptr()).is_null()
            };
            GAME_STATE.store(
                if found { STATE_GAME } else { STATE_NOT_GAME },
                Ordering::Release,
            );
            found
        }
    }
}

/// Base address of the main executable (`REL::Module::get().base()`).
#[inline]
pub fn base() -> usize {
    unsafe { GetModuleHandleW(core::ptr::null()) as usize }
}

/// Returns `(address, size)` of the requested PE segment of the main executable.
pub fn segment(segment: Segment) -> Option<(usize, usize)> {
    let base = base();
    if base == 0 {
        return None;
    }

    unsafe {
        let dos = &*(base as *const IMAGE_DOS_HEADER);
        if dos.e_magic != IMAGE_DOS_SIGNATURE {
            return None;
        }

        let nt = &*((base + dos.e_lfanew as usize) as *const IMAGE_NT_HEADERS64);
        if nt.Signature != IMAGE_NT_SIGNATURE {
            return None;
        }

        let sections = (core::ptr::from_ref(nt) as usize
            + core::mem::offset_of!(IMAGE_NT_HEADERS64, OptionalHeader)
            + nt.FileHeader.SizeOfOptionalHeader as usize)
            as *const IMAGE_SECTION_HEADER;
        let count = nt.FileHeader.NumberOfSections as usize;
        let sections = core::slice::from_raw_parts(sections, count);

        let wanted = segment.name();
        for section in sections {
            let name_len = section
                .Name
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(section.Name.len());
            if &section.Name[..name_len] != wanted {
                continue;
            }

            let executable = (section.Characteristics & IMAGE_SCN_MEM_EXECUTE) != 0;
            match segment {
                Segment::Textx if !executable => continue,
                Segment::Textw if executable => continue,
                _ => {}
            }

            let size = section.Misc.VirtualSize as usize;
            return Some((base + section.VirtualAddress as usize, size));
        }
    }

    None
}

/// File version of the main executable, read from its `VS_VERSIONINFO`
/// resource (`REL::Module::get().version()`).
pub fn version() -> Option<Version> {
    let cached = VERSION.load(Ordering::Acquire);
    if cached != 0 {
        return Some(Version::from_packed(cached as u32));
    }

    let version = read_file_version()?;
    VERSION.store(version.pack() as usize, Ordering::Release);
    Some(version)
}

fn read_file_version() -> Option<Version> {
    unsafe {
        let module = GetModuleHandleW(core::ptr::null());
        if module.is_null() {
            return None;
        }

        // MAKEINTRESOURCE(1) / MAKEINTRESOURCE(RT_VERSION)
        let resource = FindResourceW(module, 1 as *const u16, RT_VERSION as usize as *const u16);
        if resource.is_null() {
            return None;
        }

        let size = SizeofResource(module, resource) as usize;
        let handle = LoadResource(module, resource);
        if handle.is_null() || size == 0 {
            return None;
        }

        let data = LockResource(handle) as *const u8;
        if data.is_null() {
            return None;
        }

        let bytes = core::slice::from_raw_parts(data, size);
        // VS_FIXEDFILEINFO is DWORD aligned inside VS_VERSIONINFO.
        let mut offset = 0;
        while offset + 52 <= bytes.len() {
            if read_u32(bytes, offset) == VS_FIXEDFILEINFO_SIGNATURE {
                let ms = read_u32(bytes, offset + 8);
                let ls = read_u32(bytes, offset + 12);
                return Some(Version::new(
                    (ms >> 16) as u16,
                    (ms & 0xFFFF) as u16,
                    (ls >> 16) as u16,
                    (ls & 0xFFFF) as u16,
                ));
            }
            offset += 4;
        }
    }

    None
}

#[inline(always)]
fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

/// Builds a NUL-terminated UTF-16 literal from ASCII bytes at compile time.
pub(crate) const fn wide_lit<const N: usize>(ascii: &[u8]) -> [u16; N] {
    let mut out = [0u16; N];
    let mut i = 0;
    while i < ascii.len() && i + 1 < N {
        out[i] = ascii[i] as u16;
        i += 1;
    }
    out
}

/// Tiny allocation-free UTF-16 string builder used for Win32 paths and names.
pub(crate) struct WideBuf<const N: usize> {
    buf: [u16; N],
    len: usize,
}

impl<const N: usize> WideBuf<N> {
    pub(crate) const fn new() -> Self {
        Self {
            buf: [0; N],
            len: 0,
        }
    }

    pub(crate) fn push_str(&mut self, s: &str) -> &mut Self {
        for unit in s.encode_utf16() {
            if self.len + 1 >= N {
                break;
            }
            self.buf[self.len] = unit;
            self.len += 1;
        }
        self.buf[self.len] = 0;
        self
    }

    pub(crate) fn push_u32(&mut self, mut value: u32) -> &mut Self {
        let mut digits = [0u8; 10];
        let mut count = 0;
        loop {
            digits[count] = b'0' + (value % 10) as u8;
            count += 1;
            value /= 10;
            if value == 0 {
                break;
            }
        }
        while count > 0 {
            count -= 1;
            if self.len + 1 >= N {
                break;
            }
            self.buf[self.len] = digits[count] as u16;
            self.len += 1;
        }
        self.buf[self.len] = 0;
        self
    }

    /// Appends `major{sep}minor{sep}patch{sep}build`.
    pub(crate) fn push_version(&mut self, version: Version, sep: &str) -> &mut Self {
        self.push_u32(version.major() as u32)
            .push_str(sep)
            .push_u32(version.minor() as u32)
            .push_str(sep)
            .push_u32(version.patch() as u32)
            .push_str(sep)
            .push_u32(version.build() as u32)
    }

    pub(crate) fn as_ptr(&self) -> *const u16 {
        self.buf.as_ptr()
    }
}
