use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_FreeCameraState;
use crate::offsets::offsets_vtable::VTABLE_FreeCameraState;
use crate::re::{BSTPoint2, NiPoint3, PlayerInputHandler, TESCameraState};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct FreeCameraStateData {
    pub translation: NiPoint3,
    pub rotation: BSTPoint2<f32>,
    pub z_up_down: BSTPoint2<f32>,
    pub vertical_direction: i16,
    pub use_run_speed: bool,
    pub lock_to_z_plane: bool,
}

#[repr(C)]
pub struct FreeCameraState {
    pub base: TESCameraState,
}

const _: () = assert!(core::mem::size_of::<FreeCameraState>() == 0x20);

impl RttiType for FreeCameraState {
    const RTTI: VariantID = RTTI_FreeCameraState;
}

inherit!(FreeCameraState : TESCameraState, base);

impl FreeCameraState {
    pub const RTTI: VariantID = RTTI_FreeCameraState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FreeCameraState;
    pub const PLAYER_INPUT_HANDLER_OFFSET: VariantOffset = VariantOffset::new(0x20, 0x20, 0x20);
    pub const DATA_OFFSET: VariantOffset = VariantOffset::new(0x30, 0x30, 0x30);

    crate::runtime_cast_accessor! {
        pub fn player_input_handler() -> PlayerInputHandler {
            offset: Self::PLAYER_INPUT_HANDLER_OFFSET
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn player_input_handler_mut() -> PlayerInputHandler {
            offset: Self::PLAYER_INPUT_HANDLER_OFFSET
        }
    }

    crate::runtime_data_accessor! {
        pub fn data() -> FreeCameraStateData {
            offset: Self::DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn data_mut() -> FreeCameraStateData {
            offset: Self::DATA_OFFSET
        }
    }
}
