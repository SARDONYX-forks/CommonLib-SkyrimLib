use core_util::inherit;

use crate::re::{
    BSIntrusiveRefCounted, BSTAutoPointer, BSTSmallArray, BSTSmartPointer,
    BSTSmartPointerIntrusiveRefCountable, CodeTasklet, IMemoryPagePolicy, IProfilePolicy,
    IStackCallbackFunctor, MemoryPage, StackFrame, VMStackID, Variable,
};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StackState {
    Running = 0,
    Finished = 1,
    WaitingOnMemory = 2,
    WaitingOnLatentFunction = 3,
    WaitingOnOtherStackForCall = 4,
    WaitingOnOtherStackForReturn = 5,
    WaitingOnOtherStackForReturnNoPop = 6,
    RetryReturnNoPop = 7,
    RetryCall = 8,
}

core_util::impl_enumset_type!(StackState => u32);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FreezeState {
    Unfrozen = 0,
    Freezing = 1,
    Frozen = 2,
}

core_util::impl_enumset_type!(FreezeState => u32);

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StackType {
    Normal = 0,
    PropertyInitialize = 1,
    Initialize = 2,
}

core_util::impl_enumset_type!(StackType => u32);

#[repr(C)]
pub struct MemoryPageData {
    pub page: BSTAutoPointer<MemoryPage>, // 00
    pub available_memory_in_bytes: u32,   // 08
    pub pad0c: u32,                       // 0C
}

const _: () = assert!(core::mem::size_of::<MemoryPageData>() == 0x10);

#[repr(C)]
pub struct Stack {
    pub base: BSIntrusiveRefCounted,                        // 00
    pub pad04: u32,                                         // 04
    pub policy: *mut IMemoryPagePolicy,                     // 08
    pub profile_policy: *mut IProfilePolicy,                // 10
    pub pages: BSTSmallArray<MemoryPageData, 0x30>,         // 18
    pub frames: u32,                                        // 58
    pub pad5c: u32,                                         // 5C
    pub top: *mut StackFrame,                               // 60
    pub state: core_util::EnumSet<StackState, u32>,         // 68
    pub freeze_state: core_util::EnumSet<FreezeState, u32>, // 6C
    pub return_value: Variable,                             // 70
    pub stack_id: VMStackID,                                // 80
    pub stack_type: core_util::EnumSet<StackType, u32>,     // 84
    pub owning_tasklet: BSTSmartPointer<CodeTasklet>,       // 88
    pub callback: BSTSmartPointer<IStackCallbackFunctor>,   // 90
    pub next_stack: BSTSmartPointer<Stack>,                 // 98
}

const _: () = assert!(core::mem::size_of::<Stack>() == 0xA0);

inherit!(Stack : BSIntrusiveRefCounted);

impl BSTSmartPointerIntrusiveRefCountable for Stack {
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
        // C++ `~Stack()` followed by `operator delete`.
        unsafe {
            (*this).dtor();
            core::ptr::drop_in_place(core::ptr::addr_of_mut!((*this).next_stack));
            core::ptr::drop_in_place(core::ptr::addr_of_mut!((*this).callback));
            core::ptr::drop_in_place(core::ptr::addr_of_mut!((*this).owning_tasklet));
            core::ptr::drop_in_place(core::ptr::addr_of_mut!((*this).return_value));
            core::ptr::drop_in_place(core::ptr::addr_of_mut!((*this).pages));
            crate::ffi::commonlib_free(this.cast());
        }
    }
}

impl Stack {
    #[inline(always)]
    pub fn get_page_for_frame(&self, frame: *const StackFrame) -> u32 {
        let pages = unsafe { self.pages.as_slice() };
        for (i, pair) in pages.iter().enumerate() {
            if !pair.page.is_null() && unsafe { (*pair.page.get()).is_in_range(frame.cast()) } {
                return i as u32;
            }
        }
        u32::MAX
    }

