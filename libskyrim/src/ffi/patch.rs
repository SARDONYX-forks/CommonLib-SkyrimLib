//! Rust ports of `REL::safe_write`, `REL::Relocation::write_vfunc` and
//! `SKSE::Trampoline`.

use alloc::collections::BTreeMap;
use core::ffi::c_void;

use spin::Mutex;
use windows_sys::Win32::System::Diagnostics::Debug::FlushInstructionCache;
use windows_sys::Win32::System::Memory::{
    MEM_COMMIT, MEM_FREE, MEM_RESERVE, MEMORY_BASIC_INFORMATION, PAGE_EXECUTE_READWRITE,
    PAGE_PROTECTION_FLAGS, VirtualAlloc, VirtualProtect, VirtualQuery,
};
use windows_sys::Win32::System::SystemInformation::{GetSystemInfo, SYSTEM_INFO};
use windows_sys::Win32::System::Threading::GetCurrentProcess;

use super::module::{self, Segment};

// ---------------------------------------------------------------------------
// Address Library

/// `REL::ID(id).address()`. Returns `0` when the id cannot be resolved.
pub unsafe fn commonlib_id_to_address(id: usize) -> usize {
    match super::iddb::id_to_offset(id as u64) {
        Some(offset) => module::base() + offset,
        None => 0,
    }
}

/// `REL::Offset(offset).address()`.
pub unsafe fn commonlib_offset_to_address(offset: usize) -> usize {
    module::base() + offset
}

// ---------------------------------------------------------------------------
// Memory patching

/// `REL::safe_write`
pub unsafe fn commonlib_safe_write(addr: usize, data: *const u8, len: usize) {
    if addr == 0 || data.is_null() || len == 0 {
        return;
    }

    unsafe {
        let mut old: PAGE_PROTECTION_FLAGS = 0;
        let ok = VirtualProtect(addr as *const c_void, len, PAGE_EXECUTE_READWRITE, &mut old) != 0;
        core::ptr::copy_nonoverlapping(data, addr as *mut u8, len);
        if ok {
            let mut ignored: PAGE_PROTECTION_FLAGS = 0;
            VirtualProtect(addr as *const c_void, len, old, &mut ignored);
        }
        FlushInstructionCache(GetCurrentProcess(), addr as *const c_void, len);
    }
}

/// `REL::safe_fill`
pub unsafe fn commonlib_safe_fill(addr: usize, value: u8, len: usize) {
    if addr == 0 || len == 0 {
        return;
    }

    unsafe {
        let mut old: PAGE_PROTECTION_FLAGS = 0;
        let ok = VirtualProtect(addr as *const c_void, len, PAGE_EXECUTE_READWRITE, &mut old) != 0;
        core::ptr::write_bytes(addr as *mut u8, value, len);
        if ok {
            let mut ignored: PAGE_PROTECTION_FLAGS = 0;
            VirtualProtect(addr as *const c_void, len, old, &mut ignored);
        }
        FlushInstructionCache(GetCurrentProcess(), addr as *const c_void, len);
    }
}

/// `REL::Relocation<std::uintptr_t>(vtable).write_vfunc(idx, new_func)`
///
/// Returns the previous function pointer stored in the slot.
pub unsafe fn commonlib_write_vfunc(vtable_addr: usize, idx: usize, new_func: usize) -> usize {
    if vtable_addr == 0 {
        return 0;
    }

    let slot = vtable_addr + idx * core::mem::size_of::<usize>();
    unsafe {
        let original = core::ptr::read_volatile(slot as *const usize);
        commonlib_safe_write(
            slot,
            core::ptr::from_ref(&new_func).cast::<u8>(),
            core::mem::size_of::<usize>(),
        );
        original
    }
}

// ---------------------------------------------------------------------------
// Trampoline

struct Trampoline {
    data: *mut u8,
    capacity: usize,
    size: usize,
    branches5: BTreeMap<usize, usize>,
    branches6: BTreeMap<usize, usize>,
}

unsafe impl Send for Trampoline {}

impl Trampoline {
    const fn new() -> Self {
        Self {
            data: core::ptr::null_mut(),
            capacity: 0,
            size: 0,
            branches5: BTreeMap::new(),
            branches6: BTreeMap::new(),
        }
    }

