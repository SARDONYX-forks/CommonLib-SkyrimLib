use crate::re::{
    BSFixedString, BSTSmartPointer, BSTSmartPointerIntrusiveRefCountable, IObjectHandlePolicy,
    ObjectTypeInfo, VMHandle, Variable,
};

#[repr(C)]
pub struct Object {
    pub flags_and_remaining_props: u32,         // 00
    pub pad04: u32,                             // 04
    pub type_: BSTSmartPointer<ObjectTypeInfo>, // 08
    pub current_state: BSFixedString,           // 10
    pub lock_structure: *mut core::ffi::c_void, // 18
    pub handle: VMHandle,                       // 20
    pub ref_count_and_handle_lock: i32,         // 28
    pub pad2c: u32,                             // 2C
    pub variables: [Variable; 0],               // 30
}

const _: () = assert!(core::mem::size_of::<Object>() == 0x30);

impl BSTSmartPointerIntrusiveRefCountable for Object {
    #[inline(always)]
    fn bst_inc_ref(&self) {
        self.inc_ref();
    }

    #[inline(always)]
    fn bst_dec_ref(&self) -> u32 {
        self.dec_ref()
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

impl Object {
    #[inline(always)]
    pub const fn is_constructed(&self) -> bool {
        (self.flags_and_remaining_props & 0x1) != 0
    }

    #[inline(always)]
    pub const fn is_initialized(&self) -> bool {
        (self.flags_and_remaining_props & 0x2) != 0
    }

    #[inline(always)]
    pub const fn is_valid(&self) -> bool {
        (self.flags_and_remaining_props & 0x4) != 0
    }

    crate::relocation_func! {
        pub fn get_handle(&self) -> VMHandle => crate::relocation::RelocationID::new(97463, 104247)
    }

    #[inline(always)]
    pub fn get_type_info(&self) -> *mut ObjectTypeInfo {
        self.type_.get()
    }

    #[inline(always)]
    pub fn resolve(&self, type_id: crate::re::bs_core_types::VMTypeID) -> *mut core::ffi::c_void {
        let vm = crate::re::virtual_machine::VirtualMachine::get_singleton();
        let policy = if vm.is_null() {
            core::ptr::null_mut()
        } else {
            unsafe { (*vm).get_object_handle_policy() }
        };
        let my_handle = self.get_handle();
        if policy.is_null() {
            return core::ptr::null_mut();
        }

        let policy_ref = unsafe { &*policy.cast::<IObjectHandlePolicy>() };
        if policy_ref.handle_is_type(type_id, my_handle)
            && policy_ref.is_handle_object_available(my_handle)
        {
            policy_ref.get_object_for_handle(type_id, my_handle)
        } else {
            core::ptr::null_mut()
        }
    }

    crate::relocation_func! {
        pub fn inc_ref(&self) => crate::relocation::RelocationID::new(97468, 104252)
    }

    crate::relocation_func! {
        pub fn dec_ref(&self) -> u32 => crate::relocation::RelocationID::new(97469, 104253)
    }

    #[inline(always)]
    pub fn get_property(&self, name: &BSFixedString) -> *mut Variable {
        let mut idx = u32::MAX;
        let mut cls = self.type_.get();
        while !cls.is_null() && idx == u32::MAX {
            unsafe {
                idx = (*cls).get_property_index(name);
                cls = (*cls).get_parent();
            }
        }

        if idx == u32::MAX {
            core::ptr::null_mut()
        } else {
            unsafe { self.variables.as_ptr().add(idx as usize).cast_mut() }
        }
    }

    #[inline(always)]
    pub fn get_variable(&self, name: &BSFixedString) -> *mut Variable {
        let mut idx = u32::MAX;
        let mut offset = 0u32;
        let mut cls = self.type_.get();
        while !cls.is_null() {
            unsafe {
                let vars = (*cls).get_variable_iter();
                if idx == u32::MAX && !vars.is_null() {
                    for i in 0..(*cls).get_num_variables() {
                        let var = &*vars.add(i as usize);
                        if var.name == *name {
                            idx = i;
                            break;
                        }
                    }
                } else if idx != u32::MAX {
                    offset = offset.wrapping_add((*cls).get_num_variables());
                }
                cls = (*cls).get_parent();
            }
        }

        if idx == u32::MAX {
            core::ptr::null_mut()
        } else {
            unsafe {
                self.variables
                    .as_ptr()
                    .add((offset + idx) as usize)
                    .cast_mut()
            }
        }
    }

    /// C++ `~Object()`.
    ///
    /// The previous `RELOCATION_ID(97462, 104246)` binding was wrong and was
    /// replaced upstream by this inline implementation.
    pub fn dtor(&mut self) {
        if self.is_constructed() {
            let size = unsafe { self.type_.get().as_ref() }.map_or(0, |t| t.get_variable_count());
            for i in 0..size as usize {
                unsafe { (*self.variables.as_mut_ptr().add(i)).set_none() };
            }
            // constructed = false, initialized = false
            self.flags_and_remaining_props &= !0x3;
        }

        let lock = (self.lock_structure as usize & !1usize) as *const core::sync::atomic::AtomicU32;
        if let Some(lock) = unsafe { lock.as_ref() } {
            lock.fetch_sub(1, core::sync::atomic::Ordering::AcqRel);
        }

        // Member destructors (`currentState`, `type`).
        unsafe {
            core::ptr::drop_in_place(&mut self.current_state);
            core::ptr::drop_in_place(&mut self.type_);
        }
    }
}
