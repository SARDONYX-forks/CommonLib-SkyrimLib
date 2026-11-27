use core::ffi::c_char;

use crate::re::bs_atomic::{BSReadLockGuard, BSReadWriteLock, BSWriteLockGuard};
use crate::re::{
    BGSEncounterZone, BGSKeyword, BGSLocation, BSExtraData, EnchantmentItem, ExtraAshPileRef,
    ExtraCount, ExtraDataType, ExtraDataTyped, ExtraEncounterZone, ExtraFlagsFlag, ExtraHealth,
    ExtraLevCreaModifier, ExtraLinkedRef, ExtraMissingLinkedRefIDs, ExtraOwnership,
    ExtraReferenceHandle, ExtraSoul, ExtraTeleport, ExtraTextDisplayData, GameSettingCollection,
    InventoryChanges, LEV_CREA_MODIFIER, NiPoint3, ObjectRefHandle, SOUL_LEVEL, TESBoundObject,
    TESForm, TESObjectREFR,
};
use crate::relocation::{ID, RelocationID};
use crate::version::RUNTIME_SSE_1_6_629;

/// C++ `RE::BaseExtraList::PresenceBitfield`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PresenceBitfield {
    pub bits: [u8; 0x18], // 00
}

const _: () = assert!(core::mem::size_of::<PresenceBitfield>() == 0x18);

impl PresenceBitfield {
    #[inline(always)]
    pub fn has_type(&self, ty: u32) -> bool {
        let index = ty >> 3;
        if index >= 0x17 {
            return false;
        }
        let bit_mask = 1u8 << (ty % 8);
        (self.bits[index as usize] & bit_mask) != 0
    }

    #[inline(always)]
    pub fn mark_type(&mut self, ty: u32, cleared: bool) {
        let index = (ty >> 3) as usize;
        let bit_mask = 1u8 << (ty % 8);
        if cleared {
            self.bits[index] &= !bit_mask;
        } else {
            self.bits[index] |= bit_mask;
        }
    }
}

/// Common cross-runtime prefix for `RE::BaseExtraList`.
#[repr(C)]
pub struct BaseExtraList {
    _prefix: [u8; 0x10],
}

const _: () = assert!(core::mem::size_of::<BaseExtraList>() == 0x10);

impl BaseExtraList {
    crate::runtime_data_ref_accessor! {
        fn data_ref() -> *mut BSExtraData {
            version: RUNTIME_SSE_1_6_629,
            se: 0x0,
            ae: 0x8,
            vr: 0x0
        }
    }

    crate::runtime_data_mut_accessor! {
        fn data_ref_mut() -> *mut BSExtraData {
            version: RUNTIME_SSE_1_6_629,
            se: 0x0,
            ae: 0x8,
            vr: 0x0
        }
    }

    crate::runtime_data_ref_accessor! {
        fn presence_ref() -> *mut PresenceBitfield {
            version: RUNTIME_SSE_1_6_629,
            se: 0x8,
            ae: 0x10,
            vr: 0x8
        }
    }

    crate::runtime_data_mut_accessor! {
        fn presence_ref_mut() -> *mut PresenceBitfield {
            version: RUNTIME_SSE_1_6_629,
            se: 0x8,
            ae: 0x10,
            vr: 0x8
        }
    }

    #[inline(always)]
    pub fn get_data(&self) -> *mut BSExtraData {
        *self.data_ref()
    }

    #[inline(always)]
    pub fn get_data_mut(&mut self) -> &mut *mut BSExtraData {
        self.data_ref_mut()
    }

    #[inline(always)]
    pub fn get_presence(&self) -> *mut PresenceBitfield {
        *self.presence_ref()
    }

    #[inline(always)]
    pub fn get_presence_mut(&mut self) -> &mut *mut PresenceBitfield {
        self.presence_ref_mut()
    }

    #[inline(always)]
    pub fn destroy(&mut self) {
        if crate::runtime::is_ae() {
            return;
        }
        type Func = unsafe extern "C" fn(*mut BaseExtraList);
        let func: Func = unsafe { core::mem::transmute(ID::new(11572).address()) };
        unsafe { func(self as *mut Self) };
    }
}

