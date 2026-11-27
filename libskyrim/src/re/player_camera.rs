use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_PlayerCamera;
use crate::offsets::offsets_vtable::VTABLE_PlayerCamera;
use crate::re::{
    ActorHandle, BSSpinLock, BSTSingletonSDM, BSTSmallArray, BSTSmartPointer, NiPoint3, TESCamera,
    TESCameraState, bhkRigidBody, bhkSimpleShapePhantom,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CameraState(pub u32);

impl CameraState {
    pub const FIRST_PERSON: Self = Self(0);
    pub const AUTO_VANITY: Self = Self(1);
    pub const VATS: Self = Self(2);
    pub const FREE: Self = Self(3);
    pub const IRON_SIGHTS: Self = Self(4);
    pub const FURNITURE: Self = Self(5);
    pub const PC_TRANSITION: Self = Self(6);
    pub const TWEEN: Self = Self(7);
    pub const ANIMATED: Self = Self(8);
    pub const THIRD_PERSON: Self = Self(9);
    pub const MOUNT: Self = Self(10);
    pub const BLEEDOUT: Self = Self(11);
    pub const DRAGON: Self = Self(12);
    pub const VR: Self = Self(9);
    pub const VR_THIRD_PERSON: Self = Self(10);
    pub const VR_MOUNT: Self = Self(11);
    pub const VR_BLEEDOUT: Self = Self(12);
    pub const VR_DRAGON: Self = Self(13);

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

pub struct CameraStates;
impl CameraStates {
    pub const TOTAL: usize = 13;
    pub const VR_TOTAL: usize = 14;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PlayerCameraUnk120 {
    // TODO: CommonLib only reaches `bhkSimpleShapePhantom` through forward
    // declarations in this camera layer. Restore the original smart-pointer
    // field once the phantom type has a concrete same-name translation.
    pub unk00: *mut bhkSimpleShapePhantom,
    pub unk08: *mut bhkSimpleShapePhantom,
}

const _: () = assert!(core::mem::size_of::<PlayerCameraUnk120>() == 0x10);

#[repr(C)]
pub struct PlayerCameraRuntimeData {
    pub temp_return_states: BSTSmallArray<
        *mut TESCameraState,
        { CameraStates::TOTAL * core::mem::size_of::<*mut TESCameraState>() },
    >,
    pub camera_states: [BSTSmartPointer<TESCameraState>; CameraStates::TOTAL],
    pub unk120: *mut PlayerCameraUnk120,
    pub rigid_body: crate::re::NiPointer<bhkRigidBody>,
    pub object_fade_handle: u32,
    pub lock: BSSpinLock,
}

const _: () = assert!(core::mem::size_of::<PlayerCameraRuntimeData>() == 0x100);
const _: () = assert!(core::mem::offset_of!(PlayerCameraRuntimeData, camera_states) == 0x78);
const _: () = assert!(core::mem::offset_of!(PlayerCameraRuntimeData, rigid_body) == 0xE8);
const _: () = assert!(core::mem::offset_of!(PlayerCameraRuntimeData, object_fade_handle) == 0xF0);
const _: () = assert!(core::mem::offset_of!(PlayerCameraRuntimeData, lock) == 0xF4);

#[repr(C)]
pub struct PlayerCameraVRRuntimeData {
    pub temp_return_states: BSTSmallArray<
        *mut TESCameraState,
        { CameraStates::VR_TOTAL * core::mem::size_of::<*mut TESCameraState>() },
    >,
    pub camera_states: [BSTSmartPointer<TESCameraState>; CameraStates::VR_TOTAL], // 80 (C0)
    // TODO: `PlayerCamera::Unk120` is not translated; raw pointer for now.
    pub unk120: *mut core::ffi::c_void,                 // F0 (130)
    pub rigid_body: crate::re::NiPointer<bhkRigidBody>, // F8 (138)
    pub object_fade_handle: u32,                        // 100 (140)
    pub unk144: u32,                                    // 104 (144) - default 0xFFFFFFFF
    pub unk148: bool,                                   // 108 (148)
    pub pad149: [u8; 3],                                // 109
    pub lock: BSSpinLock,                               // 10C (14C)
    pub unk154: u32,                                    // 114 (154)
}

const _: () = assert!(core::mem::size_of::<PlayerCameraVRRuntimeData>() == 0x118);
const _: () = assert!(core::mem::offset_of!(PlayerCameraVRRuntimeData, camera_states) == 0x80);
const _: () = assert!(core::mem::offset_of!(PlayerCameraVRRuntimeData, unk120) == 0xF0);
const _: () = assert!(core::mem::offset_of!(PlayerCameraVRRuntimeData, rigid_body) == 0xF8);
const _: () =
    assert!(core::mem::offset_of!(PlayerCameraVRRuntimeData, object_fade_handle) == 0x100);
const _: () = assert!(core::mem::offset_of!(PlayerCameraVRRuntimeData, lock) == 0x10C);
const _: () = assert!(core::mem::offset_of!(PlayerCameraVRRuntimeData, unk154) == 0x114);

#[repr(C)]
pub struct PlayerCameraRuntimeData2 {
    pub world_fov: f32,
    pub first_person_fov: f32,
    pub pos: NiPoint3,
    pub idle_timer: f32,
    pub yaw: f32,
    pub unk158: u32,
    pub unk15c: u32,
    pub allow_auto_vanity_mode: bool,
    pub bow_zoomed_in: bool,
    pub is_weap_sheathed: bool,
    pub is_processed: bool,
    pub unk164: u8,
    pub unk165: u8,
    pub pad166: u16,
}

const _: () = assert!(core::mem::size_of::<PlayerCameraRuntimeData2>() == 0x2C);
const _: () = assert!(core::mem::offset_of!(PlayerCameraRuntimeData2, world_fov) == 0x00);
const _: () = assert!(core::mem::offset_of!(PlayerCameraRuntimeData2, first_person_fov) == 0x04);
const _: () = assert!(core::mem::offset_of!(PlayerCameraRuntimeData2, pos) == 0x08);
const _: () = assert!(core::mem::offset_of!(PlayerCameraRuntimeData2, idle_timer) == 0x14);
const _: () = assert!(core::mem::offset_of!(PlayerCameraRuntimeData2, yaw) == 0x18);
const _: () =
    assert!(core::mem::offset_of!(PlayerCameraRuntimeData2, allow_auto_vanity_mode) == 0x24);
const _: () = assert!(core::mem::offset_of!(PlayerCameraRuntimeData2, bow_zoomed_in) == 0x25);
const _: () = assert!(core::mem::offset_of!(PlayerCameraRuntimeData2, is_weap_sheathed) == 0x26);
const _: () = assert!(core::mem::offset_of!(PlayerCameraRuntimeData2, is_processed) == 0x27);

#[repr(C)]
pub struct PlayerCamera {
    pub base: TESCamera,
    pub singleton: BSTSingletonSDM<PlayerCamera>,
    pub pad39: u8,
    pub pad3a: u16,
    pub camera_target: ActorHandle,
}

const _: () = assert!(core::mem::size_of::<PlayerCamera>() == 0x40);
const _: () = assert!(core::mem::offset_of!(PlayerCamera, singleton) == 0x38);
const _: () = assert!(core::mem::offset_of!(PlayerCamera, camera_target) == 0x3C);

impl RttiType for PlayerCamera {
    const RTTI: VariantID = RTTI_PlayerCamera;
}

inherit!(PlayerCamera : TESCamera, base);
inherit!(PlayerCamera => BSTSingletonSDM<PlayerCamera>, singleton);

impl PlayerCamera {
    pub const RTTI: VariantID = RTTI_PlayerCamera;
    pub const VTABLE: &'static [VariantID] = &VTABLE_PlayerCamera;

    crate::runtime_data_accessor! {
        pub fn runtime_data() -> PlayerCameraRuntimeData {
            se: 0x40, ae: 0x40, vr: 0x0
        }
    }

    crate::runtime_data_accessor! {
        pub fn vr_runtime_data() -> PlayerCameraVRRuntimeData {
            se: 0x0, ae: 0x0, vr: 0x40
        }
    }

    crate::runtime_data_accessor! {
        pub fn runtime_data2() -> PlayerCameraRuntimeData2 {
            se: 0x13C, ae: 0x13C, vr: 0x158
        }
    }

    crate::relocation_variable! {
        fn singleton_ptr() -> &'static *mut PlayerCamera => RelocationID::new(514642, 400802)
    }

    crate::relocation_func! {
        fn force_first_person_impl(this: *mut PlayerCamera) => RelocationID::new(49858, 50790)
    }
    crate::relocation_func! {
        fn force_third_person_impl(this: *mut PlayerCamera) => RelocationID::new(49863, 50796)
    }
    crate::relocation_func! {
        fn push_camera_state_impl(this: *mut PlayerCamera, state: CameraState) => RelocationID::new(49947, 50880)
    }
    crate::relocation_func! {
        fn toggle_free_camera_mode_impl(this: *mut PlayerCamera, freeze_time: bool) => RelocationID::new(49876, 50809)
    }
    crate::relocation_func! {
        fn update_impl(this: *mut PlayerCamera) => RelocationID::new(49852, 50784)
    }
    crate::relocation_func! {
        fn update_third_person_impl(this: *mut PlayerCamera, weapon_drawn: bool) => RelocationID::new(49908, 50841)
    }

    pub fn get_singleton() -> *mut PlayerCamera {
        *Self::singleton_ptr()
    }

    pub fn force_first_person(&mut self) -> bool {
        if crate::runtime::is_vr() {
            false
        } else {
            Self::force_first_person_impl(self);
            true
        }
    }

    pub fn force_third_person(&mut self) -> bool {
        if crate::runtime::is_vr() {
            false
        } else {
            Self::force_third_person_impl(self);
            true
        }
    }

    pub fn is_in_bleedout_mode(&self) -> bool {
        self.q_camera_equals(if crate::runtime::is_vr() {
            CameraState::VR_BLEEDOUT
        } else {
            CameraState::BLEEDOUT
        })
    }
    pub fn is_in_first_person(&self) -> bool {
        self.q_camera_equals(CameraState::FIRST_PERSON)
    }
    pub fn is_in_free_camera_mode(&self) -> bool {
        self.q_camera_equals(CameraState::FREE)
    }
    pub fn is_in_third_person(&self) -> bool {
        self.q_camera_equals(if crate::runtime::is_vr() {
            CameraState::VR_THIRD_PERSON
        } else {
            CameraState::THIRD_PERSON
        })
    }
    pub fn push_camera_state(&mut self, state: CameraState) {
        Self::push_camera_state_impl(self, state)
    }
    pub fn toggle_free_camera_mode(&mut self, freeze_time: bool) {
        Self::toggle_free_camera_mode_impl(self, freeze_time)
    }
    pub fn update(&mut self) {
        Self::update_impl(self)
    }
    pub fn update_third_person(&mut self, weapon_drawn: bool) {
        Self::update_third_person_impl(self, weapon_drawn)
    }

    fn q_camera_equals(&self, camera_state: CameraState) -> bool {
        if self.base.current_state.is_null() {
            return false;
        }
        let index = camera_state.index();
        if crate::runtime::is_vr() {
            index < CameraStates::VR_TOTAL
                && self.base.current_state == self.vr_runtime_data().camera_states[index]
        } else {
            index < CameraStates::TOTAL
                && self.base.current_state == self.runtime_data().camera_states[index]
        }
    }
}
