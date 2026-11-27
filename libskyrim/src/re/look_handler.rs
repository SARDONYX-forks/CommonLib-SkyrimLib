use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_LookHandler;
use crate::offsets::offsets_vtable::VTABLE_LookHandler;
use crate::re::{PlayerInputHandler, SixaxisEvent};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::LookHandler::AE1799_RUNTIME_DATA` (AE 1.7.99+, at `0x10`).
#[repr(C)]
pub struct LookHandlerAe1799RuntimeData {
    pub unk10: u32,                  // 10 - ctor default 5; consumer unknown
    pub pad14: u32,                  // 14
    pub sixaxis_event: SixaxisEvent, // 18
    pub unk_a8: u64,                 // A8
    pub unk_b0: i32,                 // B0 - ctor default -1
    pub unk_b4: u32,                 // B4 - ctor default 0
}

const _: () = assert!(core::mem::size_of::<LookHandlerAe1799RuntimeData>() == 0xA8);
const _: () = assert!(core::mem::offset_of!(LookHandlerAe1799RuntimeData, sixaxis_event) == 0x08);

/// C++ `RE::LookHandler::VR_RUNTIME_DATA` (VR only, at `0x10`).
#[repr(C)]
pub struct LookHandlerVrRuntimeData {
    pub unk10: u64,     // 10
    pub unk18: u8,      // 18
    pub pad19: [u8; 7], // 19
}

const _: () = assert!(core::mem::size_of::<LookHandlerVrRuntimeData>() == 0x10);

/// C++ `RE::LookHandler`. Honest common prefix; runtime-specific tails are
/// exposed through the accessors below.
#[repr(C)]
pub struct LookHandler {
    pub base: PlayerInputHandler,
}

const _: () = assert!(core::mem::size_of::<LookHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(LookHandler, base) == 0x00);

impl RttiType for LookHandler {
    const RTTI: VariantID = RTTI_LookHandler;
}

inherit!(LookHandler : PlayerInputHandler, base);

impl LookHandler {
    pub const RTTI: VariantID = RTTI_LookHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_LookHandler;

    crate::runtime_optional_data_accessor! {
        pub fn ae1799_runtime_data() -> LookHandlerAe1799RuntimeData {
            version: crate::version::RUNTIME_SSE_1_7_99, se: 0, ae: 0x10, vr: 0
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn ae1799_runtime_data_mut() -> LookHandlerAe1799RuntimeData {
            version: crate::version::RUNTIME_SSE_1_7_99, se: 0, ae: 0x10, vr: 0
        }
    }

    crate::runtime_optional_data_accessor! {
        pub fn vr_runtime_data() -> LookHandlerVrRuntimeData {
            se: 0, ae: 0, vr: 0x10
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn vr_runtime_data_mut() -> LookHandlerVrRuntimeData {
            se: 0, ae: 0, vr: 0x10
        }
    }
}
