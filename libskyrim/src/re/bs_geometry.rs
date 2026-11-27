use core::ffi::CStr;

use crate::core_util::inherit;
use crate::offsets::offsets_nirtti::NiRTTI_BSGeometry;
use crate::offsets::offsets_rtti::RTTI_BSGeometry;
use crate::offsets::offsets_vtable::VTABLE_BSGeometry;
use crate::re::{
    BSGraphicsTriShape, BSGraphicsVertexDesc, BSLightingShaderProperty, BSMultiIndexTriShape,
    BSShaderProperty, BSSkinnedDecalTriShape, NiAVObject, NiAlphaProperty, NiBound, NiObject,
    NiPoint3, NiSkinInstance,
};
use crate::relocation::{RttiType, VariantID, VariantOffset};

/// C++ `RE::BSGeometry::Type`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BSGeometryType {
    Geometry = 0,
    Particles = 1,
    StripParticles = 2,
    TriShape = 3,
    DynamicTriShape = 4,
    MeshLodTriShape = 5,
    LodMultiIndexTriShape = 6,
    MultiIndexTriShape = 7,
    SubIndexTriShape = 8,
    SubIndexLandTriShape = 9,
    MultiStreamInstanceTriShape = 10,
    ParticleShaderDynamicTriShape = 11,
    Lines = 12,
    DynamicLines = 13,
    InstanceGroup = 14,
}

core_util::impl_enumset_type!(BSGeometryType => u8);
core_util::impl_enumset_type!(BSGeometryType => u32);

/// Normalized Rust-side return type for CommonLib's `REX::EnumSet<Type, u8/u32>`
/// `BSGeometry::GetType()` accessor.
pub type BSGeometryTypeSet = crate::rex::EnumSet<BSGeometryType, u32>;

/// C++ `RE::BSGeometry::MODEL_DATA` flat prefix.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BSGeometryModelData {
    pub model_bound: NiBound, // 00
}

const _: () = assert!(core::mem::size_of::<BSGeometryModelData>() == 0x10);

/// C++ `RE::BSGeometry::MODEL_DATA` VR layout.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BSGeometryModelDataVr {
    pub model_bound: NiBound, // 00
    pub unk148: NiPoint3,     // 10
    pub unk154: NiPoint3,     // 1C
}

const _: () = assert!(core::mem::size_of::<BSGeometryModelDataVr>() == 0x28);

/// C++ `RE::BSGeometry::GEOMETRY_RUNTIME_DATA`
#[repr(C)]
pub struct BSGeometryRuntimeData {
    pub alpha_property: crate::re::NiPointer<NiAlphaProperty>, // 00
    pub shader_property: crate::re::NiPointer<BSShaderProperty>, // 08
    pub skin_instance: crate::re::NiPointer<NiSkinInstance>,   // 10
    pub renderer_data: *mut BSGraphicsTriShape,                // 18
    pub unk140: *mut core::ffi::c_void,                        // 20
    pub vertex_desc: BSGraphicsVertexDesc,                     // 28
}

const _: () = assert!(core::mem::size_of::<BSGeometryRuntimeData>() == 0x30);

/// Cross-runtime honest prefix of C++ `RE::BSGeometry`.
#[repr(C)]
pub struct BSGeometry {
    pub base: NiAVObject, // 00
}

const _: () = assert!(core::mem::size_of::<BSGeometry>() == 0x110);

impl RttiType for BSGeometry {
    const RTTI: VariantID = RTTI_BSGeometry;
}

impl crate::re::ni_ref_object::NiRef for BSGeometry {
    #[inline(always)]
    fn inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn dec_ref(&self) {
        self.base.dec_ref();
    }
}

inherit!(BSGeometry : NiAVObject, base);

