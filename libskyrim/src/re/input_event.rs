#![allow(non_camel_case_types)]

use core_util::{Enum, EnumSet};

use crate::offsets::offsets_rtti::RTTI_InputEvent;
use crate::offsets::offsets_vtable::VTABLE_InputEvent;
use crate::re::{
    AmiiboEvent, BSFixedString, ButtonEvent, CharEvent, IDEvent, INPUT_DEVICE, MotionGestureEvent,
    MouseMoveEvent, SixaxisEvent, ThumbstickEvent,
};
use crate::relocation::{RttiType, VariantID};

/// C++ `RE::INPUT_EVENT_TYPE`
#[libskyrim_macros::open_enum]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum INPUT_EVENT_TYPE {
    kButton = 0,
    kMouseMove = 1,
    kChar = 2,
    kThumbstick = 3,
    kDeviceConnect = 4,
    kKinect = 5,
    /// VR only: `VrWandTouchpadPositionEvent`. Same value as [`Self::kSixaxis`] on AE 1.7.99+.
    kVrTouchpadPosition = 6,
    /// VR only: `VrWandTouchpadSwipeEvent`. Same value as [`Self::kMotionGesture`] on AE 1.7.99+.
    kVrTouchpadSwipe = 7,
    /// AE 1.7.99+ only: `AmiiboEvent`.
    kAmiibo = 8,
}

#[allow(non_upper_case_globals)]
impl INPUT_EVENT_TYPE {
    /// AE 1.7.99+ only: `SixaxisEvent` (shares its value with the VR touchpad pair).
    pub const kSixaxis: Self = Self::kVrTouchpadPosition;
    /// AE 1.7.99+ only: `MotionGestureEvent` (shares its value with the VR touchpad pair).
    pub const kMotionGesture: Self = Self::kVrTouchpadSwipe;
}

/// C++ `RE::InputEvent`
#[repr(C)]
pub struct InputEvent {
    pub vtable: *const usize,                       // 00
    pub device: EnumSet<INPUT_DEVICE, u32>,         // 08
    pub event_type: EnumSet<INPUT_EVENT_TYPE, u32>, // 0C
    pub next: *mut InputEvent,                      // 10
}

const _: () = assert!(core::mem::size_of::<InputEvent>() == 0x18);
const _: () = assert!(core::mem::offset_of!(InputEvent, device) == 0x08);
const _: () = assert!(core::mem::offset_of!(InputEvent, event_type) == 0x0C);
const _: () = assert!(core::mem::offset_of!(InputEvent, next) == 0x10);

impl RttiType for InputEvent {
    const RTTI: VariantID = RTTI_InputEvent;
}

impl InputEvent {
    pub const RTTI: VariantID = RTTI_InputEvent;
    pub const VTABLE: &'static [VariantID] = &VTABLE_InputEvent;

    crate::virtual_method! {
        pub const VFUNC_HAS_ID_CODE: usize = 0x01;
        pub fn has_id_code() -> bool
    }

    crate::virtual_method! {
        pub const VFUNC_Q_USER_EVENT: usize = 0x02;
        pub fn q_user_event() -> *const BSFixedString
    }

    #[inline(always)]
    pub const fn event_type_storage(&self) -> Enum<INPUT_EVENT_TYPE, u32> {
        Enum::from_underlying(self.event_type.underlying())
    }

    #[inline(always)]
    pub fn try_get_event_type(&self) -> Option<INPUT_EVENT_TYPE> {
        self.event_type_storage().get()
    }

    #[inline(always)]
    pub fn get_event_type(&self) -> INPUT_EVENT_TYPE {
        self.try_get_event_type()
            .unwrap_or(INPUT_EVENT_TYPE::kButton)
    }

    #[inline(always)]
    pub const fn device_storage(&self) -> Enum<INPUT_DEVICE, i32> {
        Enum::from_underlying(self.device.underlying() as i32)
    }

    #[inline(always)]
    pub fn try_get_device(&self) -> Option<INPUT_DEVICE> {
        self.device_storage().get()
    }

    #[inline(always)]
    pub fn get_device(&self) -> INPUT_DEVICE {
        self.try_get_device().unwrap_or(INPUT_DEVICE::kNone)
    }

    #[inline(always)]
    pub fn as_button_event(&self) -> Option<&ButtonEvent> {
        (self.try_get_event_type() == Some(INPUT_EVENT_TYPE::kButton))
            .then(|| unsafe { &*(self as *const Self).cast() })
    }

    #[inline(always)]
    pub fn as_button_event_mut(&mut self) -> Option<&mut ButtonEvent> {
        (self.try_get_event_type() == Some(INPUT_EVENT_TYPE::kButton))
            .then(|| unsafe { &mut *(self as *mut Self).cast() })
    }