#[derive(Clone, Copy)]
pub struct ExtraDataListIterator {
    cur: *mut BSExtraData,
}

impl Iterator for ExtraDataListIterator {
    type Item = *mut BSExtraData;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur;
        if cur.is_null() {
            None
        } else {
            unsafe {
                self.cur = (*cur).next;
            }
            Some(cur)
        }
    }
}

#[derive(Clone, Copy)]
pub struct ExtraDataListConstIterator {
    cur: *const BSExtraData,
}

impl Iterator for ExtraDataListConstIterator {
    type Item = *const BSExtraData;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur;
        if cur.is_null() {
            None
        } else {
            unsafe {
                self.cur = (*cur).next;
            }
            Some(cur)
        }
    }
}

/// Common cross-runtime prefix for `RE::ExtraDataList`.
///
/// `ExtraDataList` grows a runtime-divergent lock tail starting at `0x10`
/// (`0x18` on AE 1.6.629+), so this Rust type keeps only the honest common
/// prefix and accesses the trailing data through runtime-aware helpers.
#[repr(C)]
pub struct ExtraDataList {
    pub extra_data: BaseExtraList, // 00
}

const _: () = assert!(core::mem::size_of::<ExtraDataList>() == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraDataList, extra_data) == 0x00);

impl AsRef<ExtraDataList> for ExtraDataList {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<ExtraDataList> for ExtraDataList {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl ExtraDataList {
    #[inline(always)]
    pub fn begin(&mut self) -> ExtraDataListIterator {
        ExtraDataListIterator {
            cur: self.extra_data.get_data(),
        }
    }

    #[inline(always)]
    pub fn cbegin(&self) -> ExtraDataListConstIterator {
        ExtraDataListConstIterator {
            cur: self.extra_data.get_data(),
        }
    }

    #[inline(always)]
    pub fn end(&mut self) -> ExtraDataListIterator {
        ExtraDataListIterator {
            cur: core::ptr::null_mut(),
        }
    }

    #[inline(always)]
    pub fn cend(&self) -> ExtraDataListConstIterator {
        ExtraDataListConstIterator {
            cur: core::ptr::null(),
        }
    }

    crate::runtime_data_accessor! {
        fn get_lock() -> BSReadWriteLock {
            version: RUNTIME_SSE_1_6_629,
            se: 0x10,
            ae: 0x18,
            vr: 0x10
        }
    }

    #[inline(always)]
    fn reference_extra_list(refr: *mut TESObjectREFR) -> *mut ExtraDataList {
        if refr.is_null() {
            core::ptr::null_mut()
        } else {
            // `TESObjectREFR::extraList` is a fixed field at `0x70` in
            // CommonLibVR across SE/AE/VR; the runtime split starts later.
            unsafe { (refr as *mut u8).add(0x70).cast() }
        }
    }

    #[inline(always)]
    pub fn get_by_type(&self, ty: ExtraDataType) -> *mut BSExtraData {
        self.get_by_type_impl(ty)
    }

    #[inline(always)]
    pub fn get_by_type_typed<T: ExtraDataTyped>(&self) -> *mut T {
        self.get_by_type(T::EXTRADATATYPE).cast()
    }

    #[inline(always)]
    pub fn has_type(&self, ty: ExtraDataType) -> bool {
        let _locker = BSReadLockGuard::new(self.get_lock());
        let presence = self.extra_data.get_presence();
        !presence.is_null() && unsafe { (*presence).has_type(ty as u32) }
    }

