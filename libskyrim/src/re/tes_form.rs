use alloc::ffi::CString;
use bitflags::bitflags;
use core_util::{EnumSet, inherit};

use crate::re::base_form_component::BaseFormComponent;
use crate::re::bgs_keyword::BGSKeyword;
use crate::re::bgs_keyword_form::BGSKeywordForm;
use crate::re::bgs_list_form::BGSListForm;
use crate::re::bs_atomic::{BSReadLockGuard, BSReadWriteLock};
use crate::re::bs_container::BSContainerForEachResult;
use crate::re::bs_core_types::FormID;
use crate::re::form_type::FormType;
use crate::relocation::{
    RelocationID, RttiType, VariantID, skyrim_cast, skyrim_cast_const, skyrim_cast_mut,
    skyrim_cast_ref,
};

use crate::offsets::offsets_rtti::RTTI_TESForm;
use crate::offsets::offsets_vtable::VTABLE_TESForm;

// Dependencies
use crate::re::bgs_load_form_buffer::BGSLoadFormBuffer;
use crate::re::bgs_save_form_buffer::BGSSaveFormBuffer;
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::form::Form;
use crate::re::form::FormGroup;
use crate::re::inventory_entry_data::InventoryEntryData;
use crate::re::ni_t_pointer_map::NiTPointerMap;
use crate::re::tes_bound_object::TESBoundObject;
use crate::re::tes_data_handler::TESDataHandler;
use crate::re::tes_file::TESFile;
use crate::re::tes_full_name::TESFullName;
use crate::re::tes_model::TESModel;
use crate::re::tes_object_refr::TESObjectREFR;

bitflags! {
    /// C++ `RE::TESForm::ChangeFlags`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChangeFlags: u32 {
        const CREATED = 0;
        const FLAGS = 1 << 0;
    }
}

bitflags! {
    /// C++ `RE::TESForm::RecordFlags`
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct RecordFlag: u32 {
        const DESTRUCTIBLE = 1 << 0;
        const MASTER = 1 << 0;
        const UNLOCKED = 1 << 0;
        const ALTERED = 1 << 1;
        const NON_PLAYABLE = 1 << 2;
        const INITIALIZED = 1 << 3;
        const NON_OCCLUDER = 1 << 4;
        const DELETED = 1 << 5;
        const BORDER_REGION = 1 << 6;
        const GLOBAL_CONSTANT = 1 << 6;
        const HAS_SPOKEN_FLAG = 1 << 6;
        const KNOWN = 1 << 6;
        const IN_PLACEABLE_WATER = 1 << 6;
        const FIRE_OFF = 1 << 7;
        const MUST_UPDATE = 1 << 8;
        const ON_LOCAL_MAP = 1 << 9;
        const PERSISTENT = 1 << 10;
        const DISABLED = 1 << 11;
        const USED_AS_MOVING_PLATFORM = 1 << 11;
        const IGNORED = 1 << 12;
        const EMPTY = 1 << 13;
        const RESET_DESTRUCTION = 1 << 13;
        const TEMPORARY = 1 << 14;
        const VISIBLE_WHEN_DISTANT = 1 << 15;
        const RANDOM_ANIM = 1 << 16;
        const DANGEROUS = 1 << 17;
        const HAS_CURRENTS = 1 << 19;
        const IGNORE_FRIENDLY_HITS = 1 << 20;
        const STILL_LOADING = 1 << 21;
        const FORM_RETAINS_ID = 1 << 22;
        const DESTROYED = 1 << 23;
        const UNK24 = 1 << 24;
        const NO_AI_ACQUIRE = 1 << 25;
        const OBSTACLE = 1 << 25;
        const VATS_TARGET_OVERRIDE = 1 << 26;
        const DISABLE_FADE = 1 << 27;
        const REFLECTED_BY_AUTO_WATER = 1 << 28;
        const SHOW_ON_WORLD_MAP = 1 << 28;
        const CHILD_CAN_USE = 1 << 29;
    }
}

/// C++ `RE::TESForm::InGameFormFlag`
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InGameFormFlag {
    None = 0,
    WantsDelete = 1 << 0,
    ForcedPersistent = 1 << 1,
    NoFavorAllowed = 1 << 4,
    IsSkyObject = 1 << 5,
    RefOriginalPersistent = 1 << 6,
    RefPermanentlyDeleted = 1 << 7,
}

core_util::impl_enumset_type!(InGameFormFlag => u16);

/// C++ `RE::TESFileArray : public BSStaticArray<TESFile*>`
#[repr(C)]
pub struct TESFileArray {
    pub ptr: *mut *mut TESFile,
    pub size: u32,
    pub pad0c: u32,
}

const _: () = assert!(core::mem::size_of::<TESFileArray>() == 0x10);
const _: () = assert!(core::mem::offset_of!(TESFileArray, ptr) == 0x0);
const _: () = assert!(core::mem::offset_of!(TESFileArray, size) == 0x8);
const _: () = assert!(core::mem::offset_of!(TESFileArray, pad0c) == 0xC);

/// C++ `RE::TESFileContainer`
#[repr(C)]
pub struct TESFileContainer {
    pub array: *mut TESFileArray,
}

const _: () = assert!(core::mem::size_of::<TESFileContainer>() == 0x8);
const _: () = assert!(core::mem::offset_of!(TESFileContainer, array) == 0x0);

