use core::ffi::{c_char, c_void};

use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BShkbAnimationGraph;
use crate::offsets::offsets_vtable::VTABLE_BShkbAnimationGraph;
use crate::re::bst_smart_pointer::BSTSmartPointerIntrusiveRefCountable;
use crate::re::{
    Actor, BSAnimationGraphEvent, BSFadeNode, BSFixedString, BSIRagdollDriver,
    BSIntrusiveRefCounted, BSResourceID, BSTArray, BSTEventSink, BSTEventSource, BSTHashMap,
    BSTSmallArray, BSTransformDeltaEvent, BShkFloatController, NiNode, bhkWorld, hkbBehaviorGraph,
    hkbCharacter, hkbGeneratorOutput, hkpMotionMotionType,
};
use crate::relocation::{RelocationID, RttiType, VariantID};

/// C++ `RE::BShkbAnimationGraph::BoneNodeEntry`
#[repr(C)]
pub struct BShkbAnimationGraphBoneNodeEntry {
    pub node: *mut NiNode, // 00
    pub unk08: u32,        // 08
    pub unk0c: u32,        // 0C
}

const _: () = assert!(core::mem::size_of::<BShkbAnimationGraphBoneNodeEntry>() == 0x10);

/// C++ `RE::BShkbAnimationGraph::ProjectDBData`
#[repr(C)]
pub struct BShkbAnimationGraphProjectDBData {
    pub vtbl: *const usize,                   // 00
    pub pad08: [u8; 0x68],                    // 08
    pub unk70: BSTHashMap<*mut c_char, i32>,  // 70
    pub unk_a0: BSTHashMap<*mut c_char, i32>, // A0
    pub unk_d0: BSTArray<*mut c_char>,        // D0
    pub unk_e8: BSTArray<*mut c_char>,        // E8
}

const _: () = assert!(core::mem::size_of::<BShkbAnimationGraphProjectDBData>() == 0x100);
const _: () = assert!(core::mem::offset_of!(BShkbAnimationGraphProjectDBData, unk_a0) == 0xA0);

/// C++ `RE::BShkbAnimationGraph`
#[repr(C)]
pub struct BShkbAnimationGraph {
    pub ragdoll_driver: BSIRagdollDriver,             // 000
    pub intrusive_ref_counted: BSIntrusiveRefCounted, // 008
    pub pad0c: u32,                                   // 00C
    pub transform_delta_event_source: BSTEventSource<BSTransformDeltaEvent>, // 010
    pub animation_graph_event_source: BSTEventSource<BSAnimationGraphEvent>, // 068
    pub character_instance: hkbCharacter,             // 0C0
    pub bone_nodes: BSTArray<BShkbAnimationGraphBoneNodeEntry>, // 160
    pub fade_controllers: BSTArray<*mut BShkFloatController>, // 178
    pub unk190: BSTArray<*mut c_void>,                // 190
    pub unk1a8: BSTSmallArray<*mut c_void, 8>,        // 1A8
    pub unk1c0: BSTSmallArray<u8, 1>,                 // 1C0
    pub unk1d8: u64,                                  // 1D8
    pub unk1e0: u64,                                  // 1E0
    pub interpolation_time_offsets: [f32; 2],         // 1E8
    pub project_name: BSFixedString,                  // 1F0
    pub unk1f8: *mut BSResourceID,                    // 1F8
    pub project_db_data: *mut BShkbAnimationGraphProjectDBData, // 200
    pub behavior_graph: *mut hkbBehaviorGraph,        // 208
    pub holder: *mut Actor,                           // 210
    pub root_node: *mut BSFadeNode,                   // 218
    pub generator_outputs: [*mut hkbGeneratorOutput; 2], // 220
    pub interpolation_amounts: [f32; 2],              // 230
    pub physics_world: *mut bhkWorld,                 // 238
    pub num_anim_bones: u16,                          // 240
    pub unk242: u8,                                   // 242
    pub unk243: u8,                                   // 243
    pub unk244: u16,                                  // 244
    pub unk246: u8,                                   // 246
    pub unk247: u8,                                   // 247
    pub unk248: u8,                                   // 248
    pub do_foot_ik: u8,                               // 249
    pub unk24a: u16,                                  // 24A
    pub unk24c: u32,                                  // 24C
}

const _: () = assert!(core::mem::size_of::<BShkbAnimationGraph>() == 0x250);
const _: () = assert!(core::mem::offset_of!(BShkbAnimationGraph, ragdoll_driver) == 0x000);
const _: () = assert!(core::mem::offset_of!(BShkbAnimationGraph, intrusive_ref_counted) == 0x008);
const _: () =
    assert!(core::mem::offset_of!(BShkbAnimationGraph, transform_delta_event_source) == 0x010);
const _: () =
    assert!(core::mem::offset_of!(BShkbAnimationGraph, animation_graph_event_source) == 0x068);
const _: () = assert!(core::mem::offset_of!(BShkbAnimationGraph, character_instance) == 0x0C0);
const _: () = assert!(core::mem::offset_of!(BShkbAnimationGraph, holder) == 0x210);
const _: () = assert!(core::mem::offset_of!(BShkbAnimationGraph, physics_world) == 0x238);

impl RttiType for BShkbAnimationGraph {
    const RTTI: VariantID = RTTI_BShkbAnimationGraph;
}

inherit!(BShkbAnimationGraph : BSIRagdollDriver, ragdoll_driver);

