use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ThirdPersonState;
use crate::offsets::offsets_vtable::VTABLE_ThirdPersonState;
use crate::re::{
    BSFixedString, NiAVObject, NiNode, NiPoint2, NiPoint3, NiQuaternion, PlayerInputHandler,
    RefHandle, TESCameraState,
};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct ThirdPersonStateData {
    pub third_person_camera_obj: *mut NiAVObject,
    pub third_person_fov_control: *mut NiNode,
    pub translation: NiPoint3,
    pub rotation: NiQuaternion,
    pub pos_offset_expected: NiPoint3,
    pub pos_offset_actual: NiPoint3,
    pub target_zoom_offset: f32,
    pub current_zoom_offset: f32,
    pub target_yaw: f32,
    pub current_yaw: f32,
    pub saved_zoom_offset: f32,
    pub pitch_zoom_offset: f32,
    pub unk8c: f32,
    pub collision_pos: NiPoint3,
    pub collision_pos_valid: f32,
    pub unk_a0: u64,
    pub animated_bone_name: BSFixedString,
    pub animation_rotation: NiQuaternion,
    pub unk_c0: u64,
    pub unk_c8: u64,
    pub unk_d0: u32,
    pub free_rotation: NiPoint2,
    pub free_rotation_enabled: bool,
    pub state_not_active: bool,
    pub unkde: u16,
    pub toggle_anim_cam: bool,
    pub apply_offsets: bool,
    pub unk_e2: u16,
    pub unk_e4: u32,
}

#[repr(C)]
pub struct ThirdPersonState {
    pub base: TESCameraState,
}

const _: () = assert!(core::mem::size_of::<ThirdPersonState>() == 0x20);

impl RttiType for ThirdPersonState {
    const RTTI: VariantID = RTTI_ThirdPersonState;
}

inherit!(ThirdPersonState : TESCameraState, base);

impl ThirdPersonState {
    pub const RTTI: VariantID = RTTI_ThirdPersonState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ThirdPersonState;
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
        pub fn data() -> ThirdPersonStateData {
            offset: Self::DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn data_mut() -> ThirdPersonStateData {
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
        pub const VFUNC_UPDATE_ROTATION: VariantOffset = VariantOffset::new_se_ae(0x0E, 0x0F);
        pub fn update_rotation(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_HANDLE_LOOK_INPUT: VariantOffset = VariantOffset::new_se_ae(0x0F, 0x10);
        pub fn handle_look_input(&mut self, input: &NiPoint2)
    }
}

pub trait ThirdPersonStateExt {
    fn player_input_handler(&self) -> &PlayerInputHandler;
    fn player_input_handler_mut(&mut self) -> &mut PlayerInputHandler;
    fn data(&self) -> &ThirdPersonStateData;
    fn data_mut(&mut self) -> &mut ThirdPersonStateData;
    fn unk_03(&mut self);
    fn set_camera_handle(&mut self, handle: &mut RefHandle);
    fn unk_0a(&mut self);
    fn process_weapon_drawn_change(&mut self, drawn: bool);
    fn get_free_rotation_mode(&self) -> bool;
    fn set_free_rotation_mode(&mut self, weapon_sheathed: bool);
    fn update_rotation(&mut self);
    fn handle_look_input(&mut self, input: &NiPoint2);
}

impl<T> ThirdPersonStateExt for T
where
    T: AsRef<ThirdPersonState> + AsMut<ThirdPersonState>,
{
    #[inline(always)]
    fn player_input_handler(&self) -> &PlayerInputHandler {
        ThirdPersonState::player_input_handler(self.as_ref())
    }

    #[inline(always)]
    fn player_input_handler_mut(&mut self) -> &mut PlayerInputHandler {
        ThirdPersonState::player_input_handler_mut(self.as_mut())
    }

    #[inline(always)]
    fn data(&self) -> &ThirdPersonStateData {
        ThirdPersonState::data(self.as_ref())
    }

    #[inline(always)]
    fn data_mut(&mut self) -> &mut ThirdPersonStateData {
        ThirdPersonState::data_mut(self.as_mut())
    }

    #[inline(always)]
    fn unk_03(&mut self) {
        ThirdPersonState::unk_03(self.as_mut())
    }

    #[inline(always)]
    fn set_camera_handle(&mut self, handle: &mut RefHandle) {
        ThirdPersonState::set_camera_handle(self.as_mut(), handle)
    }

    #[inline(always)]
    fn unk_0a(&mut self) {
        ThirdPersonState::unk_0a(self.as_mut())
    }

    #[inline(always)]
    fn process_weapon_drawn_change(&mut self, drawn: bool) {
        ThirdPersonState::process_weapon_drawn_change(self.as_mut(), drawn)
    }

    #[inline(always)]
    fn get_free_rotation_mode(&self) -> bool {
        ThirdPersonState::get_free_rotation_mode(self.as_ref())
    }

    #[inline(always)]
    fn set_free_rotation_mode(&mut self, weapon_sheathed: bool) {
        ThirdPersonState::set_free_rotation_mode(self.as_mut(), weapon_sheathed)
    }

    #[inline(always)]
    fn update_rotation(&mut self) {
        ThirdPersonState::update_rotation(self.as_mut())
    }

    #[inline(always)]
    fn handle_look_input(&mut self, input: &NiPoint2) {
        ThirdPersonState::handle_look_input(self.as_mut(), input)
    }
}
