use crate::re::Actor;
use crate::re::ActorHandle;
use crate::re::ActorValue;
use crate::re::BGSAttackData;
use crate::re::InventoryEntryData;
use crate::re::MagicItem;
use crate::re::NiPoint3;
use crate::re::NiPointer;
use crate::re::ObjectRefHandle;
use crate::re::SpellItem;
use crate::re::TESObjectWEAP;
use crate::re::VATSCommand;
use crate::re::bgs_body_part_defs::LimbEnum;
use crate::relocation::RelocationID;

bitflags::bitflags! {
    /// C++ `RE::HitData::Flag`
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct HitFlag: u32 {
        const Blocked = 1 << 0;
        const BlockWithWeapon = 1 << 1;
        const BlockCandidate = 1 << 2;
        const Critical = 1 << 3;
        const CriticalOnDeath = 1 << 4;
        const Fatal = 1 << 5;
        const DismemberLimb = 1 << 6;
        const ExplodeLimb = 1 << 7;
        const CrippleLimb = 1 << 8;
        const Disarm = 1 << 9;
        const DisableWeapon = 1 << 10;
        const SneakAttack = 1 << 11;
        const IgnoreCritical = 1 << 12;
        const PredictDamage = 1 << 13;
        const Bash = 1 << 14;
        const TimedBash = 1 << 15;
        const PowerAttack = 1 << 16;
        const LeftHand = 1 << 17;
        const MeleeAttack = 1 << 18;
        const Ricochet = 1 << 19;
        const Explosion = 1 << 20;
    }
}

/// C++ `RE::HitData`
#[repr(C)]
pub struct HitData {
    pub hit_position: NiPoint3,                // 00
    pub hit_direction: NiPoint3,               // 0C
    pub aggressor: ActorHandle,                // 18
    pub target: ActorHandle,                   // 1C
    pub source_ref: ObjectRefHandle,           // 20
    pub pad24: u32,                            // 24
    pub attack_data: NiPointer<BGSAttackData>, // 28
    pub weapon: *mut TESObjectWEAP,            // 30
    pub critical_effect: *mut MagicItem,       // 38
    pub attack_data_spell: *mut SpellItem,     // 40
    pub vats_command: *mut VATSCommand,        // 48
    pub total_damage: f32,                     // 50
    pub physical_damage: f32,                  // 54
    pub targeted_limb_damage: f32,             // 58
    pub percent_blocked: f32,                  // 5C
    pub resisted_physical_damage: f32,         // 60
    pub resisted_typed_damage: f32,            // 64
    pub stagger: f32,                          // 68
    pub sneak_attack_bonus: f32,               // 6C
    pub bonus_health_damage_mult: f32,         // 70
    pub unk74: f32,                            // 74
    pub push_back: f32,                        // 78
    pub reflected_damage: f32,                 // 7C
    pub flags: HitFlag,                        // 80
    pub equip_index: u32,                      // 84
    pub skill: ActorValue,                     // 88
    pub damage_limb: LimbEnum,                 // 8C
}

const _: () = assert!(core::mem::size_of::<HitData>() == 0x90);

impl HitData {
    // RELOCATION_ID SE: 42832, AE: 44001
    crate::relocation_func! {
        pub fn populate(
            &mut self,
            aggressor: *mut Actor,
            target: *mut Actor,
            weapon: *mut InventoryEntryData,
            is_left_hand: bool,
        ) => RelocationID::new(42832, 44001)
    }