    pub fn remove(&mut self, ty: ExtraDataType, to_remove: *mut BSExtraData) -> bool {
        let _locker = BSWriteLockGuard::new(self.get_lock());

        if to_remove.is_null() {
            return false;
        }

        let head = self.extra_data.get_data_mut();
        let mut removed = false;

        if *head == to_remove {
            unsafe {
                *head = (*to_remove).next;
            }
            removed = true;
        } else {
            let mut iter = *head;
            while !iter.is_null() {
                unsafe {
                    if (*iter).next == to_remove {
                        (*iter).next = (*to_remove).next;
                        removed = true;
                        break;
                    }
                    iter = (*iter).next;
                }
            }
        }

        if removed {
            self.mark_type(ty, true);
        }
        removed
    }

    pub fn remove_typed<T: ExtraDataTyped>(&mut self, to_remove: *mut T) -> bool {
        self.remove(T::EXTRADATATYPE, to_remove.cast())
    }

    pub fn remove_by_type(&mut self, ty: ExtraDataType) -> bool {
        let _locker = BSWriteLockGuard::new(self.get_lock());

        let head = self.extra_data.get_data_mut();
        if (*head).is_null() {
            return false;
        }

        let mut removed = false;
        while !(*head).is_null() && unsafe { (**head).type_storage().underlying() == ty as i32 } {
            let tmp = *head;
            unsafe {
                *head = (*tmp).next;
            }
            Self::delete_extra(tmp);
            removed = true;
        }

        let mut prev = *head;
        let mut cur = if prev.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*prev).next }
        };
        while !cur.is_null() {
            unsafe {
                if (*cur).type_storage().underlying() == ty as i32 {
                    (*prev).next = (*cur).next;
                    let tmp = cur;
                    cur = prev;
                    Self::delete_extra(tmp);
                    removed = true;
                }
                prev = cur;
                cur = if prev.is_null() {
                    core::ptr::null_mut()
                } else {
                    (*prev).next
                };
            }
        }