/// C++ `RE::TESForm`
#[repr(C)]
pub struct TESForm {
    pub base: BaseFormComponent,                          // 00
    pub source_files: TESFileContainer,                   // 08
    pub form_flags: RecordFlag,                           // 10
    pub form_id: FormID,                                  // 14
    pub in_game_form_flags: EnumSet<InGameFormFlag, u16>, // 18
    pub form_type: EnumSet<FormType, u8>,                 // 1A
    pub pad1b: u8,                                        // 1B
    pub pad1c: u32,                                       // 1C
}

const _: () = assert!(core::mem::size_of::<TESForm>() == 0x20);
const _: () = assert!(core::mem::offset_of!(TESForm, base) == 0x0);
const _: () = assert!(core::mem::offset_of!(TESForm, source_files) == 0x8);
const _: () = assert!(core::mem::offset_of!(TESForm, form_flags) == 0x10);
const _: () = assert!(core::mem::offset_of!(TESForm, form_id) == 0x14);
const _: () = assert!(core::mem::offset_of!(TESForm, in_game_form_flags) == 0x18);
const _: () = assert!(core::mem::offset_of!(TESForm, form_type) == 0x1A);
const _: () = assert!(core::mem::offset_of!(TESForm, pad1b) == 0x1B);
const _: () = assert!(core::mem::offset_of!(TESForm, pad1c) == 0x1C);

impl RttiType for TESForm {
    const RTTI: VariantID = RTTI_TESForm;
}

inherit!(TESForm : BaseFormComponent);

use crate::re::form_traits::FormCastable;
use crate::virtual_method;
use core::ffi::c_char;

impl TESForm {
    pub const RTTI: VariantID = RTTI_TESForm;
    pub const VTABLE: &'static [VariantID] = &VTABLE_TESForm;

    #[inline(always)]
    pub const fn form_type_storage(&self) -> EnumSet<FormType, u8> {
        self.form_type
    }

    #[inline(always)]
    pub fn try_get_form_type(&self) -> Option<FormType> {
        self.form_type_storage().get()
    }

    #[inline(always)]
    pub fn get_form_type(&self) -> FormType {
        self.try_get_form_type().unwrap_or(FormType::None)
    }

    #[inline(always)]
    pub fn get_file(&self, idx: i32) -> *mut TESFile {
        let array = self.source_files.array;
        if array.is_null() {
            return core::ptr::null_mut();
        }

        let size = unsafe { (*array).size };
        if size == 0 {
            return core::ptr::null_mut();
        }

        let index = if idx < 0 || idx as u32 >= size {
            size - 1
        } else {
            idx as u32
        };

        unsafe { *(*array).ptr.add(index as usize) }
    }

    pub fn get_local_form_id(&self) -> FormID {
        let file = self.get_file(0);
        if file.is_null() {
            return 0;
        }

        let file_index = unsafe {
            ((*file).get_compile_index() as FormID) << 24
                | (((*file).get_small_file_compile_index() as FormID) << 12)
        };

        self.form_id & !file_index
    }

    // RELOCATION_ID SE: 514315, AE: 400475
    crate::relocation_variable! {
        fn skyrim_vm_singleton_ptr() -> &'static *mut crate::re::SkyrimVM => RelocationID::new(514315, 400475)
    }

    // РІвЂќР‚РІвЂќР‚РІвЂќР‚ Virtual Methods РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚

