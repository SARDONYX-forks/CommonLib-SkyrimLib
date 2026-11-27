use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ShoutHandler;
use crate::offsets::offsets_vtable::VTABLE_ShoutHandler;
use crate::re::PlayerInputHandler;
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct ShoutHandlerData {
    pub unk10: u64,
    pub unk18: u64,
}

const _: () = assert!(core::mem::size_of::<ShoutHandlerData>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ShoutHandlerData, unk10) == 0x00);
const _: () = assert!(core::mem::offset_of!(ShoutHandlerData, unk18) == 0x08);

#[repr(C)]
pub struct ShoutHandler {
    pub base: PlayerInputHandler,
}

const _: () = assert!(core::mem::size_of::<ShoutHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ShoutHandler, base) == 0x00);

impl RttiType for ShoutHandler {
    const RTTI: VariantID = RTTI_ShoutHandler;
}

inherit!(ShoutHandler : PlayerInputHandler, base);

impl ShoutHandler {
    pub const RTTI: VariantID = RTTI_ShoutHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ShoutHandler;
    pub const DATA_OFFSET: VariantOffset = VariantOffset::new(0x10, 0x10, 0x10);

    crate::runtime_data_accessor! {
        pub fn data() -> ShoutHandlerData {
            offset: Self::DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn data_mut() -> ShoutHandlerData {
            offset: Self::DATA_OFFSET
        }
    }
}
