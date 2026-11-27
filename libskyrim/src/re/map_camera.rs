use core_util::inherit;

use crate::offsets::offsets_rtti::{
    RTTI_MapCamera, RTTI_MapCameraStates__Exit, RTTI_MapCameraStates__Transition,
    RTTI_MapCameraStates__World,
};
use crate::offsets::offsets_vtable::{
    VTABLE_MapCamera, VTABLE_MapCameraStates__Exit, VTABLE_MapCameraStates__Transition,
    VTABLE_MapCameraStates__World,
};
use crate::re::{
    BSTSmartPointer, IMapCameraCallbacks, NiNode, NiPoint2, NiPoint3, TESCamera, TESCameraState,
    TESWorldSpace,
};
use crate::relocation::{RttiType, VariantID};

#[repr(C)]
pub struct MapCameraStatesExit {
    pub base: TESCameraState,
}

const _: () = assert!(core::mem::size_of::<MapCameraStatesExit>() == 0x20);

impl RttiType for MapCameraStatesExit {
    const RTTI: VariantID = RTTI_MapCameraStates__Exit;
}

inherit!(MapCameraStatesExit : TESCameraState, base);

impl crate::re::BSTSmartPointerIntrusiveRefCountable for MapCameraStatesExit {
    fn bst_inc_ref(&self) {
        self.base.bst_inc_ref();
    }
    fn bst_dec_ref(&self) -> u32 {
        self.base.bst_dec_ref()
    }
    unsafe fn bst_delete(&self) {
        unsafe { self.base.bst_delete() };
    }
}

impl MapCameraStatesExit {
    pub const RTTI: VariantID = RTTI_MapCameraStates__Exit;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MapCameraStates__Exit;
}

#[repr(C)]
pub struct MapCameraStatesTransition {
    pub base: TESCameraState,
    pub unk20: u32,
    pub unk24: u32,
    pub unk28: u32,
    pub unk2c: u32,
    pub unk30: u32,
    pub unk34: u32,
    pub unk38: u32,
    pub unk3c: u32,
    pub current_position: NiPoint3,
    pub zoom_destination: NiPoint3,
    pub unk58: u32,
    pub unk5c: u32,
    pub unk60: f32, // 60
    pub unk64: f32, // 64
    // Three ref-counted pointers (assigned with the BSTSmartPointer load/store
    // pair by the ctor); pointee types are not yet identified.
    pub unk68: *mut core::ffi::c_void, // 68
    pub unk70: *mut core::ffi::c_void, // 70
    pub unk78: *mut core::ffi::c_void, // 78
    pub unk80: u64,                    // 80
}

const _: () = assert!(core::mem::size_of::<MapCameraStatesTransition>() == 0x88);
const _: () = assert!(core::mem::offset_of!(MapCameraStatesTransition, current_position) == 0x40);
const _: () = assert!(core::mem::offset_of!(MapCameraStatesTransition, zoom_destination) == 0x4C);
const _: () = assert!(core::mem::offset_of!(MapCameraStatesTransition, unk68) == 0x68);

impl RttiType for MapCameraStatesTransition {
    const RTTI: VariantID = RTTI_MapCameraStates__Transition;
}

inherit!(MapCameraStatesTransition : TESCameraState, base);

impl crate::re::BSTSmartPointerIntrusiveRefCountable for MapCameraStatesTransition {
    fn bst_inc_ref(&self) {
        self.base.bst_inc_ref();
    }
    fn bst_dec_ref(&self) -> u32 {
        self.base.bst_dec_ref()
    }
    unsafe fn bst_delete(&self) {
        unsafe { self.base.bst_delete() };
    }
}

impl MapCameraStatesTransition {
    pub const RTTI: VariantID = RTTI_MapCameraStates__Transition;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MapCameraStates__Transition;
}

#[repr(C)]
pub struct MapCameraStatesWorldMapData {
    pub unk00: *mut core::ffi::c_void,
    pub unk08: *mut core::ffi::c_void,
    pub unk10: u32,
    pub unk14: u32,
    pub unk18: u32,
    pub minimum_coordinates: NiPoint2,
    pub maximum_coordinates: NiPoint2,
}

