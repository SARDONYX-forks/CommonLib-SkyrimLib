#![allow(non_camel_case_types)]

use crate::core_util::{EnumSet, inherit};
use crate::offsets::offsets_rtti::RTTI_hkpMotion;
use crate::offsets::offsets_vtable::VTABLE_hkpMotion;
use crate::re::{
    hkHalf, hkMatrix3, hkMotionState, hkQuaternion, hkReferencedObject, hkTransform, hkVector4,
    hkpMaxSizeMotion,
};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::hkpMotion::MotionType`
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum hkpMotionMotionType {
    Invalid = 0,
    Dynamic = 1,
    SphereInertia = 2,
    BoxInertia = 3,
    Keyframed = 4,
    Fixed = 5,
    ThinBoxInertia = 6,
    Character = 7,
    Total = 8,
}

core_util::impl_enumset_type!(hkpMotionMotionType => u8);

/// C++ `RE::hkpMotion`
#[repr(C)]
pub struct hkpMotion {
    pub base: hkReferencedObject,                      // 000
    pub motion_type: EnumSet<hkpMotionMotionType, u8>, // 010
    pub deactivation_integrate_counter: u8,            // 011
    pub deactivation_num_inactive_frames: [u16; 2],    // 012
    pub pad016: u16,                                   // 016
    pub pad018: u64,                                   // 018
    pub motion_state: hkMotionState,                   // 020
    pub inertia_and_mass_inv: hkVector4,               // 0D0
    pub linear_velocity: hkVector4,                    // 0E0
    pub angular_velocity: hkVector4,                   // 0F0
    pub deactivation_ref_position: [hkVector4; 2],     // 100
    pub deactivation_ref_orientation: [u32; 2],        // 120
    pub saved_motion: *mut hkpMaxSizeMotion,           // 128
    pub saved_quality_type_index: u16,                 // 130
    pub gravity_factor: hkHalf,                        // 132
    pub pad134: u32,                                   // 134
    pub pad138: u64,                                   // 138
}

const _: () = assert!(core::mem::size_of::<hkpMotion>() == 0x140);
const _: () = assert!(core::mem::offset_of!(hkpMotion, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(hkpMotion, motion_type) == 0x10);
const _: () = assert!(core::mem::offset_of!(hkpMotion, motion_state) == 0x20);
const _: () = assert!(core::mem::offset_of!(hkpMotion, inertia_and_mass_inv) == 0xD0);
const _: () = assert!(core::mem::offset_of!(hkpMotion, linear_velocity) == 0xE0);
const _: () = assert!(core::mem::offset_of!(hkpMotion, angular_velocity) == 0xF0);
const _: () = assert!(core::mem::offset_of!(hkpMotion, deactivation_ref_position) == 0x100);
const _: () = assert!(core::mem::offset_of!(hkpMotion, deactivation_ref_orientation) == 0x120);
const _: () = assert!(core::mem::offset_of!(hkpMotion, saved_motion) == 0x128);
const _: () = assert!(core::mem::offset_of!(hkpMotion, gravity_factor) == 0x132);

impl RttiType for hkpMotion {
    const RTTI: VariantID = RTTI_hkpMotion;
}

inherit!(hkpMotion : hkReferencedObject, base);

impl AsRef<hkpMotion> for hkpMotion {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<hkpMotion> for hkpMotion {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl hkpMotion {
    pub const RTTI: VariantID = RTTI_hkpMotion;
    pub const VTABLE: &'static [VariantID] = &VTABLE_hkpMotion;
    pub const NUM_INACTIVE_FRAMES_TO_DEACTIVATE: usize = 5;

    // override (hkReferencedObject)
    // ~hkpMotion() override;  // 00

    virtual_method! {
        pub const VFUNC_SET_MASS: usize = 0x03;
        pub fn set_mass(&mut self, a_mass: f32)
    }

    virtual_method! {
        pub const VFUNC_SET_MASS_INV: usize = 0x04;
        pub fn set_mass_inv(&mut self, a_mass_inv: f32)
    }

    virtual_method! {
        pub const VFUNC_GET_INERTIA_LOCAL: usize = 0x05;
        pub fn get_inertia_local(a_inertia_out: *mut hkMatrix3)
    }

    virtual_method! {
        pub const VFUNC_GET_INERTIA_WORLD: usize = 0x06;
        pub fn get_inertia_world(a_inertia_out: *mut hkMatrix3)
    }

    virtual_method! {
        pub const VFUNC_SET_INERTIA_LOCAL: usize = 0x07;
        pub fn set_inertia_local(&mut self, a_inertia: *const hkMatrix3)
    }

    virtual_method! {
        pub const VFUNC_SET_INERTIA_INV_LOCAL: usize = 0x08;
        pub fn set_inertia_inv_local(&mut self, a_inertia_inv: *const hkMatrix3)
    }

    virtual_method! {
        pub const VFUNC_GET_INERTIA_INV_LOCAL: usize = 0x09;
        pub fn get_inertia_inv_local(a_inertia_inv_out: *mut hkMatrix3)
    }

    virtual_method! {
        pub const VFUNC_GET_INERTIA_INV_WORLD: usize = 0x0A;
        pub fn get_inertia_inv_world(a_inertia_inv_out: *mut hkMatrix3)
    }

    virtual_method! {
        pub const VFUNC_SET_CENTER_OF_MASS_IN_LOCAL: usize = 0x0B;
        pub fn set_center_of_mass_in_local(&mut self, a_center_of_mass: *const hkVector4)
    }

    virtual_method! {
        pub const VFUNC_SET_POSITION: usize = 0x0C;
        pub fn set_position(&mut self, a_position: *const hkVector4)
    }

    virtual_method! {
        pub const VFUNC_SET_ROTATION: usize = 0x0D;
        pub fn set_rotation(&mut self, a_rotation: *const hkQuaternion)
    }

    virtual_method! {
        pub const VFUNC_SET_POSITION_AND_ROTATION: usize = 0x0E;
        pub fn set_position_and_rotation(&mut self, a_position: *const hkVector4, a_rotation: *const hkQuaternion)
    }

    virtual_method! {
        pub const VFUNC_SET_TRANSFORM: usize = 0x0F;
        pub fn set_transform(&mut self, a_transform: *const hkTransform)
    }

    virtual_method! {
        pub const VFUNC_SET_LINEAR_VELOCITY: usize = 0x10;
        pub fn set_linear_velocity(&mut self, a_new_vel: *const hkVector4)
    }

    virtual_method! {
        pub const VFUNC_SET_ANGULAR_VELOCITY: usize = 0x11;
        pub fn set_angular_velocity(&mut self, a_new_vel: *const hkVector4)
    }

    virtual_method! {
        pub const VFUNC_GET_PROJECTED_POINT_VELOCITY: usize = 0x12;
        pub fn get_projected_point_velocity(a_point: *const hkVector4, a_normal: *const hkVector4, a_vel_out: *mut f32, a_inv_virt_mass_out: *mut f32)
    }

    virtual_method! {
        pub const VFUNC_APPLY_LINEAR_IMPULSE: usize = 0x13;
        pub fn apply_linear_impulse(&mut self, a_impulse: *const hkVector4)
    }

    virtual_method! {
        pub const VFUNC_APPLY_POINT_IMPULSE: usize = 0x14;
        pub fn apply_point_impulse(&mut self, a_impulse: *const hkVector4, a_point: *const hkVector4)
    }

    virtual_method! {
        pub const VFUNC_APPLY_ANGULAR_IMPULSE: usize = 0x15;
        pub fn apply_angular_impulse(&mut self, a_impulse: *const hkVector4)
    }

    virtual_method! {
        pub const VFUNC_APPLY_FORCE: usize = 0x16;
        pub fn apply_force(&mut self, a_delta_time: f32, a_force: *const hkVector4)
    }

    virtual_method! {
        pub const VFUNC_APPLY_FORCE_AT_POINT: usize = 0x17;
        pub fn apply_force_at_point(&mut self, a_delta_time: f32, a_force: *const hkVector4, a_point: *const hkVector4)
    }

    virtual_method! {
        pub const VFUNC_APPLY_TORQUE: usize = 0x18;
        pub fn apply_torque(&mut self, a_delta_time: f32, a_torque: *const hkVector4)
    }

    virtual_method! {
        pub const VFUNC_GET_MOTION_STATE_AND_VELOCITIES_AND_DEACTIVATION_TYPE: usize = 0x19;
        pub fn get_motion_state_and_velocities_and_deactivation_type(&mut self, a_motion_out: *mut hkpMotion)
    }

    #[inline(always)]
    pub fn get_mass(&self) -> f32 {
        let mass_inv = self.inertia_and_mass_inv.quad[3];
        if mass_inv != 0.0 { 1.0 / mass_inv } else { 0.0 }
    }
}

pub trait hkpMotionExt {
    fn set_mass(&mut self, a_mass: f32);
    fn set_mass_inv(&mut self, a_mass_inv: f32);
    fn get_inertia_local(&self, a_inertia_out: *mut hkMatrix3);
    fn get_inertia_world(&self, a_inertia_out: *mut hkMatrix3);
    fn set_inertia_local(&mut self, a_inertia: *const hkMatrix3);
    fn set_inertia_inv_local(&mut self, a_inertia_inv: *const hkMatrix3);
    fn get_inertia_inv_local(&self, a_inertia_inv_out: *mut hkMatrix3);
    fn get_inertia_inv_world(&self, a_inertia_inv_out: *mut hkMatrix3);
    fn set_center_of_mass_in_local(&mut self, a_center_of_mass: *const hkVector4);
    fn set_position(&mut self, a_position: *const hkVector4);
    fn set_rotation(&mut self, a_rotation: *const hkQuaternion);
    fn set_position_and_rotation(
        &mut self,
        a_position: *const hkVector4,
        a_rotation: *const hkQuaternion,
    );
    fn set_transform(&mut self, a_transform: *const hkTransform);
    fn set_linear_velocity(&mut self, a_new_vel: *const hkVector4);
    fn set_angular_velocity(&mut self, a_new_vel: *const hkVector4);
    fn get_projected_point_velocity(
        &self,
        a_point: *const hkVector4,
        a_normal: *const hkVector4,
        a_vel_out: *mut f32,
        a_inv_virt_mass_out: *mut f32,
    );
    fn apply_linear_impulse(&mut self, a_impulse: *const hkVector4);
    fn apply_point_impulse(&mut self, a_impulse: *const hkVector4, a_point: *const hkVector4);
    fn apply_angular_impulse(&mut self, a_impulse: *const hkVector4);
    fn apply_force(&mut self, a_delta_time: f32, a_force: *const hkVector4);
    fn apply_force_at_point(
        &mut self,
        a_delta_time: f32,
        a_force: *const hkVector4,
        a_point: *const hkVector4,
    );
    fn apply_torque(&mut self, a_delta_time: f32, a_torque: *const hkVector4);
    fn get_motion_state_and_velocities_and_deactivation_type(
        &mut self,
        a_motion_out: *mut hkpMotion,
    );
    fn get_mass(&self) -> f32;
}

impl<T: AsRef<hkpMotion> + AsMut<hkpMotion>> hkpMotionExt for T {
    #[inline(always)]
    fn set_mass(&mut self, a_mass: f32) {
        hkpMotion::set_mass(self.as_mut(), a_mass)
    }

    #[inline(always)]
    fn set_mass_inv(&mut self, a_mass_inv: f32) {
        hkpMotion::set_mass_inv(self.as_mut(), a_mass_inv)
    }

    #[inline(always)]
    fn get_inertia_local(&self, a_inertia_out: *mut hkMatrix3) {
        hkpMotion::get_inertia_local(self.as_ref(), a_inertia_out)
    }

    #[inline(always)]
    fn get_inertia_world(&self, a_inertia_out: *mut hkMatrix3) {
        hkpMotion::get_inertia_world(self.as_ref(), a_inertia_out)
    }

    #[inline(always)]
    fn set_inertia_local(&mut self, a_inertia: *const hkMatrix3) {
        hkpMotion::set_inertia_local(self.as_mut(), a_inertia)
    }

    #[inline(always)]
    fn set_inertia_inv_local(&mut self, a_inertia_inv: *const hkMatrix3) {
        hkpMotion::set_inertia_inv_local(self.as_mut(), a_inertia_inv)
    }

    #[inline(always)]
    fn get_inertia_inv_local(&self, a_inertia_inv_out: *mut hkMatrix3) {
        hkpMotion::get_inertia_inv_local(self.as_ref(), a_inertia_inv_out)
    }

    #[inline(always)]
    fn get_inertia_inv_world(&self, a_inertia_inv_out: *mut hkMatrix3) {
        hkpMotion::get_inertia_inv_world(self.as_ref(), a_inertia_inv_out)
    }

    #[inline(always)]
    fn set_center_of_mass_in_local(&mut self, a_center_of_mass: *const hkVector4) {
        hkpMotion::set_center_of_mass_in_local(self.as_mut(), a_center_of_mass)
    }

    #[inline(always)]
    fn set_position(&mut self, a_position: *const hkVector4) {
        hkpMotion::set_position(self.as_mut(), a_position)
    }

    #[inline(always)]
    fn set_rotation(&mut self, a_rotation: *const hkQuaternion) {
        hkpMotion::set_rotation(self.as_mut(), a_rotation)
    }

    #[inline(always)]
    fn set_position_and_rotation(
        &mut self,
        a_position: *const hkVector4,
        a_rotation: *const hkQuaternion,
    ) {
        hkpMotion::set_position_and_rotation(self.as_mut(), a_position, a_rotation)
    }

    #[inline(always)]
    fn set_transform(&mut self, a_transform: *const hkTransform) {
        hkpMotion::set_transform(self.as_mut(), a_transform)
    }

    #[inline(always)]
    fn set_linear_velocity(&mut self, a_new_vel: *const hkVector4) {
        hkpMotion::set_linear_velocity(self.as_mut(), a_new_vel)
    }

    #[inline(always)]
    fn set_angular_velocity(&mut self, a_new_vel: *const hkVector4) {
        hkpMotion::set_angular_velocity(self.as_mut(), a_new_vel)
    }

    #[inline(always)]
    fn get_projected_point_velocity(
        &self,
        a_point: *const hkVector4,
        a_normal: *const hkVector4,
        a_vel_out: *mut f32,
        a_inv_virt_mass_out: *mut f32,
    ) {
        hkpMotion::get_projected_point_velocity(
            self.as_ref(),
            a_point,
            a_normal,
            a_vel_out,
            a_inv_virt_mass_out,
        )
    }

    #[inline(always)]
    fn apply_linear_impulse(&mut self, a_impulse: *const hkVector4) {
        hkpMotion::apply_linear_impulse(self.as_mut(), a_impulse)
    }

    #[inline(always)]
    fn apply_point_impulse(&mut self, a_impulse: *const hkVector4, a_point: *const hkVector4) {
        hkpMotion::apply_point_impulse(self.as_mut(), a_impulse, a_point)
    }

    #[inline(always)]
    fn apply_angular_impulse(&mut self, a_impulse: *const hkVector4) {
        hkpMotion::apply_angular_impulse(self.as_mut(), a_impulse)
    }

    #[inline(always)]
    fn apply_force(&mut self, a_delta_time: f32, a_force: *const hkVector4) {
        hkpMotion::apply_force(self.as_mut(), a_delta_time, a_force)
    }

    #[inline(always)]
    fn apply_force_at_point(
        &mut self,
        a_delta_time: f32,
        a_force: *const hkVector4,
        a_point: *const hkVector4,
    ) {
        hkpMotion::apply_force_at_point(self.as_mut(), a_delta_time, a_force, a_point)
    }

    #[inline(always)]
    fn apply_torque(&mut self, a_delta_time: f32, a_torque: *const hkVector4) {
        hkpMotion::apply_torque(self.as_mut(), a_delta_time, a_torque)
    }

    #[inline(always)]
    fn get_motion_state_and_velocities_and_deactivation_type(
        &mut self,
        a_motion_out: *mut hkpMotion,
    ) {
        hkpMotion::get_motion_state_and_velocities_and_deactivation_type(
            self.as_mut(),
            a_motion_out,
        )
    }

    #[inline(always)]
    fn get_mass(&self) -> f32 {
        hkpMotion::get_mass(self.as_ref())
    }
}
