use alloc::boxed::Box;
use core::ffi::c_void;

use crate::offsets::offsets_rtti::RTTI_InventoryChanges__IItemChangeVisitor;
use crate::offsets::offsets_vtable::VTABLE_InventoryChanges__IItemChangeVisitor;
use crate::re::BGSOutfit;
use crate::re::actor::Actor;
use crate::re::bgs_biped_object_form::{BGSBipedObjectFormExt, BipedObjectSlot};
use crate::re::bs_container::BSContainerForEachResult;
use crate::re::bs_pointer_handle::ObjectRefHandle;
use crate::re::bssimple_list::BSSimpleList;
use crate::re::enchantment_item::EnchantmentItem;
use crate::re::extra_data_list::ExtraDataList;
use crate::re::inventory_entry_data::InventoryEntryData;
use crate::re::item_remove_reason::ITEM_REMOVE_REASON;
use crate::re::ni_point3::NiPoint3;
use crate::re::tes_bound_object::TESBoundObject;
use crate::re::tes_form::TESForm;
use crate::re::tes_object_armo::TESObjectARMO;
use crate::re::tes_object_refr::TESObjectREFR;
use crate::relocation::{RelocationID, RttiType, VariantID, skyrim_cast};
use crate::virtual_method;

/// C++ `RE::InventoryChanges::VisitResult`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InventoryChangesVisitResult {
    Stop = 0,
    Continue = 1,
}

/// C++ `RE::InventoryChanges::IItemChangeVisitor`
#[repr(C)]
pub struct InventoryChangesIItemChangeVisitor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<InventoryChangesIItemChangeVisitor>() == 0x8);
const _: () = assert!(core::mem::offset_of!(InventoryChangesIItemChangeVisitor, vtable) == 0x00);

impl RttiType for InventoryChangesIItemChangeVisitor {
    const RTTI: VariantID = RTTI_InventoryChanges__IItemChangeVisitor;
}

impl AsRef<InventoryChangesIItemChangeVisitor> for InventoryChangesIItemChangeVisitor {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<InventoryChangesIItemChangeVisitor> for InventoryChangesIItemChangeVisitor {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl InventoryChangesIItemChangeVisitor {
    pub const RTTI: VariantID = RTTI_InventoryChanges__IItemChangeVisitor;
    pub const VTABLE: &'static [VariantID] = &VTABLE_InventoryChanges__IItemChangeVisitor;

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    virtual_method! {
        pub const VFUNC_VISIT: usize = 0x01;
        pub fn visit(entry_data: *mut InventoryEntryData) -> BSContainerForEachResult
    }

    virtual_method! {
        pub const VFUNC_SHOULD_VISIT: usize = 0x02;
        pub fn should_visit(entry_data: *mut InventoryEntryData, object: *mut TESBoundObject) -> bool
    }

    virtual_method! {
        pub const VFUNC_UNK_03: usize = 0x03;
        pub fn unk_03(entry_data: *mut InventoryEntryData, arg2: *mut c_void, arg3: *mut bool) -> BSContainerForEachResult
    }

    #[inline(always)]
    pub const fn should_visit_default(
        &self,
        _entry_data: *mut InventoryEntryData,
        _object: *mut TESBoundObject,
    ) -> bool {
        true
    }