    // override (BaseFormComponent)
    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor(&mut self)
    }
    virtual_method! {
        pub const VFUNC_INITIALIZE_DATA_COMPONENT: usize = 0x01;
        pub fn initialize_data_component(&mut self)
    }
    virtual_method! {
        pub const VFUNC_CLEAR_DATA_COMPONENT: usize = 0x02;
        pub fn clear_data_component(&mut self)
    }
    virtual_method! {
        pub const VFUNC_COPY_COMPONENT: usize = 0x03;
        pub fn copy_component(&mut self, rhs: *mut BaseFormComponent)
    }
    pub fn get_raw_form_id(&self) -> FormID {
        let mod_file = self.get_file(0);
        if mod_file.is_null() {
            return 0;
        }

        let data_handler = TESDataHandler::get_singleton(true);
        if data_handler.is_null() {
            return 0;
        }

        let form_id = self.form_id;
        let expected_file = if (form_id & 0xFF00_0000) == 0xFE00_0000 {
            unsafe {
                (*data_handler)
                    .lookup_loaded_light_mod_by_index(((form_id & 0x00FF_F000) >> 12) as u16)
            }
        } else {
            unsafe {
                (*data_handler).lookup_loaded_mod_by_index(((form_id & 0xFF00_0000) >> 24) as u8)
            }
        };

        let mod_file = unsafe { &*mod_file };
        let mut full_masters = 0_u32;
        let mut small_masters = 0_u32;

        if crate::runtime::is_vr() {
            for i in 0..mod_file.master_count {
                let master = if mod_file.master_ptrs.is_null() {
                    core::ptr::null_mut()
                } else {
                    unsafe { *mod_file.master_ptrs.add(i as usize) }
                };
                if core::ptr::eq(master as *const TESFile, expected_file) {
                    return (full_masters << 24) | (form_id & 0x00FF_FFFF);
                }
                full_masters += 1;
            }

            return (form_id & 0x00FF_FFFF) | (full_masters << 24);
        }

        for i in 0..mod_file.master_count {
            let master = if mod_file.master_ptrs.is_null() {
                core::ptr::null_mut()
            } else {
                unsafe { *mod_file.master_ptrs.add(i as usize) }
            };
            if core::ptr::eq(master as *const TESFile, expected_file) {
                if !master.is_null() && unsafe { (*master).compile_index } == 0xFE {
                    return 0xFE00_0000 | (small_masters << 12) | (form_id & 0x0000_0FFF);
                }
                return (full_masters << 24) | (form_id & 0x00FF_FFFF);
            }

            if !master.is_null() && unsafe { (*master).compile_index } == 0xFE {
                small_masters += 1;
            } else {
                full_masters += 1;
            }
        }

        if mod_file.compile_index == 0xFE {
            (form_id & 0x0000_0FFF) | 0xFE00_0000 | (small_masters << 12)
        } else {
            (form_id & 0x00FF_FFFF) | (full_masters << 24)
        }
    }

    #[inline(always)]
    pub fn init_item(&mut self) {
        self.init_item_impl();
    }

    virtual_method! {
        pub const VFUNC_INITIALIZE_DATA: usize = 0x04;
        pub fn initialize_data(&mut self)
    }
    virtual_method! {
        pub const VFUNC_CLEAR_DATA: usize = 0x05;
        pub fn clear_data(&mut self)
    }
    virtual_method! {
        pub const VFUNC_LOAD: usize = 0x06;
        pub fn load(&mut self, mod_file: *mut TESFile) -> bool
    }
    virtual_method! {
        pub const VFUNC_LOAD_PARTIAL: usize = 0x07;
        pub fn load_partial(&mut self, mod_file: *mut TESFile) -> bool
    }
    virtual_method! {
        pub const VFUNC_LOAD_EDIT: usize = 0x08;
        pub fn load_edit(&mut self, mod_file: *mut TESFile) -> bool
    }
    virtual_method! {
        pub const VFUNC_CREATE_DUPLICATE_FORM: usize = 0x09;
        pub fn create_duplicate_form(&mut self, create_editor_id: bool, copy_map: *mut NiTPointerMap) -> *mut TESForm
    }
    virtual_method! {
        pub const VFUNC_ADD_CHANGE: usize = 0x0A;
        pub fn add_change(&mut self, change_flags: u32) -> bool
    }
    virtual_method! {
        pub const VFUNC_REMOVE_CHANGE: usize = 0x0B;
        pub fn remove_change(&mut self, change_flags: u32)
    }
    virtual_method! {
        pub const VFUNC_FIND_IN_FILE_FAST: usize = 0x0C;
        pub fn find_in_file_fast(&mut self, mod_file: *mut TESFile) -> bool
    }
    virtual_method! {
        pub const VFUNC_CHECK_SAVE_GAME: usize = 0x0D;
        pub fn check_save_game(&mut self, buf: *mut BGSSaveFormBuffer) -> bool
    }
    virtual_method! {
        pub const VFUNC_SAVE_GAME: usize = 0x0E;
        pub fn save_game(&mut self, buf: *mut BGSSaveFormBuffer)
    }
    virtual_method! {
        pub const VFUNC_LOAD_GAME: usize = 0x0F;
        pub fn load_game(&mut self, buf: *mut BGSLoadFormBuffer)
    }
    virtual_method! {
        pub const VFUNC_INIT_LOAD_GAME: usize = 0x10;
        pub fn init_load_game(&mut self, buf: *mut BGSLoadFormBuffer)
    }
    virtual_method! {
        pub const VFUNC_FINISH_LOAD_GAME: usize = 0x11;
        pub fn finish_load_game(&mut self, buf: *mut BGSLoadFormBuffer)
    }
    virtual_method! {
        pub const VFUNC_REVERT: usize = 0x12;
        pub fn revert(&mut self, buf: *mut BGSLoadFormBuffer)
    }
    virtual_method! {
        pub const VFUNC_INIT_ITEM_IMPL: usize = 0x13;
        pub fn init_item_impl(&mut self)
    }
    virtual_method! {
        pub const VFUNC_GET_DESCRIPTION_OWNER_FILE: usize = 0x14;
        pub fn get_description_owner_file() -> *mut TESFile
    }
    virtual_method! {
        pub const VFUNC_GET_SAVED_FORM_TYPE: usize = 0x15;
        pub fn get_saved_form_type() -> FormType
    }
    virtual_method! {
        pub const VFUNC_GET_FORM_DETAILED_STRING: usize = 0x16;
        pub fn get_form_detailed_string(&mut self, buf: *mut c_char, buf_len: u32)
    }
    virtual_method! {
        pub const VFUNC_GET_KNOWN: usize = 0x17;
        pub fn get_known() -> bool
    }
    virtual_method! {
        pub const VFUNC_GET_RANDOM_ANIM: usize = 0x18;
        pub fn get_random_anim() -> bool
    }
    virtual_method! {
        pub const VFUNC_GET_PLAYABLE: usize = 0x19;
        pub fn get_playable() -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_HEADING_MARKER: usize = 0x1A;
        pub fn is_heading_marker() -> bool
    }
    virtual_method! {
        pub const VFUNC_GET_DANGEROUS: usize = 0x1B;
        pub fn get_dangerous() -> bool
    }
    virtual_method! {
        pub const VFUNC_Q_HAS_CURRENTS: usize = 0x1C;
        pub fn q_has_currents() -> bool
    }
    virtual_method! {
        pub const VFUNC_GET_OBSTACLE: usize = 0x1D;
        pub fn get_obstacle() -> bool
    }
    virtual_method! {
        pub const VFUNC_Q_IS_LOD_LAND_OBJECT: usize = 0x1E;
        pub fn q_is_lod_land_object() -> bool
    }
    virtual_method! {
        pub const VFUNC_GET_ON_LOCAL_MAP: usize = 0x1F;
        pub fn get_on_local_map() -> bool
    }
    virtual_method! {
        pub const VFUNC_GET_MUST_UPDATE: usize = 0x20;
        pub fn get_must_update() -> bool
    }
    virtual_method! {
        pub const VFUNC_SET_ON_LOCAL_MAP: usize = 0x21;
        pub fn set_on_local_map(&mut self, set: bool)
    }
    virtual_method! {
        pub const VFUNC_GET_IGNORED_BY_SANDBOX: usize = 0x22;
        pub fn get_ignored_by_sandbox() -> bool
    }
    virtual_method! {
        pub const VFUNC_SET_DELETE: usize = 0x23;
        pub fn set_delete(&mut self, set: bool)
    }
    virtual_method! {
        pub const VFUNC_SET_ALTERED: usize = 0x24;
        pub fn set_altered(&mut self, set: bool)
    }
    virtual_method! {
        pub const VFUNC_SAVE_OBJECT_BOUND: usize = 0x25;
        pub fn save_object_bound(&mut self)
    }
    virtual_method! {
        pub const VFUNC_LOAD_OBJECT_BOUND: usize = 0x26;
        pub fn load_object_bound(&mut self, mod_file: *mut TESFile)
    }
    virtual_method! {
        pub const VFUNC_IS_BOUND_OBJECT: usize = 0x27;
        pub fn is_bound_object() -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_OBJECT: usize = 0x28;
        pub fn is_object() -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_MAGIC_ITEM: usize = 0x29;
        pub fn is_magic_item() -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_WATER: usize = 0x2A;
        pub fn is_water() -> bool
    }
    virtual_method! {
        pub const VFUNC_AS_REFERENCE1: usize = 0x2B;
        pub fn as_reference1(&mut self) -> *mut TESObjectREFR
    }
    virtual_method! {
        pub const VFUNC_AS_REFERENCE2: usize = 0x2C;
        pub fn as_reference2() -> *const TESObjectREFR
    }
    virtual_method! {
        pub const VFUNC_GET_REF_COUNT: usize = 0x2D;
        pub fn get_ref_count() -> u32
    }
    virtual_method! {
        pub const VFUNC_GET_TEXT_FOR_PARSED_SUB_TAG: usize = 0x2E;
        pub fn get_text_for_parsed_sub_tag(tag: *const BSFixedString) -> *const c_char
    }
    virtual_method! {
        pub const VFUNC_COPY: usize = 0x2F;
        pub fn copy(&mut self, src_form: *mut TESForm)
    }
    virtual_method! {
        pub const VFUNC_BELONGS_IN_GROUP: usize = 0x30;
        pub fn belongs_in_group(&mut self, form: *mut Form, allow_parent_groups: bool, current_only: bool) -> bool
    }
    virtual_method! {
        pub const VFUNC_CREATE_GROUP_DATA: usize = 0x31;
        pub fn create_group_data(&mut self, form: *mut Form, group: *mut FormGroup)
    }
    virtual_method! {
        pub const VFUNC_GET_FORM_EDITOR_ID: usize = 0x32;
        pub fn get_form_editor_id() -> *const c_char
    }
    virtual_method! {
        pub const VFUNC_SET_FORM_EDITOR_ID: usize = 0x33;
        pub fn set_form_editor_id(&mut self, str: *const c_char) -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_PARENT_FORM: usize = 0x34;
        pub fn is_parent_form(&self) -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_PARENT_FORM_TREE: usize = 0x35;
        pub fn is_parent_form_tree(&self) -> bool
    }
    virtual_method! {
        pub const VFUNC_IS_FORM_TYPE_CHILD: usize = 0x36;
        pub fn is_form_type_child(&self, form_type: FormType) -> bool
    }
    virtual_method! {
        pub const VFUNC_ACTIVATE: usize = 0x37;
        pub fn activate(&mut self, target_ref: *mut TESObjectREFR, activator_ref: *mut TESObjectREFR, arg3: u8, object: *mut TESBoundObject, target_count: i32) -> bool
    }
    virtual_method! {
        pub const VFUNC_SET_FORM_ID: usize = 0x38;
        pub fn set_form_id(&mut self, id: FormID, update_file: bool)
    }
    virtual_method! {
        pub const VFUNC_GET_OBJECT_TYPE_NAME: usize = 0x39;
        pub fn get_object_type_name() -> *const c_char
    }
    virtual_method! {
        pub const VFUNC_Q_AVAILABLE_IN_GAME: usize = 0x3A;
        pub fn q_available_in_game() -> bool
    }

    // РІвЂќР‚РІвЂќР‚РІвЂќР‚ Non-Virtual Accessors and Helpers РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚

    #[inline(always)]
    pub fn get_form_flags(&self) -> RecordFlag {
        self.form_flags
    }

    #[inline(always)]
    pub fn get_form_id(&self) -> FormID {
        self.form_id
    }

    pub fn get_gold_value(&self) -> i32 {
        let object =
            unsafe { skyrim_cast::<TESForm, TESBoundObject>(self as *const _ as *mut TESForm) };
        if object.is_null() {
            -1
        } else {
            InventoryEntryData::new(object, 1).get_value()
        }
    }

    pub fn get_name(&self) -> *const c_char {
        let full_name =
            unsafe { skyrim_cast_const::<TESForm, TESFullName>(self as *const TESForm) };
        if full_name.is_null() {
            c"".as_ptr()
        } else {
            let name = unsafe { (*full_name).get_full_name() };
            if name.is_null() { c"".as_ptr() } else { name }
        }
    }

    #[inline]
    pub fn get_name_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_name())
    }

    #[inline]
    pub fn get_text_for_parsed_sub_tag_as_str(&self, tag: &BSFixedString) -> &str {
        core_util::ptr_to_str(self.get_text_for_parsed_sub_tag(tag))
    }

    #[inline(always)]
    pub fn player_knows(&self) -> bool {
        self.get_known()
    }

    #[inline(always)]
    pub fn is(&self, form_type: FormType) -> bool {
        self.get_form_type() == form_type
    }

    #[inline(always)]
    pub fn is_not(&self, form_type: FormType) -> bool {
        self.get_form_type() != form_type
    }

    #[inline(always)]
    pub fn is_actor(&self) -> bool {
        self.is(FormType::ActorCharacter)
    }

    #[inline(always)]
    pub fn is_ammo(&self) -> bool {
        self.is(FormType::Ammo)
    }

    #[inline(always)]
    pub fn is_armor(&self) -> bool {
        self.is(FormType::Armor)
    }

    #[inline(always)]
    pub fn is_book(&self) -> bool {
        self.is(FormType::Book)
    }

    #[inline(always)]
    pub fn is_deleted(&self) -> bool {
        self.form_flags.contains(RecordFlag::DELETED)
    }

    #[inline(always)]
    pub fn is_destroyed(&self) -> bool {
        self.form_flags.contains(RecordFlag::DESTROYED)
    }

    #[inline(always)]
    pub fn is_dynamic_form(&self) -> bool {
        self.form_id >= 0xFF000000
    }

    #[inline(always)]
    pub fn is_gold(&self) -> bool {
        self.form_id == 0x0000000F
    }

    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        self.form_flags.contains(RecordFlag::IGNORED)
    }

    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        self.form_flags.contains(RecordFlag::INITIALIZED)
    }

    #[inline(always)]
    pub fn is_key(&self) -> bool {
        self.is(FormType::KeyMaster)
    }

    #[inline(always)]
    pub fn is_lockpick(&self) -> bool {
        self.form_id == 0x0000000A
    }

    #[inline(always)]
    pub fn is_note(&self) -> bool {
        self.is(FormType::Note)
    }

    #[inline(always)]
    pub fn is_player(&self) -> bool {
        self.form_id == 0x00000007
    }

    #[inline(always)]
    pub fn is_player_ref(&self) -> bool {
        self.form_id == 0x00000014
    }

    #[inline(always)]
    pub fn is_skooma(&self) -> bool {
        self.form_id == 0x00057A7A || self.form_id == 0x0201391D
    }

    #[inline(always)]
    pub fn is_soul_gem(&self) -> bool {
        self.is(FormType::SoulGem)
    }

    #[inline(always)]
    pub fn is_weapon(&self) -> bool {
        self.is(FormType::Weapon)
    }

    pub fn has_keyword_in_array(&self, keywords: &[*mut BGSKeyword], match_all: bool) -> bool {
        let keyword_form =
            unsafe { skyrim_cast_const::<TESForm, BGSKeywordForm>(self as *const TESForm) };
        if keyword_form.is_null() {
            return false;
        }

        let mut has_keyword = false;
        for &keyword in keywords {
            has_keyword = !keyword.is_null() && unsafe { (*keyword_form).has_keyword(keyword) };
            if (match_all && !has_keyword) || has_keyword {
                break;
            }
        }

        has_keyword
    }

    pub fn has_keyword_by_editor_id(&self, editor_id: &str) -> bool {
        let keyword_form =
            unsafe { skyrim_cast_const::<TESForm, BGSKeywordForm>(self as *const TESForm) };
        if keyword_form.is_null() {
            return false;
        }

        unsafe { (*keyword_form).has_keyword_string(editor_id) }
    }

    pub fn has_any_keyword_by_editor_id(&self, editor_ids: &[&str]) -> bool {
        let keyword_form =
            unsafe { skyrim_cast_const::<TESForm, BGSKeywordForm>(self as *const TESForm) };
        if keyword_form.is_null() {
            return false;
        }

        unsafe { (*keyword_form).get_keywords() }
            .iter()
            .copied()
            .any(|keyword| {
                !keyword.is_null()
                    && editor_ids.iter().any(|editor_id| unsafe {
                        (*keyword).base.get_form_editor_id_as_str() == *editor_id
                    })
            })
    }

    pub fn has_keyword_in_list(&self, keyword_list: *mut BGSListForm, match_all: bool) -> bool {
        if keyword_list.is_null() {
            return false;
        }

        let keyword_form =
            unsafe { skyrim_cast_const::<TESForm, BGSKeywordForm>(self as *const TESForm) };
        if keyword_form.is_null() {
            return false;
        }

        let mut has_keyword = false;
        unsafe {
            (*keyword_list).for_each_form(|form| {
                let keyword = skyrim_cast::<TESForm, BGSKeyword>(form);
                has_keyword = !keyword.is_null() && (*keyword_form).has_keyword(keyword);
                if (match_all && !has_keyword) || has_keyword {
                    BSContainerForEachResult::Stop
                } else {
                    BSContainerForEachResult::Continue
                }
            });
        }

        has_keyword
    }

    pub fn has_vmad(&self) -> bool {
        let skyrim_vm = *Self::skyrim_vm_singleton_ptr();
        if skyrim_vm.is_null() {
            return false;
        }

        let vm = unsafe { (*skyrim_vm).get_virtual_machine() };
        if vm.is_null() {
            return false;
        }

        let policy = unsafe { (*vm).get_object_handle_policy() };
        if policy.is_null() {
            return false;
        }

        let handle =
            unsafe { (*policy).get_handle_for_form(self.get_form_type(), self as *const TESForm) };
        handle != unsafe { (*policy).empty_handle() }
    }

    #[inline(always)]
    pub fn has_world_model(&self) -> bool {
        !unsafe { skyrim_cast_const::<TESForm, TESModel>(self as *const TESForm) }.is_null()
    }

    pub fn is_inventory_object(&self) -> bool {
        matches!(
            self.get_form_type(),
            FormType::Scroll
                | FormType::Armor
                | FormType::Book
                | FormType::Ingredient
                | FormType::Light
                | FormType::Misc
                | FormType::Apparatus
                | FormType::Weapon
                | FormType::Ammo
                | FormType::KeyMaster
                | FormType::AlchemyItem
                | FormType::Note
                | FormType::ConstructibleObject
                | FormType::SoulGem
                | FormType::LeveledItem
        )
    }

    #[inline(always)]
    pub fn as_reference(&mut self) -> *mut TESObjectREFR {
        self.as_reference1()
    }

    #[inline(always)]
    pub fn as_reference_const(&self) -> *const TESObjectREFR {
        self.as_reference2()
    }

    #[inline(always)]
    pub fn can_cast<T>(&self) -> bool
    where
        T: FormCastable + RttiType,
    {
        self.try_cast::<T>().is_some()
    }

    #[inline(always)]
    pub fn cast_const<T>(&self) -> *const T
    where
        T: FormCastable + RttiType,
    {
        unsafe { skyrim_cast_const::<TESForm, T>(self as *const TESForm) }
    }

    #[inline(always)]
    pub fn try_cast<T>(&self) -> Option<&T>
    where
        T: FormCastable + RttiType,
    {
        skyrim_cast_ref::<TESForm, T>(self)
    }

    #[inline(always)]
    pub fn cast_raw<T>(&mut self) -> *mut T
    where
        T: FormCastable + RttiType,
    {
        unsafe { skyrim_cast::<TESForm, T>(self as *mut TESForm) }
    }

    #[inline(always)]
    pub fn try_cast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: FormCastable + RttiType,
    {
        skyrim_cast_mut::<TESForm, T>(self)
    }

    // РІвЂќР‚РІвЂќР‚РІвЂќР‚ Relocated Engine Functions РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚

    // RELOCATION_ID SE: 14509, AE: 14667
    crate::relocation_func! {
        pub fn add_compile_index(id: *mut FormID, file: *mut TESFile) => RelocationID::new(14509, 14667)
    }

    // RELOCATION_ID SE: 14809, AE: 14988
    crate::relocation_func! {
        pub fn get_weight(&self) -> f32 => RelocationID::new(14809, 14988)
    }

    // RELOCATION_ID SE: 14467, AE: 14623
    crate::relocation_func! {
        pub fn set_file(&mut self, file: *mut TESFile) => RelocationID::new(14467, 14623)
    }

    // RELOCATION_ID SE: 14482, AE: 14639
    crate::relocation_func! {
        pub fn set_player_knows(&mut self, known: bool) => RelocationID::new(14482, 14639)
    }

    // РІвЂќР‚РІвЂќР‚РІвЂќР‚ Global Variables (Maps) РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚

    // RELOCATION_ID SE: 514351, AE: 400507
    crate::relocation_variable! {
        pub fn get_all_forms_map() -> &'static *mut crate::re::bst_hash_map::BSTHashMap<FormID, *mut TESForm> => RelocationID::new(514351, 400507)
    }

    // RELOCATION_ID SE: 514360, AE: 400517
    // The address is the lock object itself (C++ `REL::Relocation<BSReadWriteLock*>` + `.get()`),
    // not a pointer to it.
    crate::relocation_variable! {
        pub fn get_all_forms_map_lock() -> &'static BSReadWriteLock => RelocationID::new(514360, 400517)
    }

    // RELOCATION_ID SE: 514352, AE: 400509
    crate::relocation_variable! {
        pub fn get_all_forms_by_editor_id_map() -> &'static *mut crate::re::bst_hash_map::BSTHashMap<BSFixedString, *mut TESForm> => RelocationID::new(514352, 400509)
    }

    // RELOCATION_ID SE: 514361, AE: 400518
    // The lock object itself, like `get_all_forms_map_lock`.
    crate::relocation_variable! {
        pub fn get_all_forms_editor_id_map_lock() -> &'static BSReadWriteLock => RelocationID::new(514361, 400518)
    }

    #[inline]
    pub fn get_form_editor_id_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_form_editor_id())
    }

    #[inline]
    pub fn get_object_type_name_as_str(&self) -> &str {
        core_util::ptr_to_str(self.get_object_type_name())
    }

    #[inline(always)]
    pub fn get_all_forms() -> (
        *mut crate::re::bst_hash_map::BSTHashMap<FormID, *mut TESForm>,
        *mut BSReadWriteLock,
    ) {
        (
            *Self::get_all_forms_map(),
            core::ptr::from_ref(Self::get_all_forms_map_lock()).cast_mut(),
        )
    }

    #[inline(always)]
    pub fn get_all_forms_by_editor_id() -> (
        *mut crate::re::bst_hash_map::BSTHashMap<BSFixedString, *mut TESForm>,
        *mut BSReadWriteLock,
    ) {
        (
            *Self::get_all_forms_by_editor_id_map(),
            core::ptr::from_ref(Self::get_all_forms_editor_id_map_lock()).cast_mut(),
        )
    }

    pub fn lookup_by_id(form_id: FormID) -> Option<*mut TESForm> {
        let (map, lock) = Self::get_all_forms();
        if map.is_null() {
            return None;
        }

        let _lock_guard = if lock.is_null() {
            None
        } else {
            Some(BSReadLockGuard::new(unsafe { &*lock }))
        };

        let value = unsafe { (*map).find(&form_id) };
        if value.is_null() {
            None
        } else {
            Some(unsafe { (*value).second })
        }
    }

    pub fn lookup_by_editor_id(editor_id: &str) -> Option<*mut TESForm> {
        let editor_id = CString::new(editor_id).ok()?;
        let form =
            unsafe { crate::ffi::commonlib_tes_form_lookup_by_editor_id(editor_id.as_ptr()) }
                .cast::<TESForm>();
        (!form.is_null()).then_some(form)
    }
}