impl BSGeometry {
    pub const RTTI: VariantID = RTTI_BSGeometry;
    pub const NI_RTTI: VariantID = NiRTTI_BSGeometry;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BSGeometry;
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0x158, 0x158, 0x198);
    pub const MODEL_DATA_OFFSET: VariantOffset = VariantOffset::new(0x110, 0x110, 0x138);
    pub const GEOMETRY_RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x120, 0x120, 0x160);
    pub const TYPE_OFFSET: VariantOffset = VariantOffset::new(0x150, 0x150, 0x190);

    crate::runtime_data_accessor! {
        pub fn model_data() -> BSGeometryModelData {
            offset: Self::MODEL_DATA_OFFSET
        }
    }

    crate::vr_runtime_data_accessor! {
        pub fn model_data_vr() -> BSGeometryModelDataVr {
            vr: 0x138
        }
    }

    crate::runtime_data_accessor! {
        pub fn geometry_runtime_data() -> BSGeometryRuntimeData {
            offset: Self::GEOMETRY_RUNTIME_DATA_OFFSET
        }
    }

    #[inline(always)]
    pub fn type_bits(&self) -> u32 {
        // `BSGeometryTypeSet` is `REX::EnumSet<Type, std::uint8_t>` on every runtime.
        unsafe {
            *((self as *const _ as *const u8).add(Self::TYPE_OFFSET.offset()) as *const u8) as u32
        }
    }

    #[inline(always)]
    pub fn get_type(&self) -> BSGeometryTypeSet {
        BSGeometryTypeSet::from_underlying(self.type_bits())
    }

    #[inline(always)]
    pub fn model_bound(&self) -> &NiBound {
        &self.model_data().model_bound
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_AS_MULTI_INDEX_TRI_SHAPE: VariantOffset = VariantOffset::new_se_ae(0x35, 0x36);
        pub fn as_multi_index_tri_shape(&mut self) -> *mut BSMultiIndexTriShape
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_AS_SKINNED_DECAL_TRI_SHAPE: VariantOffset = VariantOffset::new_se_ae(0x36, 0x37);
        pub fn as_skinned_decal_tri_shape(&mut self) -> *mut BSSkinnedDecalTriShape
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_37: VariantOffset = VariantOffset::new_se_ae(0x37, 0x38);
        pub fn unk_37(&mut self)
    }

    #[inline]
    pub fn lighting_shader_prop_cast(&self) -> *mut BSLightingShaderProperty {
        let effect = self.geometry_runtime_data().shader_property.get();
        if effect.is_null() {
            return core::ptr::null_mut();
        }

        let rtti = unsafe { (*effect.cast::<NiObject>()).get_rtti() };
        if rtti.is_null() {
            return core::ptr::null_mut();
        }

        let name = unsafe { CStr::from_ptr((*rtti).get_name()) };
        if name.to_bytes() == b"BSLightingShaderProperty" {
            effect.cast::<BSLightingShaderProperty>()
        } else {
            core::ptr::null_mut()
        }
    }
}

impl AsRef<BSGeometry> for BSGeometry {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<BSGeometry> for BSGeometry {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub trait BSGeometryExt {
    fn model_data(&self) -> &BSGeometryModelData;
    fn model_data_vr(&self) -> &BSGeometryModelDataVr;
    fn geometry_runtime_data(&self) -> &BSGeometryRuntimeData;
    fn type_bits(&self) -> u32;
    fn get_type(&self) -> BSGeometryTypeSet;
    fn model_bound(&self) -> &NiBound;
    fn as_multi_index_tri_shape(&mut self) -> *mut BSMultiIndexTriShape;
    fn as_skinned_decal_tri_shape(&mut self) -> *mut BSSkinnedDecalTriShape;
    fn unk_37(&mut self);
    fn lighting_shader_prop_cast(&self) -> *mut BSLightingShaderProperty;
}

impl<T: AsRef<BSGeometry> + AsMut<BSGeometry>> BSGeometryExt for T {
    #[inline(always)]
    fn model_data(&self) -> &BSGeometryModelData {
        BSGeometry::model_data(self.as_ref())
    }

    #[inline(always)]
    fn model_data_vr(&self) -> &BSGeometryModelDataVr {
        BSGeometry::model_data_vr(self.as_ref())
    }

    #[inline(always)]
    fn geometry_runtime_data(&self) -> &BSGeometryRuntimeData {
        BSGeometry::geometry_runtime_data(self.as_ref())
    }

    #[inline(always)]
    fn type_bits(&self) -> u32 {
        BSGeometry::type_bits(self.as_ref())
    }

    #[inline(always)]
    fn get_type(&self) -> BSGeometryTypeSet {
        BSGeometry::get_type(self.as_ref())
    }

    #[inline(always)]
    fn model_bound(&self) -> &NiBound {
        BSGeometry::model_bound(self.as_ref())
    }

    #[inline(always)]
    fn as_multi_index_tri_shape(&mut self) -> *mut BSMultiIndexTriShape {
        BSGeometry::as_multi_index_tri_shape(self.as_mut())
    }

    #[inline(always)]
    fn as_skinned_decal_tri_shape(&mut self) -> *mut BSSkinnedDecalTriShape {
        BSGeometry::as_skinned_decal_tri_shape(self.as_mut())
    }

    #[inline(always)]
    fn unk_37(&mut self) {
        BSGeometry::unk_37(self.as_mut())
    }

    #[inline(always)]
    fn lighting_shader_prop_cast(&self) -> *mut BSLightingShaderProperty {
        BSGeometry::lighting_shader_prop_cast(self.as_ref())
    }
}
