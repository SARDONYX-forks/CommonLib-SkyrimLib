use crate::ffi;
use crate::runtime;
use crate::version::Version;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;
use iced_x86::{
    BlockEncoder, BlockEncoderOptions, Decoder, DecoderOptions, Instruction, InstructionBlock,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelocationError {
    UnresolvedId(usize),
    UnresolvedOffset(usize),
    UnsupportedCallHookSize(usize),
    UnsupportedBranchHookSize(usize),
    InvalidVtableSlot(usize),
    FunctionHookSizeTooSmall(usize),
    FunctionHookDecodeFailed,
    FunctionHookRelocationFailed,
    FunctionHookTrampolineAllocFailed,
    FunctionHookUniversalInstallFailed,
    MissingRuntimeDynamicCast,
}

impl fmt::Display for RelocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnresolvedId(id) => write!(f, "Address Library failed to resolve ID {id}"),
            Self::UnresolvedOffset(offset) => {
                write!(f, "failed to resolve offset {offset:#X}")
            }
            Self::UnsupportedCallHookSize(size) => {
                write!(
                    f,
                    "unsupported call hook size {size}; only 5 or 6 are supported"
                )
            }
            Self::UnsupportedBranchHookSize(size) => {
                write!(
                    f,
                    "unsupported branch hook size {size}; only 5 or 6 are supported"
                )
            }
            Self::InvalidVtableSlot(slot) => write!(
                f,
                "invalid vtable slot {slot:#X}; slot must be aligned to pointer size {}",
                core::mem::size_of::<usize>()
            ),
            Self::FunctionHookSizeTooSmall(size) => {
                write!(f, "function hook needs at least 5 bytes, got {size}")
            }
            Self::FunctionHookDecodeFailed => f.write_str("failed to decode function prologue"),
            Self::FunctionHookRelocationFailed => {
                f.write_str("failed to relocate stolen instructions into trampoline")
            }
            Self::FunctionHookTrampolineAllocFailed => {
                f.write_str("failed to allocate trampoline storage")
            }
            Self::FunctionHookUniversalInstallFailed => {
                f.write_str("MinHook failed to install universal function hook")
            }
            Self::MissingRuntimeDynamicCast => {
                f.write_str("failed to resolve RTDynamicCast address")
            }
        }
    }
}

pub trait TryIntoAddress {
    fn try_into_address(self) -> Result<usize, RelocationError>;
}

pub trait IntoAddress: TryIntoAddress {
    fn into_address(self) -> usize
    where
        Self: Sized,
    {
        self.try_into_address().unwrap_or(0)
    }
}

impl<T: TryIntoAddress> IntoAddress for T {}

pub trait TryIntoOffset {
    fn try_into_offset(self) -> Result<usize, RelocationError>;
}

pub trait IntoOffset: TryIntoOffset {
    fn into_offset(self) -> usize
    where
        Self: Sized,
    {
        self.try_into_offset().unwrap_or(0)
    }
}

impl<T: TryIntoOffset> IntoOffset for T {}

impl TryIntoAddress for usize {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        Ok(self)
    }
}

impl TryIntoOffset for usize {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        Ok(self)
    }
}

#[inline(always)]
fn module_base() -> usize {
    unsafe { ffi::commonlib_offset_to_address(0) }
}

#[inline(always)]
fn resolve_id_address(id: usize) -> Result<usize, RelocationError> {
    let address = unsafe { ffi::commonlib_id_to_address(id) };
    if address == 0 {
        Err(RelocationError::UnresolvedId(id))
    } else {
        Ok(address)
    }
}

#[inline(always)]
fn resolve_offset_address(offset: usize) -> Result<usize, RelocationError> {
    let address = unsafe { ffi::commonlib_offset_to_address(offset) };
    if address == 0 {
        Err(RelocationError::UnresolvedOffset(offset))
    } else {
        Ok(address)
    }
}

