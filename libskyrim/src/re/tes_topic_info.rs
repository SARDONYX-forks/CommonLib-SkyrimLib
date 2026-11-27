use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::re::bgs_sound_descriptor_form::BGSSoundDescriptorForm;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::dialogue_item::DialogueItem;
use crate::re::form_traits::FormCastable;
use crate::re::form_type::FormType;
use crate::re::tes_condition::TESCondition;
use crate::re::tes_file::TESFile;
use crate::re::tes_form::TESForm;
use crate::re::tes_idle_form::TESIdleForm;
use crate::re::tes_object_refr::TESObjectREFR;
use crate::re::tes_topic::TESTopic;
use crate::relocation::{RelocationID, RttiType, VariantID};

use crate::offsets::offsets_rtti::RTTI_TESTopicInfo;
use crate::offsets::offsets_vtable::VTABLE_TESTopicInfo;

/// C++ `RE::TOPIC_INFO_DATA::TOPIC_INFO_FLAGS`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESTopicInfoFlag {
    None = 0,
    StartSceneOnEnd = 1 << 0,
    Random = 1 << 1,
    SayOnce = 1 << 2,
    RequiresPlayerActivation = 1 << 3,
    InfoRefusal = 1 << 4,
    RandomEnd = 1 << 5,
    EndRunningScene = 1 << 6,
    IsForceGreet = 1 << 7,
    PlayerAddress = 1 << 8,
    ForceSubtitle = 1 << 9,
    CanMoveWhileGreeting = 1 << 10,
    NoLIPFile = 1 << 11,
    PostProcess = 1 << 12,
    CustomSoundOutput = 1 << 13,
    SpendsFavorPoints = 1 << 14,
}

core_util::impl_enumset_type!(TESTopicInfoFlag => u16);

/// C++ `RE::TOPIC_INFO_DATA`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TOPIC_INFO_DATA {
    pub flags: EnumSet<TESTopicInfoFlag, u16>, // 00
    pub time_until_reset: u16,                 // 02
}

const _: () = assert!(core::mem::size_of::<TOPIC_INFO_DATA>() == 0x4);
const _: () = assert!(core::mem::offset_of!(TOPIC_INFO_DATA, flags) == 0x00);
const _: () = assert!(core::mem::offset_of!(TOPIC_INFO_DATA, time_until_reset) == 0x02);

impl TOPIC_INFO_DATA {
    #[inline]
    pub fn get_reset_hours(&self) -> f32 {
        self.time_until_reset as f32
    }
}

/// C++ `RE::TESTopicInfo::FavorLevel`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESTopicInfoFavorLevel {
    None = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
}

core_util::impl_enumset_type!(TESTopicInfoFavorLevel => u8);

bitflags! {
    /// C++ `RE::TESTopicInfo::ChangeFlags::ChangeFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESTopicInfoChangeFlags: u32 {
        const SAID_ONCE = 1 << 31;
    }
}

bitflags! {
    /// C++ `RE::TESTopicInfo::RecordFlags::RecordFlag`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TESTopicInfoRecordFlags: u32 {
        const DELETED = 1 << 5;
        const IGNORED = 1 << 12;
    }
}

/// C++ `RE::TESTopicInfo::TESResponse::EmotionType`
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESTopicInfoResponseEmotionType {
    Neutral = 0,
    Anger = 1,
    Disgust = 2,
    Fear = 3,
    Sad = 4,
    Happy = 5,
    Surprise = 6,
    Puzzled = 7,
}

/// C++ `RE::TESTopicInfo::TESResponse::Flag`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TESTopicInfoResponseFlag {
    None = 0,
    UseEmotionAnimation = 1 << 0,
}

core_util::impl_enumset_type!(TESTopicInfoResponseEmotionType => u32);
core_util::impl_enumset_type!(TESTopicInfoResponseFlag => u8);

/// C++ `RE::TESTopicInfo::TESResponse`
#[repr(C)]
pub struct TESResponse {
    pub emotion_type: EnumSet<TESTopicInfoResponseEmotionType, u32>, // 00
    pub emotion_value: u32,                                          // 04
    pub unk08: *mut TESTopic,                                        // 08
    pub response_number: u8,                                         // 10
    pub pad11: u8,                                                   // 11
    pub pad12: u16,                                                  // 12
    pub pad14: u32,                                                  // 14
    pub sound: *mut BGSSoundDescriptorForm,                          // 18
    pub flags: EnumSet<TESTopicInfoResponseFlag, u8>,                // 20
    pub pad21: u8,                                                   // 21
    pub pad22: u16,                                                  // 22
    pub pad24: u32,                                                  // 24
    pub response_text: BSFixedString,                                // 28 - NAM1
    pub speaker_idle: *mut TESIdleForm,                              // 30
    pub listener_idle: *mut TESIdleForm,                             // 38
    pub next: *mut TESResponse,                                      // 40
}

const _: () = assert!(core::mem::size_of::<TESResponse>() == 0x48);
const _: () = assert!(core::mem::offset_of!(TESResponse, emotion_type) == 0x00);
const _: () = assert!(core::mem::offset_of!(TESResponse, emotion_value) == 0x04);
const _: () = assert!(core::mem::offset_of!(TESResponse, unk08) == 0x08);
const _: () = assert!(core::mem::offset_of!(TESResponse, response_number) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESResponse, sound) == 0x18);
const _: () = assert!(core::mem::offset_of!(TESResponse, flags) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESResponse, response_text) == 0x28);
const _: () = assert!(core::mem::offset_of!(TESResponse, speaker_idle) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESResponse, listener_idle) == 0x38);
const _: () = assert!(core::mem::offset_of!(TESResponse, next) == 0x40);