    fn set(&mut self, mem: *mut u8, size: usize) {
        if !mem.is_null() {
            // INT3 fill, same as CommonLib.
            unsafe { core::ptr::write_bytes(mem, 0xCC, size) };
        }
        self.branches5.clear();
        self.branches6.clear();
        self.data = mem;
        self.capacity = size;
        self.size = 0;
    }

    fn allocate(&mut self, size: usize) -> *mut u8 {
        if self.data.is_null() || size > self.capacity - self.size {
            crate::skse::crash::raise_logged_runtime_error(format_args!(
                "Trampoline: failed to handle allocation request of {size}B ({}B / {}B used). \
                 Call `alloc_trampoline` with a larger size.",
                self.size, self.capacity
            ));
        }

        let mem = unsafe { self.data.add(self.size) };
        self.size += size;
        mem
    }
}

static TRAMPOLINE: Mutex<Trampoline> = Mutex::new(Trampoline::new());

const fn roundup(number: usize, multiple: usize) -> usize {
    if multiple == 0 {
        return 0;
    }
    let remainder = number % multiple;
    if remainder == 0 {
        number
    } else {
        number + multiple - remainder
    }
}

const fn rounddown(number: usize, multiple: usize) -> usize {
    if multiple == 0 {
        return 0;
    }
    number - number % multiple
}

/// `SKSE::Trampoline::do_create`: finds a free region within +-2GiB of
/// `address` so `rel32` displacements can reach it.
fn create_near(size: usize, address: usize) -> *mut u8 {
    const GIGABYTE: usize = 1 << 30;
    const MIN_RANGE: usize = GIGABYTE * 2;
    const MAX_ADDR: usize = usize::MAX;

    let granularity = unsafe {
        let mut si: SYSTEM_INFO = core::mem::zeroed();
        GetSystemInfo(&mut si);
        si.dwAllocationGranularity as usize
    };

    let mut min = if address >= MIN_RANGE {
        roundup(address - MIN_RANGE, granularity)
    } else {
        0
    };
    let max = if address < MAX_ADDR - MIN_RANGE {
        rounddown(address + MIN_RANGE, granularity)
    } else {
        MAX_ADDR
    };

    while min < max {
        let mut mbi: MEMORY_BASIC_INFORMATION = unsafe { core::mem::zeroed() };
        let written = unsafe {
            VirtualQuery(
                min as *const c_void,
                &mut mbi,
                core::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            )
        };
        if written == 0 {
            return core::ptr::null_mut();
        }

        let base_addr = mbi.BaseAddress as usize;
        min = base_addr + mbi.RegionSize;

        if mbi.State == MEM_FREE {
            let addr = roundup(base_addr, granularity);
            if addr < min && (min - addr) >= size {
                let mem = unsafe {
                    VirtualAlloc(
                        addr as *const c_void,
                        size,
                        MEM_COMMIT | MEM_RESERVE,
                        PAGE_EXECUTE_READWRITE,
                    )
                };
                if !mem.is_null() {
                    return mem.cast();
                }
            }
        }
    }

    core::ptr::null_mut()
}

/// `SKSE::AllocTrampoline(size)`: reserves `size` bytes of branch-reachable
/// memory, preferring the SKSE branch pool.
pub unsafe fn commonlib_alloc_trampoline(size: usize) {
    if size == 0 {
        return;
    }

    let mut mem = core::ptr::null_mut::<u8>();
    let interface = unsafe { super::api::commonlib_skse_get_trampoline_interface() }
        .cast::<crate::skse::TrampolineInterface>();
    if let Some(interface) = unsafe { interface.as_ref() } {
        mem = interface.allocate_from_branch_pool(size).cast();
    }

    if mem.is_null() {
        let near = module::segment(Segment::Textx)
            .map(|(addr, len)| addr + len)
            .unwrap_or_else(module::base);
        mem = create_near(size, near);
    }

    if mem.is_null() {
        crate::skse::crash::raise_logged_runtime_error(format_args!(
            "Trampoline: failed to create a {size}B trampoline"
        ));
    }

    TRAMPOLINE.lock().set(mem, size);
}

/// `SKSE::GetTrampoline().allocate(size)`
pub unsafe fn commonlib_trampoline_allocate(size: usize) -> *mut u8 {
    TRAMPOLINE.lock().allocate(size)
}

#[inline(always)]
fn in_range(disp: isize) -> bool {
    disp >= i32::MIN as isize && disp <= i32::MAX as isize
}

