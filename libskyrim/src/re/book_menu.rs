use crate::offsets::offsets_rtti::RTTI_BookMenu;
use crate::offsets::offsets_vtable::VTABLE_BookMenu;
use crate::relocation::{RttiType, VariantID};

// C++ `RE::BookMenu` minimal menu marker.
//
// TODO: SOURCE - replace this opaque menu marker with the real `IMenu`-derived
// layout from `BookMenu.h/.cpp` when fields, bases, or methods are needed;
// the current stub only preserves RTTI/VTABLE/MENU_NAME for menu lookup APIs.
core_util::abstract_type! { pub type BookMenu; }

impl RttiType for BookMenu {
    const RTTI: VariantID = RTTI_BookMenu;
}

/// C++ `RE::BookMenu::AE1799_RUNTIME_DATA` (at `0x98`).
#[repr(C)]
pub struct BookMenuAe1799RuntimeData {
    pub unk98: crate::re::NiRect<f32>, // 98
}

const _: () = assert!(core::mem::size_of::<BookMenuAe1799RuntimeData>() == 0x10);

impl BookMenu {
    pub const RTTI: VariantID = RTTI_BookMenu;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BookMenu;

    // AE 1.7.99+ only: `None` on SE, AE < 1.7.99 and VR.
    crate::runtime_optional_data_accessor! {
        pub fn ae1799_runtime_data() -> BookMenuAe1799RuntimeData {
            version: crate::version::RUNTIME_SSE_1_7_99, se: 0, ae: 0x98, vr: 0
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn ae1799_runtime_data_mut() -> BookMenuAe1799RuntimeData {
            version: crate::version::RUNTIME_SSE_1_7_99, se: 0, ae: 0x98, vr: 0
        }
    }
    pub const MENU_NAME: &'static str = "Book Menu";
}