    pub fn create(
        aggressor: *mut Actor,
        target: *mut Actor,
        weapon: *mut InventoryEntryData,
        is_left_hand: bool,
    ) -> *mut Self {
        unsafe {
            // Р В Р’В Р Р†Р вЂљРІвЂћСћР В Р Р‹Р Р†Р вЂљРІвЂћвЂ“Р В Р’В Р СћРІР‚ВР В Р’В Р вЂ™Р’ВµР В Р’В Р вЂ™Р’В»Р В Р Р‹Р В Р РЏР В Р’В Р вЂ™Р’ВµР В Р’В Р РЋР’В Р В Р’В Р РЋРІР‚вЂќР В Р’В Р вЂ™Р’В°Р В Р’В Р РЋР’ВР В Р Р‹Р В Р РЏР В Р Р‹Р Р†Р вЂљРЎв„ўР В Р Р‹Р В Р вЂ° Р В Р Р‹Р Р†Р вЂљР Р‹Р В Р’В Р вЂ™Р’ВµР В Р Р‹Р В РІР‚С™Р В Р’В Р вЂ™Р’ВµР В Р’В Р вЂ™Р’В· MemoryManager Р В Р’В Р В Р вЂ№Р В Р’В Р РЋРІР‚СњР В Р’В Р вЂ™Р’В°Р В Р’В Р Р†РІР‚С›РІР‚вЂњР В Р Р‹Р В РІР‚С™Р В Р’В Р РЋРІР‚ВР В Р’В Р РЋР’ВР В Р’В Р вЂ™Р’В°
            let size = core::mem::size_of::<Self>();
            let hit_data = crate::ffi::commonlib_malloc(size) as *mut Self;

            if !hit_data.is_null() {
                // Р В Р’В Р вЂ™Р’ВР В Р’В Р В РІР‚В¦Р В Р’В Р РЋРІР‚ВР В Р Р‹Р Р†Р вЂљР’В Р В Р’В Р РЋРІР‚ВР В Р’В Р вЂ™Р’В°Р В Р’В Р вЂ™Р’В»Р В Р’В Р РЋРІР‚ВР В Р’В Р вЂ™Р’В·Р В Р’В Р РЋРІР‚ВР В Р Р‹Р В РІР‚С™Р В Р Р‹Р РЋРІР‚СљР В Р’В Р вЂ™Р’ВµР В Р’В Р РЋР’В Р В Р’В Р РЋРІР‚СћР В Р’В Р вЂ™Р’В±Р В Р Р‹Р В РІР‚В°Р В Р’В Р вЂ™Р’ВµР В Р’В Р РЋРІР‚СњР В Р Р‹Р Р†Р вЂљРЎв„ў Р В Р’В Р СћРІР‚ВР В Р’В Р В РІР‚В Р В Р’В Р РЋРІР‚ВР В Р’В Р вЂ™Р’В¶Р В Р’В Р РЋРІР‚СњР В Р’В Р РЋРІР‚СћР В Р’В Р В РІР‚В Р В Р Р‹Р Р†Р вЂљРІвЂћвЂ“Р В Р’В Р РЋР’В Р В Р’В Р РЋРІР‚СњР В Р’В Р РЋРІР‚СћР В Р’В Р В РІР‚В¦Р В Р Р‹Р В РЎвЂњР В Р Р‹Р Р†Р вЂљРЎв„ўР В Р Р‹Р В РІР‚С™Р В Р Р‹Р РЋРІР‚СљР В Р’В Р РЋРІР‚СњР В Р Р‹Р Р†Р вЂљРЎв„ўР В Р’В Р РЋРІР‚СћР В Р Р‹Р В РІР‚С™Р В Р’В Р РЋРІР‚СћР В Р’В Р РЋР’В
                (*hit_data).ctor();
                // Р В Р’В Р Р†Р вЂљРІР‚СњР В Р’В Р вЂ™Р’В°Р В Р’В Р РЋРІР‚вЂќР В Р’В Р РЋРІР‚СћР В Р’В Р вЂ™Р’В»Р В Р’В Р В РІР‚В¦Р В Р Р‹Р В Р РЏР В Р’В Р вЂ™Р’ВµР В Р’В Р РЋР’В Р В Р’В Р СћРІР‚ВР В Р’В Р вЂ™Р’В°Р В Р’В Р В РІР‚В¦Р В Р’В Р В РІР‚В¦Р В Р Р‹Р Р†Р вЂљРІвЂћвЂ“Р В Р’В Р вЂ™Р’Вµ
                (*hit_data).populate(aggressor, target, weapon, is_left_hand);
            }

            hit_data
        }
    }

    // RELOCATION_ID SE: 42826, AE: 43995
    crate::relocation_func! {
        fn ctor(&mut self) -> *mut Self => RelocationID::new(42826, 43995)
    }
}