pub trait TESFormExt {
    fn dtor(&mut self);
    fn initialize_data_component(&mut self);
    fn clear_data_component(&mut self);
    fn copy_component(&mut self, rhs: *mut BaseFormComponent);
    fn get_form_type(&self) -> FormType;
    fn get_form_flags(&self) -> RecordFlag;
    fn get_form_id(&self) -> FormID;
    fn get_file(&self, idx: i32) -> *mut TESFile;
    fn get_local_form_id(&self) -> FormID;
    fn get_raw_form_id(&self) -> FormID;
    fn get_gold_value(&self) -> i32;
    fn get_name(&self) -> *const c_char;
    fn get_name_as_str(&self) -> &str;
    fn get_description_owner_file(&self) -> *mut TESFile;
    fn get_playable(&self) -> bool;
    fn get_object_type_name(&self) -> *const c_char;
    fn player_knows(&self) -> bool;
    fn is(&self, form_type: FormType) -> bool;
    fn is_deleted(&self) -> bool;
    fn has_world_model(&self) -> bool;
    fn is_inventory_object(&self) -> bool;
    fn can_cast<U>(&self) -> bool
    where
        U: FormCastable + RttiType;
    fn cast_const<U>(&self) -> *const U
    where
        U: FormCastable + RttiType;
    fn try_cast<U>(&self) -> Option<&U>
    where
        U: FormCastable + RttiType;
    fn cast_raw<U>(&mut self) -> *mut U
    where
        U: FormCastable + RttiType;
    fn try_cast_mut<U>(&mut self) -> Option<&mut U>
    where
        U: FormCastable + RttiType;
    fn get_weight(&self) -> f32;
    fn has_keyword_in_array(&self, keywords: &[*mut BGSKeyword], match_all: bool) -> bool;
    fn has_keyword_by_editor_id(&self, editor_id: &str) -> bool;
    fn has_any_keyword_by_editor_id(&self, editor_ids: &[&str]) -> bool;
    fn has_keyword_in_list(&self, keyword_list: *mut BGSListForm, match_all: bool) -> bool;
    fn has_vmad(&self) -> bool;
    fn set_file(&mut self, file: *mut TESFile);
    fn set_player_knows(&mut self, known: bool);

