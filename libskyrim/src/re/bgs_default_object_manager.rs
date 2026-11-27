#![allow(non_camel_case_types)]

use core::ffi::c_char;

use core_util::{EnumSet, inherit};

use crate::offsets::offsets_rtti::RTTI_BGSDefaultObjectManager;
use crate::offsets::offsets_vtable::VTABLE_BGSDefaultObjectManager;
use crate::re::{FormCastable, FormType, TESForm};
use crate::relocation::{RelocationID, RttiType, VariantID, VariantOffset};

macro_rules! default_object_const_catalog {
    ($($name:ident = $value:expr,)+) => {
        $(pub const $name: Self = Self($value);)+
    };
}

macro_rules! default_object_variant_catalog {
    ($($name:ident = ($se:expr, $ae:expr, $vr:expr),)+) => {
        $(
            #[inline(always)]
            pub fn $name() -> Self {
                Self(VariantOffset::new($se, $ae, $vr).offset() as i32)
            }
        )+
    };
}

macro_rules! default_object_non_vr_catalog {
    ($($name:ident = $value:expr,)+) => {
        $(
            #[inline(always)]
            pub fn $name() -> Option<Self> {
                (!crate::runtime::is_vr()).then_some(Self($value))
            }
        )+
    };
}

macro_rules! default_object_ae_only_catalog {
    ($($name:ident = $value:expr,)+) => {
        $(
            #[inline(always)]
            pub fn $name() -> Option<Self> {
                crate::runtime::is_ae().then_some(Self($value))
            }
        )+
    };
}

macro_rules! default_object_vr_only_catalog {
    ($($name:ident = $value:expr,)+) => {
        $(
            #[inline(always)]
            pub fn $name() -> Option<Self> {
                crate::runtime::is_vr().then_some(Self($value))
            }
        )+
    };
}

macro_rules! default_object_id_catalog {
    ($($name:ident = $value:expr,)+) => {
        $(pub const $name: Self = Self($value);)+
    };
}

/// Runtime-aware C++ `RE::DEFAULT_OBJECTS::DEFAULT_OBJECT`.
///
/// The stable prefix of the catalog is represented as associated constants.
/// The runtime-varying and runtime-exclusive tail is exposed through helper
/// methods so we do not invent fake universal flat/VR indices.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DEFAULT_OBJECT(pub i32);

#[allow(non_snake_case, non_upper_case_globals)]
impl DEFAULT_OBJECT {
    default_object_const_catalog! {
        kWerewolfSpell = 0,
        kSittingAngleLimit = 1,
        kAllowPlayerShout = 2,
        kGold = 3,
        kLockpick = 4,
        kSkeletonKey = 5,
        kPlayerFaction = 6,
        kGuardFaction = 7,
        kDefaultMusic = 8,
        kBattleMusic = 9,
        kDeathMusic = 10,
        kSuccessMusic = 11,
        kLevelUpMusic = 12,
        kDungeonClearedMusic = 13,
        kPlayerVoiceMale = 14,
        kPlayerVoiceMaleChild = 15,
        kPlayerVoiceFemale = 16,
        kPlayerVoiceFemaleChild = 17,
        kEatPackageDefaultFood = 18,
        kLeftHandEquip = 19,
        kRightHandEquip = 20,
        kEitherHandEquip = 21,
        kVoiceEquip = 22,
        kPotionEquip = 23,
        kEveryActorAbility = 24,
        kCommandedActorAbility = 25,
        kDrugWearsOffImageSpace = 26,
        kFootstepSet = 27,
        kLandscapeMaterial = 28,
        kDragonLandZoneMarker = 29,
        kDragonCrashZoneMarker = 30,
        kCombatStyle = 31,
        kDefaultPackList = 32,
        kWaitForDialoguePackage = 33,
        kLocRefTypeBoss = 34,
        kVirtualLocation = 35,
        kPersistAllLocation = 36,
        kInventoryPlayer = 37,
        kPathingTestNPC = 38,
        kFavorCostSmall = 39,
        kFavorCostMedium = 40,
        kFavorCostLarge = 41,
        kFavorGiftsPerDay = 42,
        kActionSwimStateChange = 43,
        kActionLook = 44,
        kActionLeftAttack = 45,
        kActionLeftReady = 46,
        kActionLeftRelease = 47,
        kActionLeftInterrupt = 48,
        kActionRightAttack = 49,
        kActionRightReady = 50,
        kActionRightRelease = 51,
        kActionRightInterrupt = 52,
        kActionDualAttack = 53,
        kActionDualRelease = 54,
        kActionActivate = 55,
        kActionJump = 56,
        kActionFall = 57,
        kActionLand = 58,
        kActionSneak = 59,
        kActionVoice = 60,
        kActionVoiceReady = 61,
        kActionVoiceRelease = 62,
        kActionVoiceInterrupt = 63,
        kActionIdle = 64,
        kActionSprintStart = 65,
        kActionSprintStop = 66,
        kActionDraw = 67,
        kActionSheath = 68,
        kActionLeftPowerAttack = 69,
        kActionRightPowerAttack = 70,
        kActionDualPowerAttack = 71,
        kActionStaggerStart = 72,
        kActionBlockHit = 73,
        kActionBlockAnticipate = 74,
        kActionRecoil = 75,
        kActionLargeRecoil = 76,
        kActionBleedoutStart = 77,
        kActionBleedoutStop = 78,
        kActionIdleStop = 79,
        kActionWardHit = 80,
        kActionForceEquip = 81,
        kActionShieldChange = 82,
        kActionPathStart = 83,
        kActionPathEnd = 84,
        kActionLargeMovementDelta = 85,
        kActionFlyStart = 86,
        kActionFlyStop = 87,
        kActionHoverStart = 88,
        kActionHoverStop = 89,
        kActionBumpedInto = 90,
        kActionSummonedStart = 91,
        kActionTalkingIdle = 92,
        kActionListenIdle = 93,
        kActionDeath = 94,
        kActionDeathWait = 95,
        kActionIdleWarn = 96,
        kActionMoveStart = 97,
        kActionMoveStop = 98,
        kActionTurnRight = 99,
        kActionTurnLeft = 100,
        kActionTurnStop = 101,
        kActionMoveForward = 102,
        kActionMoveBackward = 103,
        kActionMoveLeft = 104,
        kActionMoveRight = 105,
        kActionResetAnimationGraph = 106,
        kActionKnockdown = 107,
        kActionGetUp = 108,
        kActionIdleStopInstant = 109,
        kActionRagdollInstant = 110,
        kActionWaterwalkStart = 111,
        kActionReload = 112,
        kPickupSoundGeneric = 113,
        kPutdownSoundGeneric = 114,
        kPickupSoundWeapon = 115,
        kPutdownSoundWeapon = 116,
        kPickupSoundArmor = 117,
        kPutdownSoundArmor = 118,
        kPickupSoundBook = 119,
        kPutdownSoundBook = 120,
        kPickupSoundIngredient = 121,
        kPutdownSoundIngredient = 122,
        kHarvestSound = 123,
        kHarvestFailedSound = 124,
        kWardBreakSound = 125,
        kWardAbsorbSound = 126,
        kWardDeflectSound = 127,
        kMagicFailSound = 128,
        kShoutFailSound = 129,
        kHeartbeatSoundFast = 130,
        kHeartbeatSoundSlow = 131,
        kImagespaceLowHealth = 132,
        kSoulCapturedSound = 133,
        kNoActivationSound = 134,
        kMapMenuLoopingSound = 135,
        kDialogueVoiceCategory = 136,
        kNonDialogueVoiceCategory = 137,
        kSFXToFadeInDialogueCategory = 138,
        kPauseDuringMenuCategoryFade = 139,
        kPauseDuringMenuCategoryImmediate = 140,
        kPauseDuringLoadingMenuCategory = 141,
        kMusicSoundCategory = 142,
        kStatsMuteCategory = 143,
        kStatsMusic = 144,
        kMasterSoundCategory = 145,
        kTimeSensitiveSoundCategory = 146,
        kDialogueOutputModel3D = 147,
        kDialogueOutputModel2D = 148,
        kPlayersOutputModel1stPerson = 149,
        kPlayersOutputModel3rdPerson = 150,
        kInterfaceOutputModel = 151,
        kReverbType = 152,
        kUnderwaterLoopSound = 153,
        kUnderwaterReverbType = 154,
        kKeywordHorse = 155,
        kKeywordUndead = 156,
        kKeywordNPC = 157,
        kKeywordBeastRace = 158,
        kKeywordDummyObject = 159,
        kKeywordUseGeometryEmitter = 160,
        kKeywordMustStop = 161,
        kKeywordUpdateDuringArchery = 162,
        kKeywordSkipOutfitItems = 163,
        kMaleFaceTextureSetHead = 164,
        kMaleFaceTextureSetMouth = 165,
        kMaleFaceTextureSetEyes = 166,
        kFemaleFaceTextureSetHead = 167,
        kFemaleFaceTextureSetMouth = 168,
        kFemaleFaceTextureSetEyes = 169,
        kImageSpaceModifierforinventorymenu = 170,
        kPackagetemplate = 171,
        kMainMenuCell = 172,
        kDefaultMovementTypeWalk = 173,
        kDefaultMovementTypeRun = 174,
        kDefaultMovementTypeSwim = 175,
        kDefaultMovementTypeFly = 176,
        kDefaultMovementTypeSneak = 177,
        kDefaultMovementTypeSprint = 178,
        kKeywordSpecialFurniture = 179,
        kKeywordFurnitureForces1stPerson = 180,
        kKeywordFurnitureForces3rdPerson = 181,
        kKeywordActivatorFurnitureNoPlayer = 182,
        kWerewolfRace = 339,
        kVampireRace = 340,
        kVampireSpells = 341,
        kDragonMountNoLandList = 342,
        kPlayerCanMountDragonHereList = 343,
        kFlyingMountAllowedSpells = 344,
        kFlyingMountDisallowedSpells = 345,
        kKeywordMount = 346,
        kVerletCape = 347,
        kFurnitureTestNPC = 348,
        kKeywordConditionalExplosion = 349,
        kVampireFeedNoCrimeFaction = 350,
        kSkyrimWorldspace = 351,
        kKeywordArmorMaterialLightBonemold = 352,
        kKeywordArmorMaterialLightChitin = 353,
        kKeywordArmorMaterialLightNordic = 354,
        kKeywordArmorMaterialLightStalhrim = 355,
        kFlyingMountFlyFastWorldspaces = 356,
        kKeywordArmorMaterialHeavyBonemold = 357,
        kKeywordArmorMaterialHeavyChitin = 358,
        kKeywordArmorMaterialHeavyNordic = 359,
        kKeywordArmorMaterialHeavyStalhrim = 360,
        kKeywordWeaponMaterialNordic = 361,
        kKeywordWeaponMaterialStalhrim = 362,
    }

