use core_util::Enum;

use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::{RTTI_MagicCaster, RTTI_MagicCaster__PostCreationCallback};
use crate::offsets::offsets_vtable::{
    VTABLE_MagicCaster, VTABLE_MagicCaster__PostCreationCallback,
};
use crate::re::ActiveEffect;
use crate::re::Actor;
use crate::re::BGSLoadGameBuffer;
use crate::re::BGSSaveGameBuffer;
use crate::re::BSSoundHandle;
use crate::re::Effect;
use crate::re::MagicItem;
use crate::re::MagicTarget;
use crate::re::MagicTargetPostCreationModification;
use crate::re::NiNode;
use crate::re::NiPoint3;
use crate::re::ObjectRefHandle;
use crate::re::TESBoundObject;
use crate::re::TESObjectCELL;
use crate::re::TESObjectREFR;
use crate::re::bhk_pick_data::bhkPickData;
use crate::re::bst_array::BSTArray;
use crate::re::collision_layers::ColLayer;
use crate::re::magic_system::{CannotCastReason, CastingSource, SoundID};
use crate::relocation::{RelocationID, RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::MagicCaster::State`
#[libskyrim_macros::open_enum]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MagicCasterState {
    None = 0,
    Unk01 = 1,
    Unk02 = 2,
    Ready = 3,
    Unk04 = 4,
    Charging = 5,
    Casting = 6,
    Unk07 = 7,
    Unk08 = 8,
    Unk09 = 9,
}

core_util::impl_enumset_type!(MagicCasterState => u32);

/// C++ `RE::MagicCaster::PostCreationCallback`
#[repr(C)]
pub struct MagicCasterPostCreationCallback {
    pub base: MagicTargetPostCreationModification, // 00
    pub unk08: [u64; 8],                           // 08
}

const _: () = assert!(core::mem::size_of::<MagicCasterPostCreationCallback>() == 0x48);
const _: () = assert!(core::mem::offset_of!(MagicCasterPostCreationCallback, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(MagicCasterPostCreationCallback, unk08) == 0x08);

inherit!(MagicCasterPostCreationCallback : MagicTargetPostCreationModification);

impl RttiType for MagicCasterPostCreationCallback {
    const RTTI: VariantID = RTTI_MagicCaster__PostCreationCallback;
}

impl MagicCasterPostCreationCallback {
    pub const RTTI: VariantID = RTTI_MagicCaster__PostCreationCallback;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicCaster__PostCreationCallback;

    // override (MagicTarget::IPostCreationModification)
    // 0x00 ~MagicTarget::IPostCreationModification
    // 0x01 ModifyActiveEffect
}

/// C++ `RE::MagicCaster`
#[repr(C)]
pub struct MagicCaster {
    pub vtable: *const usize,                  // 00
    pub sounds: BSTArray<BSSoundHandle>,       // 08
    pub desired_target: ObjectRefHandle,       // 20
    pub pad24: u32,                            // 24
    pub current_spell: *mut MagicItem,         // 28
    pub state: EnumSet<MagicCasterState, u32>, // 30
    pub casting_timer: f32,                    // 34
    pub current_spell_cost: f32,               // 38
    pub magnitude_override: f32,               // 3C
    pub next_target_update: f32,               // 40
    pub projectile_timer: f32,                 // 44
}

const _: () = assert!(core::mem::size_of::<MagicCaster>() == 0x48);
const _: () = assert!(core::mem::offset_of!(MagicCaster, vtable) == 0x00);
const _: () = assert!(core::mem::offset_of!(MagicCaster, sounds) == 0x08);
const _: () = assert!(core::mem::offset_of!(MagicCaster, desired_target) == 0x20);
const _: () = assert!(core::mem::offset_of!(MagicCaster, pad24) == 0x24);
const _: () = assert!(core::mem::offset_of!(MagicCaster, current_spell) == 0x28);
const _: () = assert!(core::mem::offset_of!(MagicCaster, state) == 0x30);
const _: () = assert!(core::mem::offset_of!(MagicCaster, casting_timer) == 0x34);
const _: () = assert!(core::mem::offset_of!(MagicCaster, current_spell_cost) == 0x38);
const _: () = assert!(core::mem::offset_of!(MagicCaster, magnitude_override) == 0x3C);
const _: () = assert!(core::mem::offset_of!(MagicCaster, next_target_update) == 0x40);
const _: () = assert!(core::mem::offset_of!(MagicCaster, projectile_timer) == 0x44);

impl RttiType for MagicCaster {
    const RTTI: VariantID = RTTI_MagicCaster;
}

impl AsRef<MagicCaster> for MagicCaster {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<MagicCaster> for MagicCaster {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl MagicCaster {
    pub const RTTI: VariantID = RTTI_MagicCaster;
    pub const VTABLE: &'static [VariantID] = &VTABLE_MagicCaster;

    virtual_method! { pub const VFUNC_DTOR: usize = 0x00; pub fn dtor(&mut self) }
    virtual_method! { pub const VFUNC_CAST_SPELL_IMMEDIATE: usize = 0x01; pub fn cast_spell_immediate(&mut self, spell: *mut MagicItem, no_hit_effect_art: bool, target: *mut TESObjectREFR, effectiveness: f32, hostile_effectiveness_only: bool, magnitude_override: f32, blame_actor: *mut Actor) }
    virtual_method! { pub const VFUNC_FIND_TOUCH_TARGET: usize = 0x02; pub fn find_touch_target(&mut self) }
    virtual_method! { pub const VFUNC_REQUEST_CAST_IMPL: usize = 0x03; pub fn request_cast_impl(&mut self) }
    virtual_method! { pub const VFUNC_START_CHARGE_IMPL: usize = 0x04; pub fn start_charge_impl(&mut self) -> bool }
    virtual_method! { pub const VFUNC_START_READY_IMPL: usize = 0x05; pub fn start_ready_impl(&mut self) }
    virtual_method! { pub const VFUNC_START_CAST_IMPL: usize = 0x06; pub fn start_cast_impl(&mut self) }
    virtual_method! { pub const VFUNC_FINISH_CAST_IMPL: usize = 0x07; pub fn finish_cast_impl(&mut self) }
    virtual_method! { pub const VFUNC_INTERRUPT_CAST_IMPL: usize = 0x08; pub fn interrupt_cast_impl(&mut self, deplete_energy: bool) }
    virtual_method! { pub const VFUNC_SPELL_CAST: usize = 0x09; pub fn spell_cast(&mut self, do_cast: bool, arg2: u32, spell: *mut MagicItem) }
    virtual_method! { pub const VFUNC_CHECK_CAST: usize = 0x0A; pub fn check_cast(&mut self, spell: *mut MagicItem, dual_cast: bool, effect_strength: *mut f32, reason: *mut CannotCastReason, use_base_value_for_cost: bool) -> bool }
    virtual_method! { pub const VFUNC_GET_CASTER_STATS_OBJECT: usize = 0x0B; pub fn get_caster_stats_object() -> *mut TESObjectREFR }
    virtual_method! { pub const VFUNC_GET_CASTER_AS_ACTOR: usize = 0x0C; pub fn get_caster_as_actor() -> *mut Actor }
    virtual_method! { pub const VFUNC_GET_CASTER_OBJECT_REFERENCE: usize = 0x0D; pub fn get_caster_object_reference(out_caster: *mut *mut Actor) -> *mut TESObjectREFR }
    virtual_method! { pub const VFUNC_GET_MAGIC_NODE: usize = 0x0E; pub fn get_magic_node(&mut self) -> *mut NiNode }
    virtual_method! { pub const VFUNC_CLEAR_MAGIC_NODE: usize = 0x0F; pub fn clear_magic_node(&mut self) }
    virtual_method! { pub const VFUNC_SET_CURRENT_SPELL_IMPL: usize = 0x10; pub fn set_current_spell_impl(&mut self, spell: *mut MagicItem) }
    virtual_method! { pub const VFUNC_SELECT_SPELL_IMPL: usize = 0x11; pub fn select_spell_impl(&mut self) }
    virtual_method! { pub const VFUNC_DESELECT_SPELL_IMPL: usize = 0x12; pub fn deselect_spell_impl(&mut self) }
    virtual_method! { pub const VFUNC_SET_SKIP_CHECK_CAST: usize = 0x13; pub fn set_skip_check_cast(&mut self) }
    virtual_method! { pub const VFUNC_SET_CASTING_TIMER_FOR_CHARGE: usize = 0x14; pub fn set_casting_timer_for_charge(&mut self) }
    virtual_method! { pub const VFUNC_GET_CASTING_SOURCE: usize = 0x15; pub fn get_casting_source_raw() -> i32 }
    virtual_method! { pub const VFUNC_GET_IS_DUAL_CASTING: usize = 0x16; pub fn get_is_dual_casting() -> bool }
    virtual_method! { pub const VFUNC_SET_DUAL_CASTING: usize = 0x17; pub fn set_dual_casting(&mut self, set: bool) }
    virtual_method! { pub const VFUNC_SAVE_GAME: usize = 0x18; pub fn save_game(&mut self, buf: *mut BGSSaveGameBuffer) }
    virtual_method! { pub const VFUNC_LOAD_GAME: usize = 0x19; pub fn load_game(&mut self, buf: *mut BGSLoadGameBuffer) }
    virtual_method! { pub const VFUNC_FINISH_LOAD_GAME: usize = 0x1A; pub fn finish_load_game(&mut self, buf: *mut BGSLoadGameBuffer) }
    virtual_method! { pub const VFUNC_PREPARE_SOUND: usize = 0x1B; pub fn prepare_sound(&mut self, sound: SoundID, spell: *mut MagicItem) }
    virtual_method! { pub const VFUNC_ADJUST_ACTIVE_EFFECT: usize = 0x1C; pub fn adjust_active_effect(&mut self, active_effect: *mut ActiveEffect, power: f32, arg3: bool) }

    crate::relocation_func! {
        pub fn find_pick_target(&mut self, target_location: &mut NiPoint3, target_cell: *mut *mut TESObjectCELL, pick_data: &mut bhkPickData) -> *mut MagicTarget => RelocationID::new(33676, 34456)
    }

    crate::relocation_func! {
        pub fn find_targets(&mut self, effectiveness_mult: f32, target_count: &mut u32, source: *mut TESBoundObject, load_cast: bool, adjust_only_hostile_effectiveness: bool) -> bool => RelocationID::new(33632, 34410)
    }

    crate::relocation_func! {
        pub fn finish_cast(&mut self) => RelocationID::new(33657, 34435)
    }

    crate::relocation_func! {
        pub fn get_current_spell_cost(&mut self) -> f32 => RelocationID::new(33426, 34204)
    }

    crate::relocation_func! {
        pub fn interrupt_cast(&mut self, refund: bool) => RelocationID::new(33630, 34408)
    }

    crate::relocation_func! {
        pub fn play_release_sound(&mut self, item: *mut MagicItem) => RelocationID::new(33675, 34455)
    }

    crate::relocation_func! {
        pub fn set_current_spell(&mut self, item: *mut MagicItem) => RelocationID::new(33644, 34422)
    }

    #[inline(always)]
    pub fn casting_source_storage(&self) -> Enum<CastingSource, i32> {
        Enum::from_underlying(self.get_casting_source_raw())
    }

    #[inline(always)]
    pub fn try_get_casting_source(&self) -> Option<CastingSource> {
        self.casting_source_storage().get()
    }

    #[inline(always)]
    pub fn get_casting_source(&self) -> CastingSource {
        self.try_get_casting_source().unwrap_or(CastingSource::None)
    }

    #[inline]
    pub fn test_projectile_placement(effect: &Effect, pick_data: &bhkPickData) -> bool {
        let Some(base_effect) = (unsafe { effect.base_effect.as_ref() }) else {
            return true;
        };

        if base_effect.data.delivery == crate::re::magic_system::Delivery::TargetLocation
            && !base_effect.data.projectile_base.is_null()
        {
            if pick_data.pick_failed || pick_data.ray_output.root_collidable.is_null() {
                return false;
            }

            let col_layer = unsafe {
                (*pick_data.ray_output.root_collidable)
                    .broad_phase_handle
                    .collision_filter_info
            }
            .get_collision_layer();

            return matches!(
                col_layer,
                ColLayer::Static | ColLayer::Terrain | ColLayer::Ground
            );
        }

        true
    }

    crate::relocation_func! {
        pub fn update_impl(&mut self, delta: f32) => RelocationID::new(33622, 34400)
    }
}

pub trait MagicCasterExt {
    fn dtor(&mut self);
    fn cast_spell_immediate(
        &mut self,
        spell: *mut MagicItem,
        no_hit_effect_art: bool,
        target: *mut TESObjectREFR,
        effectiveness: f32,
        hostile_effectiveness_only: bool,
        magnitude_override: f32,
        blame_actor: *mut Actor,
    );
    fn find_touch_target(&mut self);
    fn request_cast_impl(&mut self);
    fn start_charge_impl(&mut self) -> bool;
    fn start_ready_impl(&mut self);
    fn start_cast_impl(&mut self);
    fn finish_cast_impl(&mut self);
    fn interrupt_cast_impl(&mut self, deplete_energy: bool);
    fn spell_cast(&mut self, do_cast: bool, arg2: u32, spell: *mut MagicItem);
    fn check_cast(
        &mut self,
        spell: *mut MagicItem,
        dual_cast: bool,
        effect_strength: *mut f32,
        reason: *mut CannotCastReason,
        use_base_value_for_cost: bool,
    ) -> bool;
    fn get_caster_stats_object(&self) -> *mut TESObjectREFR;
    fn get_caster_as_actor(&self) -> *mut Actor;
    fn get_caster_object_reference(&self, out_caster: *mut *mut Actor) -> *mut TESObjectREFR;
    fn get_magic_node(&mut self) -> *mut NiNode;
    fn clear_magic_node(&mut self);
    fn set_current_spell_impl(&mut self, spell: *mut MagicItem);
    fn select_spell_impl(&mut self);
    fn deselect_spell_impl(&mut self);
    fn set_skip_check_cast(&mut self);
    fn set_casting_timer_for_charge(&mut self);
    fn get_casting_source(&self) -> CastingSource;
    fn get_is_dual_casting(&self) -> bool;
    fn set_dual_casting(&mut self, set: bool);
    fn save_game(&mut self, buf: *mut BGSSaveGameBuffer);
    fn load_game(&mut self, buf: *mut BGSLoadGameBuffer);
    fn finish_load_game(&mut self, buf: *mut BGSLoadGameBuffer);
    fn prepare_sound(&mut self, sound: SoundID, spell: *mut MagicItem);
    fn adjust_active_effect(&mut self, active_effect: *mut ActiveEffect, power: f32, arg3: bool);
    fn find_pick_target(
        &mut self,
        target_location: &mut NiPoint3,
        target_cell: *mut *mut TESObjectCELL,
        pick_data: &mut bhkPickData,
    ) -> *mut MagicTarget;
    fn find_targets(
        &mut self,
        effectiveness_mult: f32,
        target_count: &mut u32,
        source: *mut TESBoundObject,
        load_cast: bool,
        adjust_only_hostile_effectiveness: bool,
    ) -> bool;
    fn finish_cast(&mut self);
    fn get_current_spell_cost(&mut self) -> f32;
    fn interrupt_cast(&mut self, refund: bool);
    fn play_release_sound(&mut self, item: *mut MagicItem);
    fn set_current_spell(&mut self, item: *mut MagicItem);
    fn update_impl(&mut self, delta: f32);
}

impl<T: AsRef<MagicCaster> + AsMut<MagicCaster>> MagicCasterExt for T {
    fn dtor(&mut self) {
        MagicCaster::dtor(self.as_mut())
    }

    fn cast_spell_immediate(
        &mut self,
        spell: *mut MagicItem,
        no_hit_effect_art: bool,
        target: *mut TESObjectREFR,
        effectiveness: f32,
        hostile_effectiveness_only: bool,
        magnitude_override: f32,
        blame_actor: *mut Actor,
    ) {
        MagicCaster::cast_spell_immediate(
            self.as_mut(),
            spell,
            no_hit_effect_art,
            target,
            effectiveness,
            hostile_effectiveness_only,
            magnitude_override,
            blame_actor,
        )
    }

    fn find_touch_target(&mut self) {
        MagicCaster::find_touch_target(self.as_mut())
    }

    fn request_cast_impl(&mut self) {
        MagicCaster::request_cast_impl(self.as_mut())
    }

    fn start_charge_impl(&mut self) -> bool {
        MagicCaster::start_charge_impl(self.as_mut())
    }

    fn start_ready_impl(&mut self) {
        MagicCaster::start_ready_impl(self.as_mut())
    }

    fn start_cast_impl(&mut self) {
        MagicCaster::start_cast_impl(self.as_mut())
    }

    fn finish_cast_impl(&mut self) {
        MagicCaster::finish_cast_impl(self.as_mut())
    }

    fn interrupt_cast_impl(&mut self, deplete_energy: bool) {
        MagicCaster::interrupt_cast_impl(self.as_mut(), deplete_energy)
    }

    fn spell_cast(&mut self, do_cast: bool, arg2: u32, spell: *mut MagicItem) {
        MagicCaster::spell_cast(self.as_mut(), do_cast, arg2, spell)
    }

    fn check_cast(
        &mut self,
        spell: *mut MagicItem,
        dual_cast: bool,
        effect_strength: *mut f32,
        reason: *mut CannotCastReason,
        use_base_value_for_cost: bool,
    ) -> bool {
        MagicCaster::check_cast(
            self.as_mut(),
            spell,
            dual_cast,
            effect_strength,
            reason,
            use_base_value_for_cost,
        )
    }

    fn get_caster_stats_object(&self) -> *mut TESObjectREFR {
        self.as_ref().get_caster_stats_object()
    }

    fn get_caster_as_actor(&self) -> *mut Actor {
        self.as_ref().get_caster_as_actor()
    }

    fn get_caster_object_reference(&self, out_caster: *mut *mut Actor) -> *mut TESObjectREFR {
        self.as_ref().get_caster_object_reference(out_caster)
    }

    fn get_magic_node(&mut self) -> *mut NiNode {
        MagicCaster::get_magic_node(self.as_mut())
    }

    fn clear_magic_node(&mut self) {
        MagicCaster::clear_magic_node(self.as_mut())
    }

    fn set_current_spell_impl(&mut self, spell: *mut MagicItem) {
        MagicCaster::set_current_spell_impl(self.as_mut(), spell)
    }

    fn select_spell_impl(&mut self) {
        MagicCaster::select_spell_impl(self.as_mut())
    }

    fn deselect_spell_impl(&mut self) {
        MagicCaster::deselect_spell_impl(self.as_mut())
    }

    fn set_skip_check_cast(&mut self) {
        MagicCaster::set_skip_check_cast(self.as_mut())
    }

    fn set_casting_timer_for_charge(&mut self) {
        MagicCaster::set_casting_timer_for_charge(self.as_mut())
    }

    fn get_casting_source(&self) -> CastingSource {
        self.as_ref().get_casting_source()
    }

    fn get_is_dual_casting(&self) -> bool {
        self.as_ref().get_is_dual_casting()
    }

    fn set_dual_casting(&mut self, set: bool) {
        MagicCaster::set_dual_casting(self.as_mut(), set)
    }

    fn save_game(&mut self, buf: *mut BGSSaveGameBuffer) {
        MagicCaster::save_game(self.as_mut(), buf)
    }

    fn load_game(&mut self, buf: *mut BGSLoadGameBuffer) {
        MagicCaster::load_game(self.as_mut(), buf)
    }

    fn finish_load_game(&mut self, buf: *mut BGSLoadGameBuffer) {
        MagicCaster::finish_load_game(self.as_mut(), buf)
    }

    fn prepare_sound(&mut self, sound: SoundID, spell: *mut MagicItem) {
        MagicCaster::prepare_sound(self.as_mut(), sound, spell)
    }

    fn adjust_active_effect(&mut self, active_effect: *mut ActiveEffect, power: f32, arg3: bool) {
        MagicCaster::adjust_active_effect(self.as_mut(), active_effect, power, arg3)
    }

    fn find_pick_target(
        &mut self,
        target_location: &mut NiPoint3,
        target_cell: *mut *mut TESObjectCELL,
        pick_data: &mut bhkPickData,
    ) -> *mut MagicTarget {
        MagicCaster::find_pick_target(self.as_mut(), target_location, target_cell, pick_data)
    }

    fn find_targets(
        &mut self,
        effectiveness_mult: f32,
        target_count: &mut u32,
        source: *mut TESBoundObject,
        load_cast: bool,
        adjust_only_hostile_effectiveness: bool,
    ) -> bool {
        MagicCaster::find_targets(
            self.as_mut(),
            effectiveness_mult,
            target_count,
            source,
            load_cast,
            adjust_only_hostile_effectiveness,
        )
    }

    fn finish_cast(&mut self) {
        MagicCaster::finish_cast(self.as_mut())
    }

    fn get_current_spell_cost(&mut self) -> f32 {
        MagicCaster::get_current_spell_cost(self.as_mut())
    }

    fn interrupt_cast(&mut self, refund: bool) {
        MagicCaster::interrupt_cast(self.as_mut(), refund)
    }

    fn play_release_sound(&mut self, item: *mut MagicItem) {
        MagicCaster::play_release_sound(self.as_mut(), item)
    }

    fn set_current_spell(&mut self, item: *mut MagicItem) {
        MagicCaster::set_current_spell(self.as_mut(), item)
    }

    fn update_impl(&mut self, delta: f32) {
        MagicCaster::update_impl(self.as_mut(), delta)
    }
}
