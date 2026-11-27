use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_BleedoutCameraState;
use crate::offsets::offsets_vtable::VTABLE_BleedoutCameraState;
use crate::re::{BSSoundHandle, NiAVObject, NiMatrix3, ThirdPersonState};
use crate::relocation::{RttiType, VariantID, VariantOffset};

#[repr(C)]
pub struct BleedoutCameraStateData {
    pub rotation_mtx: NiMatrix3,
    pub zoom: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub rand_heading: f32,
    pub pad11c: u32,
    pub animated_bone: crate::re::NiPointer<NiAVObject>,
    pub active_sound: BSSoundHandle,
    pub use_current_heading: bool,
    pub pad135: u8,
    pub pad136: u16,
}

#[repr(C)]
pub struct BleedoutCameraState {
    pub base: ThirdPersonState,
}

const _: () = assert!(core::mem::size_of::<BleedoutCameraState>() == 0x20);

impl RttiType for BleedoutCameraState {
    const RTTI: VariantID = RTTI_BleedoutCameraState;
}

inherit!(BleedoutCameraState : ThirdPersonState, base);

impl BleedoutCameraState {
    pub const RTTI: VariantID = RTTI_BleedoutCameraState;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BleedoutCameraState;
    pub const DATA_OFFSET: VariantOffset = VariantOffset::new(0xE8, 0xE8, 0xE8);

    crate::runtime_data_accessor! {
        pub fn data() -> BleedoutCameraStateData {
            offset: Self::DATA_OFFSET
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn data_mut() -> BleedoutCameraStateData {
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