/// Returns `src + N + disp32`, exactly like CommonLib's `write_call/write_branch`.
///
/// For `N == 5` this is the previous call/jump target. For `N == 6`
/// (`FF 15/25 [rip+disp]`) CommonLib returns the address of the pointer slot,
/// not the pointee; that behavior is preserved for compatibility.
unsafe fn original_target<const N: usize>(src: usize) -> usize {
    unsafe {
        let is_nop = core::ptr::read_unaligned(src as *const u8) == 0x90;
        if is_nop {
            return 0;
        }
        let disp = core::ptr::read_unaligned((src + N - 4) as *const i32);
        (src + N).wrapping_add_signed(disp as isize)
    }
}

unsafe fn write_5branch(src: usize, dst: usize, opcode: u8) {
    let mut trampoline = TRAMPOLINE.lock();
    let mem = match trampoline.branches5.get(&dst) {
        Some(&mem) => mem,
        None => {
            let mem = trampoline.allocate(14) as usize;
            trampoline.branches5.insert(dst, mem);
            mem
        }
    };
    drop(trampoline);

    let disp = (mem as isize).wrapping_sub((src + 5) as isize);
    if !in_range(disp) {
        crate::skse::crash::raise_logged_runtime_error(format_args!(
            "Trampoline: displacement is out of range"
        ));
    }

    // jmp qword ptr [rip+0]; dq dst
    let mut stub = [0u8; 14];
    stub[0] = 0xFF;
    stub[1] = 0x25;
    stub[6..14].copy_from_slice(&(dst as u64).to_le_bytes());
    unsafe { core::ptr::copy_nonoverlapping(stub.as_ptr(), mem as *mut u8, stub.len()) };

    let mut patch = [0u8; 5];
    patch[0] = opcode;
    patch[1..5].copy_from_slice(&(disp as i32).to_le_bytes());
    unsafe { commonlib_safe_write(src, patch.as_ptr(), patch.len()) };
}

unsafe fn write_6branch(src: usize, dst: usize, modrm: u8) {
    let mut trampoline = TRAMPOLINE.lock();
    let mem = match trampoline.branches6.get(&dst) {
        Some(&mem) => mem,
        None => {
            let mem = trampoline.allocate(8) as usize;
            trampoline.branches6.insert(dst, mem);
            mem
        }
    };
    drop(trampoline);

    let disp = (mem as isize).wrapping_sub((src + 6) as isize);
    if !in_range(disp) {
        crate::skse::crash::raise_logged_runtime_error(format_args!(
            "Trampoline: displacement is out of range"
        ));
    }

    unsafe { core::ptr::write_unaligned(mem as *mut usize, dst) };

    let mut patch = [0u8; 6];
    patch[0] = 0xFF;
    patch[1] = modrm;
    patch[2..6].copy_from_slice(&(disp as i32).to_le_bytes());
    unsafe { commonlib_safe_write(src, patch.as_ptr(), patch.len()) };
}

/// `SKSE::GetTrampoline().write_branch<5>(src, dst)`
pub unsafe fn commonlib_write_branch5(src: usize, dst: usize) -> usize {
    unsafe {
        let original = original_target::<5>(src);
        write_5branch(src, dst, 0xE9);
        original
    }
}

/// `SKSE::GetTrampoline().write_branch<6>(src, dst)`
pub unsafe fn commonlib_write_branch6(src: usize, dst: usize) -> usize {
    unsafe {
        let original = original_target::<6>(src);
        write_6branch(src, dst, 0x25);
        original
    }
}

/// `SKSE::GetTrampoline().write_call<5>(src, dst)`
pub unsafe fn commonlib_write_call5(src: usize, dst: usize) -> usize {
    unsafe {
        let original = original_target::<5>(src);
        write_5branch(src, dst, 0xE8);
        original
    }
}

/// `SKSE::GetTrampoline().write_call<6>(src, dst)`
pub unsafe fn commonlib_write_call6(src: usize, dst: usize) -> usize {
    unsafe {
        let original = original_target::<6>(src);
        write_6branch(src, dst, 0x15);
        original
    }
}

/// Function-entry detour. Formerly backed by MinHook; now uses the pure Rust
/// iced-x86 prologue relocator from [`crate::relocation`].
///
/// Returns the trampoline that calls the original function, or `0` on failure.
pub unsafe fn commonlib_write_function_hook_universal(target: usize, dst: usize) -> usize {
    crate::relocation::try_write_function_hook_auto(target, dst).unwrap_or(0)
}