    default_object_variant_catalog! {
        kTelekinesisGrabSound = (183, 183, 187),
        kTelekinesisThrowSound = (184, 184, 188),
        kWorldMapWeather = (185, 185, 189),
        kHelpManualPC = (186, 186, 190),
        kHelpManualXBox = (187, 187, 191),
        kKeywordTypeAmmo = (188, 188, 194),
        kKeywordTypeArmor = (189, 189, 195),
        kKeywordTypeBook = (190, 190, 196),
        kKeywordTypeIngredient = (191, 191, 197),
        kKeywordTypeKey = (192, 192, 198),
        kKeywordTypeMisc = (193, 193, 199),
        kKeywordTypeSoulGem = (194, 194, 200),
        kKeywordTypeWeapon = (195, 195, 201),
        kKeywordTypePotion = (196, 196, 202),
        kBaseWeaponEnchantment = (197, 197, 203),
        kBaseArmorEnchantment = (198, 198, 204),
        kBasePotion = (199, 199, 205),
        kBasePoison = (200, 200, 206),
        kKeywordDragon = (201, 201, 207),
        kKeywordMovable = (202, 202, 208),
        kArtObjectAbsorbEffect = (203, 203, 209),
        kWeaponMaterialList = (204, 204, 210),
        kArmorMaterialList = (205, 205, 211),
        kKeywordDisallowEnchanting = (206, 206, 212),
        kFavortravelmarkerlocation = (207, 207, 213),
        kKeywordHoldLocation = (208, 208, 214),
        kKeywordCivilWarOwner = (209, 209, 215),
        kKeywordCivilWarNeutral = (210, 210, 216),
        kLocRefTypeCivilWarSoldier = (211, 211, 217),
        kKeywordClearableLocation = (212, 212, 218),
        kLocRefTypeResourceDestructible = (213, 213, 219),
        kFormListHairColorList = (214, 214, 220),
        kComplexSceneObject = (215, 215, 221),
        kKeywordReusableSoulGem = (216, 216, 222),
        kKeywordAnimal = (217, 217, 223),
        kKeywordDaedra = (218, 218, 224),
        kKeywordRobot = (219, 219, 225),
        kKeywordNirnroot = (220, 220, 226),
        kFightersGuildFaction = (221, 221, 227),
        kMagesGuildFaction = (222, 222, 228),
        kThievesGuildFaction = (223, 223, 229),
        kDarkBrotherhoodFaction = (224, 224, 230),
        kJarlFaction = (225, 225, 231),
        kBunnyFaction = (226, 226, 232),
        kPlayerIsVampireVariable = (227, 227, 233),
        kPlayerIsWerewolfVariable = (228, 228, 234),
        kRoadMarker = (229, 229, 235),
        kKeywordScaleActorTo10 = (230, 230, 236),
        kKeywordVampire = (231, 231, 237),
        kKeywordForge = (232, 232, 238),
        kKeywordCookingPot = (233, 233, 239),
        kKeywordSmelter = (234, 234, 240),
        kKeywordTanningRack = (235, 235, 241),
        kHelpBasicLockpickingPC = (236, 236, 242),
        kHelpBasicLockpickingConsole = (237, 237, 243),
        kHelpBasicForging = (238, 238, 245),
        kHelpBasicCooking = (239, 239, 246),
        kHelpBasicSmelting = (240, 240, 247),
        kHelpBasicTanning = (241, 241, 248),
        kHelpBasicObjectCreation = (242, 242, 249),
        kHelpBasicEnchanting = (243, 243, 250),
        kHelpBasicSmithingWeapon = (244, 244, 251),
        kHelpBasicSmithingArmor = (245, 245, 252),
        kHelpBasicAlchemy = (246, 246, 253),
        kHelpBarter = (247, 247, 254),
        kHelpLevelingup = (248, 248, 255),
        kHelpSkillsMenu = (249, 249, 256),
        kHelpMapMenu = (250, 250, 257),
        kHelpJournal = (251, 251, 258),
        kHelpLowHealth = (252, 252, 259),
        kHelpLowMagicka = (253, 253, 260),
        kHelpLowStamina = (254, 254, 261),
        kHelpJail = (255, 255, 262),
        kHelpTeamateFavor = (256, 256, 263),
        kHelpWeaponCharge = (257, 257, 264),
        kHelpFavorites = (258, 258, 265),
        kKinectHelpFormList = (259, 259, 266),
        kHelpTargetLock = (261, 261, 267),
        kHelpAttackTarget = (262, 262, 269),
        kImagespaceLoadscreen = (263, 263, 280),
        kKeywordWeaponMaterialDaedric = (264, 264, 281),
        kKeywordWeaponMaterialDraugr = (265, 265, 282),
        kKeywordWeaponMaterialDraugrHoned = (266, 266, 283),
        kKeywordWeaponMaterialDwarven = (267, 267, 284),
        kKeywordWeaponMaterialEbony = (268, 268, 285),
        kKeywordWeaponMaterialElven = (269, 269, 286),
        kKeywordWeaponMaterialFalmer = (270, 270, 287),
        kKeywordWeaponMaterialFalmerHoned = (271, 271, 288),
        kKeywordWeaponMaterialGlass = (272, 272, 289),
        kKeywordWeaponMaterialImperial = (273, 273, 290),
        kKeywordWeaponMaterialIron = (274, 274, 291),
        kKeywordWeaponMaterialOrcish = (275, 275, 292),
        kKeywordWeaponMaterialSteel = (276, 276, 293),
        kKeywordWeaponMaterialWood = (277, 277, 294),
        kKeywordWeaponTypeBoundArrow = (278, 278, 295),
        kKeywordArmorMaterialDaedric = (279, 279, 296),
        kKeywordArmorMaterialDragonplate = (280, 280, 297),
        kKeywordArmorMaterialDragonscale = (281, 281, 298),
        kKeywordArmorMaterialDragonbone = (282, 282, 299),
        kKeywordArmorMaterialDwarven = (283, 283, 300),
        kKeywordArmorMaterialEbony = (284, 284, 301),
        kKeywordArmorMaterialElven = (285, 285, 302),
        kKeywordArmorMaterialElvenSplinted = (286, 286, 303),
        kKeywordArmorMaterialFullLeather = (287, 287, 304),
        kKeywordArmorMaterialGlass = (288, 288, 305),
        kKeywordArmorMaterialHide = (289, 289, 306),
        kKeywordArmorMaterialImperial = (290, 290, 307),
        kKeywordArmorMaterialImperialHeavy = (291, 291, 308),
        kKeywordArmorMaterialImperialReinforced = (292, 292, 309),
        kKeywordArmorMaterialIron = (293, 293, 310),
        kKeywordArmorMaterialIronBanded = (294, 294, 311),
        kKeywordArmorMaterialOrcish = (295, 295, 312),
        kKeywordArmorMaterialScaled = (296, 296, 313),
        kKeywordArmorMaterialSteel = (297, 297, 314),
        kKeywordArmorMaterialSteelPlate = (298, 298, 315),
        kKeywordArmorMaterialStormcloak = (299, 299, 316),
        kKeywordArmorMaterialStudded = (300, 300, 317),
        kKeywordGenericCraftableKeyword01 = (301, 301, 318),
        kKeywordGenericCraftableKeyword02 = (302, 302, 319),
        kKeywordGenericCraftableKeyword03 = (303, 303, 320),
        kKeywordGenericCraftableKeyword04 = (304, 304, 321),
        kKeywordGenericCraftableKeyword05 = (305, 305, 322),
        kKeywordGenericCraftableKeyword06 = (306, 306, 323),
        kKeywordGenericCraftableKeyword07 = (307, 307, 324),
        kKeywordGenericCraftableKeyword08 = (308, 308, 325),
        kKeywordGenericCraftableKeyword09 = (309, 309, 326),
        kKeywordGenericCraftableKeyword10 = (310, 310, 327),
        kKeywordJewelry = (311, 311, 328),
        kKeywordCuirass = (312, 312, 329),
        kLocalMapHidePlane = (313, 313, 330),
        kSnowLODMaterial = (314, 314, 331),
        kSnowLODMaterialHD = (315, 315, 332),
        kAshLODMaterial = (316, 316, 333),
        kAshLODMaterialHD = (317, 317, 334),
        kDialogueFollowerQuest = (318, 318, 335),
        kPotentialFollowerFaction = (319, 319, 336),
        kWerewolfAvailablePerks = (320, 320, 337),
        kVampireAvailablePerks = (321, 321, 338),
        kModsHelpFormList = (363, 365, 363),
    }