fn fatal_resolution(context: &str, error: RelocationError) -> ! {
    match error {
        RelocationError::UnresolvedId(id) => crate::skse::log::fatal_runtime(format_args!(
            "{}: Address Library failed to resolve ID {}",
            context, id
        )),
        RelocationError::UnresolvedOffset(offset) => crate::skse::log::fatal_runtime(format_args!(
            "{}: failed to resolve offset {:#X}",
            context, offset
        )),
        RelocationError::UnsupportedCallHookSize(size) => {
            crate::skse::log::fatal_runtime(format_args!(
                "{}: unsupported call hook size {}. Only 5 or 6 are supported.",
                context, size
            ))
        }
        RelocationError::UnsupportedBranchHookSize(size) => {
            crate::skse::log::fatal_runtime(format_args!(
                "{}: unsupported branch hook size {}. Only 5 or 6 are supported.",
                context, size
            ))
        }
        RelocationError::InvalidVtableSlot(slot) => crate::skse::log::fatal_runtime(format_args!(
            "{}: invalid vtable slot {:#X}; slot must be aligned to pointer size {}",
            context,
            slot,
            core::mem::size_of::<usize>()
        )),
        RelocationError::FunctionHookSizeTooSmall(size) => {
            crate::skse::log::fatal_runtime(format_args!(
                "{}: function hook needs at least 5 bytes, got {}",
                context, size
            ))
        }
        RelocationError::FunctionHookDecodeFailed => crate::skse::log::fatal_runtime(format_args!(
            "{context}: failed to decode function prologue"
        )),
        RelocationError::FunctionHookRelocationFailed => crate::skse::log::fatal_runtime(
            format_args!("{context}: failed to relocate stolen instructions into trampoline"),
        ),
        RelocationError::FunctionHookTrampolineAllocFailed => crate::skse::log::fatal_runtime(
            format_args!("{context}: failed to allocate trampoline storage"),
        ),
        RelocationError::FunctionHookUniversalInstallFailed => {
            crate::skse::log::fatal_runtime(format_args!(
                "{context}: MinHook failed to install universal function hook; see native log for details"
            ))
        }
        RelocationError::MissingRuntimeDynamicCast => crate::skse::log::fatal_runtime(
            format_args!("{}: failed to resolve RTDynamicCast address", context),
        ),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ID {
    id: usize,
}

impl ID {
    pub const fn new(id: usize) -> Self {
        Self { id }
    }

    pub const fn id(self) -> usize {
        self.id
    }

    pub fn try_offset(self) -> Result<usize, RelocationError> {
        Ok(resolve_id_address(self.id)? - module_base())
    }

    pub fn offset(self) -> usize {
        self.try_offset().unwrap_or(0)
    }

    pub fn try_address(self) -> Result<usize, RelocationError> {
        resolve_id_address(self.id)
    }

    pub fn address(self) -> usize {
        self.try_address().unwrap_or(0)
    }
}

impl TryIntoAddress for ID {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        self.try_address()
    }
}

impl TryIntoOffset for ID {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        self.try_offset()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Offset {
    offset: usize,
}

impl Offset {
    pub const fn new(offset: usize) -> Self {
        Self { offset }
    }

    pub const fn offset(self) -> usize {
        self.offset
    }

    pub fn try_address(self) -> Result<usize, RelocationError> {
        resolve_offset_address(self.offset)
    }

    pub fn address(self) -> usize {
        self.try_address().unwrap_or(0)
    }
}

impl TryIntoAddress for Offset {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        self.try_address()
    }
}

impl TryIntoOffset for Offset {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        Ok(self.offset)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RelocationID {
    pub se: usize,
    pub ae: usize,
    pub vr: usize,
}

impl RelocationID {
    pub const fn new(se: usize, ae: usize) -> Self {
        Self { se, ae, vr: se }
    }

    pub const fn with_vr(se: usize, ae: usize, vr: usize) -> Self {
        Self { se, ae, vr }
    }

    pub fn id(self) -> usize {
        if runtime::is_vr() {
            self.vr
        } else if runtime::is_ae() {
            self.ae
        } else if runtime::is_se() {
            self.se
        } else {
            0
        }
    }

    pub fn active(self) -> ID {
        ID::new(self.id())
    }

    pub fn try_offset(self) -> Result<usize, RelocationError> {
        self.active().try_offset()
    }

    pub fn offset(self) -> usize {
        self.try_offset().unwrap_or(0)
    }

    pub fn try_address(self) -> Result<usize, RelocationError> {
        self.active().try_address()
    }

    pub fn address(self) -> usize {
        self.try_address().unwrap_or(0)
    }
}

impl TryIntoAddress for RelocationID {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        self.try_address()
    }
}

impl TryIntoOffset for RelocationID {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        self.try_offset()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VersionedRelocationID {
    pub version: Version,
    pub se: usize,
    pub ae_before: usize,
    pub ae_after: usize,
    pub vr: usize,
}

impl VersionedRelocationID {
    pub const fn new(version: Version, se: usize, ae_before: usize, ae_after: usize) -> Self {
        Self {
            version,
            se,
            ae_before,
            ae_after,
            vr: se,
        }
    }

    pub const fn with_vr(
        version: Version,
        se: usize,
        ae_before: usize,
        ae_after: usize,
        vr: usize,
    ) -> Self {
        Self {
            version,
            se,
            ae_before,
            ae_after,
            vr,
        }
    }

    pub fn id(self) -> usize {
        if runtime::is_vr() {
            self.vr
        } else if runtime::is_ae() {
            if runtime::is_at_least(self.version) {
                self.ae_after
            } else {
                self.ae_before
            }
        } else if runtime::is_se() {
            self.se
        } else {
            0
        }
    }

    pub fn active(self) -> ID {
        ID::new(self.id())
    }

    pub fn try_offset(self) -> Result<usize, RelocationError> {
        self.active().try_offset()
    }

    pub fn offset(self) -> usize {
        self.try_offset().unwrap_or(0)
    }

    pub fn try_address(self) -> Result<usize, RelocationError> {
        self.active().try_address()
    }

    pub fn address(self) -> usize {
        self.try_address().unwrap_or(0)
    }
}

impl TryIntoAddress for VersionedRelocationID {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        self.try_address()
    }
}

impl TryIntoOffset for VersionedRelocationID {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        self.try_offset()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VariantID {
    pub se: usize,
    pub ae: usize,
    pub vr: usize,
}

impl VariantID {
    pub const fn new(se: usize, ae: usize, vr: usize) -> Self {
        Self { se, ae, vr }
    }

    pub fn try_offset(self) -> Result<usize, RelocationError> {
        if runtime::is_vr() {
            Ok(self.vr)
        } else if runtime::is_ae() {
            ID::new(self.ae).try_offset()
        } else if runtime::is_se() {
            ID::new(self.se).try_offset()
        } else {
            Err(RelocationError::UnresolvedOffset(0))
        }
    }

    pub fn offset(self) -> usize {
        self.try_offset().unwrap_or(0)
    }

    pub fn try_address(self) -> Result<usize, RelocationError> {
        if runtime::is_vr() {
            Offset::new(self.vr).try_address()
        } else if runtime::is_ae() {
            ID::new(self.ae).try_address()
        } else if runtime::is_se() {
            ID::new(self.se).try_address()
        } else {
            Err(RelocationError::UnresolvedOffset(0))
        }
    }

    pub fn address(self) -> usize {
        self.try_address().unwrap_or(0)
    }
}

impl TryIntoAddress for VariantID {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        self.try_address()
    }
}

impl TryIntoOffset for VariantID {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        self.try_offset()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VariantOffset {
    pub se: usize,
    pub ae: usize,
    pub vr: usize,
}

impl VariantOffset {
    pub const fn new(se: usize, ae: usize, vr: usize) -> Self {
        Self { se, ae, vr }
    }

    pub const fn new_se_ae(se_ae: usize, vr: usize) -> Self {
        Self {
            se: se_ae,
            ae: se_ae,
            vr,
        }
    }

    #[inline(always)]
    pub fn offset(self) -> usize {
        if runtime::is_vr() {
            self.vr
        } else if runtime::is_ae() {
            self.ae
        } else {
            self.se
        }
    }

    pub fn try_address(self) -> Result<usize, RelocationError> {
        Offset::new(self.offset()).try_address()
    }

    pub fn address(self) -> usize {
        self.try_address().unwrap_or(0)
    }
}

impl TryIntoAddress for VariantOffset {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        self.try_address()
    }
}

impl TryIntoOffset for VariantOffset {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        Ok(self.offset())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Relocation<T = usize> {
    address: usize,
    _marker: PhantomData<fn() -> T>,
}

impl<T> Relocation<T> {
    pub const fn from_address(address: usize) -> Self {
        Self {
            address,
            _marker: PhantomData,
        }
    }

    pub fn new<A: TryIntoAddress>(source: A) -> Self {
        match Self::try_new(source) {
            Ok(relocation) => relocation,
            Err(error) => fatal_resolution("Failed to resolve relocation", error),
        }
    }

    pub fn try_new<A: TryIntoAddress>(source: A) -> Result<Self, RelocationError> {
        Ok(Self::from_address(source.try_into_address()?))
    }

    pub const fn address(self) -> usize {
        self.address
    }

    pub fn offset(self) -> usize {
        self.address.saturating_sub(module_base())
    }

    pub const fn is_null(self) -> bool {
        self.address == 0
    }

    pub const fn cast<U>(self) -> Relocation<U> {
        Relocation::from_address(self.address)
    }

    pub const fn as_ptr(self) -> *const T {
        self.address as *const T
    }

    pub const fn as_mut_ptr(self) -> *mut T {
        self.address as *mut T
    }

    pub fn write_bytes(self, bytes: &[u8]) {
        safe_write(self.address, bytes);
    }

    pub fn fill_bytes(self, value: u8, count: usize) {
        safe_fill(self.address, value, count);
    }

    pub fn write_call(self, dst: usize, size: usize) -> usize {
        write_call(self.address, dst, size)
    }

    pub fn write_branch(self, dst: usize, size: usize) -> usize {
        write_branch(self.address, dst, size)
    }

    pub fn write_vfunc<I: IntoOffset>(self, index: I, new_func: usize) -> usize {
        write_vfunc(self.address, index, new_func)
    }
}

impl<T: Copy> Relocation<T> {
    pub unsafe fn get(self) -> T {
        debug_assert_eq!(core::mem::size_of::<T>(), core::mem::size_of::<usize>());
        unsafe { core::mem::transmute_copy(&self.address) }
    }
}

/// Resolves a relocation that points at a pointer cell and returns the pointee.
///
/// This matches CommonLib-style helpers that use `REL::Relocation<T**>` and
/// return `*singleton`, as opposed to `Relocation::<*mut T>::get()`, which
/// keeps the original address-cast semantics.
///
/// # Safety
/// `source` must resolve to a valid pointer cell containing a `*mut T`.
#[inline(always)]
pub unsafe fn read_indirect_ptr<T, A: TryIntoAddress>(source: A) -> *mut T {
    let cell = unsafe { Relocation::<*mut *mut T>::new(source).get() };
    if cell.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { *cell }
    }
}

impl<T> TryIntoAddress for Relocation<T> {
    fn try_into_address(self) -> Result<usize, RelocationError> {
        Ok(self.address)
    }
}

impl<T> TryIntoOffset for Relocation<T> {
    fn try_into_offset(self) -> Result<usize, RelocationError> {
        Ok(self.offset())
    }
}

pub fn safe_write<A: IntoAddress>(address: A, bytes: &[u8]) {
    unsafe {
        ffi::commonlib_safe_write(address.into_address(), bytes.as_ptr(), bytes.len());
    }
}

pub fn safe_fill<A: IntoAddress>(address: A, value: u8, count: usize) {
    unsafe {
        ffi::commonlib_safe_fill(address.into_address(), value, count);
    }
}

pub fn write_vfunc<A: IntoAddress, I: IntoOffset>(
    vtable_addr: A,
    index: I,
    new_func: usize,
) -> usize {
    match try_write_vfunc(vtable_addr, index, new_func) {
        Ok(address) => address,
        Err(error) => fatal_resolution("Failed to install vtable hook", error),
    }
}

pub fn write_call<A: IntoAddress>(src: A, dst: usize, size: usize) -> usize {
    match try_write_call(src, dst, size) {
        Ok(address) => address,
        Err(error) => fatal_resolution("Failed to install call hook", error),
    }
}

pub fn try_write_vfunc<A: TryIntoAddress, I: TryIntoOffset>(
    vtable_addr: A,
    index: I,
    new_func: usize,
) -> Result<usize, RelocationError> {
    Ok(unsafe {
        ffi::commonlib_write_vfunc(
            vtable_addr.try_into_address()?,
            index.try_into_offset()?,
            new_func,
        )
    })
}

pub fn try_write_call<A: TryIntoAddress>(
    src: A,
    dst: usize,
    size: usize,
) -> Result<usize, RelocationError> {
    let src = src.try_into_address()?;
    unsafe {
        match size {
            5 => Ok(ffi::commonlib_write_call5(src, dst)),
            6 => Ok(ffi::commonlib_write_call6(src, dst)),
            _ => Err(RelocationError::UnsupportedCallHookSize(size)),
        }
    }
}

pub fn write_branch<A: IntoAddress>(src: A, dst: usize, size: usize) -> usize {
    match try_write_branch(src, dst, size) {
        Ok(address) => address,
        Err(error) => fatal_resolution("Failed to install branch hook", error),
    }
}

pub fn try_write_branch<A: TryIntoAddress>(
    src: A,
    dst: usize,
    size: usize,
) -> Result<usize, RelocationError> {
    let src = src.try_into_address()?;
    unsafe {
        match size {
            5 => Ok(ffi::commonlib_write_branch5(src, dst)),
            6 => Ok(ffi::commonlib_write_branch6(src, dst)),
            _ => Err(RelocationError::UnsupportedBranchHookSize(size)),
        }
    }
}

const FUNCTION_HOOK_PATCH_SIZE: usize = 5;
const FUNCTION_HOOK_SCAN_LEN: usize = 64;

#[inline(always)]
fn function_hook_jump_back(address: usize) -> [u8; 14] {
    let mut bytes = [0u8; 14];
    bytes[0] = 0xFF;
    bytes[1] = 0x25;
    bytes[2..6].copy_from_slice(&0i32.to_le_bytes());
    bytes[6..14].copy_from_slice(&(address as u64).to_le_bytes());
    bytes
}

fn decode_function_hook_instructions(
    target: usize,
    required_len: usize,
    exact_len: bool,
) -> Result<(Vec<Instruction>, usize), RelocationError> {
    let bytes = unsafe { core::slice::from_raw_parts(target as *const u8, FUNCTION_HOOK_SCAN_LEN) };
    let mut decoder = Decoder::with_ip(64, bytes, target as u64, DecoderOptions::NONE);
    let mut instructions = Vec::new();
    let mut decoded_len = 0usize;

    while decoded_len < required_len {
        let instruction = decoder.decode();
        if instruction.is_invalid() {
            return Err(RelocationError::FunctionHookDecodeFailed);
        }

        decoded_len += instruction.len();
        instructions.push(instruction);
    }

    if exact_len && decoded_len != required_len {
        return Err(RelocationError::FunctionHookDecodeFailed);
    }

    Ok((instructions, decoded_len))
}

fn relocate_function_hook_trampoline(
    target: usize,
    dst: usize,
    stolen_len: usize,
) -> Result<usize, RelocationError> {
    if stolen_len < FUNCTION_HOOK_PATCH_SIZE {
        return Err(RelocationError::FunctionHookSizeTooSmall(stolen_len));
    }

    let (instructions, decoded_len) = decode_function_hook_instructions(target, stolen_len, true)?;
    let reserve_size = decoded_len + instructions.len() * 64 + 16;
    let trampoline_addr = unsafe { ffi::commonlib_trampoline_allocate(reserve_size) as usize };
    if trampoline_addr == 0 {
        return Err(RelocationError::FunctionHookTrampolineAllocFailed);
    }

    let relocated = BlockEncoder::encode(
        64,
        InstructionBlock::new(&instructions, trampoline_addr as u64),
        BlockEncoderOptions::NONE,
    )
    .map_err(|_| RelocationError::FunctionHookRelocationFailed)?;

    let code = relocated.code_buffer;
    let jump_back = function_hook_jump_back(target + decoded_len);
    let required_size = code.len() + jump_back.len();
    if required_size > reserve_size {
        return Err(RelocationError::FunctionHookRelocationFailed);
    }

    safe_write(trampoline_addr, &code);
    safe_write(trampoline_addr + code.len(), &jump_back);

    unsafe {
        let _ = ffi::commonlib_write_branch5(target, dst);
    }
    if decoded_len > FUNCTION_HOOK_PATCH_SIZE {
        safe_fill(
            target + FUNCTION_HOOK_PATCH_SIZE,
            0x90,
            decoded_len - FUNCTION_HOOK_PATCH_SIZE,
        );
    }

    Ok(trampoline_addr)
}

/// Installs a function-entry detour at `target`, using the exact number of
/// bytes supplied in `stolen_len` as the relocated prologue.
///
/// This is a low-level primitive. It does not inspect argument nullability,
/// resolve handles, or adapt user-friendly SDK types.
pub fn write_function_hook_explicit<A: IntoAddress>(
    target: A,
    dst: usize,
    stolen_len: usize,
) -> usize {
    match try_write_function_hook_explicit(target, dst, stolen_len) {
        Ok(address) => address,
        Err(error) => fatal_resolution("Failed to install explicit function hook", error),
    }
}

/// Installs a function-entry detour at `target`, automatically decoding enough
/// whole instructions to cover the entry patch.
///
/// This is a low-level primitive. It does not inspect argument nullability,
/// resolve handles, or adapt user-friendly SDK types.
pub fn write_function_hook_auto<A: IntoAddress>(target: A, dst: usize) -> usize {
    match try_write_function_hook_auto(target, dst) {
        Ok(address) => address,
        Err(error) => fatal_resolution("Failed to install auto function hook", error),
    }
}

pub fn try_write_function_hook_explicit<A: TryIntoAddress>(
    target: A,
    dst: usize,
    stolen_len: usize,
) -> Result<usize, RelocationError> {
    relocate_function_hook_trampoline(target.try_into_address()?, dst, stolen_len)
}

pub fn try_write_function_hook_auto<A: TryIntoAddress>(
    target: A,
    dst: usize,
) -> Result<usize, RelocationError> {
    let target = target.try_into_address()?;
    let stolen_len =
        match decode_function_hook_instructions(target, FUNCTION_HOOK_PATCH_SIZE, false) {
            Ok((_, decoded_len)) => decoded_len,
            Err(error) => return Err(error),
        };

    relocate_function_hook_trampoline(target, dst, stolen_len)
}

/// Installs a function-entry detour at `target` using the MinHook backend.
///
/// This is the most general low-level function hook path and is intended for
/// arbitrary callee-entry detours where users do not want to manage stolen-byte
/// sizing themselves.
pub fn write_function_hook_universal<A: IntoAddress>(target: A, dst: usize) -> usize {
    match try_write_function_hook_universal(target, dst) {
        Ok(address) => address,
        Err(error) => fatal_resolution("Failed to install universal function hook", error),
    }
}

pub fn try_write_function_hook_universal<A: TryIntoAddress>(
    target: A,
    dst: usize,
) -> Result<usize, RelocationError> {
    let target = target.try_into_address()?;
    let original = unsafe { ffi::commonlib_write_function_hook_universal(target, dst) };
    if original == 0 {
        Err(RelocationError::FunctionHookUniversalInstallFailed)
    } else {
        Ok(original)
    }
}

/// Backward-compatible low-level function detour entrypoint.
///
/// `size` is the number of original bytes stolen from the function prologue and
/// relocated into the trampoline returned by this function.
pub fn write_function_hook<A: IntoAddress>(target: A, dst: usize, size: usize) -> usize {
    write_function_hook_explicit(target, dst, size)
}

pub fn try_write_function_hook<A: TryIntoAddress>(
    target: A,
    dst: usize,
    size: usize,
) -> Result<usize, RelocationError> {
    try_write_function_hook_explicit(target, dst, size)
}

pub trait RttiType {
    const RTTI: VariantID;
}

pub unsafe fn skyrim_cast<T: RttiType, U: RttiType>(from: *mut T) -> *mut U {
    if from.is_null() {
        return core::ptr::null_mut();
    }

    type RTDynamicCastFn = extern "C" fn(
        inptr: *mut c_void,
        vf_delta: i32,
        src_type: *const c_void,
        target_type: *const c_void,
        is_reference: i32,
    ) -> *mut c_void;

    let rtdc = match Relocation::<RTDynamicCastFn>::try_new(RelocationID::new(102238, 109689)) {
        Ok(relocation) => relocation,
        Err(_) => fatal_resolution("skyrim_cast", RelocationError::MissingRuntimeDynamicCast),
    };

    let from_rtti = T::RTTI.address() as *const c_void;
    let to_rtti = U::RTTI.address() as *const c_void;
    if from_rtti.is_null() || to_rtti.is_null() {
        return core::ptr::null_mut();
    }

    let rtdc = unsafe { rtdc.get() };
    let result = rtdc(from as *mut c_void, 0, from_rtti, to_rtti, 0);
    result as *mut U
}

#[inline(always)]
pub unsafe fn skyrim_cast_const<T: RttiType, U: RttiType>(from: *const T) -> *const U {
    unsafe { skyrim_cast::<T, U>(from.cast_mut()) }.cast_const()
}

#[inline(always)]
pub unsafe fn skyrim_cast_nonnull<T: RttiType, U: RttiType>(from: *mut T) -> Option<NonNull<U>> {
    NonNull::new(unsafe { skyrim_cast::<T, U>(from) })
}

#[inline(always)]
pub fn skyrim_cast_ref<T: RttiType, U: RttiType>(from: &T) -> Option<&U> {
    unsafe { skyrim_cast_const::<T, U>(from as *const T).as_ref() }
}

#[inline(always)]
pub fn skyrim_cast_mut<T: RttiType, U: RttiType>(from: &mut T) -> Option<&mut U> {
    unsafe { skyrim_cast::<T, U>(from as *mut T).as_mut() }
}

#[inline(always)]
pub fn relocate<T>(se_and_vr: T, ae: T) -> T {
    if runtime::is_ae() { ae } else { se_and_vr }
}

#[inline(always)]
pub fn relocate_vr<T>(se: T, ae: T, vr: T) -> T {
    if runtime::is_vr() {
        vr
    } else if runtime::is_ae() {
        ae
    } else {
        se
    }
}

// ── МАКРОСЫ ДЛЯ ХУКОВ ────────────────────────────────────────────────

#[inline(always)]
pub unsafe fn vtable<T>(this: *const T) -> *const usize {
    debug_assert!(!this.is_null());
    unsafe { *(this as *const *const usize) }
}

#[inline(always)]
pub unsafe fn virtual_function_address<T, I: IntoOffset>(this: *const T, index: I) -> usize {
    let vtable = unsafe { vtable(this) };
    unsafe { *vtable.add(index.into_offset()) }
}

#[inline(always)]
pub unsafe fn virtual_relocation<F, T, I: IntoOffset>(this: *const T, index: I) -> Relocation<F> {
    Relocation::from_address(unsafe { virtual_function_address(this, index) })
}

#[inline(always)]
pub unsafe fn virtual_function<F: Copy, T, I: IntoOffset>(this: *const T, index: I) -> F {
    unsafe { virtual_relocation::<F, _, _>(this, index).get() }
}

#[inline(always)]
pub fn try_vtable_index_from_slot<I: TryIntoOffset>(slot: I) -> Result<usize, RelocationError> {
    let slot = slot.try_into_offset()?;
    let stride = core::mem::size_of::<usize>();
    if slot % stride != 0 {
        Err(RelocationError::InvalidVtableSlot(slot))
    } else {
        Ok(slot / stride)
    }
}

#[inline(always)]
pub fn vtable_index_from_slot<I: IntoOffset>(slot: I) -> usize {
    match try_vtable_index_from_slot(slot.into_offset()) {
        Ok(index) => index,
        Err(error) => fatal_resolution("Failed to use vtable slot", error),
    }
}

#[macro_export]
macro_rules! relocate_virtual {
    ($signature:ty, $receiver:expr, $index:expr $(, $arg:expr)* $(,)?) => {{
        let receiver = $receiver;
        let func = unsafe {
            $crate::relocation::virtual_relocation::<$signature, _, _>(
                receiver as *const _,
                $index,
            )
            .get()
        };
        func(receiver $(, $arg)*)
    }};
}

#[macro_export]
macro_rules! __abi_guard_nontrivial_handle_ty {
    (ActorHandle) => {
        compile_error!(
            "ActorHandle is a non-trivial C++ BSPointerHandle type; do not use it by value in relocation/virtual/hook macro signatures. Use an out-param wrapper or a C++ bridge."
        );
    };
    (ObjectRefHandle) => {
        compile_error!(
            "ObjectRefHandle is a non-trivial C++ BSPointerHandle type; do not use it by value in relocation/virtual/hook macro signatures. Use an out-param wrapper or a C++ bridge."
        );
    };
    (ProjectileHandle) => {
        compile_error!(
            "ProjectileHandle is a non-trivial C++ BSPointerHandle type; do not use it by value in relocation/virtual/hook macro signatures. Use an out-param wrapper or a C++ bridge."
        );
    };
    (BSString) => {
        compile_error!(
            "BSString is a non-trivial C++ owning string type; do not use it by value in relocation/virtual/hook macro signatures. Use an out-param wrapper, pointer/reference ABI, or a C++ bridge."
        );
    };
    (BSFixedString) => {
        compile_error!(
            "BSFixedString is a non-trivial C++ string type; do not use it by value in relocation/virtual/hook macro signatures. Use a borrowed string storage ABI, pointer/reference ABI, or a C++ bridge."
        );
    };
    (BSStringT<$n:tt, $a:ty>) => {
        compile_error!(
            "BSStringT is a non-trivial C++ owning string type; do not use it by value in relocation/virtual/hook macro signatures. Use an out-param wrapper, pointer/reference ABI, or a C++ bridge."
        );
    };
    (BSStaticStringT<$n:tt>) => {
        compile_error!(
            "BSStaticStringT is a non-trivial C++ owning string type; do not use it by value in relocation/virtual/hook macro signatures. Use an out-param wrapper, pointer/reference ABI, or a C++ bridge."
        );
    };
    (NiPointer<$t:ty>) => {
        compile_error!(
            "NiPointer is a non-trivial C++ intrusive smart pointer type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (BSTSmartPointer<$t:ty>) => {
        compile_error!(
            "BSTSmartPointer is a non-trivial C++ intrusive smart pointer type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (BSTSmartPointer<$t:ty, $m:ty>) => {
        compile_error!(
            "BSTSmartPointer is a non-trivial C++ intrusive smart pointer type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (GPtr<$t:ty>) => {
        compile_error!(
            "GPtr is a non-trivial C++ intrusive smart pointer type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (hkRefPtr<$t:ty>) => {
        compile_error!(
            "hkRefPtr is a non-trivial C++ intrusive smart pointer type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (BSTArray<$t:ty, $a:ty>) => {
        compile_error!(
            "BSTArray is a non-trivial C++ owning container type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI, an explicit out-param wrapper, or a C++ bridge."
        );
    };
    (BSScrapArray<$t:ty>) => {
        compile_error!(
            "BSScrapArray is a non-trivial C++ owning container type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI, an explicit out-param wrapper, or a C++ bridge."
        );
    };
    (BSTSmallArray<$t:ty, $n:tt>) => {
        compile_error!(
            "BSTSmallArray is a non-trivial C++ owning container type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI, an explicit out-param wrapper, or a C++ bridge."
        );
    };
    (BSStaticArray<$t:ty>) => {
        compile_error!(
            "BSStaticArray is a non-trivial C++ container/view type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (BSTSmallSharedArray<$t:ty>) => {
        compile_error!(
            "BSTSmallSharedArray is a non-trivial C++ container type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI, an explicit out-param wrapper, or a C++ bridge."
        );
    };
    (BSTScatterTable<$t:ty, $a:ty, $p:ty>) => {
        compile_error!(
            "BSTScatterTable is a non-trivial C++ owning container type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI, an explicit out-param wrapper, or a C++ bridge."
        );
    };
    (BSTHashMap<$k:ty, $v:ty>) => {
        compile_error!(
            "BSTHashMap is a non-trivial C++ owning container type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI, an explicit out-param wrapper, or a C++ bridge."
        );
    };
    (BSTSet<$k:ty>) => {
        compile_error!(
            "BSTSet is a non-trivial C++ owning container type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI, an explicit out-param wrapper, or a C++ bridge."
        );
    };
    (BSTFixedHashMap<$k:ty, $v:ty>) => {
        compile_error!(
            "BSTFixedHashMap is a non-trivial C++ owning container type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI, an explicit out-param wrapper, or a C++ bridge."
        );
    };
    (BSTScrapHashMap<$k:ty, $v:ty>) => {
        compile_error!(
            "BSTScrapHashMap is a non-trivial C++ owning container type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI, an explicit out-param wrapper, or a C++ bridge."
        );
    };
    (BSTStaticHashMap<$k:ty, $v:ty, $n:tt, $buf:tt>) => {
        compile_error!(
            "BSTStaticHashMap is a non-trivial C++ owning container type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI, an explicit out-param wrapper, or a C++ bridge."
        );
    };
    (BSSimpleList<$t:ty>) => {
        compile_error!(
            "BSSimpleList is a non-trivial C++ intrusive container type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (NiTMap<$k:ty, $v:ty>) => {
        compile_error!(
            "NiTMap is a non-trivial C++ owning map type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (NiTMapBase<$a:ty, $k:ty, $v:ty>) => {
        compile_error!(
            "NiTMapBase is a non-trivial C++ owning map base type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (NiTStringTemplateMap<$p:ty, $t:ty>) => {
        compile_error!(
            "NiTStringTemplateMap is a non-trivial C++ owning string-map type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (NiTStringMap<$t:ty>) => {
        compile_error!(
            "NiTStringMap is a non-trivial C++ owning string-map type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (BSTTuple<$t1:ty, $t2:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!($t1);
        $crate::__abi_guard_nontrivial_handle_ty!($t2);
    };
    (BSTArrayHeapAllocator) => {
        compile_error!(
            "BSTArrayHeapAllocator is a non-trivial C++ allocator type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (BSTSmallArrayHeapAllocator<$n:tt>) => {
        compile_error!(
            "BSTSmallArrayHeapAllocator is a non-trivial C++ allocator type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (BSScrapArrayAllocator) => {
        compile_error!(
            "BSScrapArrayAllocator is a non-trivial C++ allocator type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (BSTScatterTableHeapAllocator) => {
        compile_error!(
            "BSTScatterTableHeapAllocator is a non-trivial C++ allocator type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (BSTScatterTableScrapAllocator) => {
        compile_error!(
            "BSTScatterTableScrapAllocator is a non-trivial C++ allocator type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (BSTStaticHashMapAllocator<$n:tt, $buf:tt>) => {
        compile_error!(
            "BSTStaticHashMapAllocator is a non-trivial C++ allocator type; do not use it by value in relocation/virtual/hook macro signatures. Use pointer/reference ABI or a C++ bridge."
        );
    };
    (crate::re::ActorHandle) => {
        $crate::__abi_guard_nontrivial_handle_ty!(ActorHandle);
    };
    (crate::re::ObjectRefHandle) => {
        $crate::__abi_guard_nontrivial_handle_ty!(ObjectRefHandle);
    };
    (crate::re::ProjectileHandle) => {
        $crate::__abi_guard_nontrivial_handle_ty!(ProjectileHandle);
    };
    (crate::re::bs_pointer_handle::ActorHandle) => {
        $crate::__abi_guard_nontrivial_handle_ty!(ActorHandle);
    };
    (crate::re::bs_pointer_handle::ObjectRefHandle) => {
        $crate::__abi_guard_nontrivial_handle_ty!(ObjectRefHandle);
    };
    (crate::re::bs_pointer_handle::ProjectileHandle) => {
        $crate::__abi_guard_nontrivial_handle_ty!(ProjectileHandle);
    };
    (crate::re::BSString) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSString);
    };
    (crate::re::BSFixedString) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSFixedString);
    };
    (crate::re::BSStringT<$n:tt, $a:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSStringT<$n, $a>);
    };
    (crate::re::BSStaticStringT<$n:tt>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSStaticStringT<$n>);
    };
    (crate::re::bs_string::BSString) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSString);
    };
    (crate::re::bs_fixed_string::BSFixedString) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSFixedString);
    };
    (crate::re::bs_string::BSStringT<$n:tt, $a:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSStringT<$n, $a>);
    };
    (crate::re::bs_string::BSStaticStringT<$n:tt>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSStaticStringT<$n>);
    };
    (crate::re::BSTArray<$t:ty, $a:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTArray<$t, $a>);
    };
    (crate::re::BSScrapArray<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSScrapArray<$t>);
    };
    (crate::re::BSTSmallArray<$t:ty, $n:tt>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSmallArray<$t, $n>);
    };
    (crate::re::BSStaticArray<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSStaticArray<$t>);
    };
    (crate::re::BSTSmallSharedArray<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSmallSharedArray<$t>);
    };
    (crate::re::BSTScatterTable<$t:ty, $a:ty, $p:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTScatterTable<$t, $a, $p>);
    };
    (crate::re::BSTHashMap<$k:ty, $v:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTHashMap<$k, $v>);
    };
    (crate::re::BSTSet<$k:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSet<$k>);
    };
    (crate::re::BSTFixedHashMap<$k:ty, $v:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTFixedHashMap<$k, $v>);
    };
    (crate::re::BSTScrapHashMap<$k:ty, $v:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTScrapHashMap<$k, $v>);
    };
    (crate::re::BSTStaticHashMap<$k:ty, $v:ty, $n:tt, $buf:tt>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTStaticHashMap<$k, $v, $n, $buf>);
    };
    (crate::re::BSSimpleList<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSSimpleList<$t>);
    };
    (crate::re::BSTTuple<$t1:ty, $t2:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTTuple<$t1, $t2>);
    };
    (crate::re::NiTMap<$k:ty, $v:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiTMap<$k, $v>);
    };
    (crate::re::NiTMapBase<$a:ty, $k:ty, $v:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiTMapBase<$a, $k, $v>);
    };
    (crate::re::NiTStringTemplateMap<$p:ty, $t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiTStringTemplateMap<$p, $t>);
    };
    (crate::re::NiTStringMap<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiTStringMap<$t>);
    };
    (crate::re::bst_array::BSTArray<$t:ty, $a:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTArray<$t, $a>);
    };
    (crate::re::bst_array::BSScrapArray<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSScrapArray<$t>);
    };
    (crate::re::bst_array::BSTSmallArray<$t:ty, $n:tt>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSmallArray<$t, $n>);
    };
    (crate::re::bst_array::BSStaticArray<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSStaticArray<$t>);
    };
    (crate::re::bst_array::BSTSmallSharedArray<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSmallSharedArray<$t>);
    };
    (crate::re::bssimple_list::BSSimpleList<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSSimpleList<$t>);
    };
    (crate::re::bst_tuple::BSTTuple<$t1:ty, $t2:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTTuple<$t1, $t2>);
    };
    (crate::re::NiPointer<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiPointer<$t>);
    };
    (crate::re::BSTSmartPointer<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSmartPointer<$t>);
    };
    (crate::re::BSTSmartPointer<$t:ty, $m:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSmartPointer<$t, $m>);
    };
    (crate::re::GPtr<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(GPtr<$t>);
    };
    (crate::re::hkRefPtr<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(hkRefPtr<$t>);
    };
    (crate::re::ni_smart_pointer::NiPointer<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiPointer<$t>);
    };
    (crate::re::ActorHandlePtr) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiPointer<crate::re::Actor>);
    };
    (crate::re::actor::ActorHandlePtr) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiPointer<crate::re::Actor>);
    };
    (crate::re::bst_smart_pointer::BSTSmartPointer<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSmartPointer<$t>);
    };
    (crate::re::bst_smart_pointer::BSTSmartPointer<$t:ty, $m:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSmartPointer<$t, $m>);
    };
    (crate::re::g_ptr::GPtr<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(GPtr<$t>);
    };
    (crate::re::hk_ref_ptr::hkRefPtr<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(hkRefPtr<$t>);
    };
    (crate::re::BSAnimationGraphManagerPtr) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSmartPointer<crate::re::BSAnimationGraphManager>);
    };
    (crate::re::bs_animation_graph_manager::BSAnimationGraphManagerPtr) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSmartPointer<crate::re::BSAnimationGraphManager>);
    };
    (crate::re::BSPathingRequestArrayRefCounted) => {
        $crate::__abi_guard_nontrivial_handle_ty!(
            BSTTuple<crate::re::BSTArray<crate::re::BSPathingAvoidNode>, u32>
        );
    };
    (crate::re::bs_pathing_request::BSPathingRequestArrayRefCounted) => {
        $crate::__abi_guard_nontrivial_handle_ty!(
            BSTTuple<crate::re::BSTArray<crate::re::BSPathingAvoidNode>, u32>
        );
    };
    (crate::re::ni_t_map::NiTMap<$k:ty, $v:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiTMap<$k, $v>);
    };
    (crate::re::ni_t_map::NiTMapSetting) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiTMap<*const core::ffi::c_char, *mut crate::re::Setting>);
    };
    (crate::re::ni_t_map_base::NiTMapBase<$a:ty, $k:ty, $v:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiTMapBase<$a, $k, $v>);
    };
    (crate::re::ni_t_map_base::DefaultNiTMapBase<$k:ty, $v:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(
            NiTMapBase<crate::re::ni_t_default_allocator::NiTDefaultAllocator<crate::re::ni_t_map_base::NiTMapItem<$k, $v>>, $k, $v>
        );
    };
    (crate::re::ni_t_map_base::NiTMapBaseSetting) => {
        $crate::__abi_guard_nontrivial_handle_ty!(
            NiTMapBase<crate::re::ni_t_default_allocator::NiTDefaultAllocator<crate::re::ni_t_map_base::NiTMapItem<*const core::ffi::c_char, *mut crate::re::Setting>>, *const core::ffi::c_char, *mut crate::re::Setting>
        );
    };
    (crate::re::ni_t_string_map::NiTStringTemplateMap<$p:ty, $t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiTStringTemplateMap<$p, $t>);
    };
    (crate::re::ni_t_string_map::NiTStringMap<$t:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiTStringMap<$t>);
    };
    (crate::re::ni_t_string_map::NiTStringTemplateMapSetting) => {
        $crate::__abi_guard_nontrivial_handle_ty!(
            NiTStringTemplateMap<crate::re::NiTMap<*const core::ffi::c_char, *mut crate::re::Setting>, *mut crate::re::Setting>
        );
    };
    (crate::re::ni_t_string_map::NiTStringMapSetting) => {
        $crate::__abi_guard_nontrivial_handle_ty!(NiTStringMap<*mut crate::re::Setting>);
    };
    (crate::re::bst_array::BSTArrayHeapAllocator) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTArrayHeapAllocator);
    };
    (crate::re::bst_array::BSTSmallArrayHeapAllocator<$n:tt>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSmallArrayHeapAllocator<$n>);
    };
    (crate::re::bst_array::BSScrapArrayAllocator) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSScrapArrayAllocator);
    };
    (crate::re::bst_hash_map::BSTScatterTable<$t:ty, $a:ty, $p:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTScatterTable<$t, $a, $p>);
    };
    (crate::re::bst_hash_map::BSTHashMap<$k:ty, $v:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTHashMap<$k, $v>);
    };
    (crate::re::bst_hash_map::BSTSet<$k:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTSet<$k>);
    };
    (crate::re::bst_hash_map::BSTFixedHashMap<$k:ty, $v:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTFixedHashMap<$k, $v>);
    };
    (crate::re::bst_hash_map::BSTScrapHashMap<$k:ty, $v:ty>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTScrapHashMap<$k, $v>);
    };
    (crate::re::bst_hash_map::BSTStaticHashMap<$k:ty, $v:ty, $n:tt, $buf:tt>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTStaticHashMap<$k, $v, $n, $buf>);
    };
    (crate::re::bst_hash_map::BSTScatterTableHeapAllocator) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTScatterTableHeapAllocator);
    };
    (crate::re::bst_hash_map::BSTScatterTableScrapAllocator) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTScatterTableScrapAllocator);
    };
    (crate::re::bst_hash_map::BSTStaticHashMapAllocator<$n:tt, $buf:tt>) => {
        $crate::__abi_guard_nontrivial_handle_ty!(BSTStaticHashMapAllocator<$n, $buf>);
    };
    ($other:ty) => {};
}

