use crate::version::Version;
#[cfg(test)]
use crate::version::{RUNTIME_LATEST_AE, RUNTIME_LATEST_SE, RUNTIME_LATEST_VR};
use core_util::Later;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeType {
    Unknown,
    SE, // 1.5.97 and lower
    AE, // 1.6.x and 1.7.x
    VR, // Skyrim VR
}

pub static CURRENT_VERSION: Later<Version> = Later::new();
pub static CURRENT_RUNTIME: Later<RuntimeType> = Later::new();

#[cfg(test)]
#[inline(always)]
fn fallback_test_version() -> Version {
    if cfg!(feature = "vr") {
        RUNTIME_LATEST_VR
    } else if cfg!(feature = "ae") {
        RUNTIME_LATEST_AE
    } else {
        RUNTIME_LATEST_SE
    }
}

#[inline(always)]
fn runtime_from_version(version: Version) -> RuntimeType {
    match crate::ffi::module::ModuleRuntime::from_version(version) {
        crate::ffi::module::ModuleRuntime::VR => RuntimeType::VR,
        crate::ffi::module::ModuleRuntime::AE => RuntimeType::AE,
        crate::ffi::module::ModuleRuntime::SE => RuntimeType::SE,
    }
}

#[inline(always)]
fn active_version() -> Version {
    if CURRENT_VERSION.is_init() {
        *CURRENT_VERSION
    } else {
        #[cfg(test)]
        {
            fallback_test_version()
        }

        #[cfg(not(test))]
        {
            *CURRENT_VERSION
        }
    }
}

#[inline(always)]
fn active_runtime() -> RuntimeType {
    if CURRENT_RUNTIME.is_init() {
        *CURRENT_RUNTIME
    } else {
        #[cfg(test)]
        {
            runtime_from_version(active_version())
        }

        #[cfg(not(test))]
        {
            *CURRENT_RUNTIME
        }
    }
}

/// Инициализируется один раз при старте плагина
pub fn init(version: Version) {
    CURRENT_VERSION.init(version);
    CURRENT_RUNTIME.init(runtime_from_version(version));
}

#[inline(always)]
pub fn is_ae() -> bool {
    active_runtime() == RuntimeType::AE
}
#[inline(always)]
pub fn is_se() -> bool {
    active_runtime() == RuntimeType::SE
}
#[inline(always)]
pub fn is_vr() -> bool {
    active_runtime() == RuntimeType::VR
}
#[inline(always)]
pub fn current_runtime() -> RuntimeType {
    active_runtime()
}

#[inline(always)]
pub fn current_version() -> Version {
    active_version()
}

#[inline(always)]
pub const fn runtime_name(runtime: RuntimeType) -> &'static str {
    match runtime {
        RuntimeType::Unknown => "Unknown",
        RuntimeType::SE => "SE",
        RuntimeType::AE => "AE",
        RuntimeType::VR => "VR",
    }
}

#[inline(always)]
pub fn current_runtime_name() -> &'static str {
    runtime_name(current_runtime())
}

#[inline(always)]
pub fn is_at_least(version: Version) -> bool {
    active_version() >= version
}

#[inline(always)]
pub fn relocate<T: Copy>(se_and_vr: T, ae: T) -> T {
    if is_ae() { ae } else { se_and_vr }
}

#[inline(always)]
pub fn relocate_all<T: Copy>(se: T, ae: T, vr: T) -> T {
    if is_vr() {
        vr
    } else if is_ae() {
        ae
    } else {
        se
    }
}

#[inline(always)]
pub fn relocate_if_newer<T: Copy>(version: Version, older: T, newer: T) -> T {
    if is_at_least(version) { newer } else { older }
}

/// `REL::VersionShift`: `offset + delta` when the running game is at least
/// `threshold`, otherwise `offset`. Used for vtable slots that moved because a
/// newer runtime inserted virtual functions.
#[inline(always)]
pub fn version_shift(offset: usize, delta: usize, threshold: Version) -> usize {
    if is_at_least(threshold) {
        offset + delta
    } else {
        offset
    }
}

/// Flat (SE/AE) slot shifted by `delta` on `threshold`+, fixed slot on VR.
#[inline(always)]
pub fn version_shift_vr(flat: usize, delta: usize, threshold: Version, vr: usize) -> usize {
    if is_vr() {
        vr
    } else {
        version_shift(flat, delta, threshold)
    }
}

