use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_AttackBlockHandler;
use crate::offsets::offsets_vtable::VTABLE_AttackBlockHandler;
use crate::re::{BSFixedString, HeldStateHandler};
use crate::relocation::{RttiType, VariantID};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttackType {
    Right = 0,
    Left = 1,
    Dual = 2,
}

#[repr(C)]
pub struct AttackBlockHandlerData {
    pub held_time_ms: u32,
    pub unk1c: u32,
    pub control_id: BSFixedString,
    pub attack_type: AttackType,
    pub pad29: u8,
    pub pad2a: u16,
    pub attack_count: u8,
    pub pad2d: [u8; 3],
    pub initial_power_attack_delay: f32,
    pub pad34: u32,
    pub subsequent_power_attack_delay: f32,
    pub ignore: bool,
    pub unk41: bool,
    pub held_left: bool,
    pub held_right: bool,
    pub unk44: u32,
}

const _: () = assert!(core::mem::size_of::<AttackBlockHandlerData>() == 0x30);
const _: () = assert!(core::mem::offset_of!(AttackBlockHandlerData, held_time_ms) == 0x00);
const _: () = assert!(core::mem::offset_of!(AttackBlockHandlerData, unk1c) == 0x04);
const _: () = assert!(core::mem::offset_of!(AttackBlockHandlerData, control_id) == 0x08);
const _: () = assert!(core::mem::offset_of!(AttackBlockHandlerData, attack_type) == 0x10);
const _: () = assert!(core::mem::offset_of!(AttackBlockHandlerData, attack_count) == 0x14);
const _: () =
    assert!(core::mem::offset_of!(AttackBlockHandlerData, initial_power_attack_delay) == 0x18);
const _: () =
    assert!(core::mem::offset_of!(AttackBlockHandlerData, subsequent_power_attack_delay) == 0x20);
const _: () = assert!(core::mem::offset_of!(AttackBlockHandlerData, ignore) == 0x24);
const _: () = assert!(core::mem::offset_of!(AttackBlockHandlerData, held_right) == 0x27);
const _: () = assert!(core::mem::offset_of!(AttackBlockHandlerData, unk44) == 0x28);

#[repr(C)]
pub struct AttackBlockHandler {
    pub base: HeldStateHandler,
}

const _: () = assert!(core::mem::size_of::<AttackBlockHandler>() == 0x10);
const _: () = assert!(core::mem::offset_of!(AttackBlockHandler, base) == 0x00);

impl RttiType for AttackBlockHandler {
    const RTTI: VariantID = RTTI_AttackBlockHandler;
}

inherit!(AttackBlockHandler : HeldStateHandler, base);

impl AttackBlockHandler {
    pub const RTTI: VariantID = RTTI_AttackBlockHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_AttackBlockHandler;
    // SE / AE < 1.7.99: `0x18`, AE 1.7.99+: `0x58`, VR: `0x60`
    // (after VR's larger `HeldStateHandler`).
    crate::runtime_data_accessor! {
        pub fn data() -> AttackBlockHandlerData {
            version: crate::version::RUNTIME_SSE_1_7_99, se: 0x18, ae: 0x58, vr: 0x60
        }
    }

    crate::runtime_data_mut_accessor! {
        pub fn data_mut() -> AttackBlockHandlerData {
            version: crate::version::RUNTIME_SSE_1_7_99, se: 0x18, ae: 0x58, vr: 0x60
        }
    }
}
