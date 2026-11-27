#![allow(non_camel_case_types)]

use core::ffi::{CStr, c_char, c_void};

use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_PlayerCharacter;
use crate::offsets::offsets_vtable::VTABLE_PlayerCharacter;
use crate::re::bgs_texture_set::BGSTextureSet;
use crate::re::bst_hash_map::{UnkKey, UnkValue};
use crate::re::crime::CrimeType;
use crate::re::crosshair_pick_data::VR_DEVICE;
use crate::re::magic_system::{CannotCastReason, SpellType};
use crate::re::tes_quest::TESQuestTarget;
use crate::re::tes_race::TESRace;
use crate::re::{
    AITimeStamp, AcquireType, Actor, ActorHandle, ActorValue, AlchemyItem, BGSActorCellEvent,
    BGSActorDeathEvent, BGSInstancedQuestObjective, BGSLocation, BGSNote, BGSPerk, BIPED_OBJECT,
    BSFadeNode, BSHandleRefObject, BSLight, BSSimpleList, BSSoundHandle, BSSpinLock, BSTArray,
    BSTEventSink, BSTEventSource, BSTHashMap, BSTSmallArray, BSTSmartPointer, BSTTuple, BSTriShape,
    BipedAnim, Character, CombatGroup, Crime, DEFAULT_OBJECT, DialoguePackage, Effect,
    FormCastable, FormType, ImageSpaceModifierInstanceDOF, InventoryEntryData, MagicItem,
    MenuModeChangeEvent, MenuOpenCloseEvent, NiAVObject, NiBillboardNode, NiNode, NiPoint3,
    NiPointer, NiRef, ObjectRefHandle, PerkRankData, PositionPlayerEvent, ProjectileHandle,
    RefHandle, TESBoundObject, TESClass, TESFaction, TESForm, TESImageSpaceModifier, TESObject,
    TESObjectCELL, TESObjectREFR, TESObjectWEAP, TESQuest, TESQuestStageItem, TESTrackedStatsEvent,
    TESWorldSpace, TeleportPath, TintMask, UserEventEnabledEvent, VRDeviceConnectionChange,
    VROverlayChange, VRResetHMDHeight, WeaponType, bhkMouseSpringAction, hkRefPtr,
};
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset};

/// C++ `RE::PLAYER_ACTION`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PLAYER_ACTION {
    None = 0,
    SwingMeleeWeapon = 1,
    CastProjectileSpell = 2,
    ShootBow = 3,
    ZKeyObject = 4,
    Jumping = 5,
    KnockingOverObjects = 6,
    StandOnTableChair = 7,
    IronSights = 8,
    DestroyObject = 9,
    LockedObject = 10,
    Pickpocket = 11,
    CastSelfSpell = 12,
    Shout = 13,
    ActorCollision = 14,
    Total = 15,
    InvalidMarker = 16,
}

core_util::impl_enumset_type!(PLAYER_ACTION => u32);

/// C++ `RE::VR_Bow_State`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VR_Bow_State {
    None = 0,
    NoAmmo = 1,
    Idle = 2,
    ArrowKnocked = 3,
}

/// C++ `RE::CrimeGoldStruct`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CrimeGoldStruct {
    pub violent_cur: f32,        // 00
    pub non_violent_cur: f32,    // 04
    pub non_violent_infamy: f32, // 08
    pub violent_infamy: f32,     // 0C
}

const _: () = assert!(core::mem::size_of::<CrimeGoldStruct>() == 0x10);

/// C++ `RE::StolenItemValueStruct`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct StolenItemValueStruct {
    pub unwitnessed: i32, // 00
    pub witnessed: i32,   // 04
}

const _: () = assert!(core::mem::size_of::<StolenItemValueStruct>() == 0x08);

/// C++ `RE::FriendshipFactionsStruct`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FriendshipFactionsStruct {
    pub friend_counts: [u16; 4], // 00
}

const _: () = assert!(core::mem::size_of::<FriendshipFactionsStruct>() == 0x08);

/// C++ `RE::PLAYER_TARGET_LOC`
#[repr(C)]
pub struct PLAYER_TARGET_LOC {
    pub world: *mut TESWorldSpace,     // 00
    pub interior: *mut TESObjectCELL,  // 08
    pub location: NiPoint3,            // 10
    pub angle: NiPoint3,               // 1C
    pub arrival_func: *mut c_void,     // 28
    pub arrival_func_data: i64,        // 30
    pub furniture_ref: RefHandle,      // 38
    pub fast_travel_marker: RefHandle, // 3C
    pub reset_weather: bool,           // 40
    pub allow_auto_save: bool,         // 41
    pub is_valid: bool,                // 42
    pub pad43: u8,                     // 43
    pub pad44: u32,                    // 44
}

const _: () = assert!(core::mem::size_of::<PLAYER_TARGET_LOC>() == 0x48);

/// C++ `RE::VR_PLAYER_TARGET_LOC`
#[repr(C)]
pub struct VR_PLAYER_TARGET_LOC {
    pub world: *mut TESWorldSpace,     // 00
    pub interior: *mut TESObjectCELL,  // 08
    pub location: NiPoint3,            // 10
    pub angle: NiPoint3,               // 1C
    pub arrival_func: *mut c_void,     // 28
    pub arrival_func_data: i64,        // 30
    pub furniture_ref: RefHandle,      // 38
    pub fast_travel_marker: RefHandle, // 3C
    pub unk40: f32,                    // 40
    pub unk44: u8,                     // 44
    pub reset_weather: bool,           // 45
    pub allow_auto_save: u8,           // 46
    pub is_valid: bool,                // 47
    pub unk48: u8,                     // 48
    pub unk49: u8,                     // 49
    pub unk4_a: u8,                    // 4A
    pub unk4_b: u8,                    // 4B
    pub unk4_c: u32,                   // 4C
}

const _: () = assert!(core::mem::size_of::<VR_PLAYER_TARGET_LOC>() == 0x50);

/// Cross-runtime raw head block shared by flat and VR queued target locations.
#[repr(C)]
pub struct PLAYER_RUNTIME_TARGET_LOC_DATA {
    pub world: *mut TESWorldSpace,     // 00
    pub interior: *mut TESObjectCELL,  // 08
    pub location: NiPoint3,            // 10
    pub angle: NiPoint3,               // 1C
    pub arrival_func: *mut c_void,     // 28
    pub arrival_func_data: i64,        // 30
    pub furniture_ref: RefHandle,      // 38
    pub fast_travel_marker: RefHandle, // 3C
}

const _: () = assert!(core::mem::size_of::<PLAYER_RUNTIME_TARGET_LOC_DATA>() == 0x40);
const _: () =
    assert!(core::mem::offset_of!(PLAYER_RUNTIME_TARGET_LOC_DATA, arrival_func_data) == 0x30);
const _: () =
    assert!(core::mem::offset_of!(PLAYER_RUNTIME_TARGET_LOC_DATA, fast_travel_marker) == 0x3C);

/// C++ `RE::PlayerActionObject`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerActionObject {
    pub timer: f32,                        // 00
    pub ref_obj: RefHandle,                // 04
    pub next: EnumSet<PLAYER_ACTION, u32>, // 08
}

const _: () = assert!(core::mem::size_of::<PlayerActionObject>() == 0x0C);

/// C++ `RE::PlayerCharacter::GrabbingType`
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GrabbingType {
    None = 0,
    Normal = 1,
    Telekinesis = 2,
}

core_util::impl_enumset_type!(GrabbingType => u32);

/// C++ `RE::PlayerCharacter::ByCharGenFlag`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ByCharGenFlag {
    None = 0,
    DisableSaving = 1 << 0,
    DisableWaiting = 1 << 1,
    ShowControlsDisabledMessage = 1 << 2,
}

core_util::impl_enumset_type!(ByCharGenFlag => u8);

/// C++ `RE::PlayerCharacter::GrabData`
#[repr(C)]
pub struct GrabData {
    pub grab_spring: BSTSmallArray<
        hkRefPtr<bhkMouseSpringAction>,
        { core::mem::size_of::<hkRefPtr<bhkMouseSpringAction>>() * 4 },
    >, // 00
    pub grabbed_object: ObjectRefHandle, // 30
    pub grab_object_weight: f32,         // 34
    pub grab_distance: f32,              // 38
    pub unk004: f32,                     // 3C
    pub unk008: u64,                     // 40
}

const _: () = assert!(core::mem::size_of::<GrabData>() == 0x48);

/// C++ `RE::PlayerCharacter::VRGrabData`
#[repr(C)]
pub struct VRGrabData {
    pub grab_spring: BSTSmallArray<
        hkRefPtr<bhkMouseSpringAction>,
        { core::mem::size_of::<hkRefPtr<bhkMouseSpringAction>>() * 4 },
    >, // 00
    pub grabbed_object: ObjectRefHandle, // 30
    pub grab_object_weight: f32,         // 34
    pub grab_type: GrabbingType,         // 38
    pub grab_distance: f32,              // 3C
    pub unk40: f64,                      // 40
    pub unk48: u64,                      // 48
    pub unk50: f64,                      // 50
    pub unk58: u64,                      // 58
    pub unk60: u32,                      // 60
    pub unk64_flags: u32,                // 64
}

const _: () = assert!(core::mem::size_of::<VRGrabData>() == 0x68);

/// Raw C++ `PlayerFlags` bitfield storage.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PlayerFlags {
    pub bits: u64, // 00
}

const _: () = assert!(core::mem::size_of::<PlayerFlags>() == 0x08);

/// C++ `RE::PlayerCharacter::QueuedWeapon`
#[repr(C)]
pub struct QueuedWeapon {
    pub right_hand_weapon: *mut TESObjectWEAP, // 00
    pub left_hand_weapon: *mut TESObjectWEAP,  // 08
}

const _: () = assert!(core::mem::size_of::<QueuedWeapon>() == 0x10);

/// C++ `RE::PlayerCharacter::PreTransformationData`
#[repr(C)]
pub struct PreTransformationData {
    pub stored_selected_spells: [*mut MagicItem; 4], // 00
    pub stored_race: *mut TESRace,                   // 20
    pub stored_selected_power: *mut TESForm,         // 28
    pub stored_last_one_hand_items: [*mut TESBoundObject; 2], // 30
}

const _: () = assert!(core::mem::size_of::<PreTransformationData>() == 0x40);

/// C++ `RE::PlayerCharacter::CrimeValue`
#[repr(C)]
pub struct CrimeValue {
    pub crime_gold_map: BSTHashMap<*const TESFaction, CrimeGoldStruct>, // 00
    pub stolen_item_value_map: BSTHashMap<*const TESFaction, StolenItemValueStruct>, // 30
}

const _: () = assert!(core::mem::size_of::<CrimeValue>() == 0x60);

/// C++ `RE::PlayerCharacter::RaceData`
#[repr(C)]
pub struct PlayerCharacterRaceData {
    pub complexion: *mut BGSTextureSet, // 00
    pub char_gen_race: *mut TESRace,    // 08
    pub race2: *mut TESRace,            // 10
}

const _: () = assert!(core::mem::size_of::<PlayerCharacterRaceData>() == 0x18);

/// C++ `RE::PlayerCharacter::GameStateData`
#[repr(C)]
pub struct GameStateData {
    pub difficulty: i32,                              // 00
    pub assumed_identity: ActorHandle,                // 04
    pub murder: i8,                                   // 08
    pub perk_count: u8,                               // 09
    pub by_char_gen_flag: EnumSet<ByCharGenFlag, u8>, // 0A
    pub pad_b: u8,                                    // 0B
}

const _: () = assert!(core::mem::size_of::<GameStateData>() == 0x0C);

/// C++ `RE::PlayerCharacter::PlayerSkills::Data::Skills::Skill`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerSkill {
    OneHanded = 0,
    TwoHanded = 1,
    Archery = 2,
    Block = 3,
    Smithing = 4,
    HeavyArmor = 5,
    LightArmor = 6,
    Pickpocket = 7,
    Lockpicking = 8,
    Sneak = 9,
    Alchemy = 10,
    Speech = 11,
    Alteration = 12,
    Conjuration = 13,
    Destruction = 14,
    Illusion = 15,
    Restoration = 16,
    Enchanting = 17,
    Total = 18,
}

/// C++ `RE::PlayerCharacter::PlayerSkills::Data::SkillData`
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PlayerSkillData {
    pub level: f32,           // 00
    pub xp: f32,              // 04
    pub level_threshold: f32, // 08
}

const _: () = assert!(core::mem::size_of::<PlayerSkillData>() == 0x0C);

/// C++ `RE::PlayerCharacter::PlayerSkills::Data`
#[repr(C)]
pub struct PlayerSkillsData {
    pub xp: f32,                                                // 000
    pub level_threshold: f32,                                   // 004
    pub skills: [PlayerSkillData; PlayerSkill::Total as usize], // 008
    pub legendary_levels: [u32; PlayerSkill::Total as usize],   // 0E0
}

const _: () = assert!(core::mem::size_of::<PlayerSkillsData>() == 0x128);

/// C++ `RE::PlayerCharacter::PlayerSkills`
#[repr(C)]
pub struct PlayerSkills {
    pub data: *mut PlayerSkillsData, // 00
}

const _: () = assert!(core::mem::size_of::<PlayerSkills>() == 0x08);

impl PlayerSkills {
    #[inline(always)]
    pub fn can_level_up(&self) -> bool {
        unsafe { (*self.data).level_threshold <= (*self.data).xp }
    }

    crate::relocation_func! {
        pub fn advance_level(&mut self, add_threshold: bool) => RelocationID::new(40560, 41567)
    }
}