    #[inline]
    pub unsafe fn unk_03_default(
        &mut self,
        entry_data: *mut InventoryEntryData,
        _arg2: *mut c_void,
        arg3: *mut bool,
    ) -> BSContainerForEachResult {
        if !arg3.is_null() {
            unsafe {
                *arg3 = true;
            }
        }
        self.visit(entry_data)
    }
}

pub trait InventoryChangesIItemChangeVisitorExt {
    fn dtor(&mut self);
    fn visit(&mut self, entry_data: *mut InventoryEntryData) -> BSContainerForEachResult;
    fn should_visit(
        &mut self,
        entry_data: *mut InventoryEntryData,
        object: *mut TESBoundObject,
    ) -> bool;
    fn unk_03(
        &mut self,
        entry_data: *mut InventoryEntryData,
        arg2: *mut c_void,
        arg3: *mut bool,
    ) -> BSContainerForEachResult;
}

impl<T: AsRef<InventoryChangesIItemChangeVisitor> + AsMut<InventoryChangesIItemChangeVisitor>>
    InventoryChangesIItemChangeVisitorExt for T
{
    #[inline(always)]
    fn dtor(&mut self) {
        InventoryChangesIItemChangeVisitor::dtor(self.as_mut())
    }

    #[inline(always)]
    fn visit(&mut self, entry_data: *mut InventoryEntryData) -> BSContainerForEachResult {
        InventoryChangesIItemChangeVisitor::visit(self.as_mut(), entry_data)
    }

    #[inline(always)]
    fn should_visit(
        &mut self,
        entry_data: *mut InventoryEntryData,
        object: *mut TESBoundObject,
    ) -> bool {
        InventoryChangesIItemChangeVisitor::should_visit(self.as_mut(), entry_data, object)
    }

    #[inline(always)]
    fn unk_03(
        &mut self,
        entry_data: *mut InventoryEntryData,
        arg2: *mut c_void,
        arg3: *mut bool,
    ) -> BSContainerForEachResult {
        InventoryChangesIItemChangeVisitor::unk_03(self.as_mut(), entry_data, arg2, arg3)
    }
}

/// C++ `RE::InventoryChanges`
#[repr(C)]
pub struct InventoryChanges {
    pub entry_list: *mut BSSimpleList<*mut InventoryEntryData>, // 00
    pub owner: *mut TESObjectREFR,                              // 08
    pub total_weight: f32,                                      // 10
    pub armor_weight: f32,                                      // 14
    pub changed: bool,                                          // 18
    pub unk19: u8,                                              // 19
    pub unk1a: u8,                                              // 1A
    pub unk1b: u8,                                              // 1B
    pub unk1c: u32,                                             // 1C
}

const _: () = assert!(core::mem::size_of::<InventoryChanges>() == 0x20);
const _: () = assert!(core::mem::offset_of!(InventoryChanges, entry_list) == 0x00);
const _: () = assert!(core::mem::offset_of!(InventoryChanges, owner) == 0x08);
const _: () = assert!(core::mem::offset_of!(InventoryChanges, total_weight) == 0x10);
const _: () = assert!(core::mem::offset_of!(InventoryChanges, armor_weight) == 0x14);
const _: () = assert!(core::mem::offset_of!(InventoryChanges, changed) == 0x18);
const _: () = assert!(core::mem::offset_of!(InventoryChanges, unk19) == 0x19);
const _: () = assert!(core::mem::offset_of!(InventoryChanges, unk1a) == 0x1A);
const _: () = assert!(core::mem::offset_of!(InventoryChanges, unk1b) == 0x1B);
const _: () = assert!(core::mem::offset_of!(InventoryChanges, unk1c) == 0x1C);

impl InventoryChanges {
    #[inline(always)]
    pub const fn entry_list(&self) -> *mut BSSimpleList<*mut InventoryEntryData> {
        self.entry_list
    }

    #[inline(always)]
    pub const fn owner(&self) -> *mut TESObjectREFR {
        self.owner
    }

    crate::relocation_func! {
        fn ctor_impl(&mut self, owner: *mut TESObjectREFR) -> *mut InventoryChanges => RelocationID::new(15812, 16050)
    }

    crate::relocation_func! {
        pub fn dtor(&mut self) => RelocationID::new(15813, 16051)
    }

    #[inline(always)]
    pub fn ctor(&mut self, owner: *mut TESObjectREFR) -> *mut InventoryChanges {
        self.ctor_impl(owner)
    }

    #[inline(always)]
    pub fn ctor_default(&mut self) -> *mut InventoryChanges {
        self.ctor(core::ptr::null_mut())
    }

    #[inline]
    pub fn add_entry_data(&mut self, entry: *mut InventoryEntryData) {
        if self.entry_list.is_null() {
            self.entry_list = Box::into_raw(Box::new(BSSimpleList::default()));
        }

        unsafe {
            (*self.entry_list).push_front(entry);
        }
        self.changed = true;
    }

    crate::relocation_func! {
        pub fn enchant_object(
            &mut self,
            obj: *mut TESBoundObject,
            extra_list: *mut ExtraDataList,
            enchantment: *mut EnchantmentItem,
            charge: u16
        ) -> *mut ExtraDataList => RelocationID::new(15906, 16146)
    }

    crate::relocation_func! {
        pub fn generate_leveled_list_changes(&mut self) => RelocationID::new(15829, 16068)
    }

    crate::relocation_func! {
        fn get_armor_in_slot_flat(&mut self, slot: i32) -> *mut TESObjectARMO => RelocationID::new(15873, 16113)
    }

