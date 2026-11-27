use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_HorseCameraState;
use crate::offsets::offsets_vtable::VTABLE_HorseCameraState;
use crate::re::{NiPoint2, ObjectRefHandle, RefHandle, ThirdPersonState};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct HorseCameraStateData {
    pub horse_ref_handle: ObjectRefHandle,
    pub horse_current_direction: f32,
    pub unkf0: u64,
}

#[repr(C)]
pub struct HorseCameraState {
    pub base: ThirdPersonState,
}

const _: () = assert!(core::mem::size_of::<HorseCameraState>() == 0x20);

impl RttiType for HorseCameraState {
    const RTTI: VariantID = RTTI_HorseCameraState;
}

inherit!(HorseCameraState : ThirdPersonState, base);

impl HorseCameraState {
    pub const RTTI: VariantID = RTTI_HorseCameraState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_HorseCameraState;
    pub const DATA_OFFSET: VariantOffset = VariantOffset::new(0xE8, 0xE8, 0xE8);

    crate::runtime_data_accessor! {
        pub fn data() -> HorseCameraStateData {
            offset: Self::DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn data_mut() -> HorseCameraStateData {
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

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_CAMERA_HANDLE: VariantOffset = VariantOffset::new_se_ae(0x09, 0x0A);
        pub fn set_camera_handle(&mut self, handle: &mut RefHandle)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_0A: VariantOffset = VariantOffset::new_se_ae(0x0A, 0x0B);
        pub fn unk_0a(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_PROCESS_WEAPON_DRAWN_CHANGE: VariantOffset = VariantOffset::new_se_ae(0x0B, 0x0C);
        pub fn process_weapon_drawn_change(&mut self, drawn: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_FREE_ROTATION_MODE: VariantOffset = VariantOffset::new_se_ae(0x0C, 0x0D);
        pub fn get_free_rotation_mode(&self) -> bool
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_SET_FREE_ROTATION_MODE: VariantOffset = VariantOffset::new_se_ae(0x0D, 0x0E);
        pub fn set_free_rotation_mode(&mut self, weapon_sheathed: bool)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_HANDLE_LOOK_INPUT: VariantOffset = VariantOffset::new_se_ae(0x0F, 0x10);
        pub fn handle_look_input(&mut self, input: &NiPoint2)
    }
}
