use crate::offsets::offsets_rtti::RTTI_PlayerInputHandler;
use crate::offsets::offsets_vtable::VTABLE_PlayerInputHandler;
use crate::re::{ButtonEvent, InputEvent, MouseMoveEvent, PlayerControlsData, ThumbstickEvent};
use crate::relocation::{RttiType, VariantID};
use crate::version::RUNTIME_SSE_1_7_99;

/// C++ `RE::PlayerInputHandler`
///
/// The object is `0x10` bytes on every runtime. The vtable differs:
///
/// - SE / AE < 1.7.99: `02` thumbstick, `03` mouse move, `04` button
/// - AE >= 1.7.99: `02` motion gesture, `03` sixaxis, then thumbstick/mouse/button
///   shifted by [`Self::AE1799_ADDED_VFUNC_COUNT`]
/// - VR: `02` touchpad swipe, `03` touchpad position, `04..06` thumbstick/mouse/button
#[repr(C)]
pub struct PlayerInputHandler {
    pub vtable: *const usize,               // 00
    pub input_event_handling_enabled: bool, // 08
    pub pad09: u8,                          // 09
    pub pad0a: u16,                         // 0A
    pub pad0c: u32,                         // 0C
}

const _: () = assert!(core::mem::size_of::<PlayerInputHandler>() == 0x10);
const _: () =
    assert!(core::mem::offset_of!(PlayerInputHandler, input_event_handling_enabled) == 0x08);

impl RttiType for PlayerInputHandler {
    const RTTI: VariantID = RTTI_PlayerInputHandler;
}

impl PlayerInputHandler {
    pub const RTTI: VariantID = RTTI_PlayerInputHandler;
    pub const VTABLE: &'static [VariantID] = &VTABLE_PlayerInputHandler;

    /// Number of virtual functions inserted after `CanProcess` in AE 1.7.99
    /// (`ProcessMotionGesture`, `ProcessSixaxis`).
    pub const AE1799_ADDED_VFUNC_COUNT: usize = 2;

    crate::virtual_method! {
        pub const VFUNC_DTOR: usize = 0x00;
        pub fn dtor()
    }

    crate::virtual_method! {
        pub const VFUNC_CAN_PROCESS: usize = 0x01;
        pub fn can_process(event: *mut InputEvent) -> bool
    }

    /// Runtime vtable slot of `ProcessThumbstick`.
    #[inline(always)]
    pub fn vfunc_process_thumbstick() -> usize {
        crate::runtime::version_shift_vr(
            0x02,
            Self::AE1799_ADDED_VFUNC_COUNT,
            RUNTIME_SSE_1_7_99,
            0x04,
        )
    }

    /// Runtime vtable slot of `ProcessMouseMove`.
    #[inline(always)]
    pub fn vfunc_process_mouse_move() -> usize {
        crate::runtime::version_shift_vr(
            0x03,
            Self::AE1799_ADDED_VFUNC_COUNT,
            RUNTIME_SSE_1_7_99,
            0x05,
        )
    }

    /// Runtime vtable slot of `ProcessButton`.
    #[inline(always)]
    pub fn vfunc_process_button() -> usize {
        crate::runtime::version_shift_vr(
            0x04,
            Self::AE1799_ADDED_VFUNC_COUNT,
            RUNTIME_SSE_1_7_99,
            0x06,
        )
    }

    #[inline(always)]
    pub fn process_thumbstick(
        &mut self,
        event: *mut ThumbstickEvent,
        data: *mut PlayerControlsData,
    ) {
        crate::relocate_virtual!(
            extern "C" fn(*mut Self, *mut ThumbstickEvent, *mut PlayerControlsData),
            self as *mut Self,
            Self::vfunc_process_thumbstick(),
            event,
            data
        )
    }

    #[inline(always)]
    pub fn process_mouse_move(
        &mut self,
        event: *mut MouseMoveEvent,
        data: *mut PlayerControlsData,
    ) {
        crate::relocate_virtual!(
            extern "C" fn(*mut Self, *mut MouseMoveEvent, *mut PlayerControlsData),
            self as *mut Self,
            Self::vfunc_process_mouse_move(),
            event,
            data
        )
    }

    #[inline(always)]
    pub fn process_button(&mut self, event: *mut ButtonEvent, data: *mut PlayerControlsData) {
        crate::relocate_virtual!(
            extern "C" fn(*mut Self, *mut ButtonEvent, *mut PlayerControlsData),
            self as *mut Self,
            Self::vfunc_process_button(),
            event,
            data
        )
    }