        self.mark_type(ty, true);
        removed
    }

    /// Runtime object size: `0x18` on SE / VR / AE < 1.6.629, `0x20` on AE
    /// 1.6.629+ (virtual `~BaseExtraList` adds a vtable pointer).
    #[inline(always)]
    pub fn runtime_size() -> usize {
        crate::runtime::relocate_versioned_all(RUNTIME_SSE_1_6_629, 0x18, 0x20, 0x18)
    }

    crate::relocation_func! {
        fn init_impl(this: *mut ExtraDataList) -> *mut ExtraDataList => RelocationID::new(11437, 11583)
    }

    /// Allocates an empty `ExtraDataList` on the engine heap and runs the
    /// engine constructor (`InitExtraDataList`, the routine plugins such as
    /// iEquip use). Returns null on allocation failure.
    pub fn create() -> *mut ExtraDataList {
        let size = Self::runtime_size();
        let mem = unsafe { crate::ffi::commonlib_malloc(size) }.cast::<ExtraDataList>();
        if mem.is_null() {
            return mem;
        }
        unsafe { core::ptr::write_bytes(mem.cast::<u8>(), 0, size) };
        Self::init_impl(mem)
    }

    crate::relocation_func! {
        pub fn add(&mut self, to_add: *mut BSExtraData) -> *mut BSExtraData => RelocationID::new(12176, 12315)
    }

    crate::relocation_func! {
        pub fn add_activate_ref_child(&mut self, child_ref: *mut TESObjectREFR) => RelocationID::new(11651, 11797)
    }

    #[inline(always)]
    pub fn get_ash_pile_ref(&self) -> ObjectRefHandle {
        let x_ash_ref = self.get_by_type_typed::<ExtraAshPileRef>();
        if x_ash_ref.is_null() {
            ObjectRefHandle::new()
        } else {
            unsafe { (*x_ash_ref).ash_pile_ref }
        }
    }

    #[inline(always)]
    pub fn get_count(&self) -> i32 {
        let x_count = self.get_by_type_typed::<ExtraCount>();
        if x_count.is_null() {
            1
        } else {
            unsafe { (*x_count).count as i32 }
        }
    }

    pub fn get_display_name(&mut self, base_object: *mut TESBoundObject) -> *const c_char {
        let mut result = core::ptr::null();
        let mut health = 1.0f32;

        let x_health = self.get_by_type_typed::<ExtraHealth>();
        if !x_health.is_null() {
            health = unsafe { (*x_health).health };
        }

        let mut x_text = self.get_extra_text_display_data();
        let default_health = if health <= 1.0 {
            (1.0 - health) < 0.001
        } else {
            (health - 1.0) < 0.001
        };
        if x_text.is_null() && !default_health {
            x_text = BSExtraData::create_typed::<ExtraTextDisplayData>(
                ExtraTextDisplayData::VTABLE[0].address(),
            );
            unsafe {
                (*x_text).base.next = core::ptr::null_mut();
                (*x_text).display_name = crate::re::BSFixedString::from_str("");
                (*x_text).display_name_text = core::ptr::null_mut();
                (*x_text).owner_quest = core::ptr::null_mut();
                (*x_text).owner_instance = ExtraTextDisplayData::owner_instance_uninitialized();
                (*x_text).temper_factor = 1.0;
                (*x_text).custom_name_length = 0;
                (*x_text).pad32 = 0;
                (*x_text).pad34 = 0;
            }
            self.add(x_text.cast());
        }

        if !x_text.is_null() {
            result = unsafe { (*x_text).get_display_name(base_object, health) };
        } else if !base_object.is_null() {
            result = unsafe { (&*(base_object.cast::<TESForm>())).get_name() };
        }

        if result.is_null() || unsafe { *result } == 0 {
            let gmst = GameSettingCollection::get_singleton();
            let missing_name = if gmst.is_null() {
                core::ptr::null_mut()
            } else {
                unsafe { (*gmst).get_setting_str("sMissingName") }
            };
            result = if missing_name.is_null() {
                core::ptr::null()
            } else {
                unsafe { (*missing_name).get_string() }
            };
        }

        result
    }

    #[inline(always)]
    pub fn get_encounter_zone(&self) -> *mut BGSEncounterZone {
        let x_zone = self.get_by_type_typed::<ExtraEncounterZone>();
        if x_zone.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*x_zone).zone }
        }
    }

    pub fn get_extra_text_display_data(&mut self) -> *mut ExtraTextDisplayData {
        let x_ref = self.get_by_type_typed::<ExtraReferenceHandle>();
        let mut x_text = core::ptr::null_mut();
        if !x_ref.is_null() {
            let reference = unsafe { (*x_ref).get_original_reference().get() };
            if !reference.is_null() && !unsafe { (&*(reference.cast::<TESForm>())).is_deleted() } {
                let extra_list = Self::reference_extra_list(reference);
                if !extra_list.is_null() {
                    x_text = unsafe { (*extra_list).get_by_type_typed::<ExtraTextDisplayData>() };
                }
            }
        }

        if x_text.is_null() {
            self.get_by_type_typed::<ExtraTextDisplayData>()
        } else {
            x_text
        }
    }

    pub fn get_linked_ref(&self, keyword: *mut BGSKeyword) -> *mut TESObjectREFR {
        let _locker = BSReadLockGuard::new(self.get_lock());
        let x_linked_ref = self.get_by_type_typed::<ExtraLinkedRef>();
        if x_linked_ref.is_null() {
            return core::ptr::null_mut();
        }

        let mut linked_ref = core::ptr::null_mut();
        for entry in unsafe { (*x_linked_ref).linked_refs_slice() } {
            if entry.keyword == keyword {
                linked_ref = entry.refr;
                if linked_ref.is_null() && self.has_type(ExtraDataType::EditorID) {
                    let x_missing = self.get_by_type_typed::<ExtraMissingLinkedRefIDs>();
                    if !x_missing.is_null() {
                        linked_ref = unsafe { (*x_missing).get_linked_ref(keyword) };
                    }
                }
            }

            if !linked_ref.is_null() {
                break;
            }
        }

        linked_ref
    }

    crate::relocation_func! {
        pub fn get_object_health(&self) -> f32 => RelocationID::new(11865, 0)
    }

    #[inline(always)]
    pub fn get_owner(&self) -> *mut TESForm {
        let x_owner = self.get_by_type_typed::<ExtraOwnership>();
        if x_owner.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*x_owner).owner }
        }
    }

    #[inline(always)]
    pub fn get_soul_level(&self) -> SOUL_LEVEL {
        let x_soul = self.get_by_type_typed::<ExtraSoul>();
        if x_soul.is_null() {
            SOUL_LEVEL::None
        } else {
            unsafe { (*x_soul).get_contained_soul() }
        }
    }

    #[inline(always)]
    pub fn get_teleport_linked_door(&self) -> ObjectRefHandle {
        let x_teleport = self.get_by_type_typed::<ExtraTeleport>();
        if x_teleport.is_null() {
            ObjectRefHandle::new()
        } else {
            let teleport_data = unsafe { (*x_teleport).teleport_data };
            if teleport_data.is_null() {
                ObjectRefHandle::new()
            } else {
                unsafe { (*teleport_data).linked_door }
            }
        }
    }

    #[inline(always)]
    pub fn get_worn(&self) -> bool {
        self.has_type(ExtraDataType::Worn) || self.has_type(ExtraDataType::WornLeft)
    }

    crate::relocation_func! {
        pub fn has_quest_object_alias(&mut self) -> bool => RelocationID::new(11913, 12052)
    }

    crate::relocation_func! {
        pub fn set_activate_parent(&mut self, parent_ref: *mut TESObjectREFR, delay: f32) => RelocationID::new(11647, 11793)
    }

    crate::relocation_func! {
        pub fn set_count(&mut self, count: u16) => RelocationID::new(11471, 11617)
    }

    crate::relocation_func! {
        pub fn set_enchantment(&mut self, enchantment: *mut EnchantmentItem, charge_amount: u16, remove_on_unequip: bool) => RelocationID::new(11921, 12060)
    }

    pub fn set_encounter_zone(&mut self, zone: *mut BGSEncounterZone) {
        let x_zone = self.get_by_type_typed::<ExtraEncounterZone>();
        if !x_zone.is_null() {
            if zone.is_null() {
                self.remove_typed(x_zone);
            } else {
                unsafe {
                    (*x_zone).zone = zone;
                }
            }
        } else if !zone.is_null() {
            let x_zone = BSExtraData::create_typed::<ExtraEncounterZone>(
                ExtraEncounterZone::VTABLE[0].address(),
            );
            unsafe {
                (*x_zone).base.next = core::ptr::null_mut();
                (*x_zone).zone = zone;
            }
            self.add(x_zone.cast());
        }
    }

    crate::relocation_func! {
        pub fn set_extra_flags(&mut self, flags: ExtraFlagsFlag, enable: bool) => RelocationID::new(11903, 12042)
    }

    crate::relocation_func! {
        pub fn set_inventory_changes(&mut self, changes: *mut InventoryChanges) => RelocationID::new(11483, 11600)
    }

    pub fn set_lev_crea_modifier(&mut self, modifier: LEV_CREA_MODIFIER) {
        if modifier == LEV_CREA_MODIFIER::None {
            self.remove_by_type(ExtraDataType::LevCreaModifier);
            return;
        }

        let x_modifier = self.get_by_type_typed::<ExtraLevCreaModifier>();
        if !x_modifier.is_null() {
            unsafe {
                *(&mut (*x_modifier).modifier as *mut _ as *mut u32) = modifier as u32;
            }
        } else {
            let x_modifier = BSExtraData::create_typed::<ExtraLevCreaModifier>(
                ExtraLevCreaModifier::VTABLE[0].address(),
            );
            unsafe {
                (*x_modifier).base.next = core::ptr::null_mut();
                *(&mut (*x_modifier).modifier as *mut _ as *mut u32) = modifier as u32;
                (*x_modifier).pad14 = 0;
            }
            self.add(x_modifier.cast());
        }
    }

    crate::relocation_func! {
        pub fn set_linked_ref(&mut self, target_ref: *mut TESObjectREFR, keyword: *mut BGSKeyword) => RelocationID::new(11633, 11779)
    }

    pub fn set_override_name(&mut self, name: *const c_char) {
        let mut text_data = self.get_by_type_typed::<ExtraTextDisplayData>();
        if text_data.is_null() {
            text_data = BSExtraData::create_typed::<ExtraTextDisplayData>(
                ExtraTextDisplayData::VTABLE[0].address(),
            );
            unsafe {
                (*text_data).base.next = core::ptr::null_mut();
                (*text_data).display_name = crate::re::BSFixedString::from_str("");
                (*text_data).display_name_text = core::ptr::null_mut();
                (*text_data).owner_quest = core::ptr::null_mut();
                (*text_data).owner_instance = ExtraTextDisplayData::owner_instance_uninitialized();
                (*text_data).temper_factor = 1.0;
                (*text_data).custom_name_length = 0;
                (*text_data).pad32 = 0;
                (*text_data).pad34 = 0;
            }
            self.add(text_data.cast());
        }

        unsafe {
            if (*text_data).display_name_text.is_null() && (*text_data).owner_quest.is_null() {
                (*text_data).set_name(name);
            }
        }
    }

    pub fn set_owner(&mut self, owner: *mut TESForm) {
        if !owner.is_null() && unsafe { (*owner).is_dynamic_form() } {
            return;
        }

        let x_owner = self.get_by_type_typed::<ExtraOwnership>();
        if !x_owner.is_null() {
            if owner.is_null() {
                self.remove_typed(x_owner);
            } else {
                unsafe {
                    (*x_owner).owner = owner;
                }
            }
        } else if !owner.is_null() {
            let x_owner =
                BSExtraData::create_typed::<ExtraOwnership>(ExtraOwnership::VTABLE[0].address());
            unsafe {
                (*x_owner).base.next = core::ptr::null_mut();
                (*x_owner).owner = owner;
            }
            self.add(x_owner.cast());
        }
    }

    crate::relocation_func! {
        pub fn set_starting_position(
            &mut self,
            refr: *mut TESObjectREFR,
            position: &NiPoint3,
            rotation: &NiPoint3,
            location: *mut BGSLocation
        ) => RelocationID::new(11851, 11990)
    }

    fn get_by_type_impl(&self, ty: ExtraDataType) -> *mut BSExtraData {
        let _locker = BSReadLockGuard::new(self.get_lock());

        if !self.has_type(ty) {
            return core::ptr::null_mut();
        }

        let mut iter = self.extra_data.get_data();
        while !iter.is_null() {
            unsafe {
                if (*iter).type_storage().underlying() == ty as i32 {
                    return iter;
                }
                iter = (*iter).next;
            }
        }

        core::ptr::null_mut()
    }

    #[inline(always)]
    fn mark_type(&mut self, ty: ExtraDataType, cleared: bool) {
        let presence = self.extra_data.get_presence();
        if !presence.is_null() {
            unsafe {
                (*presence).mark_type(ty as u32, cleared);
            }
        }
    }

    #[inline(always)]
    fn delete_extra(extra: *mut BSExtraData) {
        if extra.is_null() {
            return;
        }
        unsafe {
            (*extra).dtor();
            crate::ffi::commonlib_free(extra.cast());
        }
    }
}

