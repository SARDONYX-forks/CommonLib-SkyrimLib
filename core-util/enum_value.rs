use core::marker::PhantomData;

use crate::{EnumSetInteger, EnumSetType};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Enum<E, U> {
    value: U,
    _marker: PhantomData<fn() -> E>,
}

impl<E, U> Enum<E, U>
where
    U: EnumSetInteger,
{
    #[inline(always)]
    pub const fn from_underlying(value: U) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn underlying(self) -> U {
        self.value
    }

    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.value == U::ZERO
    }

    #[inline(always)]
    pub fn get(self) -> Option<E>
    where
        E: TryFrom<U>,
    {
        E::try_from(self.value).ok()
    }

    #[inline(always)]
    pub fn is_known(self) -> bool
    where
        E: TryFrom<U>,
    {
        self.get().is_some()
    }

    #[inline(always)]
    pub fn get_or(self, default: E) -> E
    where
        E: TryFrom<U>,
    {
        self.get().unwrap_or(default)
    }

    #[inline(always)]
    pub fn get_or_else<F>(self, default: F) -> E
    where
        E: TryFrom<U>,
        F: FnOnce(U) -> E,
    {
        let raw = self.value;
        match E::try_from(raw) {
            Ok(value) => value,
            Err(_) => default(raw),
        }
    }

    #[inline(always)]
    pub fn map_or<T, F>(self, default: T, f: F) -> T
    where
        E: TryFrom<U>,
        F: FnOnce(E) -> T,
    {
        match self.get() {
            Some(value) => f(value),
            None => default,
        }
    }

    #[inline(always)]
    pub fn map_or_else<T, D, F>(self, default: D, f: F) -> T
    where
        E: TryFrom<U>,
        D: FnOnce(U) -> T,
        F: FnOnce(E) -> T,
    {
        let raw = self.value;
        match E::try_from(raw) {
            Ok(value) => f(value),
            Err(_) => default(raw),
        }
    }
}

impl<E, U> Default for Enum<E, U>
where
    U: EnumSetInteger,
{
    #[inline(always)]
    fn default() -> Self {
        Self::from_underlying(U::ZERO)
    }
}

impl<E, U> Enum<E, U>
where
    E: EnumSetType<U>,
    U: EnumSetInteger,
{
    #[inline(always)]
    pub fn new(value: E) -> Self {
        Self::from_underlying(value.to_underlying())
    }

    #[inline(always)]
    pub fn set(&mut self, value: E) -> &mut Self {
        self.value = value.to_underlying();
        self
    }
}

impl<E, U> From<E> for Enum<E, U>
where
    E: EnumSetType<U>,
    U: EnumSetInteger,
{
    #[inline(always)]
    fn from(value: E) -> Self {
        Self::new(value)
    }
}

impl<E, U> PartialEq<E> for Enum<E, U>
where
    E: EnumSetType<U>,
    U: EnumSetInteger,
{
    #[inline(always)]
    fn eq(&self, other: &E) -> bool {
        self.value == other.to_underlying()
    }
}

#[macro_export]
macro_rules! impl_enum_type {
    ($enum_ty:ty => $storage_ty:ty) => {
        impl $crate::EnumSetType<$storage_ty> for $enum_ty {
            #[inline(always)]
            fn to_underlying(self) -> $storage_ty {
                self as $storage_ty
            }
        }
    };
}

#[macro_export]
macro_rules! impl_enum_try_from_contiguous {
    ($enum_ty:ty => $storage_ty:ty, $min:path, $max:path) => {
        impl core::convert::TryFrom<$storage_ty> for $enum_ty {
            type Error = ();

            #[inline(always)]
            fn try_from(value: $storage_ty) -> Result<Self, Self::Error> {
                let min = $min as $storage_ty;
                let max = $max as $storage_ty;
                if value < min || value > max {
                    return Err(());
                }

                Ok(unsafe { core::mem::transmute::<$storage_ty, Self>(value) })
            }
        }
    };
}

#[macro_export]
macro_rules! impl_enum_try_from_sparse {
    ($enum_ty:ty => $storage_ty:ty { $($raw:expr => $variant:path),+ $(,)? }) => {
        impl core::convert::TryFrom<$storage_ty> for $enum_ty {
            type Error = ();

            #[inline(always)]
            fn try_from(value: $storage_ty) -> Result<Self, Self::Error> {
                match value {
                    $($raw => Ok($variant),)+
                    _ => Err(()),
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::Enum;

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestValue {
        Zero = 0,
        One = 1,
        Two = 2,
    }

    // impl TryFrom<u8> for TestValue {
    //     type Error = ();

    //     fn try_from(value: u8) -> Result<Self, Self::Error> {
    //         match value {
    //             0 => Ok(Self::Zero),
    //             1 => Ok(Self::One),
    //             2 => Ok(Self::Two),
    //             _ => Err(()),
    //         }
    //     }
    // }

    crate::impl_enum_type!(TestValue => i32);
    crate::impl_enum_type!(TestValue => u8);

    crate::impl_enum_try_from_contiguous!(TestValue => u8, TestValue::Zero, TestValue::Two);

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum SparseValue {
        Zero = 0,
        Five = 5,
        Nine = 9,
    }

    crate::impl_enum_try_from_sparse!(SparseValue => u8 {
        0 => SparseValue::Zero,
        5 => SparseValue::Five,
        9 => SparseValue::Nine,
    });

    #[test]
    fn stores_exact_underlying_value() {
        let value = Enum::<TestValue, i32>::from(TestValue::Two);
        assert_eq!(value.underlying(), 2);
        assert_eq!(value, TestValue::Two);
    }

    #[test]
    fn supports_narrower_storage_than_repr() {
        let mut value = Enum::<TestValue, u8>::default();
        value.set(TestValue::One);
        assert_eq!(value.underlying(), 1u8);
        assert_eq!(value.get(), Some(TestValue::One));
    }

    #[test]
    fn contiguous_try_from_rejects_out_of_range_values() {
        assert_eq!(TestValue::try_from(2u8), Ok(TestValue::Two));
        assert_eq!(TestValue::try_from(3u8), Err(()));
    }

    #[test]
    fn sparse_try_from_accepts_only_declared_values() {
        assert_eq!(SparseValue::try_from(5u8), Ok(SparseValue::Five));
        assert_eq!(SparseValue::try_from(6u8), Err(()));
    }
}
