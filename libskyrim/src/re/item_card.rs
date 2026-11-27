use core::ffi::c_void;

use crate::re::{BSString, GFxMovieView, GFxValue, GPtr, MagicItem};
use crate::relocation::RelocationID;

/// C++ `RE::ItemCard`
#[repr(C)]
pub struct ItemCard {
    pub obj: GFxValue,            // 00 - kObject
    pub info_text: BSString,      // 18
    pub unk28: *mut c_void,       // 28
    pub unk30: u32,               // 30
    pub pad34: u32,               // 34
    pub view: GPtr<GFxMovieView>, // 38
}

const _: () = assert!(core::mem::size_of::<ItemCard>() == 0x40);

impl ItemCard {
    /// A card without a movie view, enough for text-only helpers such as
    /// [`ItemCard::get_magic_item_description`].
    pub fn detached() -> Self {
        Self {
            obj: GFxValue::new(),
            info_text: BSString::new(),
            unk28: core::ptr::null_mut(),
            unk30: 0,
            pad34: 0,
            view: GPtr::null(),
        }
    }

    crate::relocation_func! {
        /// Writes the item-card description of `magic_item` (effect list
        /// with magnitudes substituted, HTML formatting included) to `out`.
        pub fn get_magic_item_description(&mut self, magic_item: *mut MagicItem, out: &mut BSString) => RelocationID::new(51022, 51900)
    }
}