pub trait ExtraDataListExt {
    fn begin(&mut self) -> ExtraDataListIterator;
    fn cbegin(&self) -> ExtraDataListConstIterator;
    fn end(&mut self) -> ExtraDataListIterator;
    fn cend(&self) -> ExtraDataListConstIterator;
    fn get_by_type(&self, ty: ExtraDataType) -> *mut BSExtraData;
    fn get_by_type_typed<U: ExtraDataTyped>(&self) -> *mut U;
    fn has_type(&self, ty: ExtraDataType) -> bool;
    fn remove(&mut self, ty: ExtraDataType, to_remove: *mut BSExtraData) -> bool;
    fn remove_typed<U: ExtraDataTyped>(&mut self, to_remove: *mut U) -> bool;
    fn remove_by_type(&mut self, ty: ExtraDataType) -> bool;
    fn add(&mut self, to_add: *mut BSExtraData) -> *mut BSExtraData;
    fn add_activate_ref_child(&mut self, child_ref: *mut TESObjectREFR);
    fn get_ash_pile_ref(&self) -> ObjectRefHandle;
    fn get_count(&self) -> i32;
    fn get_display_name(&mut self, base_object: *mut TESBoundObject) -> *const c_char;
    fn get_encounter_zone(&self) -> *mut BGSEncounterZone;
    fn get_extra_text_display_data(&mut self) -> *mut ExtraTextDisplayData;
    fn get_linked_ref(&self, keyword: *mut BGSKeyword) -> *mut TESObjectREFR;
    fn get_object_health(&self) -> f32;
    fn get_owner(&self) -> *mut TESForm;
    fn get_soul_level(&self) -> SOUL_LEVEL;
    fn get_teleport_linked_door(&self) -> ObjectRefHandle;
    fn get_worn(&self) -> bool;
    fn has_quest_object_alias(&mut self) -> bool;
    fn set_activate_parent(&mut self, parent_ref: *mut TESObjectREFR, delay: f32);
    fn set_count(&mut self, count: u16);
    fn set_enchantment(
        &mut self,
        enchantment: *mut EnchantmentItem,
        charge_amount: u16,
        remove_on_unequip: bool,
    );
    fn set_encounter_zone(&mut self, zone: *mut BGSEncounterZone);
    fn set_extra_flags(&mut self, flags: ExtraFlagsFlag, enable: bool);
    fn set_inventory_changes(&mut self, changes: *mut InventoryChanges);
    fn set_lev_crea_modifier(&mut self, modifier: LEV_CREA_MODIFIER);
    fn set_linked_ref(&mut self, target_ref: *mut TESObjectREFR, keyword: *mut BGSKeyword);
    fn set_override_name(&mut self, name: *const c_char);
    fn set_owner(&mut self, owner: *mut TESForm);
    fn set_starting_position(
        &mut self,
        refr: *mut TESObjectREFR,
        position: &NiPoint3,
        rotation: &NiPoint3,
        location: *mut BGSLocation,
    );
}