#[macro_export]
macro_rules! __abi_guard_nontrivial_handle_params {
    () => {};
    ($head:ty $(, $tail:ty)*) => {
        $crate::__abi_guard_nontrivial_handle_ty!($head);
        $crate::__abi_guard_nontrivial_handle_params!($($tail),*);
    };
}

#[macro_export]
macro_rules! __abi_guard_nontrivial_handle_return {
    () => {};
    ($ret:ty) => {
        $crate::__abi_guard_nontrivial_handle_ty!($ret);
    };
}

#[macro_export]
macro_rules! define_vtable_hook {
    (
        $vis:vis $hook_name:ident {
            vtable: $vtable:expr,
            offset: $offset:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),*) $(-> $ret:ty)? $body:block
        }
    ) => {
        #[allow(non_snake_case)]
        $vis mod $hook_name {
            #[allow(unused_imports)]
            use super::*;
            const _: () = {
                $crate::__abi_guard_nontrivial_handle_params!($($arg_type),*);
                $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            };
            type Signature = extern "C" fn($($arg_name: $arg_type),*) $(-> $ret)?;
            type Original = $crate::relocation::Relocation<Signature>;
            static ORIGINAL: $crate::core_util::Later<Original> = $crate::core_util::Later::new();

            #[inline(always)]
            pub fn original_relocation() -> Original { *ORIGINAL }

            #[inline(always)]
            pub fn original($($arg_name: $arg_type),*) $(-> $ret)? {
                let original = unsafe { original_relocation().get() };
                original($($arg_name),*)
            }

            extern "C" fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? { $body }

            pub fn install() {
                use $crate::relocation::{IntoAddress, IntoOffset};

                let orig_addr = $crate::relocation::write_vfunc(
                    $vtable.into_address(),
                    $offset.into_offset(),
                    $hook_func as *const () as usize,
                );
                ORIGINAL.init($crate::relocation::Relocation::from_address(orig_addr));
            }
        }
    };
}