    default_object_non_vr_catalog! {
        kHelpFlyingMount = 260,
        kSurvivalModeToggle = 322,
        kSurvivalModeEnabled = 323,
        kSurvivalModeShowOption = 324,
        kSurvivalTemperature = 325,
        kSurvivalColdPenalty = 326,
        kSurvivalHungerPenalty = 327,
        kSurvivalSleepPenalty = 328,
        kSurvivalKeywordCold = 329,
        kSurvivalKeywordWarm = 330,
        kSurvivalKeywordArmorHands = 331,
        kSurvivalKeywordClothingHands = 332,
        kSurvivalKeywordArmorFeet = 333,
        kSurvivalKeywordClothingFeet = 334,
        kSurvivalKeywordArmorBody = 335,
        kSurvivalKeywordClothingBody = 336,
        kSurvivalKeywordArmorHead = 337,
        kSurvivalKeywordClothingHead = 338,
    }

    default_object_ae_only_catalog! {
        kHelpManualInstalledContent = 363,
        kHelpManualInstalledContentAE = 364,
    }

    default_object_vr_only_catalog! {
        kisJarlChair = 184,
        kFurnitureAnimatesFast = 185,
        isCartTravelPlayer = 186,
        kHelpManualUnknown1 = 192,
        kHelpManualUnknown2 = 193,
        kHelpSwitchTarget = 268,
        kHelp270 = 270,
        kHelp271 = 271,
        kHelp272 = 272,
        kHelpSwimming = 273,
        kHelpArchery = 274,
        kHelp275 = 275,
        kVrPlayerStaggerImod = 364,
        kVRPlayroomQuest = 366,
        kVRPlayroom = 367,
        kVRSettingsWarning = 368,
    }