impl<T: AsRef<ExtraDataList> + AsMut<ExtraDataList>> ExtraDataListExt for T {
    fn begin(&mut self) -> ExtraDataListIterator {
        ExtraDataList::begin(self.as_mut())
    }

    fn cbegin(&self) -> ExtraDataListConstIterator {
        ExtraDataList::cbegin(self.as_ref())
    }

    fn end(&mut self) -> ExtraDataListIterator {
        ExtraDataList::end(self.as_mut())
    }

    fn cend(&self) -> ExtraDataListConstIterator {
        ExtraDataList::cend(self.as_ref())
    }

    fn get_by_type(&self, ty: ExtraDataType) -> *mut BSExtraData {
        ExtraDataList::get_by_type(self.as_ref(), ty)
    }

    fn get_by_type_typed<U: ExtraDataTyped>(&self) -> *mut U {
        ExtraDataList::get_by_type_typed(self.as_ref())
    }

    fn has_type(&self, ty: ExtraDataType) -> bool {
        ExtraDataList::has_type(self.as_ref(), ty)
    }

    fn remove(&mut self, ty: ExtraDataType, to_remove: *mut BSExtraData) -> bool {
        ExtraDataList::remove(self.as_mut(), ty, to_remove)
    }

    fn remove_typed<U: ExtraDataTyped>(&mut self, to_remove: *mut U) -> bool {
        ExtraDataList::remove_typed(self.as_mut(), to_remove)
    }