    /// `ProcessMotionGesture` (slot `02`, AE 1.7.99+ only; no-op elsewhere).
    #[inline(always)]
    pub fn process_motion_gesture(
        &mut self,
        event: *mut crate::re::MotionGestureEvent,
        data: *mut PlayerControlsData,
    ) {
        if !crate::runtime::is_vr() && crate::runtime::is_at_least(RUNTIME_SSE_1_7_99) {
            crate::relocate_virtual!(
                extern "C" fn(
                    *mut Self,
                    *mut crate::re::MotionGestureEvent,
                    *mut PlayerControlsData,
                ),
                self as *mut Self,
                0x02usize,
                event,
                data
            )
        }
    }

    /// `ProcessSixaxis` (slot `03`, AE 1.7.99+ only; no-op elsewhere).
    #[inline(always)]
    pub fn process_sixaxis(
        &mut self,
        event: *mut crate::re::SixaxisEvent,
        data: *mut PlayerControlsData,
    ) {
        if !crate::runtime::is_vr() && crate::runtime::is_at_least(RUNTIME_SSE_1_7_99) {
            crate::relocate_virtual!(
                extern "C" fn(*mut Self, *mut crate::re::SixaxisEvent, *mut PlayerControlsData),
                self as *mut Self,
                0x03usize,
                event,
                data
            )
        }
    }

    /// `ProcessVrWandTouchpadSwipe` (slot `02`, VR only; no-op elsewhere).
    #[inline(always)]
    pub fn process_vr_wand_touchpad_swipe(
        &mut self,
        event: *mut core::ffi::c_void,
        data: *mut PlayerControlsData,
    ) {
        if crate::runtime::is_vr() {
            crate::relocate_virtual!(
                extern "C" fn(*mut Self, *mut core::ffi::c_void, *mut PlayerControlsData),
                self as *mut Self,
                0x02usize,
                event,
                data
            )
        }
    }

    /// `ProcessVrWandTouchpadPosition` (slot `03`, VR only; no-op elsewhere).
    #[inline(always)]
    pub fn process_vr_wand_touchpad_position(
        &mut self,
        event: *mut core::ffi::c_void,
        data: *mut PlayerControlsData,
    ) {
        if crate::runtime::is_vr() {
            crate::relocate_virtual!(
                extern "C" fn(*mut Self, *mut core::ffi::c_void, *mut PlayerControlsData),
                self as *mut Self,
                0x03usize,
                event,
                data
            )
        }
    }

    #[inline(always)]
    pub const fn is_input_event_handling_enabled(&self) -> bool {
        self.input_event_handling_enabled
    }

    #[inline(always)]
    pub fn set_input_event_handling_enabled(&mut self, enabled: bool) {
        self.input_event_handling_enabled = enabled;
    }
}

pub trait PlayerInputHandlerExt {
    fn can_process(&self, event: *mut InputEvent) -> bool;
    fn process_thumbstick(&mut self, event: *mut ThumbstickEvent, data: *mut PlayerControlsData);
    fn process_mouse_move(&mut self, event: *mut MouseMoveEvent, data: *mut PlayerControlsData);
    fn process_button(&mut self, event: *mut ButtonEvent, data: *mut PlayerControlsData);
    fn is_input_event_handling_enabled(&self) -> bool;
    fn set_input_event_handling_enabled(&mut self, enabled: bool);
}

impl<T> PlayerInputHandlerExt for T
where
    T: AsRef<PlayerInputHandler> + AsMut<PlayerInputHandler>,
{
    #[inline(always)]
    fn can_process(&self, event: *mut InputEvent) -> bool {
        PlayerInputHandler::can_process(self.as_ref(), event)
    }

    #[inline(always)]
    fn process_thumbstick(&mut self, event: *mut ThumbstickEvent, data: *mut PlayerControlsData) {
        PlayerInputHandler::process_thumbstick(self.as_mut(), event, data)
    }

    #[inline(always)]
    fn process_mouse_move(&mut self, event: *mut MouseMoveEvent, data: *mut PlayerControlsData) {
        PlayerInputHandler::process_mouse_move(self.as_mut(), event, data)
    }

    #[inline(always)]
    fn process_button(&mut self, event: *mut ButtonEvent, data: *mut PlayerControlsData) {
        PlayerInputHandler::process_button(self.as_mut(), event, data)
    }

    #[inline(always)]
    fn is_input_event_handling_enabled(&self) -> bool {
        PlayerInputHandler::is_input_event_handling_enabled(self.as_ref())
    }

    #[inline(always)]
    fn set_input_event_handling_enabled(&mut self, enabled: bool) {
        PlayerInputHandler::set_input_event_handling_enabled(self.as_mut(), enabled)
    }
}
