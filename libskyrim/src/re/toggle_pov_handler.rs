use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_TogglePOVHandler;
use crate::offsets::offsets_vtable::VTABLE_TogglePOVHandler;
use crate::re::HeldStateHandler;
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct TogglePOVHandlerData {
    pub press_registered: bool,
    pub pad19: u8,
    pub pad1a: u16,
    pub pad1c: u32,
}

const _: () = assert!(core::mem::size_of::<TogglePOVHandlerData>() == 0x08);
const _: () = assert!(core::mem::offset_of!(TogglePOVHandlerData, press_registered) == 0x00);
const _: () = assert!(core::mem::offset_of!(TogglePOVHandlerData, pad1c) == 0x04);

#[repr(C)]
pub struct TogglePOVHandler {
    pub base: HeldStateHandler,
}

const _: () = assert!(core::mem::size_of::<TogglePOVHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TogglePOVHandler, base) == 0x00);

impl RttiType for TogglePOVHandler {
    const RTTI: VariantID = RTTI_TogglePOVHandler;
}

inherit!(TogglePOVHandler : HeldStateHandler, base);

impl TogglePOVHandler {
    pub const RTTI: VariantID = RTTI_TogglePOVHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TogglePOVHandler;
    pub const DATA_OFFSET: VariantOffset = VariantOffset::new(0x18, 0x18, 0x60);

    crate::runtime_data_accessor! {
        pub fn data() -> TogglePOVHandlerData {
            offset: Self::DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn data_mut() -> TogglePOVHandlerData {
            offset: Self::DATA_OFFSET
        }
    }
}
