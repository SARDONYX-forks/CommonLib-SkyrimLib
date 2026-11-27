use crate::offsets::offsets_rtti::RTTI_InventoryMenu;
use crate::offsets::offsets_vtable::VTABLE_InventoryMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::InventoryMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real
// `IMenu`-derived layout from `InventoryMenu.h/.cpp` when fields, bases, or
// methods are needed; the current stub only preserves RTTI/VTABLE/MENU_NAME
// for menu lookup APIs.
core_util::abstract_type! { pub type InventoryMenu; }

impl RttiType for InventoryMenu {
    const RTTI: VariantID = RTTI_InventoryMenu;
}

impl InventoryMenu {
    pub const RTTI: VariantID = RTTI_InventoryMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_InventoryMenu;
    pub const MENU_NAME: &'static str = "InventoryMenu";
}

/// C++ `InventoryMenu::RUNTIME_DATA` (at 0x30, VR 0x40).
#[repr(C)]
pub struct InventoryMenuRuntimeData {
    pub root: crate::re::GFxValue,           // 00 - "_level0.Menu_mc"
    pub item_list: *mut crate::re::ItemList, // 18
    pub item_card: *mut core::ffi::c_void,   // 20 - ItemCard
    pub bottom_bar: *mut core::ffi::c_void,  // 28 - BottomBar
    pub pending_update_objects: crate::re::BSTArray<*mut crate::re::TESBoundObject>, // 30
    pub unk78: u8,                           // 48
    pub pad79: u8,                           // 49
    pub pad7a: u16,                          // 4A
    pub unk7c: u32,                          // 4C
    pub pc_controls_ready: bool,             // 50
    pub unk81: u8,                           // 51
    pub pad82: u16,                          // 52
    pub pad84: u32,                          // 54
}

const _: () = assert!(core::mem::size_of::<InventoryMenuRuntimeData>() == 0x58);
const _: () = assert!(core::mem::offset_of!(InventoryMenuRuntimeData, item_list) == 0x18);

impl InventoryMenu {
    crate::runtime_data_accessor! {
        pub fn get_runtime_data() -> InventoryMenuRuntimeData {
            se_ae: 0x30,
            vr: 0x40,
        }
    }
}
