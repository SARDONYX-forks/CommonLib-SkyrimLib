//! Engine heap wrappers (`RE::malloc`, `RE::free`, ...), with a CRT fallback
//! when running outside the game (unit tests).

use core::ffi::c_void;

use crate::re::memory_manager::MemoryManager;
use crate::re::scrap_heap::ScrapHeap;

unsafe extern "C" {
    #[link_name = "malloc"]
    fn crt_malloc(size: usize) -> *mut c_void;
    #[link_name = "calloc"]
    fn crt_calloc(count: usize, size: usize) -> *mut c_void;
    #[link_name = "realloc"]
    fn crt_realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    #[link_name = "free"]
    fn crt_free(ptr: *mut c_void);
    fn _aligned_malloc(size: usize, alignment: usize) -> *mut c_void;
    fn _aligned_free(ptr: *mut c_void);
}

#[inline(always)]
fn normalize_alignment(alignment: usize) -> usize {
    alignment
        .max(core::mem::size_of::<*const ()>())
        .next_power_of_two()
}

#[inline(always)]
fn heap() -> Option<&'static mut MemoryManager> {
    unsafe { MemoryManager::get_singleton().as_mut() }
}

/// `RE::malloc`
pub unsafe fn commonlib_malloc(size: usize) -> *mut c_void {
    let size = size.max(1);
    if super::test_harness_active() {
        return unsafe { crt_malloc(size) };
    }
    heap().map_or(core::ptr::null_mut(), |heap| heap.allocate(size, 0, false))
}

/// `RE::free`
pub unsafe fn commonlib_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    if super::test_harness_active() {
        return unsafe { crt_free(ptr) };
    }
    if let Some(heap) = heap() {
        heap.deallocate(ptr, false);
    }
}

/// `RE::aligned_alloc`
///
/// Exported unmangled because `cstd`'s global allocator links against it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn commonlib_aligned_alloc(alignment: usize, size: usize) -> *mut c_void {
    let size = size.max(1);
    if super::test_harness_active() {
        return unsafe { _aligned_malloc(size, normalize_alignment(alignment)) };
    }
    heap().map_or(core::ptr::null_mut(), |heap| {
        heap.allocate(size, alignment as i32, true)
    })
}

/// `RE::aligned_free`
///
/// Exported unmangled because `cstd`'s global allocator links against it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn commonlib_aligned_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    if super::test_harness_active() {
        return unsafe { _aligned_free(ptr) };
    }
    if let Some(heap) = heap() {
        heap.deallocate(ptr, true);
    }
}

/// `RE::calloc`
pub unsafe fn commonlib_calloc(count: usize, size: usize) -> *mut c_void {
    if super::test_harness_active() {
        return unsafe { crt_calloc(count.max(1), size.max(1)) };
    }
    let Some(total) = count.checked_mul(size) else {
        return core::ptr::null_mut();
    };
    let ptr = unsafe { commonlib_malloc(total) };
    if !ptr.is_null() {
        unsafe { core::ptr::write_bytes(ptr.cast::<u8>(), 0, total) };
    }
    ptr
}

/// `RE::realloc`
pub unsafe fn commonlib_realloc(ptr: *mut c_void, new_size: usize) -> *mut c_void {
    let new_size = new_size.max(1);
    if super::test_harness_active() {
        return unsafe { crt_realloc(ptr, new_size) };
    }
    heap().map_or(core::ptr::null_mut(), |heap| {
        heap.reallocate(ptr, new_size, 0, false)
    })
}

/// `RE::MemoryManager::GetSingleton()`
pub unsafe fn commonlib_memory_manager_get_singleton() -> *mut c_void {
    if super::test_harness_active() {
        return core::ptr::null_mut();
    }
    MemoryManager::get_singleton().cast()
}

/// `RE::MemoryManager::GetThreadScrapHeap()`
pub unsafe fn commonlib_memory_manager_get_thread_scrap_heap(mgr: *mut c_void) -> *mut c_void {
    if super::test_harness_active() {
        return core::ptr::null_mut();
    }
    match unsafe { mgr.cast::<MemoryManager>().as_mut() } {
        Some(mgr) => mgr.get_thread_scrap_heap().cast(),
        None => core::ptr::null_mut(),
    }
}

/// `RE::ScrapHeap::Allocate`
pub unsafe fn commonlib_scrap_heap_allocate(
    heap: *mut c_void,
    size: usize,
    alignment: usize,
) -> *mut c_void {
    match unsafe { heap.cast::<ScrapHeap>().as_mut() } {
        Some(heap) if !super::test_harness_active() => heap.allocate(size, alignment),
        _ => unsafe { commonlib_aligned_alloc(alignment, size) },
    }
}

/// `RE::ScrapHeap::Deallocate`
pub unsafe fn commonlib_scrap_heap_deallocate(heap: *mut c_void, mem: *mut c_void) {
    match unsafe { heap.cast::<ScrapHeap>().as_mut() } {
        Some(heap) if !super::test_harness_active() => heap.deallocate(mem),
        _ => unsafe { commonlib_aligned_free(mem) },
    }
}
