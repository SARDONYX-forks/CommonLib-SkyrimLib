use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_AmiiboEvent;
use crate::offsets::offsets_vtable::VTABLE_AmiiboEvent;
use crate::re::IDEvent;
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::AmiiboEvent` (AE 1.7.99+ only).
///
/// Declared under `#pragma pack(push, 4)` in C++; the misaligned `u64` at `0x2C`
/// is modelled as `[u32; 2]`.
#[repr(C)]
pub struct AmiiboEvent {
    pub base: IDEvent,   // 00
    pub unk28: u32,      // 28 - not zero-initialized by the ctor
    pub unk2c: [u32; 2], // 2C (u64)
    pub unk34: u16,      // 34
    pub unk36: u16,      // 36 - not zero-initialized by the ctor
    pub unk38: u32,      // 38
    pub unk3c: u32,      // 3C - not zero-initialized by the ctor
}

const _: () = assert!(core::mem::size_of::<AmiiboEvent>() == 0x40);
const _: () = assert!(core::mem::offset_of!(AmiiboEvent, unk2c) == 0x2C);
const _: () = assert!(core::mem::offset_of!(AmiiboEvent, unk34) == 0x34);

impl RttiType for AmiiboEvent {
    const RTTI: VariantID = RTTI_AmiiboEvent;
}

inherit!(AmiiboEvent : IDEvent, base);

impl AmiiboEvent {
    pub const RTTI: VariantID = RTTI_AmiiboEvent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_AmiiboEvent;
}