    #[inline(always)]
    pub fn as_char_event(&self) -> Option<&CharEvent> {
        (self.try_get_event_type() == Some(INPUT_EVENT_TYPE::kChar))
            .then(|| unsafe { &*(self as *const Self).cast() })
    }

    #[inline(always)]
    pub fn as_char_event_mut(&mut self) -> Option<&mut CharEvent> {
        (self.try_get_event_type() == Some(INPUT_EVENT_TYPE::kChar))
            .then(|| unsafe { &mut *(self as *mut Self).cast() })
    }

    #[inline(always)]
    pub fn as_id_event(&self) -> Option<&IDEvent> {
        self.has_id_code()
            .then(|| unsafe { &*(self as *const Self).cast() })
    }

    #[inline(always)]
    pub fn as_id_event_mut(&mut self) -> Option<&mut IDEvent> {
        self.has_id_code()
            .then(|| unsafe { &mut *(self as *mut Self).cast() })
    }

    #[inline(always)]
    pub fn as_mouse_move_event(&self) -> Option<&MouseMoveEvent> {
        (self.try_get_event_type() == Some(INPUT_EVENT_TYPE::kMouseMove))
            .then(|| unsafe { &*(self as *const Self).cast() })
    }

    #[inline(always)]
    pub fn as_mouse_move_event_mut(&mut self) -> Option<&mut MouseMoveEvent> {
        (self.try_get_event_type() == Some(INPUT_EVENT_TYPE::kMouseMove))
            .then(|| unsafe { &mut *(self as *mut Self).cast() })
    }

    #[inline(always)]
    fn is_ae1799_event_type(&self, ty: INPUT_EVENT_TYPE) -> bool {
        !crate::runtime::is_vr()
            && crate::runtime::is_at_least(crate::version::RUNTIME_SSE_1_7_99)
            && self.try_get_event_type() == Some(ty)
    }

    /// AE 1.7.99+ only; `None` on every other runtime.
    #[inline(always)]
    pub fn as_amiibo_event(&self) -> Option<&AmiiboEvent> {
        self.is_ae1799_event_type(INPUT_EVENT_TYPE::kAmiibo)
            .then(|| unsafe { &*(self as *const Self).cast() })
    }

    /// AE 1.7.99+ only; `None` on every other runtime.
    #[inline(always)]
    pub fn as_amiibo_event_mut(&mut self) -> Option<&mut AmiiboEvent> {
        self.is_ae1799_event_type(INPUT_EVENT_TYPE::kAmiibo)
            .then(|| unsafe { &mut *(self as *mut Self).cast() })
    }

    /// AE 1.7.99+ only; `None` on every other runtime.
    #[inline(always)]
    pub fn as_motion_gesture_event(&self) -> Option<&MotionGestureEvent> {
        self.is_ae1799_event_type(INPUT_EVENT_TYPE::kMotionGesture)
            .then(|| unsafe { &*(self as *const Self).cast() })
    }

    /// AE 1.7.99+ only; `None` on every other runtime.
    #[inline(always)]
    pub fn as_motion_gesture_event_mut(&mut self) -> Option<&mut MotionGestureEvent> {
        self.is_ae1799_event_type(INPUT_EVENT_TYPE::kMotionGesture)
            .then(|| unsafe { &mut *(self as *mut Self).cast() })
    }

    /// AE 1.7.99+ only; `None` on every other runtime.
    #[inline(always)]
    pub fn as_sixaxis_event(&self) -> Option<&SixaxisEvent> {
        self.is_ae1799_event_type(INPUT_EVENT_TYPE::kSixaxis)
            .then(|| unsafe { &*(self as *const Self).cast() })
    }

    /// AE 1.7.99+ only; `None` on every other runtime.
    #[inline(always)]
    pub fn as_sixaxis_event_mut(&mut self) -> Option<&mut SixaxisEvent> {
        self.is_ae1799_event_type(INPUT_EVENT_TYPE::kSixaxis)
            .then(|| unsafe { &mut *(self as *mut Self).cast() })
    }

    #[inline(always)]
    pub fn as_thumbstick_event(&self) -> Option<&ThumbstickEvent> {
        (self.try_get_event_type() == Some(INPUT_EVENT_TYPE::kThumbstick))
            .then(|| unsafe { &*(self as *const Self).cast() })
    }

    #[inline(always)]
    pub fn as_thumbstick_event_mut(&mut self) -> Option<&mut ThumbstickEvent> {
        (self.try_get_event_type() == Some(INPUT_EVENT_TYPE::kThumbstick))
            .then(|| unsafe { &mut *(self as *mut Self).cast() })
    }
}

