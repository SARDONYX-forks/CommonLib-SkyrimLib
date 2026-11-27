use core_util::inherit;

use crate::offsets::offsets_rtti::RTTI_DualValueModifierEffect;
use crate::offsets::offsets_vtable::VTABLE_DualValueModifierEffect;
use crate::re::{ActorValue, ValueModifierEffect};
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

/// C++ `RE::DualValueModifierEffect`
#[repr(C)]
pub struct DualValueModifierEffect {
    pub base: ValueModifierEffect, // 00
    /// The ctor stores `mgef->data.secondAVWeight` here (0.0 when
    /// `data.secondaryAV` is `kNone`); the secondary actor value itself is not
    /// cached -- read it through `get_additional_actor_value()`.
    pub secondary_av_weight: f32, // 98
    pub pad9c: u32,                // 9C
}

const _: () = assert!(core::mem::size_of::<DualValueModifierEffect>() == 0xA0);
const _: () = assert!(core::mem::offset_of!(DualValueModifierEffect, base) == 0x00);
const _: () = assert!(core::mem::offset_of!(DualValueModifierEffect, secondary_av_weight) == 0x98);

impl RttiType for DualValueModifierEffect {
    const RTTI: VariantID = RTTI_DualValueModifierEffect;
}

impl AsRef<DualValueModifierEffect> for DualValueModifierEffect {
    #[inline(always)]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsMut<DualValueModifierEffect> for DualValueModifierEffect {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

inherit!(DualValueModifierEffect : ValueModifierEffect);

impl DualValueModifierEffect {
    pub const RTTI: VariantID = RTTI_DualValueModifierEffect;
    pub const VTABLE: &'static [VariantID] = &VTABLE_DualValueModifierEffect;

    // override (ActiveEffect)
    // bool IsCausingHealthDamage() override;       // 06
    // void SaveGame(BGSSaveFormBuffer*) override;  // 08
    // void LoadGame(BGSLoadFormBuffer*) override;  // 09
    // ~DualValueModifierEffect() override;         // 13

    // override (ValueModifierEffect)
    // void ModifyActorValue(Actor*, float, ActorValue) override;  // 20

    virtual_method! {
        pub const VFUNC_GET_ADDITIONAL_ACTOR_VALUE: usize = 0x21;
        pub fn get_additional_actor_value() -> ActorValue
    }

    virtual_method! {
        pub const VFUNC_GET_SECONDARY_AV_WEIGHT: usize = 0x22;
        pub fn get_secondary_av_weight() -> f32
    }
}

pub trait DualValueModifierEffectExt {
    fn get_additional_actor_value(&self) -> ActorValue;
    fn get_secondary_av_weight(&self) -> f32;
}

impl<T: AsRef<DualValueModifierEffect> + AsMut<DualValueModifierEffect>> DualValueModifierEffectExt
    for T
{
    #[inline(always)]
    fn get_additional_actor_value(&self) -> ActorValue {
        self.as_ref().get_additional_actor_value()
    }

    #[inline(always)]
    fn get_secondary_av_weight(&self) -> f32 {
        self.as_ref().get_secondary_av_weight()
    }
}