    #[inline(always)]
    pub fn get_stack_frame_variable(
        &self,
        frame: *const StackFrame,
        index: u32,
        page_hint: u32,
    ) -> &mut Variable {
        let func = unsafe {
            crate::relocation::Relocation::<
                extern "C" fn(*const Self, *const StackFrame, u32, u32) -> *mut Variable,
            >::new(crate::relocation::RelocationID::new(97746, 104484))
            .get()
        };
        unsafe { &mut *func(self, frame, index, page_hint) }
    }

    /// Body of C++ `~Stack()`: pops every frame. Member destructors are run by
    /// the owner (see `bst_delete`). The former `RELOCATION_ID(97742, 104480)`
    /// binding was wrong and was replaced upstream by this implementation.
    pub fn dtor(&mut self) {
        while self.pop_frame() {}
    }

    /// C++ `Stack::PopFrame()`.
    // TODO: `PushFrame` / `PushBlankFrame` (CommonLib #550) are not ported yet;
    // they need `IMemoryPagePolicy::AllocatePage/GetLargestAvailablePage` and
    // `StackFrame::InitFrame` bindings.
    pub fn pop_frame(&mut self) -> bool {
        if self.frames == 0 || self.pages.is_empty() || self.top.is_null() {
            return false;
        }

        let frame = self.top;
        unsafe {
            if let Some(profile) = self.profile_policy.as_mut() {
                // Hold a reference to the function across the call; a frame
                // without a function is not reported.
                let function = (*frame).owning_function.clone();
                if let Some(f) = function.get().as_ref() {
                    // IProfilePolicy::StackFramePopped (04)
                    let vtable =
                        *(profile as *mut crate::re::IProfilePolicy as *const *const usize);
                    let popped: extern "C" fn(
                        *mut crate::re::IProfilePolicy,
                        VMStackID,
                        u32,
                        *const Variable,
                        *const crate::re::BSFixedString,
                        *const crate::re::BSFixedString,
                        *const crate::re::BSFixedString,
                    ) = core::mem::transmute(*vtable.add(4));
                    popped(
                        profile,
                        self.stack_id,
                        self.frames - 1,
                        &(*frame).self_,
                        f.get_object_type_name(),
                        f.get_state_name(),
                        f.get_name(),
                    );
                }
            }

            self.top = (*frame).previous_frame;
            let mut kept_bytes: u32 = ((*frame).size + 4) << 4; // read before the teardown
            (*frame).dtor();

            let mut page_index = self.get_page_for_frame(frame);
            if page_index == u32::MAX {
                return false;
            }
            if frame.cast::<core::ffi::c_void>()
                != (*self.pages[page_index as usize].page.get()).get_head()
            {
                page_index += 1;
            }

            // Every page from `page_index` on is given back to the policy.
            while self.pages.size() > page_index {
                let entry = self.pages.back_mut();
                let page_size = entry.page.get().as_ref().map_or(0, |p| p.page_size);
                // before DisposePage zeroes pageSize
                kept_bytes = kept_bytes
                    .wrapping_add(entry.available_memory_in_bytes)
                    .wrapping_sub(page_size);
                // IMemoryPagePolicy::DisposePage (04)
                let policy = self.policy;
                if !policy.is_null() {
                    let vtable = *(policy as *const *const usize);
                    let dispose: extern "C" fn(
                        *mut IMemoryPagePolicy,
                        *mut BSTAutoPointer<MemoryPage>,
                    ) = core::mem::transmute(*vtable.add(4));
                    dispose(policy, &mut entry.page);
                }
                drop(self.pages.pop_back()); // frees the page if the policy did not take it
            }

            if kept_bytes != 0 && page_index != 0 {
                core::ptr::write_bytes(frame.cast::<u8>(), 0, kept_bytes as usize);
                let entry = &mut self.pages[(page_index - 1) as usize];
                entry.available_memory_in_bytes =
                    entry.available_memory_in_bytes.wrapping_add(kept_bytes);
            }
        }

        self.frames -= 1;
        true
    }
}