    fn remove_by_type(&mut self, ty: ExtraDataType) -> bool {
        ExtraDataList::remove_by_type(self.as_mut(), ty)
    }

    fn add(&mut self, to_add: *mut BSExtraData) -> *mut BSExtraData {
        ExtraDataList::add(self.as_mut(), to_add)
    }

    fn add_activate_ref_child(&mut self, child_ref: *mut TESObjectREFR) {
        ExtraDataList::add_activate_ref_child(self.as_mut(), child_ref)
    }

    fn get_ash_pile_ref(&self) -> ObjectRefHandle {
        ExtraDataList::get_ash_pile_ref(self.as_ref())
    }

    fn get_count(&self) -> i32 {
        ExtraDataList::get_count(self.as_ref())
    }

    fn get_display_name(&mut self, base_object: *mut TESBoundObject) -> *const c_char {
        ExtraDataList::get_display_name(self.as_mut(), base_object)
    }

    fn get_encounter_zone(&self) -> *mut BGSEncounterZone {
        ExtraDataList::get_encounter_zone(self.as_ref())
    }

    fn get_extra_text_display_data(&mut self) -> *mut ExtraTextDisplayData {
        ExtraDataList::get_extra_text_display_data(self.as_mut())
    }

    fn get_linked_ref(&self, keyword: *mut BGSKeyword) -> *mut TESObjectREFR {
        ExtraDataList::get_linked_ref(self.as_ref(), keyword)
    }