    #[inline]
    pub fn get_armor_in_slot(&mut self, slot: i32) -> *mut TESObjectARMO {
        if !crate::runtime::is_vr() {
            return self.get_armor_in_slot_flat(slot);
        }

        let owner = self.owner;
        if owner.is_null() {
            return core::ptr::null_mut();
        }

        let actor = unsafe { skyrim_cast::<TESObjectREFR, Actor>(owner) };
        if actor.is_null() {
            return core::ptr::null_mut();
        }

        if slot < 30 {
            return core::ptr::null_mut();
        }

        let shift = (slot - 30) as u32;
        if shift >= u32::BITS {
            return core::ptr::null_mut();
        }

        let Some(biped_slot) = BipedObjectSlot::from_bits(1u32 << shift) else {
            return core::ptr::null_mut();
        };

        let Some(entry_list) = (unsafe { self.entry_list.as_ref() }) else {
            return core::ptr::null_mut();
        };

        for &entry in entry_list.iter() {
            let Some(entry) = (unsafe { entry.as_ref() }) else {
                continue;
            };

            if !entry.is_worn() {
                continue;
            }

            let object = entry.object;
            if object.is_null() || !unsafe { (&*(object.cast::<TESForm>())).is_armor() } {
                continue;
            }

            let armor = unsafe { skyrim_cast::<TESForm, TESObjectARMO>(object.cast::<TESForm>()) };
            if !armor.is_null() && unsafe { (*armor).has_part_of(biped_slot) } {
                return armor;
            }
        }

        core::ptr::null_mut()
    }

    crate::relocation_func! {
        pub fn get_inventory_weight(&mut self) -> f32 => RelocationID::new(15883, 16123)
    }

    crate::relocation_func! {
        pub fn get_item_count(&mut self, obj: *mut TESBoundObject) -> i16 => RelocationID::new(15868, 16108)
    }

    crate::relocation_func! {
        pub fn get_next_unique_id(&mut self) -> u16 => RelocationID::new(15908, 16148)
    }

    crate::relocation_func! {
        pub fn get_worn_mask(&mut self) -> u32 => RelocationID::new(15806, 16044)
    }

    crate::relocation_func! {
        pub fn init_from_container_extra(&mut self) => RelocationID::new(15890, 16130)
    }

    crate::relocation_func! {
        pub fn init_leveled_items(&mut self) => RelocationID::new(15889, 16129)
    }

    crate::relocation_func! {
        pub fn init_outfit_items(&mut self, outfit: *mut BGSOutfit, npc_level: u16) => RelocationID::new(15833, 16072)
    }

    crate::relocation_func! {
        pub fn init_scripts(&mut self) => RelocationID::new(15829, 16068)
    }

    crate::relocation_func! {
        pub fn remove_favorite(&mut self, entry: *mut InventoryEntryData, item_list: *mut ExtraDataList) => RelocationID::new(15859, 16099)
    }

    #[inline(always)]
    pub fn remove_item(
        &mut self,
        refr: *mut TESObjectREFR,
        item: *mut TESBoundObject,
        count: i32,
        reason: ITEM_REMOVE_REASON,
        extra_data_list: *mut ExtraDataList,
        move_to_ref: *mut TESObjectREFR,
        drop_loc: &NiPoint3,
        drop_ref: *mut TESObjectREFR,
    ) -> ObjectRefHandle {
        let mut out = ObjectRefHandle::new();
        self.remove_item_impl(
            &mut out,
            refr,
            item,
            count,
            reason,
            extra_data_list,
            move_to_ref,
            drop_loc,
            drop_ref,
        );
        out
    }

    crate::relocation_func! {
        fn remove_item_impl(
            &mut self,
            out: &mut ObjectRefHandle,
            refr: *mut TESObjectREFR,
            item: *mut TESBoundObject,
            count: i32,
            reason: ITEM_REMOVE_REASON,
            extra_data_list: *mut ExtraDataList,
            move_to_ref: *mut TESObjectREFR,
            drop_loc: &NiPoint3,
            drop_ref: *mut TESObjectREFR
        ) => RelocationID::new(15821, 16059)
    }

    crate::relocation_func! {
        pub fn remove_all_items(
            &mut self,
            refr: *mut TESObjectREFR,
            move_to_ref: *mut TESObjectREFR,
            stealing: bool,
            keep_ownership: bool,
            arg6: bool
        ) => RelocationID::new(15878, 441567)
    }

    crate::relocation_func! {
        pub fn send_container_changed_event(
            &mut self,
            item_extra_list: *mut ExtraDataList,
            from_refr: *mut TESObjectREFR,
            item: *mut TESForm,
            count: i32
        ) => RelocationID::new(15909, 16149)
    }

    crate::relocation_func! {
        pub fn set_favorite(&mut self, entry: *mut InventoryEntryData, item_list: *mut ExtraDataList) => RelocationID::new(15858, 16098)
    }

    crate::relocation_func! {
        pub fn set_unique_id(
            &mut self,
            item_list: *mut ExtraDataList,
            old_form: *mut TESForm,
            new_form: *mut TESForm
        ) => RelocationID::new(15907, 16147)
    }

    crate::relocation_func! {
        pub fn visit_inventory(&mut self, visitor: &mut InventoryChangesIItemChangeVisitor) => RelocationID::new(15855, 16095)
    }

    crate::relocation_func! {
        pub fn visit_worn_items(&mut self, visitor: &mut InventoryChangesIItemChangeVisitor) => RelocationID::new(15856, 16096)
    }
}
