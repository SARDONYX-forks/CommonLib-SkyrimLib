use core_util::inherit;

use crate::re::{IVirtualMachine, SkyrimVM};

/// Minimal source-backed prefix of `RE::BSScript::Internal::VirtualMachine`.
///
/// CommonLib uses this type in `SKSE::PapyrusInterface` immediate-registration
/// helpers by calling `VirtualMachine::GetSingleton()` and then passing either
/// the full internal VM pointer or its `IVirtualMachine` base to callbacks.
///
/// TODO: Expand this into a fuller `VirtualMachine` translation when SKSE or
/// RE code needs the moved mixin bases, event-source surface, or named fields
/// beyond singleton access and `IVirtualMachine` upcasts.
#[repr(C)]
pub struct VirtualMachine {
    pub base: IVirtualMachine,
}

const _: () = assert!(core::mem::offset_of!(VirtualMachine, base) == 0x0);

inherit!(VirtualMachine : IVirtualMachine);

impl VirtualMachine {
    #[inline(always)]
    pub fn get_singleton() -> *mut VirtualMachine {
        let vm = SkyrimVM::get_singleton();
        if vm.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*vm).get_virtual_machine().cast() }
        }
    }
}
