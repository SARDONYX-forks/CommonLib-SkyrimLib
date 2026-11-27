#![allow(non_snake_case)]

use crate::re::BSTEventSource;
use crate::relocation::RelocationID;

pub mod SpellsLearned {
    use super::*;

    /// C++ `RE::SpellsLearned::Event`. Carries no data (`sizeof == 1` in C++).
    #[repr(C)]
    pub struct Event {
        pub _empty: u8,
    }

    const _: () = assert!(core::mem::size_of::<Event>() == 0x01);

    crate::relocation_func! {
        pub fn get_event_source() -> *mut BSTEventSource<Event> => RelocationID::new(37917, 38874)
    }

    #[inline(always)]
    pub unsafe fn send_event() {
        let event = Event { _empty: 0 };
        let source = get_event_source();
        if !source.is_null() {
            unsafe { (*source).send_event(core::ptr::addr_of!(event)) };
        }
    }
}
