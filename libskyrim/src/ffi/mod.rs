//! Pure-Rust replacement for the former C++ `commonlib_bridge` seam.
//!
//! Historically every function in this module was an `extern "C"` import from a
//! statically linked CommonLibSSE-NG + `bridge.cpp` build. The module keeps the
//! same function names and signatures so existing call sites keep compiling,
//! but every entrypoint is now implemented in Rust on top of:
//!
//! - [`iddb`]: an Address Library loader (formats 1/2/5 and the VR csv), which
//!   also covers the 1.7.x (format 5) runtimes
//! - [`module`]: the loaded game module (base address, PE segments, version)
//! - [`patch`]: memory patching and the branch trampoline
//! - [`api`]: SKSE interface bookkeeping (`SKSE::Init` equivalent)
//! - the remaining submodules port the individual CommonLib helpers.

use core::ffi::c_void;
use core::mem::MaybeUninit;

pub mod api;
pub mod iddb;
pub mod module;
pub mod patch;

mod cxx;
mod game;
mod memory;
mod skse;
mod strings;

pub use api::*;
pub use cxx::*;
pub use game::*;
pub use memory::*;
pub use patch::*;
pub use skse::*;
pub use strings::*;

pub type CommonlibBstEventSinkProcessCallback =
    unsafe extern "C" fn(ctx: *mut c_void, event: *const c_void, event_source: *mut c_void) -> i32;
pub type CommonlibFunctionArgumentsCollectCallback =
    unsafe extern "C" fn(ctx: *mut c_void, dst: *mut c_void) -> bool;
pub type CommonlibNativeFunctionMarshallCallback = unsafe extern "C" fn(
    ctx: *mut c_void,
    base_value: *mut c_void,
    vm: *mut c_void,
    stack_id: u32,
    result_value: *mut c_void,
    frame: *const c_void,
) -> bool;

pub type CommonlibDestroyCallback = unsafe extern "C" fn(ctx: *mut c_void);

/// Constructs an object directly into caller-provided out storage and returns the
/// initialized value on success.
///
/// This is the Rust-side half of the out-param construction pattern used for
/// smart-pointer helpers such as `make_hkref`, `make_nismart`, and `make_smart`.
#[inline(always)]
pub unsafe fn try_construct_out_param<T>(construct: impl FnOnce(*mut T) -> bool) -> Option<T> {
    let mut out = MaybeUninit::<T>::uninit();
    if construct(out.as_mut_ptr()) {
        Some(unsafe { out.assume_init() })
    } else {
        None
    }
}

/// Returns `true` when the current process is not a Skyrim runtime, i.e. when the
/// library runs inside a `cargo test` harness.
///
/// In that mode engine-backed helpers (allocators, string pool, ...) fall back to
/// process-local implementations, mirroring the old C++ test bridge.
#[inline]
pub fn test_harness_active() -> bool {
    !module::is_game_process()
}