    #[inline(always)]
    pub fn kTotal() -> Self {
        Self(DEFAULT_OBJECTS::total() as i32)
    }

    #[inline(always)]
    pub const fn value(self) -> i32 {
        self.0
    }
}

/// C++ `RE::DefaultObjectID`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DefaultObjectID(pub u32);

#[allow(non_upper_case_globals)]
impl DefaultObjectID {
    default_object_id_catalog! {
        kWerewolfSpell = 0,
        kSittingAngleLimit = 1,
        kAllowPlayerShout = 2,
        kGold = 3,
        kLockpick = 4,
        kSkeletonKey = 5,
        kPlayerFaction = 6,
        kGuardFaction = 7,
        kDefaultMusic = 8,
        kBattleMusic = 9,
        kDeathMusic = 10,
        kSuccessMusic = 11,
        kLevelUpMusic = 12,
        kDungeonClearedMusic = 13,
        kPlayerVoiceMale = 14,
        kPlayerVoiceMaleChild = 15,
        kPlayerVoiceFemale = 16,
        kPlayerVoiceFemaleChild = 17,
        kEatPackageDefaultFood = 18,
        kLeftHandEquip = 19,
        kRightHandEquip = 20,
        kEitherHandEquip = 21,
        kVoiceEquip = 22,
        kPotionEquip = 23,
        kEveryActorAbility = 24,
        kCommandedActorAbility = 25,
        kDrugWearsOffImageSpace = 26,
        kFootstepSet = 27,
        kLandscapeMaterial = 28,
        kDragonLandZoneMarker = 29,
        kDragonCrashZoneMarker = 30,
        kCombatStyle = 31,
        kDefaultPackList = 32,
        kWaitForDialoguePackage = 33,
        kLocRefTypeBoss = 34,
        kVirtualLocation = 35,
        kPersistAllLocation = 36,
        kInventoryPlayer = 37,
        kPathingTestNPC = 38,
        kFavorCostSmall = 39,
        kFavorCostMedium = 40,
        kFavorCostLarge = 41,
        kFavorGiftsPerDay = 42,
        kActionSwimStateChange = 43,
        kActionLook = 44,
        kActionLeftAttack = 45,
        kActionLeftReady = 46,
        kActionLeftRelease = 47,
        kActionLeftInterrupt = 48,
        kActionRightAttack = 49,
        kActionRightReady = 50,
        kActionRightRelease = 51,
        kActionRightInterrupt = 52,
        kActionDualAttack = 53,
        kActionDualRelease = 54,
        kActionActivate = 55,
        kActionJump = 56,
        kActionFall = 57,
        kActionLand = 58,
        kActionSneak = 59,
        kActionVoice = 60,
        kActionVoiceReady = 61,
        kActionVoiceRelease = 62,
        kActionVoiceInterrupt = 63,
        kActionIdle = 64,
        kActionSprintStart = 65,
        kActionSprintStop = 66,
        kActionDraw = 67,
        kActionSheath = 68,
        kActionLeftPowerAttack = 69,
        kActionRightPowerAttack = 70,
        kActionDualPowerAttack = 71,
        kActionStaggerStart = 72,
        kActionBlockHit = 73,
        kActionBlockAnticipate = 74,
        kActionRecoil = 75,
        kActionLargeRecoil = 76,
        kActionBleedoutStart = 77,
        kActionBleedoutStop = 78,
        kActionIdleStop = 79,
        kActionWardHit = 80,
        kActionForceEquip = 81,
        kActionShieldChange = 82,
        kActionPathStart = 83,
        kActionPathEnd = 84,
        kActionLargeMovementDelta = 85,
        kActionFlyStart = 86,
        kActionFlyStop = 87,
        kActionHoverStart = 88,
        kActionHoverStop = 89,
        kActionBumpedInto = 90,
        kActionSummonedStart = 91,
        kActionTalkingIdle = 92,
        kActionListenIdle = 93,
        kActionDeath = 94,
        kActionDeathWait = 95,
        kActionIdleWarn = 96,
        kActionMoveStart = 97,
        kActionMoveStop = 98,
        kActionTurnRight = 99,
        kActionTurnLeft = 100,
        kActionTurnStop = 101,
        kActionMoveForward = 102,
        kActionMoveBackward = 103,
        kActionMoveLeft = 104,
        kActionMoveRight = 105,
        kActionResetAnimationGraph = 106,
        kActionKnockdown = 107,
        kActionGetUp = 108,
        kActionIdleStopInstant = 109,
        kActionRagdollInstant = 110,
        kActionWaterwalkStart = 111,
        kActionReload = 112,
        kPickupSoundGeneric = 113,
        kPutdownSoundGeneric = 114,
        kPickupSoundWeapon = 115,
        kPutdownSoundWeapon = 116,
        kPickupSoundArmor = 117,
        kPutdownSoundArmor = 118,
        kPickupSoundBook = 119,
        kPutdownSoundBook = 120,
        kPickupSoundIngredient = 121,
        kPutdownSoundIngredient = 122,
        kHarvestSound = 123,
        kHarvestFailedSound = 124,
        kWardBreakSound = 125,
        kWardAbsorbSound = 126,
        kWardDeflectSound = 127,
        kMagicFailSound = 128,
        kShoutFailSound = 129,
        kHeartbeatSoundFast = 130,
        kHeartbeatSoundSlow = 131,
        kImagespaceLowHealth = 132,
        kSoulCapturedSound = 133,
        kNoActivationSound = 134,
        kMapMenuLoopingSound = 135,
        kDialogueVoiceCategory = 136,
        kNonDialogueVoiceCategory = 137,
        kSFXToFadeInDialogueCategory = 138,
        kPauseDuringMenuCategoryFade = 139,
        kPauseDuringMenuCategoryImmediate = 140,
        kPauseDuringLoadingMenuCategory = 141,
        kMusicSoundCategory = 142,
        kStatsMuteCategory = 143,
        kStatsMusic = 144,
        kMasterSoundCategory = 145,
        kTimeSensitiveSoundCategory = 146,
        kDialogueOutputModel3D = 147,
        kDialogueOutputModel2D = 148,
        kPlayersOutputModel1stPerson = 149,
        kPlayersOutputModel3rdPerson = 150,
        kInterfaceOutputModel = 151,
        kReverbType = 152,
        kUnderwaterLoopSound = 153,
        kUnderwaterReverbType = 154,
        kKeywordHorse = 155,
        kKeywordUndead = 156,
        kKeywordNPC = 157,
        kKeywordBeastRace = 158,
        kKeywordDummyObject = 159,
        kKeywordUseGeometryEmitter = 160,
        kKeywordMustStop = 161,
        kKeywordUpdateDuringArchery = 162,
        kKeywordSkipOutfitItems = 163,
        kMaleFaceTextureSetHead = 164,
        kMaleFaceTextureSetMouth = 165,
        kMaleFaceTextureSetEyes = 166,
        kFemaleFaceTextureSetHead = 167,
        kFemaleFaceTextureSetMouth = 168,
        kFemaleFaceTextureSetEyes = 169,
        kImageSpaceModifierforinventorymenu = 170,
        kPackagetemplate = 171,
        kMainMenuCell = 172,
        kDefaultMovementTypeWalk = 173,
        kDefaultMovementTypeRun = 174,
        kDefaultMovementTypeSwim = 175,
        kDefaultMovementTypeFly = 176,
        kDefaultMovementTypeSneak = 177,
        kDefaultMovementTypeSprint = 178,
        kKeywordSpecialFurniture = 179,
        kKeywordFurnitureForces1stPerson = 180,
        kKeywordFurnitureForces3rdPerson = 181,
        kKeywordActivatorFurnitureNoPlayer = 182,
        kTelekinesisGrabSound = 183 | (187 << 16),
        kTelekinesisThrowSound = 184 | (188 << 16),
        kWorldMapWeather = 185 | (189 << 16),
        kHelpManualPC = 186 | (190 << 16),
        kHelpManualXBox = 187 | (191 << 16),
        kKeywordTypeAmmo = 188 | (194 << 16),
        kKeywordTypeArmor = 189 | (195 << 16),
        kKeywordTypeBook = 190 | (196 << 16),
        kKeywordTypeIngredient = 191 | (197 << 16),
        kKeywordTypeKey = 192 | (198 << 16),
        kKeywordTypeMisc = 193 | (199 << 16),
        kKeywordTypeSoulGem = 194 | (200 << 16),
        kKeywordTypeWeapon = 195 | (201 << 16),
        kKeywordTypePotion = 196 | (202 << 16),
        kBaseWeaponEnchantment = 197 | (203 << 16),
        kBaseArmorEnchantment = 198 | (204 << 16),
        kBasePotion = 199 | (205 << 16),
        kBasePoison = 200 | (206 << 16),
        kKeywordDragon = 201 | (207 << 16),
        kKeywordMovable = 202 | (208 << 16),
        kArtObjectAbsorbEffect = 203 | (209 << 16),
        kWeaponMaterialList = 204 | (210 << 16),
        kArmorMaterialList = 205 | (211 << 16),
        kKeywordDisallowEnchanting = 206 | (212 << 16),
        kFavortravelmarkerlocation = 207 | (213 << 16),
        kKeywordHoldLocation = 208 | (214 << 16),
        kKeywordCivilWarOwner = 209 | (215 << 16),
        kKeywordCivilWarNeutral = 210 | (216 << 16),
        kLocRefTypeCivilWarSoldier = 211 | (217 << 16),
        kKeywordClearableLocation = 212 | (218 << 16),
        kLocRefTypeResourceDestructible = 213 | (219 << 16),
        kFormListHairColorList = 214 | (220 << 16),
        kComplexSceneObject = 215 | (221 << 16),
        kKeywordReusableSoulGem = 216 | (222 << 16),
        kKeywordAnimal = 217 | (223 << 16),
        kKeywordDaedra = 218 | (224 << 16),
        kKeywordRobot = 219 | (225 << 16),
        kKeywordNirnroot = 220 | (226 << 16),
        kFightersGuildFaction = 221 | (227 << 16),
        kMagesGuildFaction = 222 | (228 << 16),
        kThievesGuildFaction = 223 | (229 << 16),
        kDarkBrotherhoodFaction = 224 | (230 << 16),
        kJarlFaction = 225 | (231 << 16),
        kBunnyFaction = 226 | (232 << 16),
        kPlayerIsVampireVariable = 227 | (233 << 16),
        kPlayerIsWerewolfVariable = 228 | (234 << 16),
        kRoadMarker = 229 | (235 << 16),
        kKeywordScaleActorTo10 = 230 | (236 << 16),
        kKeywordVampire = 231 | (237 << 16),
        kKeywordForge = 232 | (238 << 16),
        kKeywordCookingPot = 233 | (239 << 16),
        kKeywordSmelter = 234 | (240 << 16),
        kKeywordTanningRack = 235 | (241 << 16),
        kHelpBasicLockpickingPC = 236 | (242 << 16),
        kHelpBasicLockpickingConsole = 237 | (243 << 16),
        kHelpBasicForging = 238 | (245 << 16),
        kHelpBasicCooking = 239 | (246 << 16),
        kHelpBasicSmelting = 240 | (247 << 16),
        kHelpBasicTanning = 241 | (248 << 16),
        kHelpBasicObjectCreation = 242 | (249 << 16),
        kHelpBasicEnchanting = 243 | (250 << 16),
        kHelpBasicSmithingWeapon = 244 | (251 << 16),
        kHelpBasicSmithingArmor = 245 | (252 << 16),
        kHelpBasicAlchemy = 246 | (252 << 16),
        kHelpBarter = 247 | (254 << 16),
        kHelpLevelingup = 248 | (255 << 16),
        kHelpSkillsMenu = 249 | (256 << 16),
        kHelpMapMenu = 250 | (257 << 16),
        kHelpJournal = 251 | (258 << 16),
        kHelpLowHealth = 252 | (259 << 16),
        kHelpLowMagicka = 253 | (260 << 16),
        kHelpLowStamina = 254 | (261 << 16),
        kHelpJail = 255 | (262 << 16),
        kHelpTeamateFavor = 256 | (263 << 16),
        kHelpWeaponCharge = 257 | (264 << 16),
        kHelpFavorites = 258 | (265 << 16),
        kKinectHelpFormList = 259 | (266 << 16),
        kHelpFlyingMount = 260 | (0 << 16),
        kHelpTargetLock = 261 | (267 << 16),
        kHelpAttackTarget = 262 | (269 << 16),
        kImagespaceLoadscreen = 263 | (280 << 16),
        kKeywordWeaponMaterialDaedric = 264 | (281 << 16),
        kKeywordWeaponMaterialDraugr = 265 | (282 << 16),
        kKeywordWeaponMaterialDraugrHoned = 266 | (283 << 16),
        kKeywordWeaponMaterialDwarven = 267 | (284 << 16),
        kKeywordWeaponMaterialEbony = 268 | (285 << 16),
        kKeywordWeaponMaterialElven = 269 | (286 << 16),
        kKeywordWeaponMaterialFalmer = 270 | (287 << 16),
        kKeywordWeaponMaterialFalmerHoned = 271 | (288 << 16),
        kKeywordWeaponMaterialGlass = 272 | (289 << 16),
        kKeywordWeaponMaterialImperial = 273 | (290 << 16),
        kKeywordWeaponMaterialIron = 274 | (291 << 16),
        kKeywordWeaponMaterialOrcish = 275 | (292 << 16),
        kKeywordWeaponMaterialSteel = 276 | (293 << 16),
        kKeywordWeaponMaterialWood = 277 | (294 << 16),
        kKeywordWeaponTypeBoundArrow = 278 | (295 << 16),
        kKeywordArmorMaterialDaedric = 279 | (296 << 16),
        kKeywordArmorMaterialDragonplate = 280 | (297 << 16),
        kKeywordArmorMaterialDragonscale = 281 | (298 << 16),
        kKeywordArmorMaterialDragonbone = 282 | (299 << 16),
        kKeywordArmorMaterialDwarven = 283 | (300 << 16),
        kKeywordArmorMaterialEbony = 284 | (301 << 16),
        kKeywordArmorMaterialElven = 285 | (302 << 16),
        kKeywordArmorMaterialElvenSplinted = 286 | (303 << 16),
        kKeywordArmorMaterialFullLeather = 287 | (304 << 16),
        kKeywordArmorMaterialGlass = 288 | (305 << 16),
        kKeywordArmorMaterialHide = 289 | (306 << 16),
        kKeywordArmorMaterialImperial = 290 | (307 << 16),
        kKeywordArmorMaterialImperialHeavy = 291 | (308 << 16),
        kKeywordArmorMaterialImperialReinforced = 292 | (309 << 16),
        kKeywordArmorMaterialIron = 293 | (310 << 16),
        kKeywordArmorMaterialIronBanded = 294 | (311 << 16),
        kKeywordArmorMaterialOrcish = 295 | (312 << 16),
        kKeywordArmorMaterialScaled = 296 | (313 << 16),
        kKeywordArmorMaterialSteel = 297 | (314 << 16),
        kKeywordArmorMaterialSteelPlate = 298 | (315 << 16),
        kKeywordArmorMaterialStormcloak = 299 | (316 << 16),
        kKeywordArmorMaterialStudded = 300 | (317 << 16),
        kKeywordGenericCraftableKeyword01 = 301 | (318 << 16),
        kKeywordGenericCraftableKeyword02 = 302 | (319 << 16),
        kKeywordGenericCraftableKeyword03 = 303 | (320 << 16),
        kKeywordGenericCraftableKeyword04 = 304 | (321 << 16),
        kKeywordGenericCraftableKeyword05 = 305 | (322 << 16),
        kKeywordGenericCraftableKeyword06 = 306 | (323 << 16),
        kKeywordGenericCraftableKeyword07 = 307 | (324 << 16),
        kKeywordGenericCraftableKeyword08 = 308 | (325 << 16),
        kKeywordGenericCraftableKeyword09 = 309 | (326 << 16),
        kKeywordGenericCraftableKeyword10 = 310 | (327 << 16),
        kKeywordJewelry = 311 | (328 << 16),
        kKeywordCuirass = 312 | (329 << 16),
        kLocalMapHidePlane = 313 | (330 << 16),
        kSnowLODMaterial = 314 | (331 << 16),
        kSnowLODMaterialHD = 315 | (332 << 16),
        kAshLODMaterial = 316 | (333 << 16),
        kAshLODMaterialHD = 317 | (334 << 16),
        kDialogueFollowerQuest = 318 | (335 << 16),
        kPotentialFollowerFaction = 319 | (336 << 16),
        kWerewolfAvailablePerks = 320 | (337 << 16),
        kVampireAvailablePerks = 321 | (338 << 16),
        kSurvivalModeToggle = 322,
        kSurvivalModeEnabled = 323,
        kSurvivalModeShowOption = 324,
        kSurvivalTemperature = 325,
        kSurvivalColdPenalty = 326,
        kSurvivalHungerPenalty = 327,
        kSurvivalSleepPenalty = 328,
        kSurvivalKeywordCold = 329,
        kSurvivalKeywordWarm = 330,
        kSurvivalKeywordArmorHands = 331,
        kSurvivalKeywordClothingHands = 332,
        kSurvivalKeywordArmorFeet = 333,
        kSurvivalKeywordClothingFeet = 334,
        kSurvivalKeywordArmorBody = 335,
        kSurvivalKeywordClothingBody = 336,
        kSurvivalKeywordArmorHead = 337,
        kSurvivalKeywordClothingHead = 338,
        kWerewolfRace = 339 | (339 << 16),
        kVampireRace = 340 | (340 << 16),
        kVampireSpells = 341 | (341 << 16),
        kDragonMountNoLandList = 342 | (342 << 16),
        kPlayerCanMountDragonHereList = 343 | (343 << 16),
        kFlyingMountAllowedSpells = 344 | (344 << 16),
        kFlyingMountDisallowedSpells = 345 | (345 << 16),
        kKeywordMount = 346 | (346 << 16),
        kVerletCape = 347 | (347 << 16),
        kFurnitureTestNPC = 348 | (348 << 16),
        kKeywordConditionalExplosion = 349 | (349 << 16),
        kVampireFeedNoCrimeFaction = 350 | (350 << 16),
        kSkyrimWorldspace = 351 | (351 << 16),
        kKeywordArmorMaterialLightBonemold = 352 | (352 << 16),
        kKeywordArmorMaterialLightChitin = 353 | (353 << 16),
        kKeywordArmorMaterialLightNordic = 354 | (354 << 16),
        kKeywordArmorMaterialLightStalhrim = 355 | (355 << 16),
        kFlyingMountFlyFastWorldspaces = 356 | (356 << 16),
        kKeywordArmorMaterialHeavyBonemold = 357 | (357 << 16),
        kKeywordArmorMaterialHeavyChitin = 358 | (358 << 16),
        kKeywordArmorMaterialHeavyNordic = 359 | (359 << 16),
        kKeywordArmorMaterialHeavyStalhrim = 360 | (360 << 16),
        kKeywordWeaponMaterialNordic = 361 | (361 << 16),
        kKeywordWeaponMaterialStalhrim = 362 | (362 << 16),
        kModsHelpFormList = 363 | (363 << 16),
        kisJarlChair = 0 | (184 << 16),
        kFurnitureAnimatesFast = 0 | (185 << 16),
        isCartTravelPlayer = 0 | (186 << 16),
        kHelpManualUnknown1 = 0 | (192 << 16),
        kHelpManualUnknown2 = 0 | (193 << 16),
        kHelpSwitchTarget = 0 | (268 << 16),
        kHelp270 = 0 | (270 << 16),
        kHelp271 = 0 | (271 << 16),
        kHelp272 = 0 | (272 << 16),
        kHelpSwimming = 0 | (273 << 16),
        kHelpArchery = 0 | (274 << 16),
        kHelp275 = 0 | (275 << 16),
        kVrPlayerStaggerImod = 0 | (364 << 16),
        kVRPlayroomQuest = 0 | (366 << 16),
        kVRPlayroom = 0 | (367 << 16),
        kVRSettingsWarning = 0 | (368 << 16),
    }