    fn get_text_for_parsed_sub_tag_as_str(&self, tag: &BSFixedString) -> &str;
    fn get_form_editor_id_as_str(&self) -> &str;
    fn get_object_type_name_as_str(&self) -> &str;

    // Selected virtuals
    fn initialize_data(&mut self);
    fn clear_data(&mut self);
    fn load(&mut self, mod_file: *mut TESFile) -> bool;
    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer);
    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer);
    fn copy(&mut self, src_form: *mut TESForm);
    fn activate(
        &mut self,
        target_ref: *mut TESObjectREFR,
        activator_ref: *mut TESObjectREFR,
        arg3: u8,
        object: *mut TESBoundObject,
        target_count: i32,
    ) -> bool;
}

impl<T: AsRef<TESForm> + AsMut<TESForm>> TESFormExt for T {
    fn dtor(&mut self) {
        TESForm::dtor(self.as_mut())
    }

    fn initialize_data_component(&mut self) {
        TESForm::initialize_data_component(self.as_mut())
    }

    fn clear_data_component(&mut self) {
        TESForm::clear_data_component(self.as_mut())
    }

    fn copy_component(&mut self, rhs: *mut BaseFormComponent) {
        TESForm::copy_component(self.as_mut(), rhs)
    }

    fn get_form_type(&self) -> FormType {
        TESForm::get_form_type(self.as_ref())
    }