#[macro_export]
macro_rules! define_call_hook {
    (
        $vis:vis $hook_name:ident {
            address: $address:expr,
            offset: $offset:expr,
            size: $size:literal,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),*) $(-> $ret:ty)? $body:block
        }
    ) => {
        #[allow(non_snake_case)]
        $vis mod $hook_name {
            #[allow(unused_imports)]
            use super::*;
            const _: () = {
                $crate::__abi_guard_nontrivial_handle_params!($($arg_type),*);
                $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            };
            type Signature = extern "C" fn($($arg_name: $arg_type),*) $(-> $ret)?;
            type Original = $crate::relocation::Relocation<Signature>;
            static ORIGINAL: $crate::core_util::Later<Original> = $crate::core_util::Later::new();

            #[inline(always)]
            pub fn original_relocation() -> Original { *ORIGINAL }

            #[inline(always)]
            pub fn original($($arg_name: $arg_type),*) $(-> $ret)? {
                let original = unsafe { original_relocation().get() };
                original($($arg_name),*)
            }

            extern "C" fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? { $body }

            pub fn install() {
                use $crate::relocation::{IntoAddress, IntoOffset};

                let target_addr = $address.into_address() + $offset.into_offset();
                let orig_addr = $crate::relocation::write_call(
                    target_addr,
                    $hook_func as *const () as usize,
                    $size,
                );
                ORIGINAL.init($crate::relocation::Relocation::from_address(orig_addr));
            }
        }
    };
}