    #[inline(always)]
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// C++ `RE::DEFAULT_OBJECTS`
pub struct DEFAULT_OBJECTS;

impl DEFAULT_OBJECTS {
    pub const KEYWORD_ACTIVATOR_FURNITURE_NO_PLAYER: usize = 182;
    pub const ACTION_IDLE: DEFAULT_OBJECT = DEFAULT_OBJECT::kActionIdle;
    /// Object count before AE 1.7.99 (SE / AE / VR).
    pub const TOTAL: VariantOffset = VariantOffset::new(364, 364, 369);
    /// Object count on AE 1.7.99+ (7 objects were inserted).
    pub const TOTAL_AE1799: usize = 372;

    /// Object count of the running game.
    #[inline(always)]
    pub fn total() -> usize {
        if !crate::runtime::is_vr()
            && crate::runtime::is_at_least(crate::version::RUNTIME_SSE_1_7_99)
        {
            Self::TOTAL_AE1799
        } else {
            Self::TOTAL.offset()
        }
    }
}

pub const DEFAULT_OBJECT_ACTION_IDLE: DEFAULT_OBJECT = DEFAULT_OBJECTS::ACTION_IDLE;

/// C++ `RE::DEFAULT_OBJECT_TYPE`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DEFAULT_OBJECT_TYPE {
    Misc = 0,
    FaceGen = 1,
    Movement = 2,
    Actions = 3,
    Items = 4,
    Sounds = 5,
    Keywords = 6,
}