/// C++ `RE::PlayerCharacter::INFO_RUNTIME_DATA`
#[repr(C, packed(4))]
pub struct INFO_RUNTIME_DATA {
    pub sleep_seconds: u32,                                        // 000
    pub large_biped: BSTSmartPointer<BipedAnim>,                   // 004
    pub first_person_3d: NiPointer<NiNode>,                        // 00C
    pub eye_height: f32,                                           // 014
    pub greet_timer: f32,                                          // 018
    pub encumbered_timer: f32,                                     // 01C
    pub power_attack_timer: f32,                                   // 020
    pub hours_to_sleep: i32,                                       // 024
    pub amount_stolen_sold: i32,                                   // 028
    pub value_stolen: u32,                                         // 02C
    pub last_ridden_mount: ActorHandle,                            // 030
    pub light_target: ActorHandle,                                 // 034
    pub sort_actor_distance_timer: f32,                            // 038
    pub sit_heading_delta: f32,                                    // 03C
    pub player_map_marker: ObjectRefHandle,                        // 040
    pub player_marker_path: *mut TeleportPath,                     // 044
    pub skill_trainings_this_level: u32,                           // 04C
    pub unk064: u32,                                               // 050
    pub default_class: *mut TESClass,                              // 054
    pub unk070: u64,                                               // 05C
    pub crime_counts: [u32; CrimeType::TOTAL],                     // 064
    pub unk094: u32,                                               // 080
    pub pending_poison: *mut AlchemyItem,                          // 084
    pub last_playing_time_update: i64,                             // 08C
    pub total_playing_time: i64,                                   // 094
    pub character_seed: i32,                                       // 09C
    pub unk0_b4: u32,                                              // 0A0
    pub last_known_good_location: *mut TESForm,                    // 0A4
    pub unk0_c0: u32,                                              // 0AC
    pub unk0_c4: u32,                                              // 0B0
    pub first_person_light: NiPointer<BSLight>,                    // 0B4
    pub third_person_light: NiPointer<BSLight>,                    // 0BC
    pub drop_angle_mod: f32,                                       // 0C4
    pub last_drop_angle_mod: f32,                                  // 0C8
    pub skills: *mut PlayerSkills,                                 // 0CC
    pub auto_aim_actor: ActorHandle,                               // 0D4
    pub unk0_ec: RefHandle,                                        // 0D8
    pub unk0_f0: u64,                                              // 0DC
    pub targeted_3d: NiPointer<NiAVObject>,                        // 0E4
    pub combat_group: *mut CombatGroup,                            // 0EC
    pub actors_to_display_on_the_hud_array: BSTArray<ActorHandle>, // 0F4
    pub advance_object: *mut TESForm,                              // 10C
    pub last_one_hand_items: [*mut TESBoundObject; 2],             // 114
    pub teammate_count: u32,                                       // 124
    pub combat_timer: f32,                                         // 128
    pub yield_timer: f32,                                          // 12C
    pub chase_timer: f32,                                          // 130
    pub draw_sheathe_safety_timer: f32,                            // 134
    pub unk14_c: u32,                                              // 138
}

const _: () = assert!(core::mem::size_of::<INFO_RUNTIME_DATA>() == 0x13C);
const _: () = assert!(core::mem::offset_of!(INFO_RUNTIME_DATA, sleep_seconds) == 0x00);
const _: () = assert!(core::mem::offset_of!(INFO_RUNTIME_DATA, player_marker_path) == 0x44);
const _: () = assert!(core::mem::offset_of!(INFO_RUNTIME_DATA, skills) == 0xCC);

/// C++ `RE::PlayerCharacter::VR_INFO_RUNTIME_DATA`
#[repr(C)]
pub struct VR_INFO_RUNTIME_DATA {
    pub sleep_seconds: u32,                                        // 000
    pub unk_fe4: u32,                                              // 004
    pub large_biped: BSTSmartPointer<BipedAnim>,                   // 008
    pub first_person_3d: NiPointer<NiNode>,                        // 010
    pub eye_height: f32,                                           // 018
    pub greet_timer: f32,                                          // 01C
    pub encumbered_timer: f32,                                     // 020
    pub power_attack_timer: f32,                                   // 024
    pub hours_to_sleep: i32,                                       // 028
    pub amount_stolen_sold: i32,                                   // 02C
    pub value_stolen: u32,                                         // 030
    pub last_ridden_mount: ActorHandle,                            // 034
    pub light_target: ActorHandle,                                 // 038
    pub sort_actor_distance_timer: f32,                            // 03C
    pub player_map_marker: ObjectRefHandle,                        // 040
    pub pad1024: u32,                                              // 044
    pub player_marker_path: *mut TeleportPath,                     // 048
    pub skill_trainings_this_level: u32,                           // 050
    pub unk1034: u32,                                              // 054
    pub default_class: *mut TESClass,                              // 058
    pub unk1040: u64,                                              // 060
    pub crime_counts: [u32; CrimeType::TOTAL],                     // 068
    pub unk1064: u32,                                              // 084
    pub pending_poison: *mut AlchemyItem,                          // 088
    pub last_playing_time_update: i64,                             // 090
    pub total_playing_time: i64,                                   // 098
    pub character_seed: i32,                                       // 0A0
    pub unk1084: u32,                                              // 0A4
    pub last_known_good_location: *mut TESForm,                    // 0A8
    pub unk1090: u32,                                              // 0B0
    pub unk1094: u32,                                              // 0B4
    pub first_person_light: NiPointer<BSLight>,                    // 0B8
    pub third_person_light: NiPointer<BSLight>,                    // 0C0
    pub drop_angle_mod: f32,                                       // 0C8
    pub last_drop_angle_mod: f32,                                  // 0CC
    pub skills: *mut PlayerSkills,                                 // 0D0
    pub auto_aim_actor: ActorHandle,                               // 0D8
    pub unk10_bc: RefHandle,                                       // 0DC
    pub unk10_c0: u64,                                             // 0E0
    pub targeted_3d: NiPointer<NiAVObject>,                        // 0E8
    pub combat_group: *mut CombatGroup,                            // 0F0
    pub actors_to_display_on_the_hud_array: BSTArray<ActorHandle>, // 0F8
    pub advance_object: *mut TESForm,                              // 110
    pub last_one_hand_items: [*mut TESBoundObject; 2],             // 118
    pub teammate_count: u32,                                       // 128
    pub combat_timer: f32,                                         // 12C
    pub yield_timer: f32,                                          // 130
    pub chase_timer: f32,                                          // 134
    pub draw_sheathe_safety_timer: f32,                            // 138
    pub unk111_c: u32,                                             // 13C
}

const _: () = assert!(core::mem::size_of::<VR_INFO_RUNTIME_DATA>() == 0x140);
const _: () = assert!(core::mem::offset_of!(VR_INFO_RUNTIME_DATA, sleep_seconds) == 0x00);
const _: () = assert!(core::mem::offset_of!(VR_INFO_RUNTIME_DATA, skills) == 0xD0);

/// Cross-runtime raw head block shared by flat and VR info runtime data.
#[repr(C)]
pub struct PLAYER_INFO_RUNTIME_HEAD_DATA {
    pub large_biped: BSTSmartPointer<BipedAnim>, // 00
    pub first_person_3d: NiPointer<NiNode>,      // 08
    pub eye_height: f32,                         // 10
    pub greet_timer: f32,                        // 14
    pub encumbered_timer: f32,                   // 18
    pub power_attack_timer: f32,                 // 1C
    pub hours_to_sleep: i32,                     // 20
    pub amount_stolen_sold: i32,                 // 24
    pub value_stolen: u32,                       // 28
    pub last_ridden_mount: ActorHandle,          // 2C
    pub light_target: ActorHandle,               // 30
    pub sort_actor_distance_timer: f32,          // 34
}

const _: () = assert!(core::mem::size_of::<PLAYER_INFO_RUNTIME_HEAD_DATA>() == 0x38);
const _: () = assert!(core::mem::offset_of!(PLAYER_INFO_RUNTIME_HEAD_DATA, large_biped) == 0x00);
const _: () = assert!(
    core::mem::offset_of!(PLAYER_INFO_RUNTIME_HEAD_DATA, sort_actor_distance_timer) == 0x34
);

/// Cross-runtime raw tail block shared by flat and VR info runtime data.
#[repr(C)]
pub struct PLAYER_INFO_RUNTIME_TAIL_DATA {
    pub player_marker_path: *mut TeleportPath,  // 000
    pub skill_trainings_this_level: u32,        // 008
    pub unk0c: u32,                             // 00C
    pub default_class: *mut TESClass,           // 010
    pub unk18: u64,                             // 018
    pub crime_counts: [u32; CrimeType::TOTAL],  // 020
    pub unk3c: u32,                             // 03C
    pub pending_poison: *mut AlchemyItem,       // 040
    pub last_playing_time_update: i64,          // 048
    pub total_playing_time: i64,                // 050
    pub character_seed: i32,                    // 058
    pub unk5c: u32,                             // 05C
    pub last_known_good_location: *mut TESForm, // 060
    pub unk68: u32,                             // 068
    pub unk6c: u32,                             // 06C
    pub first_person_light: NiPointer<BSLight>, // 070
    pub third_person_light: NiPointer<BSLight>, // 078
    pub drop_angle_mod: f32,                    // 080
    pub last_drop_angle_mod: f32,               // 084
    pub skills: *mut PlayerSkills,              // 088
    pub auto_aim_actor: ActorHandle,            // 090
    pub unk94: RefHandle,                       // 094
    pub unk98: u64,                             // 098
    pub targeted_3d: NiPointer<NiAVObject>,     // 0A0
    pub combat_group: *mut CombatGroup,         // 0A8
    pub actors_to_display_on_the_hud_array: BSTArray<ActorHandle>, // 0B0
    pub advance_object: *mut TESForm,           // 0C8
    pub last_one_hand_items: [*mut TESBoundObject; 2], // 0D0
    pub teammate_count: u32,                    // 0E0
    pub combat_timer: f32,                      // 0E4
    pub yield_timer: f32,                       // 0E8
    pub chase_timer: f32,                       // 0EC
    pub draw_sheathe_safety_timer: f32,         // 0F0
    pub unk_f4: u32,                            // 0F4
}

const _: () = assert!(core::mem::size_of::<PLAYER_INFO_RUNTIME_TAIL_DATA>() == 0xF8);
const _: () =
    assert!(core::mem::offset_of!(PLAYER_INFO_RUNTIME_TAIL_DATA, player_marker_path) == 0x00);
const _: () = assert!(core::mem::offset_of!(PLAYER_INFO_RUNTIME_TAIL_DATA, skills) == 0x88);
const _: () = assert!(core::mem::offset_of!(PLAYER_INFO_RUNTIME_TAIL_DATA, combat_timer) == 0xE4);

/// C++ `RE::VR_NODE_DATA`
#[repr(C)]
pub struct VR_NODE_DATA {
    pub player_world_node: NiPointer<NiNode>,            // 000
    pub follow_node: NiPointer<NiNode>,                  // 008
    pub follow_offset: NiPointer<NiNode>,                // 010
    pub height_offset_node: NiPointer<NiNode>,           // 018
    pub snap_walk_offset_node: NiPointer<NiNode>,        // 020
    pub room_node: NiPointer<NiNode>,                    // 028
    pub black_sphere: NiPointer<NiNode>,                 // 030
    pub ui_node: NiPointer<NiNode>,                      // 038
    pub in_world_ui_quad_geo: NiPointer<BSTriShape>,     // 040
    pub ui_pointer_node: NiPointer<NiNode>,              // 048
    pub ui_pointer_geo: NiPointer<BSTriShape>,           // 050
    pub dialogue_ui_node: NiPointer<NiNode>,             // 058
    pub teleport_destination_preview: NiPointer<NiNode>, // 060
    pub teleport_destination_fail: NiPointer<NiNode>,    // 068
    pub teleport_sprint_preview: NiPointer<NiNode>,      // 070
    pub spell_origin: NiPointer<NiNode>,                 // 078
    pub spell_destination: NiPointer<NiNode>,            // 080
    pub arrow_origin: NiPointer<NiNode>,                 // 088
    pub arrow_destination: NiPointer<NiNode>,            // 090
    pub quest_marker: NiPointer<NiNode>,                 // 098
    pub left_wand_node: NiPointer<NiNode>,               // 0A0
    pub left_wand_shake_node: NiPointer<NiNode>,         // 0A8
    pub left_valve_index_controller_node: NiPointer<NiNode>, // 0B0
    pub unk_node_0_b8: NiPointer<NiNode>,                // 0B8
    pub left_weapon_offset_node: NiPointer<NiNode>,      // 0C0
    pub left_crossbow_offset_node: NiPointer<NiNode>,    // 0C8
    pub left_melee_weapon_offset_node: NiPointer<NiNode>, // 0D0
    pub left_staff_weapon_offset_node: NiPointer<NiNode>, // 0D8
    pub left_shield_offset_node: NiPointer<NiNode>,      // 0E0
    pub right_shield_offset_node: NiPointer<NiNode>,     // 0E8
    pub secondary_magic_offset_node: NiPointer<NiNode>,  // 0F0
    pub secondary_magic_aim_node: NiPointer<NiNode>,     // 0F8
    pub secondary_staff_magic_offset_node: NiPointer<NiNode>, // 100
    pub right_wand_node: NiPointer<NiNode>,              // 108
    pub right_wand_shake_node: NiPointer<NiNode>,        // 110
    pub right_valve_index_controller_node: NiPointer<NiNode>, // 118
    pub unk_node_120: NiPointer<NiNode>,                 // 120
    pub right_weapon_offset_node: NiPointer<NiNode>,     // 128
    pub right_crossbow_offset_node: NiPointer<NiNode>,   // 130
    pub right_melee_weapon_offset_node: NiPointer<NiNode>, // 138
    pub right_staff_weapon_offset_node: NiPointer<NiNode>, // 140
    pub primary_magic_offset_node: NiPointer<NiNode>,    // 148
    pub primary_magic_aim_node: NiPointer<NiNode>,       // 150
    pub primary_staff_magic_offset_node: NiPointer<NiNode>, // 158
    pub unk160: u64,                                     // 160
    pub crosshair_parent: NiPointer<NiBillboardNode>,    // 168
    pub crosshair_secondary_parent: NiPointer<NiBillboardNode>, // 170
    pub target_lock_parent: NiPointer<NiBillboardNode>,  // 178
    pub gamepad_node: NiPointer<NiNode>,                 // 180
    pub last_sync_pos: NiPointer<NiNode>,                // 188
    pub upright_hmd_node: NiPointer<NiNode>,             // 190
    pub map_markers_3d: NiPointer<NiNode>,               // 198
    pub npc_lhnd: NiPointer<NiNode>,                     // 1A0
    pub npc_rhnd: NiPointer<NiNode>,                     // 1A8
    pub npc_lclv: NiPointer<NiNode>,                     // 1B0
    pub npc_rclv: NiPointer<NiNode>,                     // 1B8
    pub unk1_c0: u32,                                    // 1C0
    pub unk1_c4: u32,                                    // 1C4
    pub unk1_c8: u64,                                    // 1C8
    pub bow_state: VR_Bow_State,                         // 1D0
    pub unk1_d4: u32,                                    // 1D4
    pub bow_aim_node: NiPointer<NiNode>,                 // 1D8
    pub bow_rotation_node: NiPointer<NiNode>,            // 1E0
    pub arrow_snap_node: NiPointer<NiNode>,              // 1E8
    pub arrow_node: NiPointer<BSFadeNode>,               // 1F0
    pub arrow_fire_node: NiPointer<BSFadeNode>,          // 1F8
    pub unk200: u64,                                     // 200
    pub arrow_hold_offset_node: NiPointer<NiNode>,       // 208
    pub arrow_hold_node: NiPointer<NiNode>,              // 210
    pub unk218: u64,                                     // 218
    pub current_arrow_snap_distance: f32,                // 220
    pub unk224: u32,                                     // 224
    pub current_bow_draw_amount: f32,                    // 228
    pub last_rumble_bow_draw_amount: f32,                // 22C
    pub unk230: u64,                                     // 230
    pub unk238: u64,                                     // 238
    pub unk240: u64,                                     // 240
    // TODO: `PlayerCharacter.h` still leaves these arrays as `void*` placeholders in the source.
    pub quest_marker_billboards_node_array: *mut c_void, // 248
    pub teleport_node_array: *mut c_void,                // 250
    pub quest_marker_billboards_node_array2: *mut c_void, // 258
    pub unk260: u64,                                     // 260
    pub teleport_node_array2: *mut c_void,               // 268
    pub quest_marker_billboards_node_array3: *mut c_void, // 270
    pub unk278: u64,                                     // 278
    pub unk_float_280: f32,                              // 280
    pub unk284: u32,                                     // 284
    pub teleport_node_array3: *mut c_void,               // 288
}

