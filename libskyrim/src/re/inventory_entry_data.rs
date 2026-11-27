use alloc::boxed::Box;
use core::ffi::c_char;

use crate::re::alchemy_item::AlchemyItem;
use crate::re::bssimple_list::BSSimpleList;
use crate::re::enchantment_item::EnchantmentItem;
use crate::re::extra_charge::ExtraCharge;
use crate::re::extra_data_list::ExtraDataList;
use crate::re::extra_enchantment::ExtraEnchantment;
use crate::re::extra_hotkey::ExtraHotkey;
use crate::re::extra_leveled_item::ExtraLeveledItem;
use crate::re::extra_poison::ExtraPoison;
use crate::re::extra_worn::ExtraWorn;
use crate::re::extra_worn_left::ExtraWornLeft;
use crate::re::form_type::FormType;
use crate::re::game_setting_collection::GameSettingCollection;
use crate::re::soul_levels::SOUL_LEVEL;
use crate::re::tes_bound_object::TESBoundObject;
use crate::re::tes_enchantable_form::TESEnchantableForm;
use crate::re::tes_form::TESForm;
use crate::re::tes_soul_gem::TESSoulGem;
use crate::re::{Actor, ExtraDataTyped};
use crate::relocation::{RelocationID, skyrim_cast};

/// C++ `RE::InventoryEntryData`
#[repr(C)]
pub struct InventoryEntryData {
    pub object: *mut TESBoundObject,                        // 00
    pub extra_lists: *mut BSSimpleList<*mut ExtraDataList>, // 08
    pub count_delta: i32,                                   // 10
    pub pad14: u32,                                         // 14
}

const _: () = assert!(core::mem::size_of::<InventoryEntryData>() == 0x18);
const _: () = assert!(core::mem::offset_of!(InventoryEntryData, object) == 0x00);
const _: () = assert!(core::mem::offset_of!(InventoryEntryData, extra_lists) == 0x08);
const _: () = assert!(core::mem::offset_of!(InventoryEntryData, count_delta) == 0x10);
const _: () = assert!(core::mem::offset_of!(InventoryEntryData, pad14) == 0x14);

impl Default for InventoryEntryData {
    #[inline(always)]
    fn default() -> Self {
        Self::new(core::ptr::null_mut(), 0)
    }
}

impl InventoryEntryData {
    #[inline(always)]
    pub const fn new(object: *mut TESBoundObject, count_delta: i32) -> Self {
        Self {
            object,
            extra_lists: core::ptr::null_mut(),
            count_delta,
            pad14: 0,
        }
    }

    #[inline(always)]
    pub const fn get_object(&self) -> *mut TESBoundObject {
        self.object
    }

    pub fn add_extra_list(&mut self, extra: *mut ExtraDataList) {
        if extra.is_null() {
            return;
        }

        if self.extra_lists.is_null() {
            self.extra_lists = Box::into_raw(Box::new(BSSimpleList::default()));
        }

        unsafe {
            (*self.extra_lists).push_front(extra);
        }
    }

    crate::relocation_func! {
        pub fn deep_copy_raw(&mut self, other: *const InventoryEntryData) -> *mut InventoryEntryData => RelocationID::new(15745, 15983)
    }

    #[inline(always)]
    pub fn deep_copy(&mut self, other: &InventoryEntryData) -> &mut Self {
        unsafe { &mut *self.deep_copy_raw(other) }
    }

    pub fn get_display_name(&mut self) -> *const c_char {
        let mut name = core::ptr::null();

        if !self.extra_lists.is_null() {
            for &x_list in unsafe { (*self.extra_lists).iter() } {
                if !x_list.is_null() {
                    name = unsafe { (*x_list).get_display_name(self.object) };
                }
            }
        }

        if (name.is_null() || unsafe { *name } == 0) && !self.object.is_null() {
            name = unsafe { (&*(self.object.cast::<TESForm>())).get_name() };
        }

        if name.is_null() || unsafe { *name } == 0 {
            let gmst = GameSettingCollection::get_singleton();
            let missing_name = if gmst.is_null() {
                core::ptr::null_mut()
            } else {
                unsafe { (*gmst).get_setting_str("sMissingName") }
            };

            name = if missing_name.is_null() {
                c"".as_ptr()
            } else {
                unsafe { (*missing_name).get_string() }
            };
        }

        name
    }

    crate::relocation_func! {
        pub fn get_enchantment(&self) -> *mut EnchantmentItem => RelocationID::new(15788, 16026)
    }

    pub fn get_enchantment_charge(&self) -> Option<f64> {
        let mut result = None;
        let object = self.get_object();
        let enchantable = if object.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { skyrim_cast::<TESForm, TESEnchantableForm>(object.cast::<TESForm>()) }
        };

        if !enchantable.is_null()
            && unsafe { !(*enchantable).form_enchanting.is_null() }
            && unsafe { (*enchantable).amount_of_enchantment != 0 }
        {
            result = Some(100.0);
        }

        if !self.extra_lists.is_null() {
            for &x_list in unsafe { (*self.extra_lists).iter() } {
                if x_list.is_null() {
                    continue;
                }

                let x_charge = unsafe { (*x_list).get_by_type_typed::<ExtraCharge>() };
                let x_ench = unsafe { (*x_list).get_by_type_typed::<ExtraEnchantment>() };

                if !x_ench.is_null()
                    && unsafe { !(*x_ench).enchantment.is_null() }
                    && unsafe { (*x_ench).charge != 0 }
                {
                    result = Some(if x_charge.is_null() {
                        100.0
                    } else {
                        (f64::from(unsafe { (*x_charge).charge })
                            / f64::from(unsafe { (*x_ench).charge }))
                            * 100.0
                    });
                    break;
                } else if !x_charge.is_null()
                    && !enchantable.is_null()
                    && unsafe { !(*enchantable).form_enchanting.is_null() }
                    && unsafe { (*enchantable).amount_of_enchantment != 0 }
                {
                    result = Some(
                        (f64::from(unsafe { (*x_charge).charge })
                            / f64::from(unsafe { (*enchantable).amount_of_enchantment }))
                            * 100.0,
                    );
                    break;
                }
            }
        }