#[macro_export]
macro_rules! define_vcall_hook {
    (
        $vis:vis $hook_name:ident {
            address: $address:expr,
            offset: $offset:expr,
            size: $size:literal,
            receiver: $receiver:ident,
            index: $index:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        #[allow(non_snake_case)]
        $vis mod $hook_name {
            #[allow(unused_imports)]
            use super::*;
            const _: () = {
                $crate::__abi_guard_nontrivial_handle_params!($($arg_type),*);
                $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            };
            type Signature = extern "C" fn($($arg_name: $arg_type),*) $(-> $ret)?;

            #[inline(always)]
            pub fn callsite_relocation() -> $crate::relocation::Relocation<()> {
                use $crate::relocation::{IntoAddress, IntoOffset};
                $crate::relocation::Relocation::from_address(
                    $address.into_address() + $offset.into_offset(),
                )
            }

            #[inline(always)]
            pub fn vtable_index() -> usize {
                use $crate::relocation::IntoOffset;
                $index.into_offset()
            }

            #[inline(always)]
            pub fn original_virtual_relocation<T>(
                receiver: *const T,
            ) -> $crate::relocation::Relocation<Signature> {
                unsafe {
                    $crate::relocation::virtual_relocation::<Signature, _, _>(
                        receiver,
                        vtable_index(),
                    )
                }
            }

            #[inline(always)]
            pub fn original($($arg_name: $arg_type),*) $(-> $ret)? {
                let original = unsafe { original_virtual_relocation($receiver as *const _).get() };
                original($($arg_name),*)
            }

            extern "C" fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? { $body }

            pub fn install() {
                use $crate::relocation::{IntoAddress, IntoOffset};

                let target_addr = $address.into_address() + $offset.into_offset();
                let _ = $crate::relocation::write_call(
                    target_addr,
                    $hook_func as *const () as usize,
                    $size,
                );
            }
        }
    };
}

#[macro_export]
macro_rules! define_function_hook {
    (
        $vis:vis $hook_name:ident {
            target: $target:expr,
            size: $size:literal,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        #[allow(non_snake_case)]
        $vis mod $hook_name {
            #[allow(unused_imports)]
            use super::*;
            const _: () = {
                $crate::__abi_guard_nontrivial_handle_params!($($arg_type),*);
                $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            };
            type Signature = extern "C" fn($($arg_name: $arg_type),*) $(-> $ret)?;
            type Original = $crate::relocation::Relocation<Signature>;
            static ORIGINAL: $crate::core_util::Later<Original> = $crate::core_util::Later::new();

            #[inline(always)]
            pub fn original_relocation() -> Original { *ORIGINAL }

            #[inline(always)]
            pub fn original($($arg_name: $arg_type),*) $(-> $ret)? {
                let original = unsafe { original_relocation().get() };
                original($($arg_name),*)
            }

            extern "C" fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? { $body }

            pub fn install() {
                use $crate::relocation::IntoAddress;

                let orig_addr = $crate::relocation::write_function_hook_explicit(
                    $target.into_address(),
                    $hook_func as *const () as usize,
                    $size,
                );
                ORIGINAL.init($crate::relocation::Relocation::from_address(orig_addr));
            }
        }
    };
}

#[macro_export]
macro_rules! define_auto_function_hook {
    (
        $vis:vis $hook_name:ident {
            target: $target:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        #[allow(non_snake_case)]
        $vis mod $hook_name {
            #[allow(unused_imports)]
            use super::*;
            const _: () = {
                $crate::__abi_guard_nontrivial_handle_params!($($arg_type),*);
                $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            };
            type Signature = extern "C" fn($($arg_name: $arg_type),*) $(-> $ret)?;
            type Original = $crate::relocation::Relocation<Signature>;
            static ORIGINAL: $crate::core_util::Later<Original> = $crate::core_util::Later::new();

            #[inline(always)]
            pub fn original_relocation() -> Original { *ORIGINAL }

            #[inline(always)]
            pub fn original($($arg_name: $arg_type),*) $(-> $ret)? {
                let original = unsafe { original_relocation().get() };
                original($($arg_name),*)
            }

            extern "C" fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? { $body }

            pub fn install() {
                use $crate::relocation::IntoAddress;

                let orig_addr = $crate::relocation::write_function_hook_auto(
                    $target.into_address(),
                    $hook_func as *const () as usize,
                );
                ORIGINAL.init($crate::relocation::Relocation::from_address(orig_addr));
            }
        }
    };
}

#[macro_export]
macro_rules! define_universal_function_hook {
    (
        $vis:vis $hook_name:ident {
            target: $target:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        #[allow(non_snake_case)]
        $vis mod $hook_name {
            #[allow(unused_imports)]
            use super::*;
            const _: () = {
                $crate::__abi_guard_nontrivial_handle_params!($($arg_type),*);
                $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            };
            type Signature = extern "C" fn($($arg_name: $arg_type),*) $(-> $ret)?;
            type Original = $crate::relocation::Relocation<Signature>;
            static ORIGINAL: $crate::core_util::Later<Original> = $crate::core_util::Later::new();

            #[inline(always)]
            pub fn original_relocation() -> Original { *ORIGINAL }

            #[inline(always)]
            pub fn original($($arg_name: $arg_type),*) $(-> $ret)? {
                let original = unsafe { original_relocation().get() };
                original($($arg_name),*)
            }

            extern "C" fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? { $body }

            pub fn install() {
                use $crate::relocation::IntoAddress;

                let orig_addr = $crate::relocation::write_function_hook_universal(
                    $target.into_address(),
                    $hook_func as *const () as usize,
                );
                ORIGINAL.init($crate::relocation::Relocation::from_address(orig_addr));
            }
        }
    };
}

#[macro_export]
macro_rules! hook {
    (
        $vis:vis function $hook_name:ident {
            target: $target:expr,
            size: $size:literal,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::define_function_hook! {
            $vis $hook_name {
                target: $target,
                size: $size,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis function $hook_name:ident {
            target: $target:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::define_universal_function_hook! {
            $vis $hook_name {
                target: $target,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis function $hook_name:ident {
            address: $target:expr,
            size: $size:literal,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::hook! {
            $vis function $hook_name {
                target: $target,
                size: $size,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis function $hook_name:ident {
            address: $target:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::hook! {
            $vis function $hook_name {
                target: $target,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis detour $hook_name:ident {
            target: $target:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::define_auto_function_hook! {
            $vis $hook_name {
                target: $target,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis detour $hook_name:ident {
            address: $target:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::hook! {
            $vis detour $hook_name {
                target: $target,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis universal $hook_name:ident {
            target: $target:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::define_universal_function_hook! {
            $vis $hook_name {
                target: $target,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis universal $hook_name:ident {
            address: $target:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::hook! {
            $vis universal $hook_name {
                target: $target,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis call $hook_name:ident {
            target: $address:expr,
            offset: $offset:expr,
            size: $size:literal,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::define_call_hook! {
            $vis $hook_name {
                address: $address,
                offset: $offset,
                size: $size,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis call $hook_name:ident {
            address: $address:expr,
            offset: $offset:expr,
            size: $size:literal,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::hook! {
            $vis call $hook_name {
                target: $address,
                offset: $offset,
                size: $size,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis vcall $hook_name:ident {
            target: $address:expr,
            offset: $offset:expr,
            size: $size:literal,
            receiver: $receiver:ident,
            index: $index:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::define_vcall_hook! {
            $vis $hook_name {
                address: $address,
                offset: $offset,
                size: $size,
                receiver: $receiver,
                index: $index,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis vcall $hook_name:ident {
            address: $address:expr,
            offset: $offset:expr,
            size: $size:literal,
            receiver: $receiver:ident,
            index: $index:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::hook! {
            $vis vcall $hook_name {
                target: $address,
                offset: $offset,
                size: $size,
                receiver: $receiver,
                index: $index,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis vcall $hook_name:ident {
            target: $address:expr,
            offset: $offset:expr,
            size: $size:literal,
            receiver: $receiver:ident,
            slot: $slot:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::hook! {
            $vis vcall $hook_name {
                target: $address,
                offset: $offset,
                size: $size,
                receiver: $receiver,
                index: $crate::relocation::vtable_index_from_slot($slot),
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis vcall $hook_name:ident {
            address: $address:expr,
            offset: $offset:expr,
            size: $size:literal,
            receiver: $receiver:ident,
            slot: $slot:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::hook! {
            $vis vcall $hook_name {
                target: $address,
                offset: $offset,
                size: $size,
                receiver: $receiver,
                slot: $slot,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis vtable $hook_name:ident {
            vtable: $vtable:expr,
            index: $index:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::define_vtable_hook! {
            $vis $hook_name {
                vtable: $vtable,
                offset: $index,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };

    (
        $vis:vis vtable $hook_name:ident {
            vtable: $vtable:expr,
            offset: $offset:expr,
            fn $hook_func:ident($($arg_name:ident: $arg_type:ty),* $(,)?) $(-> $ret:ty)? $body:block
        }
    ) => {
        $crate::hook! {
            $vis vtable $hook_name {
                vtable: $vtable,
                index: $offset,
                fn $hook_func($($arg_name: $arg_type),*) $(-> $ret)? $body
            }
        }
    };
}

#[macro_export]
macro_rules! virtual_method {
    (
        $const_vis:vis const $const_name:ident: $const_ty:ty = $const_expr:expr;
        $fn_vis:vis fn $func_name:ident(&self $(, $arg_name:ident: $arg_ty:ty)*) $(-> $ret:ty)?
    ) => {
        $const_vis const $const_name: $const_ty = $const_expr;

        #[inline(always)]
        $fn_vis fn $func_name(&self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::__abi_guard_nontrivial_handle_params!($($arg_ty),*);
            $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            let func: extern "C" fn(*const Self $(, $arg_ty)*) $(-> $ret)? = unsafe {
                $crate::relocation::virtual_function(self as *const Self, Self::$const_name)
            };
            func(self $(, $arg_name)*)
        }
    };

    (
        $const_vis:vis const $const_name:ident: $const_ty:ty = $const_expr:expr;
        $fn_vis:vis fn $func_name:ident(&mut self $(, $arg_name:ident: $arg_ty:ty)*) $(-> $ret:ty)?
    ) => {
        $const_vis const $const_name: $const_ty = $const_expr;

        #[inline(always)]
        $fn_vis fn $func_name(&mut self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::__abi_guard_nontrivial_handle_params!($($arg_ty),*);
            $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            let func: extern "C" fn(*mut Self $(, $arg_ty)*) $(-> $ret)? = unsafe {
                $crate::relocation::virtual_function(self as *mut Self, Self::$const_name)
            };
            func(self $(, $arg_name)*)
        }
    };

    (
        $const_vis:vis const $const_name:ident: $const_ty:ty = $const_expr:expr;
        $fn_vis:vis fn $func_name:ident($($arg_name:ident: $arg_ty:ty),*) $(-> $ret:ty)?
    ) => {
        $const_vis const $const_name: $const_ty = $const_expr;

        #[inline(always)]
        $fn_vis fn $func_name(&self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::__abi_guard_nontrivial_handle_params!($($arg_ty),*);
            $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            let func: extern "C" fn(*const Self $(, $arg_ty)*) $(-> $ret)? = unsafe {
                $crate::relocation::virtual_function(self as *const Self, Self::$const_name)
            };
            func(self $(, $arg_name)*)
        }
    };
}

#[macro_export]
macro_rules! relocated_virtual_method {
    (
        $const_vis:vis const $const_name:ident: $const_ty:ty = $const_expr:expr;
        $fn_vis:vis fn $func_name:ident(&self $(, $arg_name:ident: $arg_ty:ty)*) $(-> $ret:ty)?
    ) => {
        $const_vis const $const_name: $const_ty = $const_expr;

        #[inline(always)]
        $fn_vis fn $func_name(&self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::__abi_guard_nontrivial_handle_params!($($arg_ty),*);
            $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            $crate::relocate_virtual!(
                extern "C" fn(*const Self $(, $arg_ty)*) $(-> $ret)?,
                self as *const Self,
                Self::$const_name
                $(, $arg_name)*
            )
        }
    };

    (
        $const_vis:vis const $const_name:ident: $const_ty:ty = $const_expr:expr;
        $fn_vis:vis fn $func_name:ident(&mut self $(, $arg_name:ident: $arg_ty:ty)*) $(-> $ret:ty)?
    ) => {
        $const_vis const $const_name: $const_ty = $const_expr;

        #[inline(always)]
        $fn_vis fn $func_name(&mut self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::__abi_guard_nontrivial_handle_params!($($arg_ty),*);
            $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            $crate::relocate_virtual!(
                extern "C" fn(*mut Self $(, $arg_ty)*) $(-> $ret)?,
                self as *mut Self,
                Self::$const_name
                $(, $arg_name)*
            )
        }
    };
}

#[macro_export]
macro_rules! relocation_variable {
    (@no_inline $vis:vis fn $name:ident() -> &'static $ty:ty => $id:expr ) => {
        $vis fn $name() -> &'static $ty {
            $crate::relocation_variable!(@body $ty, $id)
        }
    };

    ( $vis:vis fn $name:ident() -> &'static $ty:ty => $id:expr ) => {
        #[inline]
        $vis fn $name() -> &'static $ty {
            $crate::relocation_variable!(@body $ty, $id)
        }
    };

    (@no_inline $vis:vis fn $name:ident() -> &'static mut $ty:ty => $id:expr ) => {
        $vis fn $name() -> &'static mut $ty {
            $crate::relocation_variable!(@body_mut $ty, $id)
        }
    };

    ( $vis:vis fn $name:ident() -> &'static mut $ty:ty => $id:expr ) => {
        #[inline]
        $vis fn $name() -> &'static mut $ty {
            $crate::relocation_variable!(@body_mut $ty, $id)
        }
    };

    ( @no_inline $vis:vis fn $name:ident() -> *mut $ty:ty => $id:expr, is_ptr ) => {
        $vis fn $name() -> *mut $ty {
            $crate::relocation_variable!(@body_ptr $ty, $id)
        }
    };

    ( $vis:vis fn $name:ident() -> *mut $ty:ty => $id:expr, is_ptr ) => {
        #[inline]
        $vis fn $name() -> *mut $ty {
            $crate::relocation_variable!(@body_ptr $ty, $id)
        }
    };

    ( @no_inline $vis:vis fn $name:ident() -> *mut $ty:ty => $id:expr, is_indirect_ptr ) => {
        $vis fn $name() -> *mut $ty {
            $crate::relocation_variable!(@body_indirect_ptr $ty, $id)
        }
    };

    ( $vis:vis fn $name:ident() -> *mut $ty:ty => $id:expr, is_indirect_ptr ) => {
        #[inline]
        $vis fn $name() -> *mut $ty {
            $crate::relocation_variable!(@body_indirect_ptr $ty, $id)
        }
    };

    (@body $ty:ty, $id:expr) => {{
        let relocation = $crate::relocation::Relocation::<$ty>::new($id);
        unsafe { &*relocation.as_ptr() }
    }};

    (@body_mut $ty:ty, $id:expr) => {{
        let relocation = $crate::relocation::Relocation::<$ty>::new($id);
        unsafe { &mut *(relocation.as_ptr() as *mut $ty) }
    }};

    (@body_ptr $ty:ty, $id:expr) => {{
        unsafe { $crate::relocation::Relocation::<*mut $ty>::new($id).get() }
    }};

    (@body_indirect_ptr $ty:ty, $id:expr) => {{
        unsafe { $crate::relocation::read_indirect_ptr::<$ty, _>($id) }
    }};
}

#[macro_export]
macro_rules! relocation_func {
    ( $(#[$attr:meta])* @no_inline $vis:vis fn $name:ident($($arg_name:ident: $arg_ty:ty),* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        $(#[$attr])*
        $vis fn $name($($arg_name: $arg_ty),*) $(-> $ret)? {
            $crate::__abi_guard_nontrivial_handle_params!($($arg_ty),*);
            $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            $crate::relocation_func!(@body ($($arg_ty),*), ($($arg_name),*), $(-> $ret)?, $id)
        }
    };

    ( $(#[$attr:meta])* $vis:vis fn $name:ident($($arg_name:ident: $arg_ty:ty),* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        #[inline]
        $(#[$attr])*
        $vis fn $name($($arg_name: $arg_ty),*) $(-> $ret)? {
            $crate::__abi_guard_nontrivial_handle_params!($($arg_ty),*);
            $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            $crate::relocation_func!(@body ($($arg_ty),*), ($($arg_name),*), $(-> $ret)?, $id)
        }
    };

    ( $(#[$attr:meta])* @no_inline $vis:vis fn $name:ident(&self $(, $arg_name:ident: $arg_ty:ty)* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        $(#[$attr])*
        $vis fn $name(&self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::__abi_guard_nontrivial_handle_params!($($arg_ty),*);
            $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            $crate::relocation_func!(@body (*const Self $(, $arg_ty)*), (self as *const Self $(, $arg_name)*), $(-> $ret)?, $id)
        }
    };

    ( $(#[$attr:meta])* $vis:vis fn $name:ident(&self $(, $arg_name:ident: $arg_ty:ty)* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        #[inline]
        $(#[$attr])*
        $vis fn $name(&self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::__abi_guard_nontrivial_handle_params!($($arg_ty),*);
            $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            $crate::relocation_func!(@body (*const Self $(, $arg_ty)*), (self as *const Self $(, $arg_name)*), $(-> $ret)?, $id)
        }
    };

    ( $(#[$attr:meta])* @no_inline $vis:vis fn $name:ident(&mut self $(, $arg_name:ident: $arg_ty:ty)* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        $(#[$attr])*
        $vis fn $name(&mut self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::__abi_guard_nontrivial_handle_params!($($arg_ty),*);
            $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            $crate::relocation_func!(@body (*mut Self $(, $arg_ty)*), (self as *mut Self $(, $arg_name)*), $(-> $ret)?, $id)
        }
    };

    ( $(#[$attr:meta])* $vis:vis fn $name:ident(&mut self $(, $arg_name:ident: $arg_ty:ty)* $(,)?) $(-> $ret:ty)? => $id:expr ) => {
        #[inline]
        $(#[$attr])*
        $vis fn $name(&mut self $(, $arg_name: $arg_ty)*) $(-> $ret)? {
            $crate::__abi_guard_nontrivial_handle_params!($($arg_ty),*);
            $crate::__abi_guard_nontrivial_handle_return!($($ret)?);
            $crate::relocation_func!(@body (*mut Self $(, $arg_ty)*), (self as *mut Self $(, $arg_name)*), $(-> $ret)?, $id)
        }
    };

    (@body ($($arg_types:ty),*), ($($arg_names:expr),*), $(-> $ret:ty)?, $id:expr) => {{
        let func = unsafe {
            $crate::relocation::Relocation::<unsafe extern "C-unwind" fn($($arg_types),*) $(-> $ret)?>
                ::new($id)
                .get()
        };
        unsafe { func($($arg_names),*) }
    }};
}

#[cfg(test)]
mod tests {
    use super::{Relocation, read_indirect_ptr};
    use core::marker::PhantomData;

    struct GenericVirtual<T> {
        _vtable: *const usize,
        _marker: PhantomData<T>,
    }

    impl<T> GenericVirtual<T> {
        crate::virtual_method! {
            pub const VFUNC_VIRTUAL_CONST_ECHO: usize = 0x00;
            pub fn virtual_const_echo(&self, value: Option<T>) -> Option<T>
        }

        crate::virtual_method! {
            pub const VFUNC_VIRTUAL_ECHO: usize = 0x00;
            pub fn virtual_echo(value: Option<T>) -> Option<T>
        }

        crate::relocated_virtual_method! {
            pub const VFUNC_RELOCATED_ECHO: usize = 0x00;
            pub fn relocated_echo(&self, value: Option<T>) -> Option<T>
        }

        crate::relocated_virtual_method! {
            pub const VFUNC_RELOCATED_ECHO_MUT: usize = 0x00;
            pub fn relocated_echo_mut(&mut self, value: Result<T, T>) -> Result<T, T>
        }
    }

    #[test]
    fn virtual_macros_support_generic_impl_types() {
        let _ = GenericVirtual::<u32>::VFUNC_VIRTUAL_CONST_ECHO;
        let _ = GenericVirtual::<u32>::VFUNC_VIRTUAL_ECHO;
        let _ = GenericVirtual::<u32>::VFUNC_RELOCATED_ECHO;
        let _ = GenericVirtual::<u32>::VFUNC_RELOCATED_ECHO_MUT;
        let _ = GenericVirtual::<u32>::virtual_const_echo
            as fn(&GenericVirtual<u32>, Option<u32>) -> Option<u32>;
        let _ = GenericVirtual::<u32>::virtual_echo
            as fn(&GenericVirtual<u32>, Option<u32>) -> Option<u32>;
        let _ = GenericVirtual::<u32>::relocated_echo
            as fn(&GenericVirtual<u32>, Option<u32>) -> Option<u32>;
        let _ = GenericVirtual::<u32>::relocated_echo_mut
            as fn(&mut GenericVirtual<u32>, Result<u32, u32>) -> Result<u32, u32>;
    }

    crate::hook! {
        pub function TestFunctionHook {
            target: 0usize,
            size: 5,
            fn detour(value: u32) -> u32 {
                original(value)
            }
        }
    }

    crate::hook! {
        pub function TestUniversalFunctionHook {
            target: 0usize,
            fn detour(value: u32) -> u32 {
                original(value)
            }
        }
    }

    crate::hook! {
        pub detour TestDetourHook {
            target: 0usize,
            fn detour(value: u32) -> u32 {
                original(value)
            }
        }
    }

    crate::hook! {
        pub universal TestUniversalAliasHook {
            target: 0usize,
            fn detour(value: u32) -> u32 {
                original(value)
            }
        }
    }

    crate::hook! {
        pub call TestCallHook {
            target: 0usize,
            offset: 0usize,
            size: 5,
            fn detour(value: u32) -> u32 {
                original(value)
            }
        }
    }

    crate::hook! {
        pub vtable TestVtableHook {
            vtable: 0usize,
            index: 0usize,
            fn detour(value: u32) -> u32 {
                original(value)
            }
        }
    }

    crate::hook! {
        pub vcall TestVcallHook {
            target: 0usize,
            offset: 0usize,
            size: 6,
            receiver: this,
            index: 0usize,
            fn detour(this: *mut u8, value: u32) -> u32 {
                original(this, value)
            }
        }
    }

    crate::hook! {
        pub vcall TestVcallSlotHook {
            target: 0usize,
            offset: 0usize,
            size: 6,
            receiver: this,
            slot: 0usize,
            fn detour(this: *mut u8, value: u32) -> u32 {
                original(this, value)
            }
        }
    }

    crate::relocation_func! {
        pub fn tuple_identity(value: crate::re::BSTTuple<u32, u32>) -> crate::re::BSTTuple<u32, u32> => 0usize
    }

    #[test]
    fn hook_macro_expands_for_all_low_level_modes() {
        let _ = TestFunctionHook::install as fn();
        let _ = TestFunctionHook::original as fn(u32) -> u32;
        let _ = TestUniversalFunctionHook::install as fn();
        let _ = TestUniversalFunctionHook::original as fn(u32) -> u32;
        let _ = TestDetourHook::install as fn();
        let _ = TestDetourHook::original as fn(u32) -> u32;
        let _ = TestUniversalAliasHook::install as fn();
        let _ = TestUniversalAliasHook::original as fn(u32) -> u32;
        let _ = TestCallHook::install as fn();
        let _ = TestCallHook::original as fn(u32) -> u32;
        let _ = TestVtableHook::install as fn();
        let _ = TestVtableHook::original as fn(u32) -> u32;
        let _ = TestVcallHook::install as fn();
        let _ = TestVcallHook::callsite_relocation as fn() -> crate::relocation::Relocation<()>;
        let _ = TestVcallHook::vtable_index as fn() -> usize;
        let _ = TestVcallHook::original as fn(*mut u8, u32) -> u32;
        let _ = TestVcallHook::original_virtual_relocation::<u8>
            as fn(*const u8) -> crate::relocation::Relocation<extern "C" fn(*mut u8, u32) -> u32>;
        let _ = TestVcallSlotHook::install as fn();
        let _ = TestVcallSlotHook::callsite_relocation as fn() -> crate::relocation::Relocation<()>;
        let _ = TestVcallSlotHook::vtable_index as fn() -> usize;
        let _ = TestVcallSlotHook::original as fn(*mut u8, u32) -> u32;
        let _ = TestVcallSlotHook::original_virtual_relocation::<u8>
            as fn(*const u8) -> crate::relocation::Relocation<extern "C" fn(*mut u8, u32) -> u32>;
    }

    #[test]
    fn relocation_func_accepts_plain_bst_tuple_by_value() {
        let _ =
            tuple_identity as fn(crate::re::BSTTuple<u32, u32>) -> crate::re::BSTTuple<u32, u32>;
    }

    #[test]
    fn relocation_pointer_get_keeps_address_cast_semantics() {
        let mut value = 41u32;
        let mut stored_ptr = core::ptr::from_mut(&mut value);
        let cell = core::ptr::from_mut(&mut stored_ptr);

        let raw = unsafe { Relocation::<*mut u32>::from_address(cell as usize).get() };
        assert_eq!(raw, cell.cast());
        assert_ne!(raw, core::ptr::from_mut(&mut value));
    }

    #[test]
    fn read_indirect_ptr_dereferences_singleton_cells_once() {
        let mut value = 99u32;
        let mut stored_ptr = core::ptr::from_mut(&mut value);
        let cell = core::ptr::from_mut(&mut stored_ptr);

        let resolved = unsafe { read_indirect_ptr::<u32, _>(cell as usize) };
        assert_eq!(resolved, core::ptr::from_mut(&mut value));
    }
}
