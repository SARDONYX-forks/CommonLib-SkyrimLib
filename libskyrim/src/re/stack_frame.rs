use crate::re::{BSTSmartPointer, IFunction, ObjectTypeInfo, Stack, Variable};

#[repr(C)]
pub struct StackFrame {
    pub parent: *mut Stack,                                  // 00
    pub previous_frame: *mut StackFrame,                     // 08
    pub owning_function: BSTSmartPointer<IFunction>,         // 10
    pub owning_object_type: BSTSmartPointer<ObjectTypeInfo>, // 18
    pub instruction_pointer: u32,                            // 20
    pub pad24: u32,                                          // 24
    pub self_: Variable,                                     // 28
    pub size: u32,                                           // 38
    pub instructions_valid: bool,                            // 3C
    pub pad3d: u8,                                           // 3D
    pub pad3e: u16,                                          // 3E
}

const _: () = assert!(core::mem::size_of::<StackFrame>() == 0x40);

impl StackFrame {
    #[inline(always)]
    /// C++ `~StackFrame()`: resets every frame variable, then the members.
    pub fn dtor(&mut self) {
        let page_hint = self.get_page_for_frame();
        for i in 0..self.size {
            *self.get_stack_frame_variable(i, page_hint) = Variable::new();
        }
        unsafe {
            core::ptr::drop_in_place(&mut self.self_);
            core::ptr::drop_in_place(&mut self.owning_object_type);
            core::ptr::drop_in_place(&mut self.owning_function);
        }
    }

    pub fn get_page_for_frame(&self) -> u32 {
        unsafe { (*self.parent).get_page_for_frame(self) }
    }

    #[inline(always)]
    pub fn get_stack_frame_variable(&self, index: u32, page_hint: u32) -> &mut Variable {
        unsafe { (*self.parent).get_stack_frame_variable(self, index, page_hint) }
    }
}
