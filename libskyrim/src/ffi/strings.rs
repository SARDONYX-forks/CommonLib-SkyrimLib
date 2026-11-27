//! `RE::BSFixedString` / `RE::BSFixedStringW` / `RE::BSStringPool::Entry`
//! helpers.
//!
//! Inside the game every string lives in the engine string pool and is
//! preceded by a `BSStringPool::Entry` header. Outside the game (unit tests)
//! strings are plain CRT heap duplicates, mirroring the old C++ test bridge.

use core::ffi::{c_char, c_void};

use crate::re::bs_string_pool::BSStringPoolEntry;
use crate::re::crc::BSTHash;
use crate::relocation::RelocationID;

unsafe extern "C" {
    #[link_name = "malloc"]
    fn crt_malloc(size: usize) -> *mut c_void;
    #[link_name = "free"]
    fn crt_free(ptr: *mut c_void);
}

static EMPTY8: [c_char; 1] = [0];
static EMPTY16: [u16; 1] = [0];

crate::relocation_func! {
    fn game_ctor8(this: *mut *const c_char, data: *const c_char) -> *mut *const c_char
        => RelocationID::new(67819, 69161)
}

crate::relocation_func! {
    fn game_ctor16(this: *mut *const u16, data: *const u16) -> *mut *const u16
        => RelocationID::new(67834, 69176)
}

crate::relocation_func! {
    fn game_release8(entry: *mut *const c_char) => RelocationID::new(67847, 69192)
}

crate::relocation_func! {
    fn game_release16(entry: *mut *const u16) => RelocationID::new(67848, 69193)
}

#[inline(always)]
unsafe fn proxy<T>(data: *const T) -> Option<&'static BSStringPoolEntry> {
    if data.is_null() {
        None
    } else {
        unsafe { data.cast::<BSStringPoolEntry>().sub(1).as_ref() }
    }
}

#[inline(always)]
unsafe fn raw_len<T: Copy + Default + PartialEq>(mut ptr: *const T) -> usize {
    let mut len = 0;
    unsafe {
        while *ptr != T::default() {
            ptr = ptr.add(1);
            len += 1;
        }
    }
    len
}

unsafe fn test_duplicate<T: Copy + Default + PartialEq>(string: *const T) -> *const T {
    if string.is_null() {
        return core::ptr::null();
    }
    unsafe {
        let len = raw_len(string);
        if len == 0 {
            return core::ptr::null();
        }
        let bytes = (len + 1) * core::mem::size_of::<T>();
        let out = crt_malloc(bytes).cast::<T>();
        if !out.is_null() {
            core::ptr::copy_nonoverlapping(string, out, len + 1);
        }
        out
    }
}

unsafe fn test_slice<'a, T: Copy + Default + PartialEq>(data: *const T) -> &'a [T] {
    if data.is_null() {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(data, raw_len(data)) }
    }
}

fn fnv1a(bytes: impl Iterator<Item = u8>) -> u32 {
    let mut hash = 0x811C_9DC5u32;
    for b in bytes {
        hash ^= b as u32;
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}

// ---------------------------------------------------------------------------
// Generic implementation shared by the narrow and wide variants.

trait Char: Copy + Default + PartialEq + 'static {
    fn empty() -> *const Self;
    unsafe fn ctor(out: *mut *const Self, string: *const Self);
    unsafe fn release(entry: *mut *const Self);
}

impl Char for c_char {
    fn empty() -> *const Self {
        EMPTY8.as_ptr()
    }
    unsafe fn ctor(out: *mut *const Self, string: *const Self) {
        game_ctor8(out, string);
    }
    unsafe fn release(entry: *mut *const Self) {
        game_release8(entry);
    }
}

impl Char for u16 {
    fn empty() -> *const Self {
        EMPTY16.as_ptr()
    }
    unsafe fn ctor(out: *mut *const Self, string: *const Self) {
        game_ctor16(out, string);
    }
    unsafe fn release(entry: *mut *const Self) {
        game_release16(entry);
    }
}

unsafe fn ctor<T: Char>(out: *mut c_void, string: *const T) {
    let out = out.cast::<*const T>();
    if out.is_null() {
        return;
    }
    unsafe {
        out.write(core::ptr::null());
        if string.is_null() {
            return;
        }
        if super::test_harness_active() {
            out.write(test_duplicate(string));
        } else {
            T::ctor(out, string);
        }
    }
}

unsafe fn copy<T: Char>(out: *mut c_void, src: *const c_void) {
    let out = out.cast::<*const T>();
    if out.is_null() {
        return;
    }
    unsafe {
        let data = src
            .cast::<*const T>()
            .as_ref()
            .copied()
            .unwrap_or(core::ptr::null());
        if super::test_harness_active() {
            out.write(test_duplicate(data));
        } else {
            out.write(data);
            if let Some(proxy) = proxy(data) {
                proxy.acquire();
            }
        }
    }
}