        result
    }

    pub fn get_owner(&mut self) -> *mut TESForm {
        if !self.extra_lists.is_null() {
            for &x_list in unsafe { (*self.extra_lists).iter() } {
                let owner = if x_list.is_null() {
                    core::ptr::null_mut()
                } else {
                    unsafe { (*x_list).get_owner() }
                };
                if !owner.is_null() {
                    return owner;
                }
            }
        }

        core::ptr::null_mut()
    }

    pub fn get_soul_level(&self) -> SOUL_LEVEL {
        if !self.extra_lists.is_null() {
            for &x_list in unsafe { (*self.extra_lists).iter() } {
                if x_list.is_null() {
                    continue;
                }

                let level = unsafe { (*x_list).get_soul_level() };
                if level > SOUL_LEVEL::None {
                    return level;
                }
            }
        }

        if !self.object.is_null() {
            let form = unsafe { &*(self.object.cast::<TESForm>()) };
            if form.is(FormType::SoulGem) {
                let soul_gem =
                    unsafe { skyrim_cast::<TESForm, TESSoulGem>(self.object.cast::<TESForm>()) };
                if !soul_gem.is_null() {
                    return unsafe { (*soul_gem).get_contained_soul() };
                }
            }
        }

        SOUL_LEVEL::None
    }

    #[inline(always)]
    pub fn get_weight(&self) -> f32 {
        if self.object.is_null() {
            -1.0
        } else {
            unsafe { (&*(self.object.cast::<TESForm>())).get_weight() }
        }
    }

    // RELOCATION_ID SE: 15757, AE: 15995
    crate::relocation_func! {
        pub fn get_value(&self) -> i32 => RelocationID::new(15757, 15995)
    }

    pub fn is_enchanted(&self) -> bool {
        if !self.object.is_null() {
            let enchantable = unsafe {
                skyrim_cast::<TESForm, TESEnchantableForm>(self.object.cast::<TESForm>())
            };
            if !enchantable.is_null() && unsafe { !(*enchantable).form_enchanting.is_null() } {
                return true;
            }
        }

        if !self.extra_lists.is_null() {
            for &x_list in unsafe { (*self.extra_lists).iter() } {
                if x_list.is_null() {
                    continue;
                }

                let x_ench = unsafe { (*x_list).get_by_type_typed::<ExtraEnchantment>() };
                if !x_ench.is_null() && unsafe { !(*x_ench).enchantment.is_null() } {
                    return true;
                }
            }
        }

        false
    }

    #[inline(always)]
    pub fn is_favorited(&self) -> bool {
        self.has_extra_data_type::<ExtraHotkey>()
    }

    #[inline(always)]
    pub fn is_leveled(&self) -> bool {
        self.has_extra_data_type::<ExtraLeveledItem>()
    }

    #[inline(always)]
    pub fn is_poisoned(&self) -> bool {
        self.has_extra_data_type::<ExtraPoison>()
    }

    #[inline(always)]
    pub fn is_worn(&self) -> bool {
        self.is_worn_on(false) || self.is_worn_on(true)
    }

    pub fn is_worn_on(&self, left: bool) -> bool {
        if !self.extra_lists.is_null() {
            for &x_list in unsafe { (*self.extra_lists).iter() } {
                if x_list.is_null() {
                    continue;
                }

                let has_type = unsafe {
                    if left {
                        (*x_list).has_type(ExtraWornLeft::EXTRADATATYPE)
                    } else {
                        (*x_list).has_type(ExtraWorn::EXTRADATATYPE)
                    }
                };

                if has_type {
                    return true;
                }
            }
        }

        false
    }

    #[inline(always)]
    pub fn is_owned_by(&mut self, test_owner: *mut Actor, default_to: bool) -> bool {
        let item_owner = self.get_owner();
        self.is_owned_by_with_owner(test_owner, item_owner, default_to)
    }

    #[inline(always)]
    pub fn is_owned_by_with_owner(
        &mut self,
        test_owner: *mut Actor,
        item_owner: *mut TESForm,
        default_to: bool,
    ) -> bool {
        self.is_owned_by_impl(test_owner, item_owner, default_to)
    }

    pub fn is_quest_object(&self) -> bool {
        if !self.extra_lists.is_null() {
            for &x_list in unsafe { (*self.extra_lists).iter() } {
                if !x_list.is_null() && unsafe { (*x_list).has_quest_object_alias() } {
                    return true;
                }
            }
        }

        false
    }

    crate::relocation_func! {
        pub fn poison_object(&mut self, alch_item: *mut AlchemyItem, count: u32) => RelocationID::new(15786, 16024)
    }

    crate::relocation_func! {
        pub fn set_worn(&mut self, worn: bool, left: bool, delete_extra_list: bool) => RelocationID::new(15789, 16027)
    }

    crate::relocation_func! {
        fn is_owned_by_impl(&self, test_owner: *mut Actor, item_owner: *mut TESForm, default_to: bool) -> bool => RelocationID::new(15782, 16020)
    }

    fn has_extra_data_type<T: ExtraDataTyped>(&self) -> bool {
        if !self.extra_lists.is_null() {
            for &x_list in unsafe { (*self.extra_lists).iter() } {
                if !x_list.is_null() && unsafe { (*x_list).has_type(T::EXTRADATATYPE) } {
                    return true;
                }
            }
        }

        false
    }
}
