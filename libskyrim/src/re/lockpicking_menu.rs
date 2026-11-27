use crate::offsets::offsets_rtti::RTTI_LockpickingMenu;
use crate::offsets::offsets_vtable::VTABLE_LockpickingMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::LockpickingMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `LockpickingMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type LockpickingMenu; }

impl RttiType for LockpickingMenu {
    const RTTI: VariantID = RTTI_LockpickingMenu;
}

/// C++ `RE::LockpickingMenu::AE1799_RUNTIME_DATA` (at `0x110`).
#[repr(C)]
pub struct LockpickingMenuAe1799RuntimeData {
    pub unk110: f32,     // 110
    pub unk114: f32,     // 114
    pub unk118: u32,     // 118
    pub unk11c: bool,    // 11C
    pub unk11d: bool,    // 11D
    pub unk11e: bool,    // 11E
    pub unk11f: bool,    // 11F
    pub unk120: bool,    // 120
    pub unk121: bool,    // 121
    pub unk122: bool,    // 122
    pub pad123: [u8; 5], // 123
}

const _: () = assert!(core::mem::size_of::<LockpickingMenuAe1799RuntimeData>() == 0x18);

impl LockpickingMenu {
    pub const RTTI: VariantID = RTTI_LockpickingMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_LockpickingMenu;

    // AE 1.7.99+ only: `None` on SE, AE < 1.7.99 and VR.
    crate::runtime_optional_data_accessor! {
        pub fn ae1799_runtime_data() -> LockpickingMenuAe1799RuntimeData {
            version: crate::version::RUNTIME_SSE_1_7_99, se: 0, ae: 0x110, vr: 0
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn ae1799_runtime_data_mut() -> LockpickingMenuAe1799RuntimeData {
            version: crate::version::RUNTIME_SSE_1_7_99, se: 0, ae: 0x110, vr: 0
        }
    }
    pub const MENU_NAME: &'static str = "Lockpicking Menu";
}