#[inline(always)]
pub fn relocate_versioned_all<T: Copy>(version: Version, se: T, ae: T, vr: T) -> T {
    if is_vr() {
        vr
    } else if is_at_least(version) {
        ae
    } else {
        se
    }
}

#[inline(always)]
#[track_caller]
pub fn require_offset(offset: usize, accessor: &str) -> usize {
    if offset == 0 {
        crate::skse::log::fatal_runtime(format_args!(
            "{accessor} is unavailable for runtime {}",
            current_runtime_name()
        ));
    }
    offset
}

#[inline(always)]
#[track_caller]
pub fn require_se(accessor: &str) {
    if !is_se() {
        crate::skse::log::fatal_runtime(format_args!(
            "{accessor} is SE-only, current runtime is {}",
            current_runtime_name()
        ));
    }
}

#[inline(always)]
#[track_caller]
pub fn require_ae(accessor: &str) {
    if !is_ae() {
        crate::skse::log::fatal_runtime(format_args!(
            "{accessor} is AE-only, current runtime is {}",
            current_runtime_name()
        ));
    }
}

#[inline(always)]
#[track_caller]
pub fn require_flat(accessor: &str) {
    if is_vr() {
        crate::skse::log::fatal_runtime(format_args!(
            "{accessor} is only available in SE/AE, current runtime is {}",
            current_runtime_name()
        ));
    }
}

#[inline(always)]
#[track_caller]
pub fn require_vr(accessor: &str) {
    if !is_vr() {
        crate::skse::log::fatal_runtime(format_args!(
            "{accessor} is VR-only, current runtime is {}",
            current_runtime_name()
        ));
    }
}

#[inline(always)]
#[track_caller]
pub fn require_non_vr(accessor: &str) {
    require_flat(accessor);
}

#[inline(always)]
pub fn should_validate_layout(expected: usize) -> bool {
    expected != 0
}

#[macro_export]
macro_rules! runtime_offset {
    (offset: $offset:expr $(,)?) => {
        $crate::relocation::IntoOffset::into_offset($offset)
    };
    (se_ae: $se_ae:expr, vr: $vr:expr $(,)?) => {
        $crate::runtime::relocate_all($se_ae, $se_ae, $vr)
    };
    (se_and_vr: $se_and_vr:expr, ae: $ae:expr $(,)?) => {
        $crate::runtime::relocate($se_and_vr, $ae)
    };
    (version: $version:expr, older: $older:expr, newer: $newer:expr $(,)?) => {
        $crate::runtime::relocate_if_newer($version, $older, $newer)
    };
    (se: $se:expr, ae: $ae:expr, vr: $vr:expr $(,)?) => {
        $crate::runtime::relocate_all($se, $ae, $vr)
    };
    (version: $version:expr, se: $se:expr, ae: $ae:expr, vr: $vr:expr $(,)?) => {
        $crate::runtime::relocate_versioned_all($version, $se, $ae, $vr)
    };
}

#[cfg(test)]
mod tests {
    use crate::relocation::VariantOffset;

    struct RuntimeAccessorMacroDummy {
        _value: u32,
    }

    impl RuntimeAccessorMacroDummy {
        const VALUE_OFFSET: VariantOffset = VariantOffset::new(0x0, 0x0, 0x0);

        crate::runtime_data_accessor! {
            fn value_ref() -> u32 {
                offset: Self::VALUE_OFFSET
            }
        }

        crate::runtime_data_mut_accessor! {
            fn value_mut() -> u32 {
                offset: Self::VALUE_OFFSET
            }
        }

        crate::runtime_cast_accessor! {
            fn value_cast() -> u32 {
                offset: Self::VALUE_OFFSET
            }
        }

        crate::runtime_cast_mut_accessor! {
            fn value_cast_mut() -> u32 {
                offset: Self::VALUE_OFFSET
            }
        }
    }

    #[test]
    fn runtime_accessor_macros_support_predeclared_variant_offsets() {
        let _ = RuntimeAccessorMacroDummy::value_ref as fn(&RuntimeAccessorMacroDummy) -> &u32;
        let _ =
            RuntimeAccessorMacroDummy::value_mut as fn(&mut RuntimeAccessorMacroDummy) -> &mut u32;
        let _ = RuntimeAccessorMacroDummy::value_cast as fn(&RuntimeAccessorMacroDummy) -> &u32;
        let _ = RuntimeAccessorMacroDummy::value_cast_mut
            as fn(&mut RuntimeAccessorMacroDummy) -> &mut u32;
    }
}

