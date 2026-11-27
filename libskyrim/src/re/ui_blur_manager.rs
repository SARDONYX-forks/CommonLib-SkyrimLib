use core::ffi::c_void;

use crate::offsets::offsets_rtti::RTTI_UIBlurManager;
use crate::offsets::offsets_vtable::VTABLE_UIBlurManager;
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::UIBlurManager`
///
/// Bases: `BSTEventSink<DefaultObjectsReadyEvent::Event>` (00) and the empty
/// `BSTSingletonSDM<UIBlurManager>` (08, one byte).
#[repr(C)]
pub struct UIBlurManager {
    pub vtable: *const usize, // 00 - BSTEventSink<DefaultObjectsReadyEvent::Event>
    pub singleton_sdm: u8,    // 08 - BSTSingletonSDM
    pub pad09: u8,            // 09
    pub pad0a: u16,           // 0A
    pub pad0c: u32,           // 0C
    // TODO: `TESImageSpaceModifier` pointer kept untyped until that RE type is needed here.
    pub blur_effect: *mut c_void, // 10
    pub blur_count: u16,          // 18
    pub pad1a: u16,               // 1A
    pub pad1c: u32,               // 1C
}

const _: () = assert!(core::mem::size_of::<UIBlurManager>() == 0x20);
const _: () = assert!(core::mem::offset_of!(UIBlurManager, blur_count) == 0x18);

impl RttiType for UIBlurManager {
    const RTTI: VariantID = RTTI_UIBlurManager;
}

impl UIBlurManager {
    pub const RTTI: VariantID = RTTI_UIBlurManager;
    pub const VTABLE: &'static [VariantID] = &VTABLE_UIBlurManager;

    crate::relocation_variable! {
        fn singleton() -> &'static *mut UIBlurManager => RelocationID::new(516871, 403350)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut UIBlurManager {
        *Self::singleton()
    }

    crate::relocation_func! {
        pub fn decrement_blur_count(&mut self) => RelocationID::new(51900, 52777)
    }

    crate::relocation_func! {
        pub fn increment_blur_count(&mut self) => RelocationID::new(51899, 52776)
    }
}