impl TESResponse {
    crate::relocation_func! {
        pub fn load_response_text(&mut self, file: *mut TESFile) => RelocationID::new(24985, 25491)
    }
}

/// C++ `RE::TESTopicInfo::TESResponseList`
#[repr(C)]
pub struct TESResponseList {
    pub head: *mut TESResponse, // 00
}

const _: () = assert!(core::mem::size_of::<TESResponseList>() == 0x8);
const _: () = assert!(core::mem::offset_of!(TESResponseList, head) == 0x00);

/// C++ `RE::TESTopicInfo`
#[repr(C)]
pub struct TESTopicInfo {
    pub base: TESForm,                                    // 00
    pub parent_topic: *mut TESTopic,                      // 20
    pub data_info: *mut TESTopicInfo,                     // 28 - DNAM
    pub obj_conditions: TESCondition,                     // 30 - CTDA
    pub info_index: u16,                                  // 38
    pub said_once: bool,                                  // 3A
    pub favor_level: EnumSet<TESTopicInfoFavorLevel, u8>, // 3B - CNAM
    pub data: TOPIC_INFO_DATA,                            // 3C - ENAM
    pub file_offset: u32,                                 // 40
    pub pad44: u32,                                       // 44
}

const _: () = assert!(core::mem::size_of::<TESTopicInfo>() == 0x48);
const _: () = assert!(core::mem::offset_of!(TESTopicInfo, parent_topic) == 0x20);
const _: () = assert!(core::mem::offset_of!(TESTopicInfo, data_info) == 0x28);
const _: () = assert!(core::mem::offset_of!(TESTopicInfo, obj_conditions) == 0x30);
const _: () = assert!(core::mem::offset_of!(TESTopicInfo, info_index) == 0x38);
const _: () = assert!(core::mem::offset_of!(TESTopicInfo, said_once) == 0x3A);
const _: () = assert!(core::mem::offset_of!(TESTopicInfo, favor_level) == 0x3B);
const _: () = assert!(core::mem::offset_of!(TESTopicInfo, data) == 0x3C);
const _: () = assert!(core::mem::offset_of!(TESTopicInfo, file_offset) == 0x40);

impl RttiType for TESTopicInfo {
    const RTTI: VariantID = RTTI_TESTopicInfo;
}

impl FormCastable for TESTopicInfo {
    const TARGET_FORM_TYPE: FormType = FormType::Info;
}

inherit!(TESTopicInfo : TESForm);

/// C++ `RE::TESTopicInfo::AE1799_RUNTIME_DATA` (at `0x48`).
#[repr(C)]
pub struct TESTopicInfoAe1799RuntimeData {
    pub unk48: [u8; 0x8], // 48
}

impl TESTopicInfo {
    pub const RTTI: VariantID = RTTI_TESTopicInfo;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESTopicInfo;

    // AE 1.7.99+ only: `None` on SE, AE < 1.7.99 and VR.
    crate::runtime_optional_data_accessor! {
        pub fn ae1799_runtime_data() -> TESTopicInfoAe1799RuntimeData {
            version: crate::version::RUNTIME_SSE_1_7_99, se: 0, ae: 0x48, vr: 0
        }
    }

    crate::runtime_optional_data_mut_accessor! {
        pub fn ae1799_runtime_data_mut() -> TESTopicInfoAe1799RuntimeData {
            version: crate::version::RUNTIME_SSE_1_7_99, se: 0, ae: 0x48, vr: 0
        }
    }
    pub const FORMTYPE: FormType = FormType::Info;

    // override (TESForm)
    // void InitializeData() override;                                                            // 04
    // void ClearData() override;                                                                 // 05
    // bool Load(TESFile* a_mod) override;                                                        // 06
    // void LoadGame(BGSLoadFormBuffer* a_buf) override;                                          // 0F
    // void Revert(BGSLoadFormBuffer* a_buf) override;                                            // 12
    // void InitItemImpl() override;                                                              // 13
    // void GetFormDetailedString(char* a_buf, std::uint32_t a_bufLen) override;                  // 16
    // void SetAltered(bool a_set) override;                                                      // 24
    // bool BelongsInGroup(FORM* a_form, bool a_allowParentGroups, bool a_currentOnly) override;  // 30
    // void CreateGroupData(FORM* a_form, FORM_GROUP* a_group) override;                          // 31

    #[inline]
    pub fn get_dialogue_data(&self, speaker: *mut TESObjectREFR) -> DialogueItem {
        let parent_topic = unsafe { &mut *self.parent_topic };
        DialogueItem::new(
            parent_topic.owner_quest,
            self.parent_topic,
            self as *const _ as *mut _,
            speaker,
        )
    }

    crate::relocation_func! {
        pub fn get_response_list(&mut self, list: *mut TESResponseList) -> *mut TESResponseList => RelocationID::new(25083, 25626)
    }

    #[inline]
    pub fn get_response_list_default(&mut self) -> *mut TESResponseList {
        self.get_response_list(core::ptr::null_mut())
    }
}