    fn get_object_health(&self) -> f32 {
        ExtraDataList::get_object_health(self.as_ref())
    }

    fn get_owner(&self) -> *mut TESForm {
        ExtraDataList::get_owner(self.as_ref())
    }

    fn get_soul_level(&self) -> SOUL_LEVEL {
        ExtraDataList::get_soul_level(self.as_ref())
    }

    fn get_teleport_linked_door(&self) -> ObjectRefHandle {
        ExtraDataList::get_teleport_linked_door(self.as_ref())
    }

    fn get_worn(&self) -> bool {
        ExtraDataList::get_worn(self.as_ref())
    }

    fn has_quest_object_alias(&mut self) -> bool {
        ExtraDataList::has_quest_object_alias(self.as_mut())
    }

    fn set_activate_parent(&mut self, parent_ref: *mut TESObjectREFR, delay: f32) {
        ExtraDataList::set_activate_parent(self.as_mut(), parent_ref, delay)
    }

    fn set_count(&mut self, count: u16) {
        ExtraDataList::set_count(self.as_mut(), count)
    }

    fn set_enchantment(
        &mut self,
        enchantment: *mut EnchantmentItem,
        charge_amount: u16,
        remove_on_unequip: bool,
    ) {
        ExtraDataList::set_enchantment(self.as_mut(), enchantment, charge_amount, remove_on_unequip)
    }

    fn set_encounter_zone(&mut self, zone: *mut BGSEncounterZone) {
        ExtraDataList::set_encounter_zone(self.as_mut(), zone)
    }

    fn set_extra_flags(&mut self, flags: ExtraFlagsFlag, enable: bool) {
        ExtraDataList::set_extra_flags(self.as_mut(), flags, enable)
    }

    fn set_inventory_changes(&mut self, changes: *mut InventoryChanges) {
        ExtraDataList::set_inventory_changes(self.as_mut(), changes)
    }

    fn set_lev_crea_modifier(&mut self, modifier: LEV_CREA_MODIFIER) {
        ExtraDataList::set_lev_crea_modifier(self.as_mut(), modifier)
    }

    fn set_linked_ref(&mut self, target_ref: *mut TESObjectREFR, keyword: *mut BGSKeyword) {
        ExtraDataList::set_linked_ref(self.as_mut(), target_ref, keyword)
    }

    fn set_override_name(&mut self, name: *const c_char) {
        ExtraDataList::set_override_name(self.as_mut(), name)
    }

    fn set_owner(&mut self, owner: *mut TESForm) {
        ExtraDataList::set_owner(self.as_mut(), owner)
    }

    fn set_starting_position(
        &mut self,
        refr: *mut TESObjectREFR,
        position: &NiPoint3,
        rotation: &NiPoint3,
        location: *mut BGSLocation,
    ) {
        ExtraDataList::set_starting_position(self.as_mut(), refr, position, rotation, location)
    }
}