core_util::impl_enumset_type!(DEFAULT_OBJECT_TYPE => u32);

/// C++ `RE::DEFAULT_OBJECT_DATA`
#[repr(C)]
pub struct DEFAULT_OBJECT_DATA {
    pub name: *const c_char,                        // 00
    pub type_: EnumSet<FormType, u8>,               // 08
    pub pad09: u8,                                  // 09
    pub pad0a: u16,                                 // 0A
    pub unique_id: [c_char; 4],                     // 0C
    pub do_type: EnumSet<DEFAULT_OBJECT_TYPE, u32>, // 10
    pub pad14: u32,                                 // 14
}

const _: () = assert!(core::mem::size_of::<DEFAULT_OBJECT_DATA>() == 0x18);

impl DEFAULT_OBJECT_DATA {
    #[inline(always)]
    pub fn get_name_as_str(&self) -> &str {
        core_util::ptr_to_str(self.name)
    }
}

/// C++ `RE::BGSDefaultObjectManager::RecordFlags`
pub struct BGSDefaultObjectManagerRecordFlags;

/// Honest common prefix of C++ `RE::BGSDefaultObjectManager`.
///
/// The TESForm prefix is stable, but the trailing object catalog diverges
/// across runtimes because the total count and the `objectInit` offset change.
/// The overlapping `BSTSingletonImplicit<BGSDefaultObjectManager>` secondary
/// base is empty in C++ and is not modeled as fixed storage here.
#[repr(C)]
pub struct BGSDefaultObjectManager {
    pub base: TESForm, // 00
}

