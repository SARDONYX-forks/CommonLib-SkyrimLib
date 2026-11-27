use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_SixaxisEvent;
use crate::offsets::offsets_vtable::VTABLE_SixaxisEvent;
use crate::re::{IDEvent, NiQuaternion};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::SixaxisEvent` (AE 1.7.99+ only).
///
/// The C++ type is declared under `#pragma pack(push, 4)`, so several `u64`
/// members sit on 4-byte boundaries. They are modelled as `[u32; 2]` here to
/// keep the struct naturally aligned (and borrowable) in Rust.
#[repr(C)]
pub struct SixaxisEvent {
    pub base: IDEvent,             // 00
    pub unk28: u32,                // 28
    pub unk2c: u32,                // 2C - not zero-initialized by the ctor
    pub unk30: u64,                // 30
    pub unk38: u64,                // 38
    pub unk40: u32,                // 40
    pub unk44: [u32; 2],           // 44 (u64)
    pub unk4c: u32,                // 4C
    pub unk50: u64,                // 50
    pub unk58: u32,                // 58
    pub unk5c: [u32; 2],           // 5C (u64)
    pub unk64: u32,                // 64
    pub unk68: u64,                // 68
    pub unk70: u32,                // 70
    pub unk74: [u32; 2],           // 74 (u64)
    pub unk7c: u32,                // 7C
    pub orientation: NiQuaternion, // 80
}

const _: () = assert!(core::mem::size_of::<SixaxisEvent>() == 0x90);
const _: () = assert!(core::mem::offset_of!(SixaxisEvent, unk28) == 0x28);
const _: () = assert!(core::mem::offset_of!(SixaxisEvent, unk44) == 0x44);
const _: () = assert!(core::mem::offset_of!(SixaxisEvent, unk5c) == 0x5C);
const _: () = assert!(core::mem::offset_of!(SixaxisEvent, unk74) == 0x74);
const _: () = assert!(core::mem::offset_of!(SixaxisEvent, orientation) == 0x80);

impl RttiType for SixaxisEvent {
    const RTTI: VariantID = RTTI_SixaxisEvent;
}

inherit!(SixaxisEvent : IDEvent, base);

impl SixaxisEvent {
    pub const RTTI: VariantID = RTTI_SixaxisEvent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_SixaxisEvent;
}