    fn get_form_flags(&self) -> RecordFlag {
        TESForm::get_form_flags(self.as_ref())
    }

    fn get_form_id(&self) -> FormID {
        TESForm::get_form_id(self.as_ref())
    }

    fn get_file(&self, idx: i32) -> *mut TESFile {
        TESForm::get_file(self.as_ref(), idx)
    }

    fn get_local_form_id(&self) -> FormID {
        TESForm::get_local_form_id(self.as_ref())
    }

    fn get_raw_form_id(&self) -> FormID {
        TESForm::get_raw_form_id(self.as_ref())
    }

    fn get_gold_value(&self) -> i32 {
        TESForm::get_gold_value(self.as_ref())
    }

    fn get_name(&self) -> *const c_char {
        TESForm::get_name(self.as_ref())
    }

    fn get_name_as_str(&self) -> &str {
        TESForm::get_name_as_str(self.as_ref())
    }

    fn get_description_owner_file(&self) -> *mut TESFile {
        TESForm::get_description_owner_file(self.as_ref())
    }

    fn get_playable(&self) -> bool {
        TESForm::get_playable(self.as_ref())
    }

    fn get_object_type_name(&self) -> *const c_char {
        TESForm::get_object_type_name(self.as_ref())
    }

    fn player_knows(&self) -> bool {
        TESForm::player_knows(self.as_ref())
    }