const _: () = assert!(core::mem::size_of::<VR_NODE_DATA>() == 0x290);
const _: () =
    assert!(core::mem::offset_of!(VR_NODE_DATA, quest_marker_billboards_node_array) == 0x248);
const _: () = assert!(core::mem::offset_of!(VR_NODE_DATA, teleport_node_array3) == 0x288);

/// Flat-runtime layout from C++ `RE::PlayerCharacter::PLAYER_RUNTIME_DATA`.
///
/// CommonLib exposes `GetPlayerRuntimeData()` as a versioned SE/AE accessor and the non-VR source
/// uses that whole-tail accessor directly. Keep this as the raw `_flat` runtime-data struct for
/// now. If future source-backed work proves an AE-only divergence inside this tail, split the
/// shared prefix from the version-specific overlays instead of pushing that complexity to callers.
#[repr(C)]
pub struct PLAYER_RUNTIME_DATA {
    pub quest_targets_lock: BSSpinLock,       // 000
    pub crime_value: CrimeValue,              // 008
    pub command_wait_marker: ObjectRefHandle, // 068
    pub pad444: u32,                          // 06C
    pub faction_owner_friends_map: BSTHashMap<*const TESFaction, FriendshipFactionsStruct>, // 070
    pub last_known_good_position: NiPoint3,   // 0A0
    pub bullet_auto_aim: NiPoint3,            // 0AC
    pub cached_velocity: NiPoint3,            // 0B8
    pub pad49_c: f32,                         // 0C4
    pub unused_note: *mut BGSNote,            // 0C8
    pub unused_note2: *mut BGSNote,           // 0D0
    pub added_perks: BSTArray<*mut PerkRankData>, // 0D8
    pub perks: BSTArray<*mut BGSPerk>,        // 0F0
    pub standing_stone_perks: BSTArray<*mut BGSPerk>, // 108
    pub current_map_markers: BSTArray<ObjectRefHandle>, // 120
    pub velocity_array: BSTArray<BSTTuple<NiPoint3, AITimeStamp>>, // 138
    pub runes_cast: BSTArray<ProjectileHandle>, // 150
    pub image_space_modifier_anims1: BSTArray<*mut c_void>, // 168
    pub image_space_modifier_anims2: BSTArray<*mut c_void>, // 180
    pub quest_log: BSSimpleList<*mut TESQuestStageItem>, // 198
    pub objectives: BSTArray<BGSInstancedQuestObjective>, // 1A8
    pub quest_targets: BSTHashMap<*mut TESQuest, *mut BSTArray<*mut TESQuestTarget>>, // 1C0
    pub current_say_once_infos_map: BSTHashMap<UnkKey, UnkValue>, // 1F0
    pub dropped_ref_list: BSSimpleList<ObjectRefHandle>, // 220
    pub random_door_space_map: crate::re::NiTMap<u32, u8>, // 230
    pub cached_world_space: *mut TESWorldSpace, // 250
    pub exterior_position: NiPoint3,          // 258
    pub pad63_c: u32,                         // 264
    pub queued_target_loc: PLAYER_TARGET_LOC, // 268
    pub unused_sound: BSSoundHandle,          // 2B0
    pub magic_failure_sound: BSSoundHandle,   // 2BC
    pub shout_failure_sound: BSSoundHandle,   // 2C8
    pub pad6_ac: u32,                         // 2D4
    pub closest_conversation: *mut DialoguePackage, // 2D8
    pub unk6_b8: u32,                         // 2E0
    pub unk6_bc: u32,                         // 2E4
    pub ai_conversation_running: *mut DialoguePackage, // 2E8
    pub number_of_steal_warnings: i32,        // 2F0
    pub steal_warning_timer: f32,             // 2F4
    pub number_of_pickpocket_warnings: i32,   // 2F8
    pub pick_pocket_warning_timer: f32,       // 2FC
    pub warn_to_leave_timestamp: AITimeStamp, // 300
    pub pad6_dc: u32,                         // 304
    pub ironsights_dof_instance: *mut ImageSpaceModifierInstanceDOF, // 308
    pub vats_dof_instance: *mut ImageSpaceModifierInstanceDOF, // 310
    pub dynamic_dof_instance: *mut ImageSpaceModifierInstanceDOF, // 318
    pub dynamic_dof_focus_time: f32,          // 320
    pub dynamic_dof_focused: bool,            // 324
    pub unk6_fd: u8,                          // 325
    pub unk6_fe: u16,                         // 326
    pub dynamic_dof_last_angle: NiPoint3,     // 328
    pub dynamic_dof_last_position: NiPoint3,  // 334
    pub current_prison_faction: *mut TESFaction, // 340
    pub jail_sentence: i32,                   // 348
    pub pad724: u32,                          // 34C
    // TODO: `PlayerCharacter.h` only documents this slot as `void*` smart-pointer storage.
    pub unk728: *mut c_void, // 350
    pub queued_weapon_attachs: [QueuedWeapon; WeaponType::Total as usize], // 358
    pub vampire_feed_detection: i32, // 3F8
    pub map_marker_iterator: u32, // 3FC
    pub force_activate_ref: RefHandle, // 400
    pub player_action_objects: [PlayerActionObject; 15], // 404
    pub most_recent_action: PLAYER_ACTION, // 4B8
    pub actor_doing_player_command: ActorHandle, // 4BC
    pub grab_data: GrabData, // 4C0
    pub unk8_e0: u32,        // 508
    pub info_runtime_data: INFO_RUNTIME_DATA, // 50C
    pub unk_a20: [u8; 0xA0], // 648
    pub unk_ac0: u32,        // 6E8
    pub unk_ac4: u32,        // 6EC
    pub current_location: *mut BGSLocation, // 6F0
    pub cached_velocity_timestamp: AITimeStamp, // 6F8
    pub telekinesis_distance: f32, // 6FC
    pub command_timer: f32,  // 700
    pub sun_gaze_timer: f32, // 704
    pub sun_gaze_image_space_modifier: *mut TESImageSpaceModifier, // 708
    pub advance_skill: ActorValue, // 710
    pub advance_action: u32, // 714
    pub animation_object_action: DEFAULT_OBJECT, // 718
    pub grab_type: EnumSet<GrabbingType, u32>, // 71C
    pub game_state_data: GameStateData, // 720
    pub unk_b04: u32,        // 72C
    pub resist_arrest_crime: *mut Crime, // 730
    pub tint_masks: BSTArray<*mut TintMask>, // 738
    pub overlay_tint_masks: *mut BSTArray<*mut TintMask>, // 750
    pub race_data: PlayerCharacterRaceData, // 758
    pub unk_b48: i32,        // 770
    pub pad_b4_c: u32,       // 774
    pub unk_b50: BSTArray<u64>, // 778
    pub unk_b68: u64,        // 790
    pub unk_b70: u64,        // 798
    pub unk_b78: u64,        // 7A0
    pub unk_b80: u64,        // 7A8
    pub unk_b88: i32,        // 7B0
    pub pad_b8_c: u32,       // 7B4
    pub unk_b90: u64,        // 7B8
    pub tempering_item: *mut InventoryEntryData, // 7C0
    pub unk_ba0: BSTSmallArray<*mut c_void, { core::mem::size_of::<*mut c_void>() * 4 }>, // 7C8
    pub pre_transformation_data: *mut PreTransformationData, // 7F8
    pub player_flags: PlayerFlags, // 800
}

/// Flat runtime-data surface shared by the non-VR runtimes.
pub type PLAYER_RUNTIME_DATA_FLAT = PLAYER_RUNTIME_DATA;
/// SE-specific raw accessor surface. This is currently identical to `_flat`.
pub type PLAYER_RUNTIME_DATA_SE = PLAYER_RUNTIME_DATA_FLAT;
/// AE-specific raw accessor surface. This is currently identical to `_flat`.
pub type PLAYER_RUNTIME_DATA_AE = PLAYER_RUNTIME_DATA_FLAT;

/// Cross-runtime raw sub-structure covering the shared location/timer block.
#[repr(C)]
pub struct PLAYER_RUNTIME_LOCATION_STATE_DATA {
    pub current_location: *mut BGSLocation,     // 00
    pub cached_velocity_timestamp: AITimeStamp, // 08
    pub telekinesis_distance: f32,              // 0C
    pub command_timer: f32,                     // 10
    pub sun_gaze_timer: f32,                    // 14
    pub sun_gaze_image_space_modifier: *mut TESImageSpaceModifier, // 18
}

const _: () = assert!(core::mem::size_of::<PLAYER_RUNTIME_LOCATION_STATE_DATA>() == 0x20);
const _: () =
    assert!(core::mem::offset_of!(PLAYER_RUNTIME_LOCATION_STATE_DATA, current_location) == 0x00);
const _: () = assert!(
    core::mem::offset_of!(
        PLAYER_RUNTIME_LOCATION_STATE_DATA,
        sun_gaze_image_space_modifier
    ) == 0x18
);

/// Cross-runtime raw sub-structure covering the shared skill/object advancement block.
#[repr(C)]
pub struct PLAYER_RUNTIME_ADVANCE_DATA {
    pub advance_skill: ActorValue,               // 00
    pub advance_action: u32,                     // 04
    pub animation_object_action: DEFAULT_OBJECT, // 08
}

const _: () = assert!(core::mem::size_of::<PLAYER_RUNTIME_ADVANCE_DATA>() == 0x0C);
const _: () = assert!(core::mem::offset_of!(PLAYER_RUNTIME_ADVANCE_DATA, advance_skill) == 0x00);
const _: () =
    assert!(core::mem::offset_of!(PLAYER_RUNTIME_ADVANCE_DATA, animation_object_action) == 0x08);

/// Cross-runtime raw sub-structure covering the shared arrest/tint/race block.
#[repr(C)]
pub struct PLAYER_RUNTIME_TINT_RACE_DATA {
    pub resist_arrest_crime: *mut Crime,                  // 00
    pub tint_masks: BSTArray<*mut TintMask>,              // 08
    pub overlay_tint_masks: *mut BSTArray<*mut TintMask>, // 20
    pub race_data: PlayerCharacterRaceData,               // 28
}

const _: () = assert!(core::mem::size_of::<PLAYER_RUNTIME_TINT_RACE_DATA>() == 0x40);
const _: () =
    assert!(core::mem::offset_of!(PLAYER_RUNTIME_TINT_RACE_DATA, resist_arrest_crime) == 0x00);
const _: () = assert!(core::mem::offset_of!(PLAYER_RUNTIME_TINT_RACE_DATA, race_data) == 0x28);

/// Cross-runtime raw sub-structure covering the shared non-VR/VR world-state block before
/// `queued_target_loc`.
#[repr(C)]
pub struct PLAYER_RUNTIME_WORLD_DATA {
    pub quest_targets_lock: BSSpinLock,       // 000
    pub crime_value: CrimeValue,              // 008
    pub command_wait_marker: ObjectRefHandle, // 068
    pub unk6c: u32,                           // 06C
    pub faction_owner_friends_map: BSTHashMap<*const TESFaction, FriendshipFactionsStruct>, // 070
    pub last_known_good_position: NiPoint3,   // 0A0
    pub bullet_auto_aim: NiPoint3,            // 0AC
    pub cached_velocity: NiPoint3,            // 0B8
    pub unk_c4: u32,                          // 0C4
    pub unused_note: *mut BGSNote,            // 0C8
    pub unused_note2: *mut BGSNote,           // 0D0
    pub added_perks: BSTArray<*mut PerkRankData>, // 0D8
    pub perks: BSTArray<*mut BGSPerk>,        // 0F0
    pub standing_stone_perks: BSTArray<*mut BGSPerk>, // 108
    pub current_map_markers: BSTArray<ObjectRefHandle>, // 120
    pub velocity_array: BSTArray<BSTTuple<NiPoint3, AITimeStamp>>, // 138
    pub runes_cast: BSTArray<ProjectileHandle>, // 150
    pub image_space_modifier_anims1: BSTArray<*mut c_void>, // 168
    pub image_space_modifier_anims2: BSTArray<*mut c_void>, // 180
    pub quest_log: BSSimpleList<*mut TESQuestStageItem>, // 198
    pub objectives: BSTArray<BGSInstancedQuestObjective>, // 1A8
    pub quest_targets: BSTHashMap<*mut TESQuest, *mut BSTArray<*mut TESQuestTarget>>, // 1C0
    pub current_say_once_infos_map: BSTHashMap<UnkKey, UnkValue>, // 1F0
    pub dropped_ref_list: BSSimpleList<ObjectRefHandle>, // 220
    pub random_door_space_map: crate::re::NiTMap<u32, u8>, // 230
    pub cached_world_space: *mut TESWorldSpace, // 250
    pub exterior_position: NiPoint3,          // 258
    pub unk264: u32,                          // 264
}

const _: () = assert!(core::mem::size_of::<PLAYER_RUNTIME_WORLD_DATA>() == 0x268);
const _: () = assert!(core::mem::offset_of!(PLAYER_RUNTIME_WORLD_DATA, crime_value) == 0x08);
const _: () =
    assert!(core::mem::offset_of!(PLAYER_RUNTIME_WORLD_DATA, random_door_space_map) == 0x230);
const _: () = assert!(core::mem::offset_of!(PLAYER_RUNTIME_WORLD_DATA, exterior_position) == 0x258);