#[repr(C)]
pub struct MapCameraStatesWorld {
    pub base: TESCameraState,
    pub current_position: NiPoint3,
    pub current_position_scroll_offset: NiPoint3,
    pub unk38: NiPoint3,
    pub unk44: NiPoint3,
    pub unk50: u32,
    pub unk54: u32,
    pub multiplier_of_unk44: f32,
    pub unk5c: u32,
    pub unk60: u32,
    pub unk64: u32,
    pub map_data: *mut MapCameraStatesWorldMapData,
    pub unk70: bool,    // 70
    pub pad71: [u8; 7], // 71
}

const _: () = assert!(core::mem::size_of::<MapCameraStatesWorld>() == 0x78);
const _: () = assert!(core::mem::offset_of!(MapCameraStatesWorld, current_position) == 0x20);
const _: () =
    assert!(core::mem::offset_of!(MapCameraStatesWorld, current_position_scroll_offset) == 0x2C);
const _: () = assert!(core::mem::offset_of!(MapCameraStatesWorld, multiplier_of_unk44) == 0x58);
const _: () = assert!(core::mem::offset_of!(MapCameraStatesWorld, map_data) == 0x68);

impl RttiType for MapCameraStatesWorld {
    const RTTI: VariantID = RTTI_MapCameraStates__World;
}

inherit!(MapCameraStatesWorld : TESCameraState, base);

impl crate::re::BSTSmartPointerIntrusiveRefCountable for MapCameraStatesWorld {
    fn bst_inc_ref(&self) {
        self.base.bst_inc_ref();
    }
    fn bst_dec_ref(&self) -> u32 {
        self.base.bst_dec_ref()
    }
    unsafe fn bst_delete(&self) {
        unsafe { self.base.bst_delete() };
    }
}

impl MapCameraStatesWorld {
    pub const RTTI: VariantID = RTTI_MapCameraStates__World;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MapCameraStates__World;
}

#[repr(C)]
pub struct MapCamera {
    pub base: TESCamera,
    pub unk38: crate::re::BSTPoint2<f32>,
    pub unk40: crate::re::BSTPoint3<f32>,
    pub unk4c: u32,
    pub world_space: *mut TESWorldSpace,
    pub unk58: *mut IMapCameraCallbacks,
    pub unk60: u32,
    pub unk64: u32,
    pub unk68: [BSTSmartPointer<MapCameraStatesWorld>; 2],
    pub unk78: BSTSmartPointer<MapCameraStatesExit>,
    pub unk80: BSTSmartPointer<MapCameraStatesTransition>,
    pub unk88: u8,
    pub pad89: u8,
    pub pad8a: u16,
    pub pad8c: u32,
}

const _: () = assert!(core::mem::size_of::<MapCamera>() == 0x90);
const _: () = assert!(core::mem::offset_of!(MapCamera, unk38) == 0x38);
const _: () = assert!(core::mem::offset_of!(MapCamera, world_space) == 0x50);
const _: () = assert!(core::mem::offset_of!(MapCamera, unk58) == 0x58);
const _: () = assert!(core::mem::offset_of!(MapCamera, unk68) == 0x68);
const _: () = assert!(core::mem::offset_of!(MapCamera, unk78) == 0x78);
const _: () = assert!(core::mem::offset_of!(MapCamera, unk80) == 0x80);

impl RttiType for MapCamera {
    const RTTI: VariantID = RTTI_MapCamera;
}

inherit!(MapCamera : TESCamera, base);

impl MapCamera {
    pub const RTTI: VariantID = RTTI_MapCamera;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MapCamera;

    crate::virtual_method! {
        pub const VFUNC_SET_MAP_CAMERA_ROOT: usize = 0x03;
        pub fn set_map_camera_root(root: *mut NiNode, map_pos: &NiPoint3)
    }
}