    fn is_deleted(&self) -> bool {
        TESForm::is_deleted(self.as_ref())
    }

    fn has_world_model(&self) -> bool {
        TESForm::has_world_model(self.as_ref())
    }

    fn is_inventory_object(&self) -> bool {
        TESForm::is_inventory_object(self.as_ref())
    }

    fn is(&self, form_type: FormType) -> bool {
        TESForm::is(self.as_ref(), form_type)
    }

    fn can_cast<U>(&self) -> bool
    where
        U: FormCastable + RttiType,
    {
        TESForm::can_cast::<U>(self.as_ref())
    }

    fn cast_const<U>(&self) -> *const U
    where
        U: FormCastable + RttiType,
    {
        TESForm::cast_const::<U>(self.as_ref())
    }

    fn try_cast<U>(&self) -> Option<&U>
    where
        U: FormCastable + RttiType,
    {
        TESForm::try_cast::<U>(self.as_ref())
    }

    fn cast_raw<U>(&mut self) -> *mut U
    where
        U: FormCastable + RttiType,
    {
        TESForm::cast_raw::<U>(self.as_mut())
    }

    fn try_cast_mut<U>(&mut self) -> Option<&mut U>
    where
        U: FormCastable + RttiType,
    {
        TESForm::try_cast_mut::<U>(self.as_mut())
    }

