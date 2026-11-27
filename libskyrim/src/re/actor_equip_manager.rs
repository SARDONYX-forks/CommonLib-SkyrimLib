use crate::re::{Actor, BGSEquipSlot, ExtraDataList, SpellItem, TESBoundObject, TESShout};
use crate::relocation::RelocationID;

/// C++ `RE::ActorEquipManager` (`BSTSingletonSDM<ActorEquipManager>`).
///
/// The equip/unequip functions take the C++ default arguments explicitly;
/// see [`EquipOptions`] for the usual values.
#[repr(C)]
pub struct ActorEquipManager {
    pub singleton_sdm: u8, // 00 - BSTSingletonSDM
    pub unk01: bool,       // 01
}

const _: () = assert!(core::mem::size_of::<ActorEquipManager>() == 0x2);

/// The trailing `bool` arguments of `EquipObject` / `UnequipObject`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquipOptions {
    pub queue_equip: bool,
    pub force_equip: bool,
    pub play_sounds: bool,
    pub apply_now: bool,
}

impl EquipOptions {
    /// C++ defaults: queue, no force, play sounds, not immediately.
    pub const DEFAULT: Self = Self {
        queue_equip: true,
        force_equip: false,
        play_sounds: true,
        apply_now: false,
    };
}

impl Default for EquipOptions {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl ActorEquipManager {
    crate::relocation_variable! {
        fn singleton() -> &'static *mut ActorEquipManager => RelocationID::new(514494, 400636)
    }

    #[inline(always)]
    pub fn get_singleton() -> *mut ActorEquipManager {
        *Self::singleton()
    }

    crate::relocation_func! {
        #[allow(clippy::too_many_arguments)]
        pub fn equip_object_raw(
            &mut self,
            actor: *mut Actor,
            object: *mut TESBoundObject,
            extra_data: *mut ExtraDataList,
            count: u32,
            slot: *const BGSEquipSlot,
            queue_equip: bool,
            force_equip: bool,
            play_sounds: bool,
            apply_now: bool
        ) => RelocationID::new(37938, 38894)
    }

    crate::relocation_func! {
        pub fn equip_shout(&mut self, actor: *mut Actor, shout: *mut TESShout) => RelocationID::new(37941, 38897)
    }

    crate::relocation_func! {
        pub fn equip_spell(&mut self, actor: *mut Actor, spell: *mut SpellItem, slot: *const BGSEquipSlot) => RelocationID::new(37939, 38895)
    }

    crate::relocation_func! {
        #[allow(clippy::too_many_arguments)]
        pub fn unequip_object_raw(
            &mut self,
            actor: *mut Actor,
            object: *mut TESBoundObject,
            extra_data: *mut ExtraDataList,
            count: u32,
            slot: *const BGSEquipSlot,
            queue_equip: bool,
            force_equip: bool,
            play_sounds: bool,
            apply_now: bool,
            slot_to_replace: *const BGSEquipSlot
        ) -> bool => RelocationID::new(37945, 38901)
    }

    crate::relocation_func! {
        /// `ActorEquipManager::UnEquipSpell` (not in CommonLibSSE-NG):
        /// `hand` is 0 = left, 1 = right, 2 = power/voice.
        pub fn unequip_spell(&mut self, actor: *mut Actor, spell: *mut SpellItem, hand: i32) => RelocationID::new(37947, 38903)
    }

    crate::relocation_func! {
        /// `ActorEquipManager::UnEquipShout` (not in CommonLibSSE-NG).
        pub fn unequip_shout(&mut self, actor: *mut Actor, shout: *mut TESShout) => RelocationID::new(37948, 38904)
    }

    /// C++ `EquipObject`.
    pub fn equip_object(
        &mut self,
        actor: *mut Actor,
        object: *mut TESBoundObject,
        extra_data: *mut ExtraDataList,
        count: u32,
        slot: *const BGSEquipSlot,
        options: EquipOptions,
    ) {
        self.equip_object_raw(
            actor,
            object,
            extra_data,
            count,
            slot,
            options.queue_equip,
            options.force_equip,
            options.play_sounds,
            options.apply_now,
        );
    }

    /// C++ `UnequipObject` (no `slotToReplace`).
    pub fn unequip_object(
        &mut self,
        actor: *mut Actor,
        object: *mut TESBoundObject,
        extra_data: *mut ExtraDataList,
        count: u32,
        slot: *const BGSEquipSlot,
        options: EquipOptions,
    ) -> bool {
        self.unequip_object_raw(
            actor,
            object,
            extra_data,
            count,
            slot,
            options.queue_equip,
            options.force_equip,
            options.play_sounds,
            options.apply_now,
            core::ptr::null(),
        )
    }
}

crate::relocation_func! {
    /// The default `BGSEquipSlot` for the left hand.
    pub fn get_left_hand_slot() -> *mut BGSEquipSlot => RelocationID::new(23150, 23607)
}

crate::relocation_func! {
    /// The default `BGSEquipSlot` for the right hand.
    pub fn get_right_hand_slot() -> *mut BGSEquipSlot => RelocationID::new(23151, 23608)
}

crate::relocation_func! {
    /// The default `BGSEquipSlot` for shouts and powers.
    pub fn get_voice_slot() -> *mut BGSEquipSlot => RelocationID::new(23153, 23610)
}
