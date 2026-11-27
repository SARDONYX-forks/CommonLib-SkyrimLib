use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_ExtraUniqueID;
use crate::offsets::offsets_vtable::VTABLE_ExtraUniqueID;
use crate::re::bs_core_types::FormID;
use crate::re::{BSExtraData, ExtraDataType, ExtraDataTyped};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::ExtraUniqueID`
#[repr(C)]
pub struct ExtraUniqueID {
    pub base: BSExtraData, // 00
    pub base_id: FormID,   // 10
    pub unique_id: u16,    // 14
    pub pad16: u16,        // 16
}

const _: () = assert!(core::mem::size_of::<ExtraUniqueID>() == 0x18);
const _: () = assert!(core::mem::offset_of!(ExtraUniqueID, base_id) == 0x10);
const _: () = assert!(core::mem::offset_of!(ExtraUniqueID, unique_id) == 0x14);

impl RttiType for ExtraUniqueID {
    const RTTI: VariantID = RTTI_ExtraUniqueID;
}

impl ExtraDataTyped for ExtraUniqueID {
    const EXTRADATATYPE: ExtraDataType = ExtraDataType::UniqueID;
}

inherit!(ExtraUniqueID : BSExtraData);

impl ExtraUniqueID {
    pub const RTTI: VariantID = RTTI_ExtraUniqueID;
    pub const VTABLE: &'static [VariantID] = &VTABLE_ExtraUniqueID;
    pub const EXTRADATATYPE: ExtraDataType = ExtraDataType::UniqueID;

    // override (BSExtraData)
    // ExtraDataType GetType() const override;                    // 01 - { return kUniqueID; }
    // bool IsNotEqual(const BSExtraData* a_rhs) const override;  // 02

    /// C++ `new ExtraUniqueID(a_baseID, a_uniqueID)`: allocated on the engine
    /// heap with the engine vtable, ready to be handed to `ExtraDataList::add`.
    pub fn create(base_id: FormID, unique_id: u16) -> *mut Self {
        let this = BSExtraData::create_typed::<Self>(Self::VTABLE[0].address());
        unsafe {
            (*this).base_id = base_id;
            (*this).unique_id = unique_id;
            (*this).pad16 = 0;
        }
        this
    }
}