impl AsRef<BShkbAnimationGraph> for BShkbAnimationGraph {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BShkbAnimationGraph> for BShkbAnimationGraph {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl BSTSmartPointerIntrusiveRefCountable for BShkbAnimationGraph {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.intrusive_ref_counted.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.intrusive_ref_counted.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let func: extern "C" fn(*mut Self) =
            unsafe { crate::relocation::virtual_function(self as *const Self, 0usize) };
        func(self as *const Self as *mut Self);
    }
}

impl BShkbAnimationGraph {
    pub const RTTI: VariantID = RTTI_BShkbAnimationGraph;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BShkbAnimationGraph;

    // ~BShkbAnimationGraph() override;  // 00

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }

    // override (BSIRagdollDriver)
    crate::virtual_method! {
        pub const VFUNC_HAS_RAGDOLL: usize = 0x01;
        pub fn has_ragdoll() -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_ADD_RAGDOLL_TO_WORLD: usize = 0x02;
        pub fn add_ragdoll_to_world() -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_REMOVE_RAGDOLL_FROM_WORLD: usize = 0x03;
        pub fn remove_ragdoll_from_world() -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_SET_WORLD: usize = 0x04;
        pub fn set_world(&mut self, world: *mut bhkWorld)
    }

    crate::virtual_method! {
        pub const VFUNC_RESET_RAGDOLL: usize = 0x05;
        pub fn reset_ragdoll(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_06: usize = 0x06;
        pub fn unk_06(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_RAGDOLL_CONSTRAINTS_FROM_BHK_CONSTRAINTS: usize = 0x07;
        pub fn set_ragdoll_constraints_from_bhk_constraints(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_SET_MOTION_TYPE: usize = 0x08;
        pub fn set_motion_type(&mut self, motion_type: hkpMotionMotionType)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_09: usize = 0x09;
        pub fn unk_09(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_TOGGLE_SYNC_ON_UPDATE: usize = 0x0A;
        pub fn toggle_sync_on_update(&mut self, disable: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0B: usize = 0x0B;
        pub fn unk_0b(&mut self)
    }

    crate::virtual_method! {
        pub const VFUNC_TOGGLE_CONSTRAINTS: usize = 0x0C;
        pub fn toggle_constraints(&mut self, disable: bool)
    }

    crate::virtual_method! {
        pub const VFUNC_UNK_0D: usize = 0x0D;
        pub fn unk_0d(&mut self)
    }

    #[inline(always)]
    pub fn transform_delta_event_source(&self) -> &BSTEventSource<BSTransformDeltaEvent> {
        &self.transform_delta_event_source
    }

    #[inline(always)]
    pub fn transform_delta_event_source_mut(
        &mut self,
    ) -> &mut BSTEventSource<BSTransformDeltaEvent> {
        &mut self.transform_delta_event_source
    }

    #[inline(always)]
    pub fn animation_graph_event_source(&self) -> &BSTEventSource<BSAnimationGraphEvent> {
        &self.animation_graph_event_source
    }

    #[inline(always)]
    pub fn animation_graph_event_source_mut(
        &mut self,
    ) -> &mut BSTEventSource<BSAnimationGraphEvent> {
        &mut self.animation_graph_event_source
    }

    #[inline(always)]
    pub unsafe fn add_transform_delta_event_sink(
        &mut self,
        event_sink: *mut BSTEventSink<BSTransformDeltaEvent>,
    ) {
        unsafe { self.transform_delta_event_source.add_event_sink(event_sink) };
    }

    #[inline(always)]
    pub unsafe fn remove_transform_delta_event_sink(
        &mut self,
        event_sink: *mut BSTEventSink<BSTransformDeltaEvent>,
    ) {
        unsafe {
            self.transform_delta_event_source
                .remove_event_sink(event_sink)
        };
    }

    #[inline(always)]
    pub unsafe fn add_animation_graph_event_sink(
        &mut self,
        event_sink: *mut BSTEventSink<BSAnimationGraphEvent>,
    ) {
        unsafe { self.animation_graph_event_source.add_event_sink(event_sink) };
    }

    #[inline(always)]
    pub unsafe fn remove_animation_graph_event_sink(
        &mut self,
        event_sink: *mut BSTEventSink<BSAnimationGraphEvent>,
    ) {
        unsafe {
            self.animation_graph_event_source
                .remove_event_sink(event_sink)
        };
    }

    crate::relocation_func! {
        pub fn get_graph_variable_bool(
            &self,
            variable_name: &BSFixedString,
            out: &mut bool
        ) -> bool => RelocationID::new(62696, 63613)
    }

    crate::relocation_func! {
        pub fn get_graph_variable_float(
            &self,
            variable_name: &BSFixedString,
            out: &mut f32
        ) -> bool => RelocationID::new(62695, 63614)
    }

    crate::relocation_func! {
        pub fn get_graph_variable_int(
            &self,
            variable_name: &BSFixedString,
            out: &mut i32
        ) -> bool => RelocationID::new(62694, 63615)
    }

    crate::relocation_func! {
        pub fn set_graph_variable_bool(
            &mut self,
            variable_name: &BSFixedString,
            input: bool
        ) -> bool => RelocationID::new(62710, 63607)
    }

    crate::relocation_func! {
        pub fn set_graph_variable_float(
            &mut self,
            variable_name: &BSFixedString,
            input: f32
        ) -> bool => RelocationID::new(62709, 63608)
    }

    crate::relocation_func! {
        pub fn set_graph_variable_int(
            &mut self,
            variable_name: &BSFixedString,
            input: i32
        ) -> bool => RelocationID::new(62708, 63609)
    }
}
