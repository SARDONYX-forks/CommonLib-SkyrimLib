use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ActivateHandler;
use crate::offsets::offsets_vtable::VTABLE_ActivateHandler;
use crate::re::HeldStateHandler;
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct ActivateHandlerData {
    pub unk18: u8,
    pub unk19: u8,
    pub held_button_action_success: bool,
    pub disabled: bool,
    pub unk1c: u32,
}

const _: () = assert!(core::mem::size_of::<ActivateHandlerData>() == 0x08);
const _: () = assert!(core::mem::offset_of!(ActivateHandlerData, unk18) == 0x00);
const _: () = assert!(core::mem::offset_of!(ActivateHandlerData, unk19) == 0x01);
const _: () =
    assert!(core::mem::offset_of!(ActivateHandlerData, held_button_action_success) == 0x02);
const _: () = assert!(core::mem::offset_of!(ActivateHandlerData, disabled) == 0x03);
const _: () = assert!(core::mem::offset_of!(ActivateHandlerData, unk1c) == 0x04);

#[repr(C)]
pub struct ActivateHandler {
    pub base: HeldStateHandler,
}

const _: () = assert!(core::mem::size_of::<ActivateHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ActivateHandler, base) == 0x00);

impl RttiType for ActivateHandler {
    const RTTI: VariantID = RTTI_ActivateHandler;
}

inherit!(ActivateHandler : HeldStateHandler, base);

impl ActivateHandler {
    pub const RTTI: VariantID = RTTI_ActivateHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ActivateHandler;
    pub const DATA_OFFSET: VariantOffset = VariantOffset::new(0x18, 0x18, 0x60);

    crate::runtime_data_accessor! {
        pub fn data() -> ActivateHandlerData {
            offset: Self::DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn data_mut() -> ActivateHandlerData {
            offset: Self::DATA_OFFSET
        }
    }

    #[inline(always)]
    pub fn set_held_button_action_success(&mut self, success: bool) {
        self.data_mut().held_button_action_success = success;
    }
}