const _: () = assert!(core::mem::size_of::<BGSDefaultObjectManager>() == 0x20);
const _: () = assert!(core::mem::offset_of!(BGSDefaultObjectManager, base) == 0x00);

impl RttiType for BGSDefaultObjectManager {
    const RTTI: VariantID = RTTI_BGSDefaultObjectManager;
}

impl FormCastable for BGSDefaultObjectManager {
    const TARGET_FORM_TYPE: FormType = FormType::DefaultObject;
}

inherit!(BGSDefaultObjectManager : TESForm);

impl BGSDefaultObjectManager {
    pub const RTTI: VariantID = RTTI_BGSDefaultObjectManager;
    pub const VTABLE: &'static [VariantID] = &VTABLE_BGSDefaultObjectManager;
    pub const FORMTYPE: FormType = FormType::DefaultObject;

    pub const OBJECTS_OFFSET: VariantOffset = VariantOffset::new(0x20, 0x20, 0x20);
    /// `objectInit` before AE 1.7.99: SE/AE `0xB80`, VR `0xBA8`.
    /// See [`Self::object_init_offset`] for the runtime value (`0xBC0` on 1.7.99+).
    pub const OBJECT_INIT_OFFSET: VariantOffset = VariantOffset::new(0xB80, 0xB80, 0xBA8);
    pub const OBJECT_INIT_OFFSET_AE1799: usize = 0xBC0;
    // TODO: CommonLib does not state the AE 1.7.99 object size; only the pre-1.7.99
    // sizes are recorded here.
    pub const FULL_SIZE: VariantOffset = VariantOffset::new(0xCF0, 0xD08, 0xD20);
    pub const TOTAL: VariantOffset = DEFAULT_OBJECTS::TOTAL;

    /// C++ `BGSDefaultObjectManager::Ae1799Object`: objects that only exist on
    /// AE 1.7.99+ (raw indices into `objects`).
    pub const AE1799_HELP_MANUAL_NX: usize = 188;
    pub const AE1799_HELP_MOTION_ATTACK_BLOCK: usize = 264;
    pub const AE1799_HELP_MOTION_BOW_AIMING: usize = 265;
    pub const AE1799_HELP_MOTION_MAGIC_AIMING: usize = 266;
    pub const AE1799_HELP_MOTION_LOCKPICKING: usize = 267;
    pub const AE1799_HELP_AMIIBO: usize = 268;

    #[inline(always)]
    fn is_ae1799() -> bool {
        !crate::runtime::is_vr() && crate::runtime::is_at_least(crate::version::RUNTIME_SSE_1_7_99)
    }

    /// `objectInit` offset of the running game.
    #[inline(always)]
    pub fn object_init_offset() -> usize {
        if Self::is_ae1799() {
            Self::OBJECT_INIT_OFFSET_AE1799
        } else {
            Self::OBJECT_INIT_OFFSET.offset()
        }
    }

    /// C++ `GetAe1799Object`: `None` before AE 1.7.99 or when uninitialized.
    #[inline(always)]
    pub fn get_ae1799_object(&mut self, raw_index: usize) -> *mut *mut TESForm {
        if !Self::is_ae1799() || !self.is_object_initialized_index(raw_index) {
            return core::ptr::null_mut();
        }
        unsafe { self.objects_ptr().add(raw_index) }
    }

