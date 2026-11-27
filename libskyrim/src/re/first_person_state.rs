use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_FirstPersonState;
use crate::offsets::offsets_vtable::VTABLE_FirstPersonState;
use crate::re::{NiAVObject, NiNode, NiPoint3, PlayerInputHandler, TESCameraState};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct FirstPersonStateData {
    pub last_position: NiPoint3,
    pub last_frame_spring_velocity: NiPoint3,
    pub dampening_offset: NiPoint3,
    pub pad54: u32,
    pub first_person_camera_obj: *mut NiAVObject,
    pub first_person_fov_control: *mut NiNode,
    pub sitting_rotation: f32,
    pub unk6c: f32,
    pub unk70: f32,
    pub current_pitch_offset: f32,
    pub target_pitch_offset: f32,
    // The tail from object offset 0x7C differs between SE and AE; see
    // `FirstPersonStateSeRuntimeData` / `FirstPersonStateAeRuntimeData`.
}

/// C++ `RE::FirstPersonState::SE_RUNTIME_DATA` (SE / VR, at `0x7C`; `pack(4)`).
#[repr(C)]
pub struct FirstPersonStateSeRuntimeData {
    pub unk7c: f32,                  // 7C
    pub unk80: u32,                  // 80
    pub camera_override: bool,       // 84
    pub camera_pitch_override: bool, // 85
    pub unk86: u16,                  // 86
    pub unk88: [u32; 2],             // 88 (u64, tightly packed)
}

const _: () = assert!(core::mem::size_of::<FirstPersonStateSeRuntimeData>() == 0x14);

/// C++ `RE::FirstPersonState::AE_RUNTIME_DATA` (AE, at `0x7C`; `pack(4)`).
#[repr(C)]
pub struct FirstPersonStateAeRuntimeData {
    pub unk7c: u32,                  // 7C
    pub unk80: f32,                  // 80
    pub unk84: u32,                  // 84
    pub camera_override: bool,       // 88
    pub camera_pitch_override: bool, // 89
    pub unk8a: u16,                  // 8A
    pub pad8c: u32,                  // 8C
    pub unk90: [u32; 2],             // 90 (u64)
}

const _: () = assert!(core::mem::size_of::<FirstPersonStateAeRuntimeData>() == 0x1C);

#[repr(C)]
pub struct FirstPersonState {
    pub base: TESCameraState,
}

const _: () = assert!(core::mem::size_of::<FirstPersonState>() == 0x20);

impl RttiType for FirstPersonState {
    const RTTI: VariantID = RTTI_FirstPersonState;
}

inherit!(FirstPersonState : TESCameraState, base);

impl FirstPersonState {
    pub const RTTI: VariantID = RTTI_FirstPersonState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_FirstPersonState;
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

    crate::runtime_optional_data_accessor! {
        pub fn se_runtime_data() -> FirstPersonStateSeRuntimeData {
            se: 0x7C, ae: 0, vr: 0x7C
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn se_runtime_data_mut() -> FirstPersonStateSeRuntimeData {
            se: 0x7C, ae: 0, vr: 0x7C
        }
    }

    crate::runtime_optional_data_accessor! {
        pub fn ae_runtime_data() -> FirstPersonStateAeRuntimeData {
            se: 0, ae: 0x7C, vr: 0
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn ae_runtime_data_mut() -> FirstPersonStateAeRuntimeData {
            se: 0, ae: 0x7C, vr: 0
        }
    }

    crate::runtime_data_accessor! {
        pub fn data() -> FirstPersonStateData {
            offset: Self::DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn data_mut() -> FirstPersonStateData {
            offset: Self::DATA_OFFSET
        }
    }

    pub fn unk_03(&mut self) {
        if crate::runtime::is_vr() {
            crate::relocate_virtual!(
                extern "C" fn(*mut Self),
                self as *mut Self,
                VariantOffset::new_se_ae(0x0, 0x03)
            );
        }
    }
}
