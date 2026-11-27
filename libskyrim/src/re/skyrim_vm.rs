use crate::re::IVirtualMachine;
use crate::relocation::RelocationID;

/// Minimal source-backed `RE::SkyrimVM` prefix.
///
/// `VM_RUNTIME_DATA` (starting with `impl`) lives at `0x200` on SE / AE < 1.7.99
/// / VR and at `0x210` on AE 1.7.99+ (two Amiibo event-sink bases were inserted).
/// Always go through [`SkyrimVM::get_virtual_machine`].
///
/// TODO: Expand this into a fuller `SkyrimVM` translation (event sinks,
/// `VM_RUNTIME_DATA`, `RUNTIME_DATA(2)`) when needed beyond `IVirtualMachine`.
#[repr(C)]
pub struct SkyrimVM {
    pub pad00: [u8; 0x200],
}

impl SkyrimVM {
    /// AE 1.7.99 base-class shift.
    pub const AE1799_AMIIBO_SHIFT: usize = 0x10;

    crate::relocation_variable! {
        fn singleton_ptr() -> &'static *mut SkyrimVM => RelocationID::new(514315, 400475)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut SkyrimVM {
        *Self::singleton_ptr()
    }

    /// Offset of `VM_RUNTIME_DATA` for the running game.
    #[inline(always)]
    pub fn vm_runtime_data_offset() -> usize {
        if crate::runtime::is_ae()
            && crate::runtime::is_at_least(crate::version::RUNTIME_SSE_1_7_99)
        {
            0x200 + Self::AE1799_AMIIBO_SHIFT
        } else {
            0x200
        }
    }

    /// `GetVMRuntimeData().impl`
    #[inline(always)]
    pub fn get_virtual_machine(&self) -> *mut IVirtualMachine {
        unsafe {
            *(core::ptr::from_ref(self)
                .cast::<u8>()
                .add(Self::vm_runtime_data_offset()) as *const *mut IVirtualMachine)
        }
    }

    crate::relocation_func! {
        // Blocks the calling thread, pumping `ProcessRegisteredUpdates`, until
        // the VM reports frozen or a timeout.
        pub fn freeze(&mut self) => RelocationID::new(53204, 54015)
    }
}