/// Cross-runtime raw sub-structure covering the shared non-VR/VR interaction/action block after
/// `queued_target_loc`.
#[repr(C)]
pub struct PLAYER_RUNTIME_INTERACTION_DATA {
    pub unused_sound: BSSoundHandle,                   // 000
    pub magic_failure_sound: BSSoundHandle,            // 00C
    pub shout_failure_sound: BSSoundHandle,            // 018
    pub unk24: u32,                                    // 024
    pub closest_conversation: *mut DialoguePackage,    // 028
    pub unk30: u64,                                    // 030
    pub ai_conversation_running: *mut DialoguePackage, // 038
    pub number_of_steal_warnings: i32,                 // 040
    pub steal_warning_timer: f32,                      // 044
    pub number_of_pickpocket_warnings: u32,            // 048
    pub pick_pocket_warning_timer: f32,                // 04C
    pub warn_to_leave_timestamp: AITimeStamp,          // 050
    pub unk54: u32,                                    // 054
    pub ironsights_dof_instance: *mut ImageSpaceModifierInstanceDOF, // 058
    pub vats_dof_instance: *mut ImageSpaceModifierInstanceDOF, // 060
    pub dynamic_dof_instance: *mut ImageSpaceModifierInstanceDOF, // 068
    pub dynamic_dof_focus_time: f32,                   // 070
    pub dynamic_dof_focused: bool,                     // 074
    pub pad75_77: [u8; 3],                             // 075
    pub dynamic_dof_last_angle: NiPoint3,              // 078
    pub dynamic_dof_last_position: NiPoint3,           // 084
    pub current_prison_faction: *mut TESFaction,       // 090
    pub jail_sentence: i32,                            // 098
    pub unk9c: u32,                                    // 09C
    pub unk_a0: u64,                                   // 0A0
    pub queued_weapon_attachs: [QueuedWeapon; WeaponType::Total as usize], // 0A8
    pub vampire_feed_detection: u32,                   // 148
    pub map_marker_iterator: u32,                      // 14C
    pub force_activate_ref: RefHandle,                 // 150
    pub player_action_objects: [PlayerActionObject; 15], // 154
    pub most_recent_action: PLAYER_ACTION,             // 208
    pub actor_doing_player_command: ActorHandle,       // 20C
}

const _: () = assert!(core::mem::size_of::<PLAYER_RUNTIME_INTERACTION_DATA>() == 0x210);
const _: () =
    assert!(core::mem::offset_of!(PLAYER_RUNTIME_INTERACTION_DATA, current_prison_faction) == 0x90);
const _: () =
    assert!(core::mem::offset_of!(PLAYER_RUNTIME_INTERACTION_DATA, queued_weapon_attachs) == 0xA8);
const _: () = assert!(
    core::mem::offset_of!(PLAYER_RUNTIME_INTERACTION_DATA, actor_doing_player_command) == 0x20C
);

const _: () = assert!(core::mem::size_of::<PLAYER_RUNTIME_DATA>() == 0x808);
const _: () = assert!(core::mem::offset_of!(PLAYER_RUNTIME_DATA, crime_value) == 0x08);
const _: () = assert!(core::mem::offset_of!(PLAYER_RUNTIME_DATA, info_runtime_data) == 0x50C);
const _: () =
    assert!(core::mem::offset_of!(PLAYER_RUNTIME_DATA, actor_doing_player_command) == 0x4BC);
const _: () = assert!(core::mem::offset_of!(PLAYER_RUNTIME_DATA, grab_data) == 0x4C0);
const _: () = assert!(core::mem::offset_of!(PLAYER_RUNTIME_DATA, game_state_data) == 0x720);
const _: () = assert!(core::mem::offset_of!(PLAYER_RUNTIME_DATA, race_data) == 0x758);
const _: () = assert!(core::mem::offset_of!(PLAYER_RUNTIME_DATA, player_flags) == 0x800);

/// C++ `RE::PlayerCharacter::VR_PLAYER_RUNTIME_DATA`
#[repr(C)]
pub struct VR_PLAYER_RUNTIME_DATA {
    pub unk3_d8: u64,                         // 000
    pub unk3_e0: u64,                         // 008
    pub unk3_e8: u64,                         // 010
    pub vr_node_data: VR_NODE_DATA,           // 018
    pub unk680: u64,                          // 2A8
    pub unk688: u64,                          // 2B0
    pub unk690: u64,                          // 2B8
    pub unk698: u64,                          // 2C0
    pub unk6_a0: u64,                         // 2C8
    pub unk6_a8: [u64; 5],                    // 2D0
    pub unk6_d0: u32,                         // 2F8
    pub is_right_hand_main_hand: u32,         // 2FC
    pub is_left_hand_main_hand: u32,          // 300
    pub unk6_dc: u32,                         // 304
    pub pad308: [u8; 0x10],                   // 308
    pub unk6_f0: [u64; 0x5B],                 // 318
    pub quest_targets_lock: BSSpinLock,       // 5F0
    pub crime_value: CrimeValue,              // 5F8
    pub command_wait_marker: ObjectRefHandle, // 658
    pub pad_a34: u32,                         // 65C
    pub faction_owner_friends_map: BSTHashMap<*const TESFaction, FriendshipFactionsStruct>, // 660
    pub last_known_good_position: NiPoint3,   // 690
    pub bullet_auto_aim: NiPoint3,            // 69C
    pub cached_velocity: NiPoint3,            // 6A8
    pub pad_a8_c: u32,                        // 6B4
    pub unused_note: *mut BGSNote,            // 6B8
    pub unused_note2: *mut BGSNote,           // 6C0
    pub added_perks: BSTArray<*mut PerkRankData>, // 6C8
    pub perks: BSTArray<*mut BGSPerk>,        // 6E0
    pub standing_stone_perks: BSTArray<*mut BGSPerk>, // 6F8
    pub current_map_markers: BSTArray<ObjectRefHandle>, // 710
    pub velocity_array: BSTArray<BSTTuple<NiPoint3, AITimeStamp>>, // 728
    pub runes_cast: BSTArray<ProjectileHandle>, // 740
    pub image_space_modifier_anims1: BSTArray<*mut c_void>, // 758
    pub image_space_modifier_anims2: BSTArray<*mut c_void>, // 770
    pub quest_log: BSSimpleList<*mut TESQuestStageItem>, // 788
    pub objectives: BSTArray<BGSInstancedQuestObjective>, // 798
    pub quest_targets: BSTHashMap<*mut TESQuest, *mut BSTArray<*mut TESQuestTarget>>, // 7B0
    pub current_say_once_infos_map: BSTHashMap<UnkKey, UnkValue>, // 7E0
    pub dropped_ref_list: BSSimpleList<ObjectRefHandle>, // 810
    pub random_door_space_map: crate::re::NiTMap<u32, u8>, // 820
    pub cached_world_space: *mut TESWorldSpace, // 840
    pub exterior_position: NiPoint3,          // 848
    pub pad_c2_c: u32,                        // 854
    pub queued_target_loc: VR_PLAYER_TARGET_LOC, // 858
    pub unused_sound: BSSoundHandle,          // 8A8
    pub magic_failure_sound: BSSoundHandle,   // 8B4
    pub shout_failure_sound: BSSoundHandle,   // 8C0
    pub unk_ca4: u32,                         // 8CC
    pub closest_conversation: *mut DialoguePackage, // 8D0
    pub unk_cb0: u64,                         // 8D8
    pub ai_conversation_running: *mut DialoguePackage, // 8E0
    pub number_of_steal_warnings: i32,        // 8E8
    pub steal_warning_timer: f32,             // 8EC
    pub number_of_pickpocket_warnings: u32,   // 8F0
    pub pick_pocket_warning_timer: f32,       // 8F4
    pub warn_to_leave_timestamp: AITimeStamp, // 8F8
    pub unk_cd4: u32,                         // 8FC
    pub ironsights_dof_instance: *mut ImageSpaceModifierInstanceDOF, // 900
    pub vats_dof_instance: *mut ImageSpaceModifierInstanceDOF, // 908
    pub dynamic_dof_instance: *mut ImageSpaceModifierInstanceDOF, // 910
    pub dynamic_dof_focus_time: f32,          // 918
    pub dynamic_dof_focused: bool,            // 91C
    pub pad_cf5_cf7: [u8; 3],                 // 91D
    pub dynamic_dof_last_angle: NiPoint3,     // 920
    pub dynamic_dof_last_position: NiPoint3,  // 92C
    pub current_prison_faction: *mut TESFaction, // 938
    pub jail_sentence: i32,                   // 940
    pub unk_d1_c: i32,                        // 944
    pub unk_d20: u64,                         // 948
    pub queued_weapon_attachs: [QueuedWeapon; WeaponType::Total as usize], // 950
    pub vampire_feed_detection: u32,          // 9F0
    pub map_marker_iterator: u32,             // 9F4
    pub force_activate_ref: RefHandle,        // 9F8
    pub player_action_objects: [PlayerActionObject; 0xF], // 9FC
    pub most_recent_action: PLAYER_ACTION,    // AB0
    pub actor_doing_player_command: ActorHandle, // AB4
    pub unk_e90: u64,                         // AB8
    pub grabbed_object_data: [VRGrabData; VR_DEVICE::kTotal as usize], // AC0
    pub unk_fd0: f32,                         // BF8
    pub unk_float_fd4: f32,                   // BFC
    pub unk_fd8: u64,                         // C00
    pub vr_info_runtime_data: VR_INFO_RUNTIME_DATA, // C08
    pub unk1120: [u8; 0xA0],                  // D48
    pub unk11_c0: u32,                        // DE8
    pub unk11_c4: u32,                        // DEC
    pub current_location: *mut BGSLocation,   // DF0
    pub cached_velocity_timestamp: AITimeStamp, // DF8
    pub telekinesis_distance: f32,            // DFC
    pub command_timer: f32,                   // E00
    pub sun_gaze_timer: f32,                  // E04
    pub sun_gaze_image_space_modifier: *mut TESImageSpaceModifier, // E08
    pub advance_skill: ActorValue,            // E10
    pub advance_action: u32,                  // E14
    pub animation_object_action: DEFAULT_OBJECT, // E18
    pub game_state_data: GameStateData,       // E1C
    pub resist_arrest_crime: *mut Crime,      // E28
    pub tint_masks: BSTArray<*mut TintMask>,  // E30
    pub overlay_tint_masks: *mut BSTArray<*mut TintMask>, // E48
    pub race_data: PlayerCharacterRaceData,   // E50
    pub unk1240: [u64; 0x11],                 // E68
    pub pre_transformation_data: *mut PreTransformationData, // EF0
    pub player_flags: PlayerFlags,            // EF8
    pub pad_f00: [u8; 0x18],                  // F00
}

const _: () = assert!(core::mem::size_of::<VR_PLAYER_RUNTIME_DATA>() == 0xF18);
const _: () = assert!(core::mem::offset_of!(VR_PLAYER_RUNTIME_DATA, crime_value) == 0x5F8);
const _: () =
    assert!(core::mem::offset_of!(VR_PLAYER_RUNTIME_DATA, actor_doing_player_command) == 0xAB4);
const _: () = assert!(core::mem::offset_of!(VR_PLAYER_RUNTIME_DATA, grabbed_object_data) == 0xAC0);
const _: () = assert!(core::mem::offset_of!(VR_PLAYER_RUNTIME_DATA, vr_info_runtime_data) == 0xC08);
const _: () = assert!(core::mem::offset_of!(VR_PLAYER_RUNTIME_DATA, current_location) == 0xDF0);
const _: () = assert!(core::mem::offset_of!(VR_PLAYER_RUNTIME_DATA, game_state_data) == 0xE1C);
const _: () = assert!(core::mem::offset_of!(VR_PLAYER_RUNTIME_DATA, race_data) == 0xE50);
const _: () = assert!(core::mem::offset_of!(VR_PLAYER_RUNTIME_DATA, player_flags) == 0xEF8);

/// C++ `RE::PlayerCharacter`
#[repr(C)]
pub struct PlayerCharacter {
    pub base: Character,     // 000
    pub unk080: [u8; 0x168], // 080 - honest common prefix before moved runtime tails
}