    // override (TESForm)
    // bool Load(TESFile* a_mod) override;  // 06
    // void InitItemImpl() override;        // 13

    crate::relocation_func! {
        pub fn get_singleton() -> *mut BGSDefaultObjectManager => RelocationID::new(10878, 13894)
    }

    #[inline(always)]
    pub fn get_object(&self, object: DEFAULT_OBJECT) -> *mut TESForm {
        // C++ routes DEFAULT_OBJECT through the DefaultObjectID mapping so the
        // AE 1.7.99 index shift is applied.
        let idx = Self::map_index(DefaultObjectID(object.value() as u32));
        if idx == usize::MAX {
            core::ptr::null_mut()
        } else {
            self.get_object_by_index(idx)
        }
    }

    #[inline(always)]
    pub fn get_object_as<T: FormCastable>(&self, object: DEFAULT_OBJECT) -> *mut T {
        let obj = self.get_object(object);
        if obj.is_null() || unsafe { !(*obj).is(T::TARGET_FORM_TYPE) } {
            core::ptr::null_mut()
        } else {
            obj.cast()
        }
    }

    #[inline(always)]
    pub fn get_object_ref(&self, object: DEFAULT_OBJECT) -> Option<&TESForm> {
        unsafe { self.get_object(object).as_ref() }
    }

    #[inline(always)]
    pub fn get_object_as_ref<T: FormCastable>(&self, object: DEFAULT_OBJECT) -> Option<&T> {
        unsafe { self.get_object_as::<T>(object).as_ref() }
    }

    #[inline(always)]
    pub fn get_object_by_index(&self, idx: usize) -> *mut TESForm {
        if !self.is_object_initialized_index(idx) {
            return core::ptr::null_mut();
        }

        unsafe { *self.objects_ptr().add(idx) }
    }

    #[inline(always)]
    pub fn get_object_by_index_as<T: FormCastable>(&self, idx: usize) -> *mut T {
        let obj = self.get_object_by_index(idx);
        if obj.is_null() || unsafe { !(*obj).is(T::TARGET_FORM_TYPE) } {
            core::ptr::null_mut()
        } else {
            obj.cast()
        }
    }

    #[inline(always)]
    pub fn get_object_by_index_ref(&self, idx: usize) -> Option<&TESForm> {
        unsafe { self.get_object_by_index(idx).as_ref() }
    }

    #[inline(always)]
    pub fn get_object_by_index_as_ref<T: FormCastable>(&self, idx: usize) -> Option<&T> {
        unsafe { self.get_object_by_index_as::<T>(idx).as_ref() }
    }

    #[inline(always)]
    pub fn get_object_by_id_ref(&self, object: DefaultObjectID) -> Option<&TESForm> {
        let idx = Self::map_index(object);
        (idx != usize::MAX)
            .then(|| self.get_object_by_index_ref(idx))
            .flatten()
    }

    #[inline(always)]
    pub fn get_object_by_id_as_ref<T: FormCastable>(&self, object: DefaultObjectID) -> Option<&T> {
        let idx = Self::map_index(object);
        (idx != usize::MAX)
            .then(|| self.get_object_by_index_as_ref::<T>(idx))
            .flatten()
    }

    #[inline(always)]
    pub fn get_object_ptr(&mut self, object: DefaultObjectID) -> *mut *mut TESForm {
        let idx = Self::map_index(object);
        if idx == usize::MAX || !self.is_object_initialized_index(idx) {
            core::ptr::null_mut()
        } else {
            unsafe { self.objects_ptr().add(idx) }
        }
    }

    #[inline(always)]
    pub fn get_object_ptr_as<T: FormCastable>(&mut self, object: DefaultObjectID) -> *mut *mut T {
        let obj = self.get_object_ptr(object);
        if obj.is_null() || unsafe { (*obj).is_null() || !(**obj).is(T::TARGET_FORM_TYPE) } {
            core::ptr::null_mut()
        } else {
            obj.cast()
        }
    }

    #[inline(always)]
    pub fn is_object_initialized(&self, object: DEFAULT_OBJECT) -> bool {
        self.is_object_initialized_id(DefaultObjectID(object.value() as u32))
    }

    #[inline(always)]
    pub fn is_object_initialized_id(&self, object: DefaultObjectID) -> bool {
        let idx = Self::map_index(object);
        idx != usize::MAX && self.is_object_initialized_index(idx)
    }

    #[inline(always)]
    pub fn is_object_initialized_index(&self, idx: usize) -> bool {
        if idx >= DEFAULT_OBJECTS::total() {
            return false;
        }
        unsafe { *self.object_init_ptr().add(idx) }
    }

    #[inline(always)]
    pub fn supports_vr(object: DefaultObjectID) -> bool {
        let idx = object.value() as usize;
        idx <= DEFAULT_OBJECTS::KEYWORD_ACTIVATOR_FURNITURE_NO_PLAYER || (idx & 0xFFFF_0000) != 0
    }

    #[inline(always)]
    pub fn supports_se(object: DefaultObjectID) -> bool {
        ((object.value() as usize) & 0x0000_FFFF) != 0 || object != DefaultObjectID::kWerewolfSpell
    }

    #[inline(always)]
    pub fn supports_ae(object: DefaultObjectID) -> bool {
        Self::supports_se(object)
    }

    #[inline(always)]
    pub fn supports_current_runtime(object: DefaultObjectID) -> bool {
        Self::map_index(object) != usize::MAX
    }

    #[inline(always)]
    fn map_index(idx: DefaultObjectID) -> usize {
        let idx = idx.value() as usize;
        if idx <= DEFAULT_OBJECTS::KEYWORD_ACTIVATOR_FURNITURE_NO_PLAYER {
            return idx;
        }

        let result = if crate::runtime::is_vr() {
            (idx & 0xFFFF_0000) >> 16
        } else {
            let mut result = idx & 0x0000_FFFF;
            // AE 1.7.99 inserted one object at 188 and six at 263.
            if crate::runtime::is_at_least(crate::version::RUNTIME_SSE_1_7_99) {
                if result >= 263 {
                    result += 6;
                } else if result >= 188 {
                    result += 1;
                }
            }
            result
        };

        if result == 0 { usize::MAX } else { result }
    }

    #[inline(always)]
    fn objects_ptr(&self) -> *mut *mut TESForm {
        unsafe {
            (self as *const Self as *mut u8)
                .add(Self::OBJECTS_OFFSET.offset())
                .cast::<*mut TESForm>()
        }
    }

    #[inline(always)]
    fn object_init_ptr(&self) -> *mut bool {
        unsafe {
            (self as *const Self as *mut u8)
                .add(Self::object_init_offset())
                .cast::<bool>()
        }
    }
}
