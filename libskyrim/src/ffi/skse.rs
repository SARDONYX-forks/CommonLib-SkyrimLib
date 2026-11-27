//! Ports of `SKSE::GetIATAddr/PatchIAT`, `SKSE::Translation` and the SEH
//! crash-forwarding helper.

use core::ffi::{CStr, c_char, c_void};

use windows_sys::Win32::System::Diagnostics::Debug::{
    AddVectoredExceptionHandler, EXCEPTION_POINTERS, IMAGE_DIRECTORY_ENTRY_IMPORT,
    IMAGE_NT_HEADERS64, RaiseException, RemoveVectoredExceptionHandler, UnhandledExceptionFilter,
};
use windows_sys::Win32::System::SystemServices::{
    IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_IMPORT_BY_NAME, IMAGE_IMPORT_DESCRIPTOR,
};
use windows_sys::Win32::System::Threading::{GetCurrentProcess, TerminateProcess};

// ---------------------------------------------------------------------------
// IAT

const IMAGE_ORDINAL_FLAG64: u64 = 0x8000_0000_0000_0000;

fn eq_ignore_case(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.eq_ignore_ascii_case(y))
}

/// `SKSE::GetIATPtr(module, dll, function)`
unsafe fn iat_ptr(module: usize, dll: &[u8], function: &[u8]) -> *mut usize {
    if module == 0 {
        return core::ptr::null_mut();
    }

    unsafe {
        let dos = &*(module as *const IMAGE_DOS_HEADER);
        if dos.e_magic != IMAGE_DOS_SIGNATURE {
            crate::skse_error!(log, "Invalid DOS header");
            return core::ptr::null_mut();
        }

        let nt = &*((module + dos.e_lfanew as usize) as *const IMAGE_NT_HEADERS64);
        let dir = nt.OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT as usize];
        let mut import = (module + dir.VirtualAddress as usize) as *const IMAGE_IMPORT_DESCRIPTOR;

        while (*import).Anonymous.Characteristics != 0 {
            let name = CStr::from_ptr((module + (*import).Name as usize) as *const c_char);
            // NOTE: CommonLib only skips on a same-length case-insensitive
            // mismatch; a different-length name is still scanned. Kept as-is.
            if name.to_bytes().len() == dll.len() && !eq_ignore_case(name.to_bytes(), dll) {
                import = import.add(1);
                continue;
            }

            let thunk = (module + (*import).Anonymous.OriginalFirstThunk as usize) as *const u64;
            let mut i = 0;
            while *thunk.add(i) != 0 {
                let entry = *thunk.add(i);
                if entry & IMAGE_ORDINAL_FLAG64 == 0 {
                    let by_name = (module + entry as usize) as *const IMAGE_IMPORT_BY_NAME;
                    let fn_name = CStr::from_ptr((*by_name).Name.as_ptr().cast::<c_char>());
                    if eq_ignore_case(fn_name.to_bytes(), function) {
                        let iat = (module + (*import).FirstThunk as usize) as *mut usize;
                        return iat.add(i);
                    }
                }
                i += 1;
            }

            import = import.add(1);
        }
    }

    crate::skse_warning!(
        log,
        "Failed to find {} ({})",
        core::str::from_utf8(dll).unwrap_or("?"),
        core::str::from_utf8(function).unwrap_or("?")
    );
    core::ptr::null_mut()
}

#[inline]
unsafe fn cstr_bytes<'a>(ptr: *const c_char) -> &'a [u8] {
    unsafe { CStr::from_ptr(ptr).to_bytes() }
}

pub unsafe fn commonlib_skse_iat_get_addr(dll: *const i8, function: *const i8) -> usize {
    if dll.is_null() || function.is_null() {
        return 0;
    }
    unsafe { iat_ptr(super::module::base(), cstr_bytes(dll), cstr_bytes(function)) as usize }
}

pub unsafe fn commonlib_skse_iat_get_addr_for_module(
    module: *mut c_void,
    dll: *const i8,
    function: *const i8,
) -> usize {
    if module.is_null() || dll.is_null() || function.is_null() {
        return 0;
    }
    unsafe { iat_ptr(module as usize, cstr_bytes(dll), cstr_bytes(function)) as usize }
}

/// `SKSE::PatchIAT(new_func, dll, function)`; returns the previous import.
pub unsafe fn commonlib_skse_iat_patch(
    new_func: usize,
    dll: *const i8,
    function: *const i8,
) -> usize {
    let slot = unsafe { commonlib_skse_iat_get_addr(dll, function) };
    if slot == 0 {
        crate::skse_warning!(log, "Failed to patch IAT entry");
        return 0;
    }
    unsafe {
        let original = core::ptr::read_volatile(slot as *const usize);
        super::commonlib_safe_write(
            slot,
            core::ptr::from_ref(&new_func).cast::<u8>(),
            core::mem::size_of::<usize>(),
        );
        original
    }
}

// ---------------------------------------------------------------------------
// Translation

// TODO: `SKSE::Translation::ParseTranslation/Translate` need
// `BSScaleformManager::loader`, `GFxLoader::GetStateAddRef<GFxTranslator>`,
// `BSScaleformTranslator::translationMap`, `BSResourceNiBinaryStream` and
// `GFxWStringBuffer`, none of which are translated in `re/` yet. Until they are,
// these entrypoints log and report "no translation" instead of calling into
// the engine.
pub unsafe fn commonlib_skse_translation_parse_translation(name: *const i8) {
    if name.is_null() {
        return;
    }
    crate::skse_warning!(
        log,
        "SKSE::Translation::ParseTranslation is not available in the pure Rust runtime yet ({})",
        unsafe { CStr::from_ptr(name) }.to_string_lossy()
    );
}

pub unsafe fn commonlib_skse_translation_translate(
    _key: *const i8,
    _out_buf: *mut i8,
    _out_buf_len: usize,
) -> usize {
    0
}

// ---------------------------------------------------------------------------
// SEH

const EXCEPTION_NONCONTINUABLE: u32 = 0x1;

static mut RAISED_CODE: u32 = 0;

unsafe extern "system" fn forward_to_unhandled_filter(info: *mut EXCEPTION_POINTERS) -> i32 {
    unsafe {
        let record = (*info).ExceptionRecord;
        if !record.is_null() && (*record).ExceptionCode as u32 == RAISED_CODE {
            // Hand the captured context to the process-wide unhandled exception
            // filter so external crash loggers see a real crash, then stop.
            UnhandledExceptionFilter(info);
            TerminateProcess(GetCurrentProcess(), RAISED_CODE);
        }
    }
    0 // EXCEPTION_CONTINUE_SEARCH
}

/// Raises a real SEH exception so vectored handlers (crash loggers) run, then
/// forwards it to `UnhandledExceptionFilter` and terminates the process.
///
/// The C++ bridge used `__try/__except`; Rust has no SEH frames, so a vectored
/// handler registered *last* plays the role of the `__except` filter.
pub unsafe fn commonlib_raise_seh_exception(code: u32) {
    unsafe {
        RAISED_CODE = code;
        let handler = AddVectoredExceptionHandler(0, Some(forward_to_unhandled_filter));
        RaiseException(code, EXCEPTION_NONCONTINUABLE, 0, core::ptr::null());
        if !handler.is_null() {
            RemoveVectoredExceptionHandler(handler);
        }
        TerminateProcess(GetCurrentProcess(), code);
    }
}