#[macro_export]
macro_rules! runtime_data_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> &$ret_ty {
            let offset = $crate::runtime::require_offset(
                $crate::runtime_offset! { $($selector)* },
                stringify!($func_name)
            );
            unsafe { &*((self as *const _ as *const u8).add(offset) as *const $ret_ty) }
        }
    };
}

#[macro_export]
macro_rules! runtime_data_ref_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> &$ret_ty {
            let offset = $crate::runtime::require_offset(
                $crate::runtime_offset! { $($selector)* },
                stringify!($func_name)
            );
            unsafe { &*((self as *const _ as *const u8).add(offset) as *const $ret_ty) }
        }
    };
}

#[macro_export]
macro_rules! runtime_data_mut_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&mut self) -> &mut $ret_ty {
            let offset = $crate::runtime::require_offset(
                $crate::runtime_offset! { $($selector)* },
                stringify!($func_name)
            );
            unsafe { &mut *((self as *mut _ as *mut u8).add(offset) as *mut $ret_ty) }
        }
    };
}

#[macro_export]
macro_rules! runtime_optional_data_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> Option<&$ret_ty> {
            let offset = $crate::runtime_offset! { $($selector)* };
            if offset == 0 {
                None
            } else {
                Some(unsafe { &*((self as *const _ as *const u8).add(offset) as *const $ret_ty) })
            }
        }
    };
}

#[macro_export]
macro_rules! runtime_optional_data_mut_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&mut self) -> Option<&mut $ret_ty> {
            let offset = $crate::runtime_offset! { $($selector)* };
            if offset == 0 {
                None
            } else {
                Some(unsafe { &mut *((self as *mut _ as *mut u8).add(offset) as *mut $ret_ty) })
            }
        }
    };
}

#[macro_export]
macro_rules! runtime_data_ptr_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> *mut $ret_ty {
            let offset = $crate::runtime::require_offset(
                $crate::runtime_offset! { $($selector)* },
                stringify!($func_name)
            );
            unsafe { (self as *const _ as *mut u8).add(offset) as *mut $ret_ty }
        }
    };
}

#[macro_export]
macro_rules! runtime_pointer_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> $ret_ty {
            let offset = $crate::runtime::require_offset(
                $crate::runtime_offset! { $($selector)* },
                stringify!($func_name)
            );
            unsafe {
                let ptr = (self as *const _ as *const u8).add(offset) as *const $ret_ty;
                *ptr
            }
        }
    };
}

#[macro_export]
macro_rules! vr_runtime_data_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            vr: $vr:expr $(,)?
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> &$ret_ty {
            $crate::runtime::require_vr(stringify!($func_name));
            unsafe { &*((self as *const _ as *const u8).add($vr) as *const $ret_ty) }
        }
    };
}

#[macro_export]
macro_rules! vr_runtime_data_mut_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            vr: $vr:expr $(,)?
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&mut self) -> &mut $ret_ty {
            $crate::runtime::require_vr(stringify!($func_name));
            unsafe { &mut *((self as *mut _ as *mut u8).add($vr) as *mut $ret_ty) }
        }
    };
}

#[macro_export]
macro_rules! se_only_pointer_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            se: $se:expr $(,)?
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> *mut $ret_ty {
            if $crate::runtime::is_vr() {
                core::ptr::null_mut()
            } else {
                unsafe { (self as *const _ as *mut u8).add($se) as *mut $ret_ty }
            }
        }
    };
}

#[macro_export]
macro_rules! vr_only_pointer_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            vr: $vr:expr $(,)?
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> *mut $ret_ty {
            if !$crate::runtime::is_vr() {
                core::ptr::null_mut()
            } else {
                unsafe { (self as *const _ as *mut u8).add($vr) as *mut $ret_ty }
            }
        }
    };
}

#[macro_export]
macro_rules! runtime_cast_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        $crate::runtime_data_ref_accessor! {
            $vis fn $func_name() -> $ret_ty {
                $($selector)*
            }
        }
    };
}

#[macro_export]
macro_rules! runtime_cast_mut_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        $crate::runtime_data_mut_accessor! {
            $vis fn $func_name() -> $ret_ty {
                $($selector)*
            }
        }
    };
}

