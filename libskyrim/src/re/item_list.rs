use crate::re::{BSTArray, GFxMovieView, GFxValue, GPtr, InventoryEntryData};

/// C++ `RE::StandardItemData`
#[repr(C)]
pub struct StandardItemData {
    pub vtable: *const usize,              // 00
    pub obj_desc: *mut InventoryEntryData, // 08
    pub owner: u32,                        // 10 - RefHandle
    pub pad14: u32,                        // 14
}

const _: () = assert!(core::mem::size_of::<StandardItemData>() == 0x18);

/// C++ `RE::ItemList::Item`
#[repr(C)]
pub struct ItemListItem {
    pub data: StandardItemData, // 00
    pub obj: GFxValue,          // 18 - kObject
    pub unk30: u64,             // 30
    pub unk38: u64,             // 38
}

const _: () = assert!(core::mem::size_of::<ItemListItem>() == 0x40);

/// C++ `RE::ItemList`
#[repr(C)]
pub struct ItemList {
    pub view: GPtr<GFxMovieView>,           // 00
    pub root: GFxValue, // 08 - "_level0.Menu_mc.inventoryLists.panelContainer.itemList"
    pub entry_list: GFxValue, // 20 - kArray
    pub items: BSTArray<*mut ItemListItem>, // 38
    pub update_pending: bool, // 50
    pub pad51: u8,      // 51
    pub pad52: u16,     // 52
    pub pad54: u32,     // 54
}

const _: () = assert!(core::mem::size_of::<ItemList>() == 0x58);
const _: () = assert!(core::mem::offset_of!(ItemList, items) == 0x38);

impl ItemList {
    /// C++ `ItemList::GetSelectedItem`: the entry at the list's
    /// `selectedIndex`, or null while an update is pending.
    pub fn get_selected_item(&self) -> *mut ItemListItem {
        if self.update_pending {
            return core::ptr::null_mut();
        }
        let mut selected_index = GFxValue::new();
        if !self
            .root
            .get_member_c_str(c"selectedIndex", &mut selected_index)
            || !selected_index.is_number()
        {
            return core::ptr::null_mut();
        }
        let idx = selected_index.get_number() as i32;
        if idx < 0 || idx as u32 >= self.items.len() {
            return core::ptr::null_mut();
        }
        self.items[idx as usize]
    }
}