    fn get_weight(&self) -> f32 {
        TESForm::get_weight(self.as_ref())
    }

    fn has_keyword_in_array(&self, keywords: &[*mut BGSKeyword], match_all: bool) -> bool {
        TESForm::has_keyword_in_array(self.as_ref(), keywords, match_all)
    }

    fn has_keyword_by_editor_id(&self, editor_id: &str) -> bool {
        TESForm::has_keyword_by_editor_id(self.as_ref(), editor_id)
    }

    fn has_any_keyword_by_editor_id(&self, editor_ids: &[&str]) -> bool {
        TESForm::has_any_keyword_by_editor_id(self.as_ref(), editor_ids)
    }

    fn has_keyword_in_list(&self, keyword_list: *mut BGSListForm, match_all: bool) -> bool {
        TESForm::has_keyword_in_list(self.as_ref(), keyword_list, match_all)
    }

    fn has_vmad(&self) -> bool {
        TESForm::has_vmad(self.as_ref())
    }

    fn set_file(&mut self, file: *mut TESFile) {
        TESForm::set_file(self.as_mut(), file)
    }

    fn set_player_knows(&mut self, known: bool) {
        TESForm::set_player_knows(self.as_mut(), known)
    }

    fn get_text_for_parsed_sub_tag_as_str(&self, tag: &BSFixedString) -> &str {
        TESForm::get_text_for_parsed_sub_tag_as_str(self.as_ref(), tag)
    }

    fn get_form_editor_id_as_str(&self) -> &str {
        TESForm::get_form_editor_id_as_str(self.as_ref())
    }

    fn get_object_type_name_as_str(&self) -> &str {
        TESForm::get_object_type_name_as_str(self.as_ref())
    }

    fn initialize_data(&mut self) {
        TESForm::initialize_data(self.as_mut())
    }

    fn clear_data(&mut self) {
        TESForm::clear_data(self.as_mut())
    }

    fn load(&mut self, mod_file: *mut TESFile) -> bool {
        TESForm::load(self.as_mut(), mod_file)
    }

    fn save_game(&mut self, buf: *mut BGSSaveFormBuffer) {
        TESForm::save_game(self.as_mut(), buf)
    }

    fn load_game(&mut self, buf: *mut BGSLoadFormBuffer) {
        TESForm::load_game(self.as_mut(), buf)
    }

    fn copy(&mut self, src_form: *mut TESForm) {
        TESForm::copy(self.as_mut(), src_form)
    }

    fn activate(
        &mut self,
        target_ref: *mut TESObjectREFR,
        activator_ref: *mut TESObjectREFR,
        arg3: u8,
        object: *mut TESBoundObject,
        target_count: i32,
    ) -> bool {
        TESForm::activate(
            self.as_mut(),
            target_ref,
            activator_ref,
            arg3,
            object,
            target_count,
        )
    }
}

// РІвЂќР‚РІвЂќР‚РІвЂќР‚ Fast Form Downcasting РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚РІвЂќР‚

impl FormCastable for TESForm {
    const TARGET_FORM_TYPE: FormType = FormType::None;
}