const _: () = assert!(core::mem::size_of::<PlayerCharacter>() == 0x1E8);
const _: () = assert!(core::mem::offset_of!(PlayerCharacter, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(PlayerCharacter, unk080) == 0x80);

impl RttiType for PlayerCharacter {
    const RTTI: VariantID = RTTI_PlayerCharacter;
}

impl FormCastable for PlayerCharacter {
    const TARGET_FORM_TYPE: FormType = FormType::ActorCharacter;
}

inherit!(PlayerCharacter : Character, base);

impl NiRef for PlayerCharacter {
    #[inline(always)]
    fn inc_ref(&self) {
        unsafe { (&*(self as *const Self as *const BSHandleRefObject)).inc_ref_count() };
    }

    #[inline(always)]
    fn dec_ref(&self) {
        unsafe { (&*(self as *const Self as *const BSHandleRefObject)).dec_ref_count() };
    }
}

impl PlayerCharacter {
    pub const RTTI: VariantID = RTTI_PlayerCharacter;
    pub const VTABLE: &'static [VariantID] = &VTABLE_PlayerCharacter;
    pub const FORMTYPE: FormType = FormType::ActorCharacter;
    /// Size before AE 1.7.99 (SE / AE / VR). AE 1.7.99+ adds `0x8`.
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0xBE0, 0xBE8, 0x12D8);

    /// AE 1.7.99 inserted a `BSTEventSink<BSSystemEvent>` base at `0x2D8`, which
    /// shifts every later AE member by this many bytes.
    pub const AE1799_SHIFT_BYTES: usize = 0x8;
    /// First AE offset affected by [`Self::AE1799_SHIFT_BYTES`].
    pub const AE1799_SHIFT_START: usize = 0x2D8;
    /// `BSTEventSink<BSSystemEvent>` base, AE 1.7.99+ only.
    pub const BS_SYSTEM_EVENT_SINK_OFFSET_AE1799: usize = 0x2D8;

    /// Resolves an SE/AE/VR offset for the running game, applying the AE 1.7.99
    /// base-class shift (C++ `REL::VersionShift(..., kAE1799ShiftBytes, 1.7.99)`).
    #[inline(always)]
    pub fn runtime_offset(offset: VariantOffset) -> usize {
        let value = offset.offset();
        if crate::runtime::is_ae()
            && value >= Self::AE1799_SHIFT_START
            && crate::runtime::is_at_least(crate::version::RUNTIME_SSE_1_7_99)
        {
            value + Self::AE1799_SHIFT_BYTES
        } else {
            value
        }
    }

    pub const MENU_OPEN_CLOSE_EVENT_SINK_OFFSET: VariantOffset =
        VariantOffset::new(0x2B0, 0x2B8, 0x2B0);
    pub const MENU_MODE_CHANGE_EVENT_SINK_OFFSET: VariantOffset =
        VariantOffset::new(0x2B8, 0x2C0, 0x2B8);
    pub const USER_EVENT_ENABLED_EVENT_SINK_OFFSET: VariantOffset =
        VariantOffset::new(0x2C0, 0x2C8, 0x2C0);
    pub const TES_TRACKED_STATS_EVENT_SINK_OFFSET: VariantOffset =
        VariantOffset::new(0x2C8, 0x2D0, 0x2C8);
    pub const BGSACTOR_CELL_EVENT_SOURCE_OFFSET: VariantOffset =
        VariantOffset::new(0x2D0, 0x2D8, 0x2E8);
    pub const BGSACTOR_DEATH_EVENT_SOURCE_OFFSET: VariantOffset =
        VariantOffset::new(0x328, 0x330, 0x340);
    pub const POSITION_PLAYER_EVENT_SOURCE_OFFSET: VariantOffset =
        VariantOffset::new(0x380, 0x388, 0x398);
    pub const VR_OVERLAY_CHANGE_EVENT_SINK_OFFSET: VariantOffset =
        VariantOffset::new(0x0, 0x0, 0x2D0);
    pub const VR_DEVICE_CONNECTION_CHANGE_EVENT_SINK_OFFSET: VariantOffset =
        VariantOffset::new(0x0, 0x0, 0x2D8);
    pub const VR_RESET_HMD_HEIGHT_EVENT_SINK_OFFSET: VariantOffset =
        VariantOffset::new(0x0, 0x0, 0x2E0);

    pub const PLAYER_RUNTIME_DATA_FLAT_OFFSET: VariantOffset =
        VariantOffset::new(0x3D8, 0x3E0, 0x0);
    pub const PLAYER_INFO_RUNTIME_HEAD_DATA_OFFSET: VariantOffset =
        VariantOffset::new(0x8E8, 0x8F0, 0xFE8);
    pub const PLAYER_INFO_RUNTIME_TAIL_DATA_OFFSET: VariantOffset =
        VariantOffset::new(0x928, 0x930, 0x1028);
    pub const PLAYER_RUNTIME_TARGET_LOC_DATA_OFFSET: VariantOffset =
        VariantOffset::new(0x640, 0x648, 0xC30);
    pub const QUEUED_TARGET_LOC_FLAT_OFFSET: VariantOffset = VariantOffset::new(0x640, 0x648, 0x0);
    pub const PLAYER_RUNTIME_LOCATION_STATE_DATA_OFFSET: VariantOffset =
        VariantOffset::new(0xAC8, 0xAD0, 0x11C8);
    pub const PLAYER_RUNTIME_ADVANCE_DATA_OFFSET: VariantOffset =
        VariantOffset::new(0xAE8, 0xAF0, 0x11E8);
    pub const PLAYER_RUNTIME_TINT_RACE_DATA_OFFSET: VariantOffset =
        VariantOffset::new(0xB08, 0xB10, 0x1200);
    pub const PLAYER_RUNTIME_WORLD_DATA_OFFSET: VariantOffset =
        VariantOffset::new(0x3D8, 0x3E0, 0x9C8);
    pub const PLAYER_RUNTIME_INTERACTION_DATA_OFFSET: VariantOffset =
        VariantOffset::new(0x688, 0x690, 0xA80);
    pub const GRAB_DATA_FLAT_OFFSET: VariantOffset = VariantOffset::new(0x898, 0x8A0, 0x0);
    pub const CRIME_VALUE_OFFSET: VariantOffset = VariantOffset::new(0x3E0, 0x3E8, 0x9D0);
    pub const RACE_DATA_OFFSET: VariantOffset = VariantOffset::new(0xB30, 0xB38, 0x1228);
    pub const GAME_STATE_DATA_OFFSET: VariantOffset = VariantOffset::new(0xAF8, 0xB00, 0x11F4);
    pub const INFO_RUNTIME_DATA_OFFSET: VariantOffset = VariantOffset::new(0x8E4, 0x8EC, 0x0);
    pub const ACTOR_DOING_PLAYER_COMMAND_HANDLE_OFFSET: VariantOffset =
        VariantOffset::new(0x894, 0x89C, 0xE8C);
    pub const FLAT_GRABBED_OBJECT_HANDLE_OFFSET: VariantOffset =
        VariantOffset::new(0x8C8, 0x8D0, 0x0);
    pub const FLAT_GRAB_TYPE_OFFSET: VariantOffset = VariantOffset::new(0xAF4, 0xAFC, 0x0);
    pub const TINT_MASKS_OFFSET: VariantOffset = VariantOffset::new(0xB10, 0xB18, 0x0);
    pub const OVERLAY_TINT_MASKS_OFFSET: VariantOffset = VariantOffset::new(0xB28, 0xB30, 0x0);

    crate::relocation_variable! {
        fn singleton_storage() -> *mut NiPointer<PlayerCharacter> => RelocationID::new(517014, 403521), is_ptr
    }

    crate::relocation_variable! {
        fn god_mode_flag_storage() -> *mut bool => RelocationID::new(517711, 404238), is_ptr
    }

    crate::relocation_func! {
        pub fn activate_pick_ref(&mut self) => RelocationID::new(39471, 40548)
    }

    crate::relocation_func! {
        pub fn activate_pick_ref_vr(&mut self, device: VR_DEVICE) => RelocationID::new(39471, 40548)
    }

    crate::relocation_func! {
        pub fn add_player_add_item_event(
            &mut self,
            object: *mut TESObject,
            owner: *mut TESForm,
            container: *mut TESObjectREFR,
            acquire_type: AcquireType
        ) => RelocationID::new(39384, 40456)
    }

    crate::relocation_func! {
        fn add_skill_experience_impl(
            &mut self,
            skill: ActorValue,
            experience: f32,
            advance_object: *mut TESForm,
            advance_action: u32
        ) => RelocationID::new(39413, 40488)
    }

    /// C++ `AddSkillExperience(skill, experience, advanceObject = nullptr, advanceAction = 0)`.
    #[inline(always)]
    pub fn add_skill_experience(&mut self, skill: ActorValue, experience: f32) {
        self.add_skill_experience_impl(skill, experience, core::ptr::null_mut(), 0);
    }

    #[inline(always)]
    pub fn add_skill_experience_ex(
        &mut self,
        skill: ActorValue,
        experience: f32,
        advance_object: *mut TESForm,
        advance_action: u32,
    ) {
        self.add_skill_experience_impl(skill, experience, advance_object, advance_action);
    }

    crate::relocation_func! {
        pub fn check_poison_weapon(&mut self, poison: *mut crate::re::AlchemyItem) => RelocationID::new(39406, 40481)
    }

    crate::relocation_func! {
        pub fn use_poison_from_inventory(&mut self, poison: *mut crate::re::AlchemyItem) => RelocationID::new(39406, 40481)
    }

    crate::relocation_func! {
        pub fn destroy_mouse_springs_vr(&mut self, device: VR_DEVICE) => RelocationID::new(39480, 40557)
    }

    crate::relocation_func! {
        fn set_waypoint_impl(&mut self, position: *const crate::re::NiPoint3, world_space_or_cell: *mut TESForm) => RelocationID::new(39458, 40535)
    }

    #[inline(always)]
    pub fn set_waypoint(
        &mut self,
        position: &crate::re::NiPoint3,
        world_space_or_cell: *mut TESForm,
    ) {
        self.set_waypoint_impl(position, world_space_or_cell);
    }

    crate::relocation_func! {
        pub fn remove_waypoint(&mut self) => RelocationID::new(39459, 40536)
    }

    crate::relocation_func! {
        // Does not check whether waiting is currently allowed (combat, etc.).
        pub fn start_waiting(&mut self, hours: i32) => RelocationID::new(39344, 40415)
    }

    /// Per-frame VR comfort-radius check; no-op outside VR.
    #[inline(always)]
    pub fn update_vr_comfort_check(&mut self) {
        if crate::runtime::is_vr() {
            let func = unsafe {
                crate::relocation::Relocation::<extern "C" fn(*mut Self)>::new(VariantID::new(
                    0, 0, 0x6B9AA0,
                ))
                .get()
            };
            func(self);
        }
    }

    crate::relocation_func! {
        pub fn attempt_pickpocket(
            &mut self,
            container_ref: *mut TESObjectREFR,
            entry: *mut InventoryEntryData,
            number: i32,
            from_container: bool
        ) -> bool => RelocationID::new(39568, 40654)
    }

    crate::relocation_func! {
        pub fn check_cast(
            &mut self,
            spell: *mut MagicItem,
            effect: *mut Effect,
            reason: &mut CannotCastReason
        ) -> bool => RelocationID::new(39409, 40484)
    }

    crate::relocation_func! {
        pub fn destroy_mouse_springs(&mut self) => RelocationID::new(39480, 40557)
    }

    crate::relocation_func! {
        pub fn get_armor_value(&mut self, form: *mut InventoryEntryData) -> f32 => RelocationID::new(39175, 40249)
    }

    crate::relocation_func! {
        pub fn get_damage(&mut self, form: *mut InventoryEntryData) -> f32 => RelocationID::new(39179, 40253)
    }

    crate::relocation_func! {
        fn get_equipped_weapons_damage_ae(&self) -> f32 => RelocationID::new(0, 40252)
    }

    crate::relocation_func! {
        pub fn get_item_count(&mut self, object: *mut TESBoundObject) -> i32 => RelocationID::new(19275, 19701)
    }

    crate::relocation_func! {
        pub fn get_num_tints(&mut self, tint_type: u32) -> u32 => RelocationID::new(39614, 40700)
    }

    crate::relocation_func! {
        fn get_tint_mask_flat(&self, tint_type: u32, index: u32) -> *mut TintMask => RelocationID::new(39612, 40698)
    }

    crate::relocation_func! {
        pub fn play_magic_failure_sound(&mut self, spell_type: SpellType) => RelocationID::new(39486, 40565)
    }

    crate::relocation_func! {
        pub fn set_ai_driven(&mut self, enable: bool) => RelocationID::new(39507, 40586)
    }

    crate::relocation_func! {
        pub fn set_escaping(&mut self, flag: bool, escaped: bool) => RelocationID::new(39574, 40660)
    }

    crate::relocation_func! {
        pub fn set_god_mode(&mut self, enable: bool) => RelocationID::new(39424, 40500)
    }

    crate::relocation_func! {
        pub fn start_grab_object(&mut self, device: VR_DEVICE) => RelocationID::new(39475, 40552)
    }

    crate::relocation_func! {
        pub fn update_crosshairs(&mut self) => RelocationID::new(39535, 40621)
    }

    crate::relocation_func! {
        fn center_on_cell_impl(
            &mut self,
            cell_name: *const c_char,
            cell: *mut TESObjectCELL
        ) -> bool => RelocationID::new(39365, 40437)
    }

    // override (Character)
    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    // override (TESObjectREFR)
    pub const VFUNC_ATTACH_WEAPON: VariantOffset = VariantOffset::new(0x0, 0x0, 0x82);

    // override (Actor)
    crate::relocated_virtual_method! {
        pub const VFUNC_REMOVE_WEAPON: VariantOffset = VariantOffset::new(0x82, 0x82, 0x83);
        pub fn remove_weapon(&mut self, equip_index: BIPED_OBJECT)
    }

    // add
    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_12A: VariantOffset = VariantOffset::new(0x12A, 0x12A, 0x12C);
        pub fn unk_12a(&mut self)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_VIOLENT_CRIME_GOLD_VALUE: VariantOffset =
            VariantOffset::new(0x12B, 0x12B, 0x12D);
        pub fn get_violent_crime_gold_value(&self, faction: *const TESFaction) -> u32
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_GET_NON_VIOLENT_CRIME_GOLD_VALUE: VariantOffset =
            VariantOffset::new(0x12C, 0x12C, 0x12E);
        pub fn get_non_violent_crime_gold_value(&self, faction: *const TESFaction) -> u32
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_CLEAR_ALL_CRIME_GOLD: VariantOffset = VariantOffset::new(0x12D, 0x12D, 0x12F);
        pub fn clear_all_crime_gold(&mut self, faction: *mut TESFaction)
    }

    crate::relocated_virtual_method! {
        pub const VFUNC_UNK_12E: VariantOffset = VariantOffset::new(0x12E, 0x12E, 0x130);
        pub fn unk_12e(&mut self)
    }
}

