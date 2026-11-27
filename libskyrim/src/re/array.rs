use core_util::inherit;

use crate::re::{
    BSIntrusiveRefCounted, BSSpinLock, BSTSmartPointerIntrusiveRefCountable, TypeInfo, Variable,
};

/// C++ `RE::BSScript::Array`
#[repr(C)]
pub struct Array {
    pub base: BSIntrusiveRefCounted, // 00
    pub pad04: u32,                  // 04
    pub element_type: TypeInfo,      // 08
    pub size_: u32,                  // 10
    pub lock: BSSpinLock,            // 14
    pub pad1c: u32,                  // 1C
    pub data: [Variable; 0],         // 20
}

const _: () = assert!(core::mem::size_of::<Array>() == 0x20);
const _: () = assert!(core::mem::offset_of!(Array, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(Array, element_type) == 0x08);
const _: () = assert!(core::mem::offset_of!(Array, size_) == 0x10);
const _: () = assert!(core::mem::offset_of!(Array, lock) == 0x14);
const _: () = assert!(core::mem::offset_of!(Array, data) == 0x20);

inherit!(Array : BSIntrusiveRefCounted);

impl BSTSmartPointerIntrusiveRefCountable for Array {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.base.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.base.dec_ref()
    }

    #[inline(always)]
    unsafe fn bst_delete(&self) {
        let this = (self as *const Self).cast_mut();
        unsafe {
            (*this).dtor();
            crate::ffi::commonlib_free(this.cast());
        }
    }
}

impl Array {
    pub const MAX_SIZE: u32 = 128;

    #[inline(always)]
    pub fn dtor(&mut self) {
        for i in 0..self.size() as usize {
            unsafe { core::ptr::drop_in_place(self.data.as_mut_ptr().add(i)) };
        }
    }

    #[inline(always)]
    pub fn get(&self, pos: u32) -> &Variable {
        debug_assert!(pos < self.size());
        unsafe { &*self.data.as_ptr().add(pos as usize) }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, pos: u32) -> &mut Variable {
        debug_assert!(pos < self.size());
        unsafe { &mut *self.data.as_mut_ptr().add(pos as usize) }
    }

    #[inline(always)]
    pub fn front(&self) -> &Variable {
        self.get(0)
    }

    #[inline(always)]
    pub fn back(&self) -> &Variable {
        self.get(self.size() - 1)
    }

    #[inline(always)]
    pub fn data_ptr(&self) -> *mut Variable {
        if self.size() > 0 {
            self.data.as_ptr().cast_mut()
        } else {
            core::ptr::null_mut()
        }
    }

    #[inline(always)]
    pub fn data(&self) -> *mut Variable {
        self.data_ptr()
    }

    #[inline(always)]
    pub fn begin(&self) -> *mut Variable {
        self.data_ptr()
    }

    #[inline(always)]
    pub fn cbegin(&self) -> *const Variable {
        self.begin()
    }

    #[inline(always)]
    pub fn end(&self) -> *mut Variable {
        if self.size() > 0 {
            unsafe { self.data.as_ptr().add(self.size() as usize).cast_mut() }
        } else {
            core::ptr::null_mut()
        }
    }

    #[inline(always)]
    pub fn cend(&self) -> *const Variable {
        self.end()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.is_empty()
    }

    #[inline(always)]
    pub const fn size(&self) -> u32 {
        self.size_
    }

    #[inline(always)]
    pub const fn max_size(&self) -> u32 {
        Self::MAX_SIZE
    }

    #[inline(always)]
    pub fn type_info(&self) -> &TypeInfo {
        &self.element_type
    }

    #[inline(always)]
    pub fn type_info_mut(&mut self) -> &mut TypeInfo {
        &mut self.element_type
    }

    #[inline(always)]
    pub fn type_(&self) -> core_util::EnumSet<crate::re::type_info::RawType, usize> {
        let type_id = self.element_type.get_raw_type();
        match self.element_type.get_unmangled_raw_type() {
            crate::re::type_info::RawType::None
            | crate::re::type_info::RawType::Object
            | crate::re::type_info::RawType::String
            | crate::re::type_info::RawType::Int
            | crate::re::type_info::RawType::Float
            | crate::re::type_info::RawType::Bool => core_util::EnumSet::from_underlying(
                type_id.underlying() + crate::re::type_info::RawType::NoneArray as usize,
            ),
            _ => core_util::EnumSet::from_underlying(
                type_id.underlying() + crate::re::type_info::RawType::Object as usize,
            ),
        }
    }
}