unsafe fn destroy<T: Char>(string: *mut c_void) {
    let string = string.cast::<*const T>();
    let Some(data) = (unsafe { string.as_mut() }) else {
        return;
    };
    if data.is_null() {
        return;
    }
    unsafe {
        if super::test_harness_active() {
            crt_free(data.cast_mut().cast());
            *data = core::ptr::null();
        } else {
            T::release(data);
        }
    }
}

unsafe fn data_of<T: Char>(string: *const c_void) -> *const T {
    unsafe {
        string
            .cast::<*const T>()
            .as_ref()
            .copied()
            .unwrap_or(core::ptr::null())
    }
}

unsafe fn size<T: Char>(string: *const c_void) -> u32 {
    let data = unsafe { data_of::<T>(string) };
    if super::test_harness_active() {
        return unsafe { test_slice(data).len() as u32 };
    }
    unsafe { proxy(data).map_or(0, BSStringPoolEntry::length) }
}

unsafe fn c_str<T: Char>(string: *const c_void) -> *const T {
    let data = unsafe { data_of::<T>(string) };
    if data.is_null() { T::empty() } else { data }
}

unsafe fn eq<T: Char>(lhs: *const c_void, rhs: *const c_void) -> bool {
    if lhs.is_null() || rhs.is_null() {
        return lhs == rhs;
    }
    unsafe {
        let (l, r) = (data_of::<T>(lhs), data_of::<T>(rhs));
        if super::test_harness_active() {
            return test_slice(l) == test_slice(r);
        }
        l == r || (size::<T>(lhs) == 0 && size::<T>(rhs) == 0)
    }
}

unsafe fn hash<T: Char>(string: *const c_void) -> u32 {
    let data = unsafe { data_of::<T>(string) };
    if super::test_harness_active() {
        let slice = unsafe { test_slice(data) };
        let bytes = unsafe {
            core::slice::from_raw_parts(slice.as_ptr().cast::<u8>(), core::mem::size_of_val(slice))
        };
        return fnv1a(bytes.iter().copied());
    }
    // The game hashes the raw pool pointer, null included.
    data.cast::<c_void>().bst_hash()
}

// ---------------------------------------------------------------------------
// BSFixedString (char)

pub unsafe fn commonlib_bs_fixed_string_ctor8(out: *mut c_void, string: *const c_char) {
    unsafe { ctor::<c_char>(out, string) }
}

pub unsafe fn commonlib_bs_fixed_string_copy(out: *mut c_void, src: *const c_void) {
    unsafe { copy::<c_char>(out, src) }
}

pub unsafe fn commonlib_bs_fixed_string_destroy(string: *mut c_void) {
    unsafe { destroy::<c_char>(string) }
}

pub unsafe fn commonlib_bs_fixed_string_size(string: *const c_void) -> u32 {
    unsafe { size::<c_char>(string) }
}

pub unsafe fn commonlib_bs_fixed_string_c_str(string: *const c_void) -> *const c_char {
    unsafe { c_str::<c_char>(string) }
}

pub unsafe fn commonlib_bs_fixed_string_eq(lhs: *const c_void, rhs: *const c_void) -> bool {
    unsafe { eq::<c_char>(lhs, rhs) }
}

pub unsafe fn commonlib_bs_fixed_string_hash(string: *const c_void) -> u32 {
    unsafe { hash::<c_char>(string) }
}

// ---------------------------------------------------------------------------
// BSFixedStringW (wchar_t)

pub unsafe fn commonlib_bs_fixed_string_ctor16(out: *mut c_void, string: *const u16) {
    unsafe { ctor::<u16>(out, string) }
}

pub unsafe fn commonlib_bs_fixed_string_w_copy(out: *mut c_void, src: *const c_void) {
    unsafe { copy::<u16>(out, src) }
}

pub unsafe fn commonlib_bs_fixed_string_w_destroy(string: *mut c_void) {
    unsafe { destroy::<u16>(string) }
}

pub unsafe fn commonlib_bs_fixed_string_w_size(string: *const c_void) -> u32 {
    unsafe { size::<u16>(string) }
}

pub unsafe fn commonlib_bs_fixed_string_w_c_str(string: *const c_void) -> *const u16 {
    unsafe { c_str::<u16>(string) }
}

pub unsafe fn commonlib_bs_fixed_string_w_eq(lhs: *const c_void, rhs: *const c_void) -> bool {
    unsafe { eq::<u16>(lhs, rhs) }
}

pub unsafe fn commonlib_bs_fixed_string_w_hash(string: *const c_void) -> u32 {
    unsafe { hash::<u16>(string) }
}

// ---------------------------------------------------------------------------
// BSStringPool::Entry

pub unsafe fn commonlib_bs_string_pool_release8(entry: *mut *const c_char) {
    unsafe { destroy::<c_char>(entry.cast()) }
}

pub unsafe fn commonlib_bs_string_pool_release16(entry: *mut *const u16) {
    unsafe { destroy::<u16>(entry.cast()) }
}