pub trait InputEventExt {
    fn event_type_storage(&self) -> Enum<INPUT_EVENT_TYPE, u32>;
    fn has_id_code(&self) -> bool;
    fn q_user_event(&self) -> *const BSFixedString;
    fn try_get_event_type(&self) -> Option<INPUT_EVENT_TYPE>;
    fn get_event_type(&self) -> INPUT_EVENT_TYPE;
    fn device_storage(&self) -> Enum<INPUT_DEVICE, i32>;
    fn try_get_device(&self) -> Option<INPUT_DEVICE>;
    fn get_device(&self) -> INPUT_DEVICE;
    fn as_button_event(&self) -> Option<&ButtonEvent>;
    fn as_button_event_mut(&mut self) -> Option<&mut ButtonEvent>;
    fn as_char_event(&self) -> Option<&CharEvent>;
    fn as_char_event_mut(&mut self) -> Option<&mut CharEvent>;
    fn as_id_event(&self) -> Option<&IDEvent>;
    fn as_id_event_mut(&mut self) -> Option<&mut IDEvent>;
    fn as_mouse_move_event(&self) -> Option<&MouseMoveEvent>;
    fn as_mouse_move_event_mut(&mut self) -> Option<&mut MouseMoveEvent>;
    fn as_thumbstick_event(&self) -> Option<&ThumbstickEvent>;
    fn as_thumbstick_event_mut(&mut self) -> Option<&mut ThumbstickEvent>;
}

impl<T: AsRef<InputEvent> + AsMut<InputEvent>> InputEventExt for T {
    #[inline(always)]
    fn has_id_code(&self) -> bool {
        InputEvent::has_id_code(self.as_ref())
    }

    #[inline(always)]
    fn q_user_event(&self) -> *const BSFixedString {
        InputEvent::q_user_event(self.as_ref())
    }

    #[inline(always)]
    fn event_type_storage(&self) -> Enum<INPUT_EVENT_TYPE, u32> {
        InputEvent::event_type_storage(self.as_ref())
    }

    #[inline(always)]
    fn try_get_event_type(&self) -> Option<INPUT_EVENT_TYPE> {
        InputEvent::try_get_event_type(self.as_ref())
    }

    #[inline(always)]
    fn get_event_type(&self) -> INPUT_EVENT_TYPE {
        InputEvent::get_event_type(self.as_ref())
    }

    #[inline(always)]
    fn device_storage(&self) -> Enum<INPUT_DEVICE, i32> {
        InputEvent::device_storage(self.as_ref())
    }

    #[inline(always)]
    fn try_get_device(&self) -> Option<INPUT_DEVICE> {
        InputEvent::try_get_device(self.as_ref())
    }

    #[inline(always)]
    fn get_device(&self) -> INPUT_DEVICE {
        InputEvent::get_device(self.as_ref())
    }

    #[inline(always)]
    fn as_button_event(&self) -> Option<&ButtonEvent> {
        InputEvent::as_button_event(self.as_ref())
    }

    #[inline(always)]
    fn as_button_event_mut(&mut self) -> Option<&mut ButtonEvent> {
        InputEvent::as_button_event_mut(self.as_mut())
    }

    #[inline(always)]
    fn as_char_event(&self) -> Option<&CharEvent> {
        InputEvent::as_char_event(self.as_ref())
    }

    #[inline(always)]
    fn as_char_event_mut(&mut self) -> Option<&mut CharEvent> {
        InputEvent::as_char_event_mut(self.as_mut())
    }

    #[inline(always)]
    fn as_id_event(&self) -> Option<&IDEvent> {
        InputEvent::as_id_event(self.as_ref())
    }

    #[inline(always)]
    fn as_id_event_mut(&mut self) -> Option<&mut IDEvent> {
        InputEvent::as_id_event_mut(self.as_mut())
    }

    #[inline(always)]
    fn as_mouse_move_event(&self) -> Option<&MouseMoveEvent> {
        InputEvent::as_mouse_move_event(self.as_ref())
    }

    #[inline(always)]
    fn as_mouse_move_event_mut(&mut self) -> Option<&mut MouseMoveEvent> {
        InputEvent::as_mouse_move_event_mut(self.as_mut())
    }

    #[inline(always)]
    fn as_thumbstick_event(&self) -> Option<&ThumbstickEvent> {
        InputEvent::as_thumbstick_event(self.as_ref())
    }

    #[inline(always)]
    fn as_thumbstick_event_mut(&mut self) -> Option<&mut ThumbstickEvent> {
        InputEvent::as_thumbstick_event_mut(self.as_mut())
    }
}
