use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_MotionGestureEvent;
use crate::offsets::offsets_vtable::VTABLE_MotionGestureEvent;
use crate::re::IDEvent;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::MotionGestureEvent` (AE 1.7.99+ only).
#[repr(C)]
pub struct MotionGestureEvent {
    pub base: IDEvent, // 00
    pub unk28: i32,    // 28
    pub unk2c: i32,    // 2C
    pub unk30: bool,   // 30
    pub pad31: u8,     // 31
    pub pad32: u16,    // 32
    pub unk34: i32,    // 34
}

const _: () = assert!(core::mem::size_of::<MotionGestureEvent>() == 0x38);
const _: () = assert!(core::mem::offset_of!(MotionGestureEvent, unk30) == 0x30);
const _: () = assert!(core::mem::offset_of!(MotionGestureEvent, unk34) == 0x34);

impl RttiType for MotionGestureEvent {
    const RTTI: VariantID = RTTI_MotionGestureEvent;
}

inherit!(MotionGestureEvent : IDEvent, base);

impl MotionGestureEvent {
    pub const RTTI: VariantID = RTTI_MotionGestureEvent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MotionGestureEvent;
}