#[macro_export]
macro_rules! runtime_optional_pointer_accessor {
    (
        $vis:vis fn $func_name:ident() -> $ret_ty:ty {
            $($selector:tt)*
        }
    ) => {
        #[inline(always)]
        $vis fn $func_name(&self) -> Option<$ret_ty> {
            let offset = $crate::runtime_offset! { $($selector)* };
            if offset == 0 {
                None
            } else {
                unsafe {
                    let ptr = (self as *const _ as *const u8).add(offset) as *const $ret_ty;
                    Some(*ptr)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! runtime_assert_size {
    ($ty:ty, se_ae: $se_ae:expr, vr: $vr:expr $(,)?) => {
        $crate::runtime_assert_size!($ty, se: $se_ae, ae: $se_ae, vr: $vr);
    };
    ($ty:ty, version: $version:expr, older: $older:expr, newer: $newer:expr $(,)?) => {{
        let expected = $crate::runtime::relocate_if_newer($version, $older, $newer);
        if $crate::runtime::should_validate_layout(expected) {
            assert_eq!(
                ::core::mem::size_of::<$ty>(),
                expected,
                "size_of::<{}>() mismatch for runtime {}",
                stringify!($ty),
                $crate::runtime::current_runtime_name()
            );
        }
    }};
    ($ty:ty, se: $se:expr, ae: $ae:expr, vr: $vr:expr $(,)?) => {{
        let expected = $crate::runtime::relocate_all($se, $ae, $vr);
        if $crate::runtime::should_validate_layout(expected) {
            assert_eq!(
                ::core::mem::size_of::<$ty>(),
                expected,
                "size_of::<{}>() mismatch for runtime {}",
                stringify!($ty),
                $crate::runtime::current_runtime_name()
            );
        }
    }};
    ($ty:ty, version: $version:expr, se: $se:expr, ae: $ae:expr, vr: $vr:expr $(,)?) => {{
        let expected = $crate::runtime::relocate_versioned_all($version, $se, $ae, $vr);
        if $crate::runtime::should_validate_layout(expected) {
            assert_eq!(
                ::core::mem::size_of::<$ty>(),
                expected,
                "size_of::<{}>() mismatch for runtime {}",
                stringify!($ty),
                $crate::runtime::current_runtime_name()
            );
        }
    }};
}

#[macro_export]
macro_rules! runtime_assert_offset {
    ($ty:ty, $field:tt, se_ae: $se_ae:expr, vr: $vr:expr $(,)?) => {
        $crate::runtime_assert_offset!($ty, $field, se: $se_ae, ae: $se_ae, vr: $vr);
    };
    ($ty:ty, $field:tt, version: $version:expr, older: $older:expr, newer: $newer:expr $(,)?) => {{
        let expected = $crate::runtime::relocate_if_newer($version, $older, $newer);
        if $crate::runtime::should_validate_layout(expected) {
            assert_eq!(
                ::core::mem::offset_of!($ty, $field),
                expected,
                "offset_of!({}, {}) mismatch for runtime {}",
                stringify!($ty),
                stringify!($field),
                $crate::runtime::current_runtime_name()
            );
        }
    }};
    ($ty:ty, $field:tt, se: $se:expr, ae: $ae:expr, vr: $vr:expr $(,)?) => {{
        let expected = $crate::runtime::relocate_all($se, $ae, $vr);
        if $crate::runtime::should_validate_layout(expected) {
            assert_eq!(
                ::core::mem::offset_of!($ty, $field),
                expected,
                "offset_of!({}, {}) mismatch for runtime {}",
                stringify!($ty),
                stringify!($field),
                $crate::runtime::current_runtime_name()
            );
        }
    }};
    ($ty:ty, $field:tt, version: $version:expr, se: $se:expr, ae: $ae:expr, vr: $vr:expr $(,)?) => {{
        let expected = $crate::runtime::relocate_versioned_all($version, $se, $ae, $vr);
        if $crate::runtime::should_validate_layout(expected) {
            assert_eq!(
                ::core::mem::offset_of!($ty, $field),
                expected,
                "offset_of!({}, {}) mismatch for runtime {}",
                stringify!($ty),
                stringify!($field),
                $crate::runtime::current_runtime_name()
            );
        }
    }};
}