impl PlayerCharacter {
    // C++ `AsBSSystemEventSink` (AE 1.7.99+ only).
    crate::runtime_optional_data_accessor! {
        pub fn as_bs_system_event_sink() -> BSTEventSink<crate::re::BSSystemEvent> {
            version: crate::version::RUNTIME_SSE_1_7_99,
            se: 0,
            ae: Self::BS_SYSTEM_EVENT_SINK_OFFSET_AE1799,
            vr: 0
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_bgs_actor_cell_event_source() -> BSTEventSource<BGSActorCellEvent> {
            offset: Self::runtime_offset(Self::BGSACTOR_CELL_EVENT_SOURCE_OFFSET)
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_bgs_actor_cell_event_source_mut() -> BSTEventSource<BGSActorCellEvent> {
            offset: Self::runtime_offset(Self::BGSACTOR_CELL_EVENT_SOURCE_OFFSET)
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_bgs_actor_death_event_source() -> BSTEventSource<BGSActorDeathEvent> {
            offset: Self::runtime_offset(Self::BGSACTOR_DEATH_EVENT_SOURCE_OFFSET)
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_bgs_actor_death_event_source_mut() -> BSTEventSource<BGSActorDeathEvent> {
            offset: Self::runtime_offset(Self::BGSACTOR_DEATH_EVENT_SOURCE_OFFSET)
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_position_player_event_source() -> BSTEventSource<PositionPlayerEvent> {
            offset: Self::runtime_offset(Self::POSITION_PLAYER_EVENT_SOURCE_OFFSET)
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_position_player_event_source_mut() -> BSTEventSource<PositionPlayerEvent> {
            offset: Self::runtime_offset(Self::POSITION_PLAYER_EVENT_SOURCE_OFFSET)
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_menu_open_close_event_sink() -> BSTEventSink<MenuOpenCloseEvent> {
            offset: Self::runtime_offset(Self::MENU_OPEN_CLOSE_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_menu_open_close_event_sink_mut() -> BSTEventSink<MenuOpenCloseEvent> {
            offset: Self::runtime_offset(Self::MENU_OPEN_CLOSE_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_menu_mode_change_event_sink() -> BSTEventSink<MenuModeChangeEvent> {
            offset: Self::runtime_offset(Self::MENU_MODE_CHANGE_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_menu_mode_change_event_sink_mut() -> BSTEventSink<MenuModeChangeEvent> {
            offset: Self::runtime_offset(Self::MENU_MODE_CHANGE_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_user_event_enabled_event_sink() -> BSTEventSink<UserEventEnabledEvent> {
            offset: Self::runtime_offset(Self::USER_EVENT_ENABLED_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_user_event_enabled_event_sink_mut() -> BSTEventSink<UserEventEnabledEvent> {
            offset: Self::runtime_offset(Self::USER_EVENT_ENABLED_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_cast_accessor! {
        pub fn as_tes_tracked_stats_event_sink() -> BSTEventSink<TESTrackedStatsEvent> {
            offset: Self::runtime_offset(Self::TES_TRACKED_STATS_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_cast_mut_accessor! {
        pub fn as_tes_tracked_stats_event_sink_mut() -> BSTEventSink<TESTrackedStatsEvent> {
            offset: Self::runtime_offset(Self::TES_TRACKED_STATS_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_optional_data_accessor! {
        pub fn as_vr_overlay_change_event_sink() -> BSTEventSink<VROverlayChange> {
            offset: Self::runtime_offset(Self::VR_OVERLAY_CHANGE_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn as_vr_overlay_change_event_sink_mut() -> BSTEventSink<VROverlayChange> {
            offset: Self::runtime_offset(Self::VR_OVERLAY_CHANGE_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_optional_data_accessor! {
        pub fn as_vr_device_connection_change_event_sink() -> BSTEventSink<VRDeviceConnectionChange> {
            offset: Self::runtime_offset(Self::VR_DEVICE_CONNECTION_CHANGE_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn as_vr_device_connection_change_event_sink_mut() -> BSTEventSink<VRDeviceConnectionChange> {
            offset: Self::runtime_offset(Self::VR_DEVICE_CONNECTION_CHANGE_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_optional_data_accessor! {
        pub fn as_vr_reset_hmd_height_event_sink() -> BSTEventSink<VRResetHMDHeight> {
            offset: Self::runtime_offset(Self::VR_RESET_HMD_HEIGHT_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn as_vr_reset_hmd_height_event_sink_mut() -> BSTEventSink<VRResetHMDHeight> {
            offset: Self::runtime_offset(Self::VR_RESET_HMD_HEIGHT_EVENT_SINK_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn player_runtime_data_flat_impl() -> PLAYER_RUNTIME_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_DATA_FLAT_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn player_runtime_data_flat_impl_mut() -> PLAYER_RUNTIME_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_DATA_FLAT_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn player_runtime_target_loc_data_impl() -> PLAYER_RUNTIME_TARGET_LOC_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_TARGET_LOC_DATA_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn player_runtime_target_loc_data_impl_mut() -> PLAYER_RUNTIME_TARGET_LOC_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_TARGET_LOC_DATA_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn queued_target_loc_flat_impl() -> PLAYER_TARGET_LOC {
            offset: Self::runtime_offset(Self::QUEUED_TARGET_LOC_FLAT_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn queued_target_loc_flat_impl_mut() -> PLAYER_TARGET_LOC {
            offset: Self::runtime_offset(Self::QUEUED_TARGET_LOC_FLAT_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn player_runtime_location_state_data_impl() -> PLAYER_RUNTIME_LOCATION_STATE_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_LOCATION_STATE_DATA_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn player_runtime_location_state_data_impl_mut() -> PLAYER_RUNTIME_LOCATION_STATE_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_LOCATION_STATE_DATA_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn player_runtime_advance_data_impl() -> PLAYER_RUNTIME_ADVANCE_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_ADVANCE_DATA_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn player_runtime_advance_data_impl_mut() -> PLAYER_RUNTIME_ADVANCE_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_ADVANCE_DATA_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn player_runtime_tint_race_data_impl() -> PLAYER_RUNTIME_TINT_RACE_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_TINT_RACE_DATA_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn player_runtime_tint_race_data_impl_mut() -> PLAYER_RUNTIME_TINT_RACE_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_TINT_RACE_DATA_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn player_runtime_world_data_impl() -> PLAYER_RUNTIME_WORLD_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_WORLD_DATA_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn player_runtime_world_data_impl_mut() -> PLAYER_RUNTIME_WORLD_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_WORLD_DATA_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn player_runtime_interaction_data_impl() -> PLAYER_RUNTIME_INTERACTION_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_INTERACTION_DATA_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn player_runtime_interaction_data_impl_mut() -> PLAYER_RUNTIME_INTERACTION_DATA {
            offset: Self::runtime_offset(Self::PLAYER_RUNTIME_INTERACTION_DATA_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn grab_data_flat_impl() -> GrabData {
            offset: Self::runtime_offset(Self::GRAB_DATA_FLAT_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn grab_data_flat_impl_mut() -> GrabData {
            offset: Self::runtime_offset(Self::GRAB_DATA_FLAT_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn player_info_runtime_head_data_impl() -> PLAYER_INFO_RUNTIME_HEAD_DATA {
            offset: Self::runtime_offset(Self::PLAYER_INFO_RUNTIME_HEAD_DATA_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn player_info_runtime_head_data_impl_mut() -> PLAYER_INFO_RUNTIME_HEAD_DATA {
            offset: Self::runtime_offset(Self::PLAYER_INFO_RUNTIME_HEAD_DATA_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn player_info_runtime_tail_data_impl() -> PLAYER_INFO_RUNTIME_TAIL_DATA {
            offset: Self::runtime_offset(Self::PLAYER_INFO_RUNTIME_TAIL_DATA_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn player_info_runtime_tail_data_impl_mut() -> PLAYER_INFO_RUNTIME_TAIL_DATA {
            offset: Self::runtime_offset(Self::PLAYER_INFO_RUNTIME_TAIL_DATA_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn race_data_impl() -> PlayerCharacterRaceData {
            offset: Self::runtime_offset(Self::RACE_DATA_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn race_data_impl_mut() -> PlayerCharacterRaceData {
            offset: Self::runtime_offset(Self::RACE_DATA_OFFSET)
        }
    }

    crate::runtime_data_accessor! {
        fn game_state_data_impl() -> GameStateData {
            offset: Self::runtime_offset(Self::GAME_STATE_DATA_OFFSET)
        }
    }

    crate::runtime_data_mut_accessor! {
        fn game_state_data_impl_mut() -> GameStateData {
            offset: Self::runtime_offset(Self::GAME_STATE_DATA_OFFSET)
        }
    }

    crate::vr_runtime_data_accessor! {
        fn player_runtime_data_vr_impl() -> VR_PLAYER_RUNTIME_DATA {
            vr: 0x3D8
        }
    }

    crate::vr_runtime_data_mut_accessor! {
        fn player_runtime_data_vr_impl_mut() -> VR_PLAYER_RUNTIME_DATA {
            vr: 0x3D8
        }
    }

    crate::vr_only_pointer_accessor! {
        fn info_runtime_data_vr_ptr() -> VR_INFO_RUNTIME_DATA {
            vr: 0xFE0
        }
    }

    crate::vr_only_pointer_accessor! {
        fn node_data_vr_ptr() -> VR_NODE_DATA {
            vr: 0x3F0
        }
    }
}

/// Ergonomic runtime-aware view over `PlayerCharacter` tail data.
///
/// This centralizes SE/AE/VR branching for consumers while keeping the low-level
/// layout contract honest: flat raw `PLAYER_RUNTIME_DATA` stays `_flat`,
/// flat-only substructures stay `_flat`, and VR-only data stays `_vr`.
#[derive(Clone, Copy)]
pub struct PlayerCharacterRuntimeDataView<'a> {
    player: &'a PlayerCharacter,
}

impl<'a> PlayerCharacterRuntimeDataView<'a> {
    #[inline(always)]
    pub fn player_runtime_data_flat(&self) -> Option<&PLAYER_RUNTIME_DATA_FLAT> {
        if crate::runtime::is_vr() {
            None
        } else {
            Some(self.player.get_player_runtime_data_flat())
        }
    }

    #[inline(always)]
    pub fn player_runtime_data_se(&self) -> Option<&PLAYER_RUNTIME_DATA_SE> {
        if crate::runtime::is_se() {
            Some(self.player.get_player_runtime_data_se())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn player_runtime_data_ae(&self) -> Option<&PLAYER_RUNTIME_DATA_AE> {
        if crate::runtime::is_ae() {
            Some(self.player.get_player_runtime_data_ae())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn player_runtime_data_vr(&self) -> Option<&VR_PLAYER_RUNTIME_DATA> {
        if crate::runtime::is_vr() {
            Some(self.player.get_player_runtime_data_vr())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn crime_value(&self) -> &CrimeValue {
        self.player.get_crime_value()
    }

    #[inline(always)]
    pub fn info_runtime_head_data(&self) -> &PLAYER_INFO_RUNTIME_HEAD_DATA {
        self.player.get_player_info_runtime_head_data()
    }

    #[inline(always)]
    pub fn info_runtime_tail_data(&self) -> &PLAYER_INFO_RUNTIME_TAIL_DATA {
        self.player.get_player_info_runtime_tail_data()
    }

    #[inline(always)]
    pub fn location_state_data(&self) -> &PLAYER_RUNTIME_LOCATION_STATE_DATA {
        self.player.get_player_runtime_location_state_data()
    }

    #[inline(always)]
    pub fn target_loc_data(&self) -> &PLAYER_RUNTIME_TARGET_LOC_DATA {
        self.player.get_player_runtime_target_loc_data()
    }

    #[inline(always)]
    pub fn queued_target_loc_flat(&self) -> Option<&PLAYER_TARGET_LOC> {
        if crate::runtime::is_vr() {
            None
        } else {
            Some(self.player.get_queued_target_loc_flat())
        }
    }

    #[inline(always)]
    pub fn queued_target_loc_vr(&self) -> Option<&VR_PLAYER_TARGET_LOC> {
        if crate::runtime::is_vr() {
            Some(self.player.get_queued_target_loc_vr())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn advance_data(&self) -> &PLAYER_RUNTIME_ADVANCE_DATA {
        self.player.get_player_runtime_advance_data()
    }

    #[inline(always)]
    pub fn tint_race_data(&self) -> &PLAYER_RUNTIME_TINT_RACE_DATA {
        self.player.get_player_runtime_tint_race_data()
    }

    #[inline(always)]
    pub fn world_data(&self) -> &PLAYER_RUNTIME_WORLD_DATA {
        self.player.get_player_runtime_world_data()
    }

    #[inline(always)]
    pub fn interaction_data(&self) -> &PLAYER_RUNTIME_INTERACTION_DATA {
        self.player.get_player_runtime_interaction_data()
    }

    #[inline(always)]
    pub fn game_state_data(&self) -> &GameStateData {
        self.player.get_game_stats_data()
    }

    #[inline(always)]
    pub fn race_data(&self) -> &PlayerCharacterRaceData {
        self.player.get_race_data()
    }

    #[inline(always)]
    pub fn info_runtime_data_flat(&self) -> Option<&INFO_RUNTIME_DATA> {
        if crate::runtime::is_vr() {
            None
        } else {
            Some(self.player.get_info_runtime_data_flat())
        }
    }

    #[inline(always)]
    pub fn info_runtime_data_vr(&self) -> Option<&VR_INFO_RUNTIME_DATA> {
        self.player.get_info_runtime_data_vr()
    }

    #[inline(always)]
    pub fn actor_doing_player_command(&self) -> NiPointer<Actor> {
        self.player.get_actor_doing_player_command()
    }

    #[inline(always)]
    pub fn grab_data_flat(&self) -> Option<&GrabData> {
        if crate::runtime::is_vr() {
            None
        } else {
            Some(self.player.get_grab_data_flat())
        }
    }

    #[inline(always)]
    pub fn grab_data_vr(&self, device: VR_DEVICE) -> Option<&VRGrabData> {
        self.player.get_grab_data_vr(device)
    }

    #[inline(always)]
    pub fn queued_target_loc_reset_weather(&self) -> bool {
        self.player.queued_target_loc_reset_weather()
    }

    #[inline(always)]
    pub fn queued_target_loc_allow_auto_save(&self) -> bool {
        self.player.queued_target_loc_allow_auto_save()
    }

    #[inline(always)]
    pub fn queued_target_loc_is_valid(&self) -> bool {
        self.player.queued_target_loc_is_valid()
    }

    #[inline(always)]
    pub fn has_actor_doing_command(&self) -> bool {
        self.player.has_actor_doing_command()
    }

    #[inline(always)]
    pub fn grabbed_ref(&self, device: VR_DEVICE) -> NiPointer<TESObjectREFR> {
        self.player.get_grabbed_ref(device)
    }

    #[inline(always)]
    pub fn grab_object_weight(&self, device: VR_DEVICE) -> f32 {
        self.player.grab_object_weight(device)
    }

    #[inline(always)]
    pub fn grab_distance(&self, device: VR_DEVICE) -> f32 {
        self.player.grab_distance(device)
    }

    #[inline(always)]
    pub fn is_grabbing(&self) -> bool {
        self.player.is_grabbing()
    }

    #[inline(always)]
    pub fn is_grabbing_with_device(&self, device: VR_DEVICE) -> bool {
        self.player.is_grabbing_with_device(device)
    }
}

pub struct PlayerCharacterRuntimeDataViewMut<'a> {
    player: &'a mut PlayerCharacter,
}

impl<'a> PlayerCharacterRuntimeDataViewMut<'a> {
    #[inline(always)]
    pub fn as_ref(&self) -> PlayerCharacterRuntimeDataView<'_> {
        PlayerCharacterRuntimeDataView {
            player: &*self.player,
        }
    }

    #[inline(always)]
    pub fn player_runtime_data_flat(&mut self) -> Option<&mut PLAYER_RUNTIME_DATA_FLAT> {
        if crate::runtime::is_vr() {
            None
        } else {
            Some(self.player.get_player_runtime_data_flat_mut())
        }
    }

    #[inline(always)]
    pub fn player_runtime_data_se(&mut self) -> Option<&mut PLAYER_RUNTIME_DATA_SE> {
        if crate::runtime::is_se() {
            Some(self.player.get_player_runtime_data_se_mut())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn player_runtime_data_ae(&mut self) -> Option<&mut PLAYER_RUNTIME_DATA_AE> {
        if crate::runtime::is_ae() {
            Some(self.player.get_player_runtime_data_ae_mut())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn player_runtime_data_vr(&mut self) -> Option<&mut VR_PLAYER_RUNTIME_DATA> {
        if crate::runtime::is_vr() {
            Some(self.player.get_player_runtime_data_vr_mut())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn crime_value(&mut self) -> &mut CrimeValue {
        self.player.get_crime_value_mut()
    }

    #[inline(always)]
    pub fn info_runtime_head_data(&mut self) -> &mut PLAYER_INFO_RUNTIME_HEAD_DATA {
        self.player.get_player_info_runtime_head_data_mut()
    }

    #[inline(always)]
    pub fn info_runtime_tail_data(&mut self) -> &mut PLAYER_INFO_RUNTIME_TAIL_DATA {
        self.player.get_player_info_runtime_tail_data_mut()
    }

    #[inline(always)]
    pub fn location_state_data(&mut self) -> &mut PLAYER_RUNTIME_LOCATION_STATE_DATA {
        self.player.get_player_runtime_location_state_data_mut()
    }

    #[inline(always)]
    pub fn target_loc_data(&mut self) -> &mut PLAYER_RUNTIME_TARGET_LOC_DATA {
        self.player.get_player_runtime_target_loc_data_mut()
    }

    #[inline(always)]
    pub fn queued_target_loc_flat(&mut self) -> Option<&mut PLAYER_TARGET_LOC> {
        if crate::runtime::is_vr() {
            None
        } else {
            Some(self.player.get_queued_target_loc_flat_mut())
        }
    }

    #[inline(always)]
    pub fn queued_target_loc_vr(&mut self) -> Option<&mut VR_PLAYER_TARGET_LOC> {
        if crate::runtime::is_vr() {
            Some(self.player.get_queued_target_loc_vr_mut())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn advance_data(&mut self) -> &mut PLAYER_RUNTIME_ADVANCE_DATA {
        self.player.get_player_runtime_advance_data_mut()
    }

    #[inline(always)]
    pub fn tint_race_data(&mut self) -> &mut PLAYER_RUNTIME_TINT_RACE_DATA {
        self.player.get_player_runtime_tint_race_data_mut()
    }

    #[inline(always)]
    pub fn world_data(&mut self) -> &mut PLAYER_RUNTIME_WORLD_DATA {
        self.player.get_player_runtime_world_data_mut()
    }

    #[inline(always)]
    pub fn interaction_data(&mut self) -> &mut PLAYER_RUNTIME_INTERACTION_DATA {
        self.player.get_player_runtime_interaction_data_mut()
    }

    #[inline(always)]
    pub fn game_state_data(&mut self) -> &mut GameStateData {
        self.player.get_game_stats_data_mut()
    }

    #[inline(always)]
    pub fn race_data(&mut self) -> &mut PlayerCharacterRaceData {
        self.player.get_race_data_mut()
    }

    #[inline(always)]
    pub fn info_runtime_data_flat(&mut self) -> Option<&mut INFO_RUNTIME_DATA> {
        if crate::runtime::is_vr() {
            None
        } else {
            Some(self.player.get_info_runtime_data_flat_mut())
        }
    }

    #[inline(always)]
    pub fn info_runtime_data_vr(&mut self) -> Option<&mut VR_INFO_RUNTIME_DATA> {
        self.player.get_info_runtime_data_vr_mut()
    }

    #[inline(always)]
    pub fn actor_doing_player_command(&self) -> NiPointer<Actor> {
        self.player.get_actor_doing_player_command()
    }

    #[inline(always)]
    pub fn grab_data_flat(&mut self) -> Option<&mut GrabData> {
        if crate::runtime::is_vr() {
            None
        } else {
            Some(self.player.get_grab_data_flat_mut())
        }
    }

    #[inline(always)]
    pub fn grab_data_vr(&mut self, device: VR_DEVICE) -> Option<&mut VRGrabData> {
        self.player.get_grab_data_vr_mut(device)
    }

    #[inline(always)]
    pub fn queued_target_loc_reset_weather(&self) -> bool {
        self.player.queued_target_loc_reset_weather()
    }

    #[inline(always)]
    pub fn set_queued_target_loc_reset_weather(&mut self, value: bool) {
        self.player.set_queued_target_loc_reset_weather(value);
    }

    #[inline(always)]
    pub fn queued_target_loc_allow_auto_save(&self) -> bool {
        self.player.queued_target_loc_allow_auto_save()
    }

    #[inline(always)]
    pub fn set_queued_target_loc_allow_auto_save(&mut self, value: bool) {
        self.player.set_queued_target_loc_allow_auto_save(value);
    }

    #[inline(always)]
    pub fn queued_target_loc_is_valid(&self) -> bool {
        self.player.queued_target_loc_is_valid()
    }

    #[inline(always)]
    pub fn set_queued_target_loc_is_valid(&mut self, value: bool) {
        self.player.set_queued_target_loc_is_valid(value);
    }

    #[inline(always)]
    pub fn has_actor_doing_command(&self) -> bool {
        self.player.has_actor_doing_command()
    }

    #[inline(always)]
    pub fn grabbed_ref(&self, device: VR_DEVICE) -> NiPointer<TESObjectREFR> {
        self.player.get_grabbed_ref(device)
    }

    #[inline(always)]
    pub fn grab_object_weight(&self, device: VR_DEVICE) -> f32 {
        self.player.grab_object_weight(device)
    }

    #[inline(always)]
    pub fn set_grab_object_weight(&mut self, device: VR_DEVICE, value: f32) {
        self.player.set_grab_object_weight(device, value);
    }

    #[inline(always)]
    pub fn grab_distance(&self, device: VR_DEVICE) -> f32 {
        self.player.grab_distance(device)
    }

    #[inline(always)]
    pub fn set_grab_distance(&mut self, device: VR_DEVICE, value: f32) {
        self.player.set_grab_distance(device, value);
    }

    #[inline(always)]
    pub fn is_grabbing(&self) -> bool {
        self.player.is_grabbing()
    }

    #[inline(always)]
    pub fn is_grabbing_with_device(&self, device: VR_DEVICE) -> bool {
        self.player.is_grabbing_with_device(device)
    }
}

impl PlayerCharacter {
    #[inline(always)]
    pub fn get_singleton() -> *mut PlayerCharacter {
        let singleton = Self::singleton_storage();
        if singleton.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*singleton).get() }
        }
    }

    #[inline(always)]
    pub fn is_god_mode() -> bool {
        let flag = Self::god_mode_flag_storage();
        !flag.is_null() && unsafe { *flag }
    }

    #[inline(always)]
    pub fn runtime_view(&self) -> PlayerCharacterRuntimeDataView<'_> {
        PlayerCharacterRuntimeDataView { player: self }
    }

    #[inline(always)]
    pub fn runtime_view_mut(&mut self) -> PlayerCharacterRuntimeDataViewMut<'_> {
        PlayerCharacterRuntimeDataViewMut { player: self }
    }

    #[inline(always)]
    pub fn attach_weapon(&mut self, weapon: *mut TESObjectWEAP, attach_to_shield_hand: bool) {
        let vfunc = crate::runtime::require_offset(
            Self::VFUNC_ATTACH_WEAPON.offset(),
            "PlayerCharacter::attach_weapon",
        );
        let func: extern "C" fn(*mut Self, *mut TESObjectWEAP, bool) =
            unsafe { crate::relocation::virtual_function(self as *mut Self, vfunc) };
        func(self, weapon, attach_to_shield_hand);
    }

    #[inline(always)]
    pub fn center_on_cell_name(&mut self, cell_name: *const c_char) -> bool {
        self.center_on_cell_impl(cell_name, core::ptr::null_mut())
    }

    #[inline(always)]
    pub fn center_on_cell_name_cstr(&mut self, cell_name: &CStr) -> bool {
        self.center_on_cell_name(cell_name.as_ptr())
    }

    #[inline(always)]
    pub fn center_on_cell(&mut self, cell: *mut TESObjectCELL) -> bool {
        self.center_on_cell_impl(core::ptr::null(), cell)
    }

    #[inline(always)]
    pub fn end_grab_object(&mut self) {
        if crate::runtime::is_vr() {
            // TODO: CommonLib still exposes one shared `EndGrabObject()` body, but the vendored
            // VR surface does not prove a single player-wide grab-type state equivalent to flat
            // `grabType`. Keep the translated VR path as a no-op until that contract is source-backed.
            return;
        }

        if self
            .get_player_runtime_data_flat()
            .grab_type
            .all(GrabbingType::Normal)
        {
            self.destroy_mouse_springs();
        }
    }

    #[inline(always)]
    pub fn get_actor_doing_player_command(&self) -> NiPointer<Actor> {
        self.get_player_runtime_interaction_data()
            .actor_doing_player_command
            .get()
    }

    #[inline(always)]
    pub fn get_crime_value(&self) -> &CrimeValue {
        crate::runtime_assert_size!(CrimeValue, se_ae: 0x60, vr: 0x60);
        &self.get_player_runtime_world_data().crime_value
    }

    #[inline(always)]
    pub fn get_crime_value_mut(&mut self) -> &mut CrimeValue {
        crate::runtime_assert_size!(CrimeValue, se_ae: 0x60, vr: 0x60);
        &mut self.get_player_runtime_world_data_mut().crime_value
    }

    #[inline(always)]
    pub fn get_equipped_weapons_damage(&self) -> f32 {
        if crate::runtime::is_ae() {
            self.get_equipped_weapons_damage_ae()
        } else {
            // TODO: `PlayerCharacter.cpp` only provides `RELOCATION_ID(0, 40252)` for
            // `GetEquippedWeaponsDamage()`. Keep non-AE runtimes unavailable instead of calling an
            // unresolved ID or inventing a replacement implementation.
            0.0
        }
    }

    #[inline(always)]
    pub fn get_game_stats_data(&self) -> &GameStateData {
        crate::runtime_assert_size!(GameStateData, se_ae: 0x0C, vr: 0x0C);
        self.game_state_data_impl()
    }

    #[inline(always)]
    pub fn get_game_stats_data_mut(&mut self) -> &mut GameStateData {
        crate::runtime_assert_size!(GameStateData, se_ae: 0x0C, vr: 0x0C);
        self.game_state_data_impl_mut()
    }

    #[inline(always)]
    pub fn get_player_runtime_target_loc_data(&self) -> &PLAYER_RUNTIME_TARGET_LOC_DATA {
        crate::runtime_assert_size!(PLAYER_RUNTIME_TARGET_LOC_DATA, se_ae: 0x40, vr: 0x40);
        self.player_runtime_target_loc_data_impl()
    }

    #[inline(always)]
    pub fn get_player_runtime_target_loc_data_mut(
        &mut self,
    ) -> &mut PLAYER_RUNTIME_TARGET_LOC_DATA {
        crate::runtime_assert_size!(PLAYER_RUNTIME_TARGET_LOC_DATA, se_ae: 0x40, vr: 0x40);
        self.player_runtime_target_loc_data_impl_mut()
    }

    #[inline(always)]
    pub fn get_queued_target_loc_flat(&self) -> &PLAYER_TARGET_LOC {
        crate::runtime::require_flat("PlayerCharacter::get_queued_target_loc_flat");
        crate::runtime_assert_size!(PLAYER_TARGET_LOC, se_ae: 0x48, vr: 0x0);
        self.queued_target_loc_flat_impl()
    }

    #[inline(always)]
    pub fn get_queued_target_loc_flat_mut(&mut self) -> &mut PLAYER_TARGET_LOC {
        crate::runtime::require_flat("PlayerCharacter::get_queued_target_loc_flat_mut");
        crate::runtime_assert_size!(PLAYER_TARGET_LOC, se_ae: 0x48, vr: 0x0);
        self.queued_target_loc_flat_impl_mut()
    }

    #[inline(always)]
    pub fn get_queued_target_loc_vr(&self) -> &VR_PLAYER_TARGET_LOC {
        crate::runtime::require_vr("PlayerCharacter::get_queued_target_loc_vr");
        crate::runtime_assert_size!(VR_PLAYER_TARGET_LOC, se: 0x0, ae: 0x0, vr: 0x50);
        &self.get_player_runtime_data_vr().queued_target_loc
    }

    #[inline(always)]
    pub fn get_queued_target_loc_vr_mut(&mut self) -> &mut VR_PLAYER_TARGET_LOC {
        crate::runtime::require_vr("PlayerCharacter::get_queued_target_loc_vr_mut");
        crate::runtime_assert_size!(VR_PLAYER_TARGET_LOC, se: 0x0, ae: 0x0, vr: 0x50);
        &mut self.get_player_runtime_data_vr_mut().queued_target_loc
    }

    #[inline(always)]
    pub fn get_grab_data_flat(&self) -> &GrabData {
        crate::runtime::require_flat("PlayerCharacter::get_grab_data_flat");
        crate::runtime_assert_size!(GrabData, se_ae: 0x48, vr: 0x0);
        self.grab_data_flat_impl()
    }

    #[inline(always)]
    pub fn get_grab_data_flat_mut(&mut self) -> &mut GrabData {
        crate::runtime::require_flat("PlayerCharacter::get_grab_data_flat_mut");
        crate::runtime_assert_size!(GrabData, se_ae: 0x48, vr: 0x0);
        self.grab_data_flat_impl_mut()
    }

    #[inline(always)]
    pub fn get_grab_data_vr(&self, device: VR_DEVICE) -> Option<&VRGrabData> {
        if crate::runtime::is_vr() {
            Some(&self.get_player_runtime_data_vr().grabbed_object_data[device as usize])
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_grab_data_vr_mut(&mut self, device: VR_DEVICE) -> Option<&mut VRGrabData> {
        if crate::runtime::is_vr() {
            Some(&mut self.get_player_runtime_data_vr_mut().grabbed_object_data[device as usize])
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_grabbed_ref(&self, device: VR_DEVICE) -> NiPointer<TESObjectREFR> {
        if crate::runtime::is_vr() {
            self.get_grab_data_vr(device)
                .expect("PlayerCharacter::get_grabbed_ref requires VR grab data in VR")
                .grabbed_object
                .get()
        } else {
            self.get_grab_data_flat().grabbed_object.get()
        }
    }

    #[inline(always)]
    pub fn get_info_runtime_data_flat(&self) -> &INFO_RUNTIME_DATA {
        crate::runtime::require_flat("PlayerCharacter::get_info_runtime_data_flat");
        crate::runtime_assert_size!(INFO_RUNTIME_DATA, se: 0x13C, ae: 0x13C, vr: 0x0);
        &self.get_player_runtime_data_flat().info_runtime_data
    }

    #[inline(always)]
    pub fn get_info_runtime_data_flat_mut(&mut self) -> &mut INFO_RUNTIME_DATA {
        crate::runtime::require_flat("PlayerCharacter::get_info_runtime_data_flat_mut");
        crate::runtime_assert_size!(INFO_RUNTIME_DATA, se: 0x13C, ae: 0x13C, vr: 0x0);
        &mut self.get_player_runtime_data_flat_mut().info_runtime_data
    }

    #[inline(always)]
    pub fn get_overlay_tint_mask(&self, original: *mut TintMask) -> *mut TintMask {
        // TODO: `PlayerCharacter.cpp` still returns `nullptr` in VR and comments that the VR tint
        // layout is not yet understood. Mirror that honest limitation here instead of guessing a
        // universal VR tint storage contract.
        if crate::runtime::is_vr() {
            return core::ptr::null_mut();
        }

        let flat = self.get_player_runtime_data_flat();
        let tint_masks = &flat.tint_masks;
        let overlay_tint_masks = flat.overlay_tint_masks;
        if overlay_tint_masks.is_null() {
            return core::ptr::null_mut();
        }

        let tint_masks = unsafe { tint_masks.as_slice() };
        let overlay_tint_masks = unsafe { (*overlay_tint_masks).as_slice() };
        for (idx, &tint_mask) in tint_masks.iter().enumerate() {
            if tint_mask == original {
                return overlay_tint_masks
                    .get(idx)
                    .copied()
                    .unwrap_or(core::ptr::null_mut());
            }
        }

        core::ptr::null_mut()
    }

    #[inline(always)]
    pub fn get_race_data(&self) -> &PlayerCharacterRaceData {
        crate::runtime_assert_size!(PlayerCharacterRaceData, se_ae: 0x18, vr: 0x18);
        self.race_data_impl()
    }

    #[inline(always)]
    pub fn get_race_data_mut(&mut self) -> &mut PlayerCharacterRaceData {
        crate::runtime_assert_size!(PlayerCharacterRaceData, se_ae: 0x18, vr: 0x18);
        self.race_data_impl_mut()
    }

    #[inline(always)]
    pub fn get_tint_list(&self) -> *mut BSTArray<*mut TintMask> {
        // TODO: `PlayerCharacter.cpp` still returns `nullptr` in VR for tint-list access because
        // the VR tint storage layout is unresolved in CommonLib itself.
        if crate::runtime::is_vr() {
            return core::ptr::null_mut();
        }

        let flat = self.get_player_runtime_data_flat();
        if !flat.overlay_tint_masks.is_null() {
            return flat.overlay_tint_masks;
        }

        &flat.tint_masks as *const _ as *mut _
    }

    #[inline(always)]
    pub fn get_tint_mask(&self, tint_type: u32, index: u32) -> *mut TintMask {
        // TODO: `PlayerCharacter.cpp` still returns `nullptr` in VR and leaves the VR tint-member
        // path unresolved. Keep that source-backed compromise here too.
        if crate::runtime::is_vr() {
            core::ptr::null_mut()
        } else {
            self.get_tint_mask_flat(tint_type, index)
        }
    }

    #[inline(always)]
    pub fn get_info_runtime_data_vr(&self) -> Option<&VR_INFO_RUNTIME_DATA> {
        crate::runtime_assert_size!(VR_INFO_RUNTIME_DATA, se: 0x0, ae: 0x0, vr: 0x140);
        unsafe { self.info_runtime_data_vr_ptr().as_ref() }
    }

    #[inline(always)]
    pub fn get_info_runtime_data_vr_mut(&mut self) -> Option<&mut VR_INFO_RUNTIME_DATA> {
        crate::runtime_assert_size!(VR_INFO_RUNTIME_DATA, se: 0x0, ae: 0x0, vr: 0x140);
        unsafe { self.info_runtime_data_vr_ptr().as_mut() }
    }

    #[inline(always)]
    pub fn get_player_info_runtime_head_data(&self) -> &PLAYER_INFO_RUNTIME_HEAD_DATA {
        crate::runtime_assert_size!(PLAYER_INFO_RUNTIME_HEAD_DATA, se_ae: 0x38, vr: 0x38);
        self.player_info_runtime_head_data_impl()
    }

    #[inline(always)]
    pub fn get_player_info_runtime_head_data_mut(&mut self) -> &mut PLAYER_INFO_RUNTIME_HEAD_DATA {
        crate::runtime_assert_size!(PLAYER_INFO_RUNTIME_HEAD_DATA, se_ae: 0x38, vr: 0x38);
        self.player_info_runtime_head_data_impl_mut()
    }

    #[inline(always)]
    pub fn get_player_info_runtime_tail_data(&self) -> &PLAYER_INFO_RUNTIME_TAIL_DATA {
        crate::runtime_assert_size!(PLAYER_INFO_RUNTIME_TAIL_DATA, se_ae: 0xF8, vr: 0xF8);
        self.player_info_runtime_tail_data_impl()
    }

    #[inline(always)]
    pub fn get_player_info_runtime_tail_data_mut(&mut self) -> &mut PLAYER_INFO_RUNTIME_TAIL_DATA {
        crate::runtime_assert_size!(PLAYER_INFO_RUNTIME_TAIL_DATA, se_ae: 0xF8, vr: 0xF8);
        self.player_info_runtime_tail_data_impl_mut()
    }

    #[inline(always)]
    pub fn get_node_data_vr(&self) -> Option<&VR_NODE_DATA> {
        crate::runtime_assert_size!(VR_NODE_DATA, se: 0x0, ae: 0x0, vr: 0x290);
        unsafe { self.node_data_vr_ptr().as_ref() }
    }

    #[inline(always)]
    pub fn get_node_data_vr_mut(&mut self) -> Option<&mut VR_NODE_DATA> {
        crate::runtime_assert_size!(VR_NODE_DATA, se: 0x0, ae: 0x0, vr: 0x290);
        unsafe { self.node_data_vr_ptr().as_mut() }
    }

    #[inline(always)]
    pub fn get_player_runtime_data_vr(&self) -> &VR_PLAYER_RUNTIME_DATA {
        crate::runtime::require_vr("PlayerCharacter::get_player_runtime_data_vr");
        crate::runtime_assert_size!(VR_PLAYER_RUNTIME_DATA, se: 0x0, ae: 0x0, vr: 0xF18);
        self.player_runtime_data_vr_impl()
    }

    #[inline(always)]
    pub fn get_player_runtime_data_vr_mut(&mut self) -> &mut VR_PLAYER_RUNTIME_DATA {
        crate::runtime::require_vr("PlayerCharacter::get_player_runtime_data_vr_mut");
        crate::runtime_assert_size!(VR_PLAYER_RUNTIME_DATA, se: 0x0, ae: 0x0, vr: 0xF18);
        self.player_runtime_data_vr_impl_mut()
    }

    #[inline(always)]
    pub fn get_player_runtime_data_flat(&self) -> &PLAYER_RUNTIME_DATA_FLAT {
        crate::runtime::require_flat("PlayerCharacter::get_player_runtime_data_flat");
        crate::runtime_assert_size!(PLAYER_RUNTIME_DATA_FLAT, se_ae: 0x808, vr: 0x0);
        self.player_runtime_data_flat_impl()
    }

    #[inline(always)]
    pub fn get_player_runtime_data_flat_mut(&mut self) -> &mut PLAYER_RUNTIME_DATA_FLAT {
        crate::runtime::require_flat("PlayerCharacter::get_player_runtime_data_flat_mut");
        crate::runtime_assert_size!(PLAYER_RUNTIME_DATA_FLAT, se_ae: 0x808, vr: 0x0);
        self.player_runtime_data_flat_impl_mut()
    }

    #[inline(always)]
    pub fn get_player_runtime_data_se(&self) -> &PLAYER_RUNTIME_DATA_SE {
        crate::runtime::require_se("PlayerCharacter::get_player_runtime_data_se");
        crate::runtime_assert_size!(PLAYER_RUNTIME_DATA_SE, se: 0x808, ae: 0x0, vr: 0x0);
        self.get_player_runtime_data_flat()
    }

    #[inline(always)]
    pub fn get_player_runtime_data_se_mut(&mut self) -> &mut PLAYER_RUNTIME_DATA_SE {
        crate::runtime::require_se("PlayerCharacter::get_player_runtime_data_se_mut");
        crate::runtime_assert_size!(PLAYER_RUNTIME_DATA_SE, se: 0x808, ae: 0x0, vr: 0x0);
        self.get_player_runtime_data_flat_mut()
    }

    #[inline(always)]
    pub fn get_player_runtime_data_ae(&self) -> &PLAYER_RUNTIME_DATA_AE {
        crate::runtime::require_ae("PlayerCharacter::get_player_runtime_data_ae");
        crate::runtime_assert_size!(PLAYER_RUNTIME_DATA_AE, se: 0x0, ae: 0x808, vr: 0x0);
        self.get_player_runtime_data_flat()
    }

    #[inline(always)]
    pub fn get_player_runtime_data_ae_mut(&mut self) -> &mut PLAYER_RUNTIME_DATA_AE {
        crate::runtime::require_ae("PlayerCharacter::get_player_runtime_data_ae_mut");
        crate::runtime_assert_size!(PLAYER_RUNTIME_DATA_AE, se: 0x0, ae: 0x808, vr: 0x0);
        self.get_player_runtime_data_flat_mut()
    }

    #[inline(always)]
    pub fn get_player_runtime_location_state_data(&self) -> &PLAYER_RUNTIME_LOCATION_STATE_DATA {
        crate::runtime_assert_size!(PLAYER_RUNTIME_LOCATION_STATE_DATA, se_ae: 0x20, vr: 0x20);
        self.player_runtime_location_state_data_impl()
    }

    #[inline(always)]
    pub fn get_player_runtime_location_state_data_mut(
        &mut self,
    ) -> &mut PLAYER_RUNTIME_LOCATION_STATE_DATA {
        crate::runtime_assert_size!(PLAYER_RUNTIME_LOCATION_STATE_DATA, se_ae: 0x20, vr: 0x20);
        self.player_runtime_location_state_data_impl_mut()
    }

    #[inline(always)]
    pub fn get_player_runtime_advance_data(&self) -> &PLAYER_RUNTIME_ADVANCE_DATA {
        crate::runtime_assert_size!(PLAYER_RUNTIME_ADVANCE_DATA, se_ae: 0x0C, vr: 0x0C);
        self.player_runtime_advance_data_impl()
    }

    #[inline(always)]
    pub fn get_player_runtime_advance_data_mut(&mut self) -> &mut PLAYER_RUNTIME_ADVANCE_DATA {
        crate::runtime_assert_size!(PLAYER_RUNTIME_ADVANCE_DATA, se_ae: 0x0C, vr: 0x0C);
        self.player_runtime_advance_data_impl_mut()
    }

    #[inline(always)]
    pub fn get_player_runtime_tint_race_data(&self) -> &PLAYER_RUNTIME_TINT_RACE_DATA {
        crate::runtime_assert_size!(PLAYER_RUNTIME_TINT_RACE_DATA, se_ae: 0x40, vr: 0x40);
        self.player_runtime_tint_race_data_impl()
    }

    #[inline(always)]
    pub fn get_player_runtime_tint_race_data_mut(&mut self) -> &mut PLAYER_RUNTIME_TINT_RACE_DATA {
        crate::runtime_assert_size!(PLAYER_RUNTIME_TINT_RACE_DATA, se_ae: 0x40, vr: 0x40);
        self.player_runtime_tint_race_data_impl_mut()
    }

    #[inline(always)]
    pub fn get_player_runtime_world_data(&self) -> &PLAYER_RUNTIME_WORLD_DATA {
        crate::runtime_assert_size!(PLAYER_RUNTIME_WORLD_DATA, se_ae: 0x268, vr: 0x268);
        self.player_runtime_world_data_impl()
    }

    #[inline(always)]
    pub fn get_player_runtime_world_data_mut(&mut self) -> &mut PLAYER_RUNTIME_WORLD_DATA {
        crate::runtime_assert_size!(PLAYER_RUNTIME_WORLD_DATA, se_ae: 0x268, vr: 0x268);
        self.player_runtime_world_data_impl_mut()
    }

    #[inline(always)]
    pub fn get_player_runtime_interaction_data(&self) -> &PLAYER_RUNTIME_INTERACTION_DATA {
        crate::runtime_assert_size!(PLAYER_RUNTIME_INTERACTION_DATA, se_ae: 0x210, vr: 0x210);
        self.player_runtime_interaction_data_impl()
    }

    #[inline(always)]
    pub fn get_player_runtime_interaction_data_mut(
        &mut self,
    ) -> &mut PLAYER_RUNTIME_INTERACTION_DATA {
        crate::runtime_assert_size!(PLAYER_RUNTIME_INTERACTION_DATA, se_ae: 0x210, vr: 0x210);
        self.player_runtime_interaction_data_impl_mut()
    }

    #[inline(always)]
    pub fn queued_target_loc_reset_weather(&self) -> bool {
        if crate::runtime::is_vr() {
            self.get_queued_target_loc_vr().reset_weather
        } else {
            self.get_queued_target_loc_flat().reset_weather
        }
    }

    #[inline(always)]
    pub fn set_queued_target_loc_reset_weather(&mut self, value: bool) {
        if crate::runtime::is_vr() {
            self.get_queued_target_loc_vr_mut().reset_weather = value;
        } else {
            self.get_queued_target_loc_flat_mut().reset_weather = value;
        }
    }

    #[inline(always)]
    pub fn queued_target_loc_allow_auto_save(&self) -> bool {
        if crate::runtime::is_vr() {
            self.get_queued_target_loc_vr().allow_auto_save != 0
        } else {
            self.get_queued_target_loc_flat().allow_auto_save
        }
    }

    #[inline(always)]
    pub fn set_queued_target_loc_allow_auto_save(&mut self, value: bool) {
        if crate::runtime::is_vr() {
            self.get_queued_target_loc_vr_mut().allow_auto_save = value.into();
        } else {
            self.get_queued_target_loc_flat_mut().allow_auto_save = value;
        }
    }

    #[inline(always)]
    pub fn queued_target_loc_is_valid(&self) -> bool {
        if crate::runtime::is_vr() {
            self.get_queued_target_loc_vr().is_valid
        } else {
            self.get_queued_target_loc_flat().is_valid
        }
    }

    #[inline(always)]
    pub fn set_queued_target_loc_is_valid(&mut self, value: bool) {
        if crate::runtime::is_vr() {
            self.get_queued_target_loc_vr_mut().is_valid = value;
        } else {
            self.get_queued_target_loc_flat_mut().is_valid = value;
        }
    }

    #[inline(always)]
    pub fn grab_object_weight(&self, device: VR_DEVICE) -> f32 {
        if crate::runtime::is_vr() {
            self.get_grab_data_vr(device)
                .expect("PlayerCharacter::grab_object_weight requires VR grab data in VR")
                .grab_object_weight
        } else {
            self.get_grab_data_flat().grab_object_weight
        }
    }

    #[inline(always)]
    pub fn set_grab_object_weight(&mut self, device: VR_DEVICE, value: f32) {
        if crate::runtime::is_vr() {
            self.get_grab_data_vr_mut(device)
                .expect("PlayerCharacter::set_grab_object_weight requires VR grab data in VR")
                .grab_object_weight = value;
        } else {
            self.get_grab_data_flat_mut().grab_object_weight = value;
        }
    }

    #[inline(always)]
    pub fn grab_distance(&self, device: VR_DEVICE) -> f32 {
        if crate::runtime::is_vr() {
            self.get_grab_data_vr(device)
                .expect("PlayerCharacter::grab_distance requires VR grab data in VR")
                .grab_distance
        } else {
            self.get_grab_data_flat().grab_distance
        }
    }

    #[inline(always)]
    pub fn set_grab_distance(&mut self, device: VR_DEVICE, value: f32) {
        if crate::runtime::is_vr() {
            self.get_grab_data_vr_mut(device)
                .expect("PlayerCharacter::set_grab_distance requires VR grab data in VR")
                .grab_distance = value;
        } else {
            self.get_grab_data_flat_mut().grab_distance = value;
        }
    }

    #[inline(always)]
    pub fn has_actor_doing_command(&self) -> bool {
        self.get_player_runtime_interaction_data()
            .actor_doing_player_command
            .has_value()
    }

    #[inline(always)]
    pub fn is_grabbing(&self) -> bool {
        if crate::runtime::is_vr() {
            self.get_player_runtime_data_vr()
                .grabbed_object_data
                .iter()
                .any(|grab_data| grab_data.grabbed_object.has_value())
        } else {
            self.get_grab_data_flat().grabbed_object.has_value()
        }
    }

    #[inline(always)]
    pub fn is_grabbing_with_device(&self, device: VR_DEVICE) -> bool {
        crate::runtime::is_vr()
            && self.get_player_runtime_data_vr().grabbed_object_data[device as usize]
                .grabbed_object
                .has_value()
    }
}
