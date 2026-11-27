//! Translation of `RE::BSTArray.h` / `RE::BSTArray.cpp`.
//!
//! Contains:
//! - `BSTArrayBase` вЂ” base class storing the array size
//! - `BSTArrayHeapAllocator` вЂ” default heap allocator using engine's `RE::malloc`/`RE::free`
//! - `BSScrapArrayAllocator` вЂ” per-thread scrap heap allocator
//! - `BSTArray<T, A>` вЂ” generic dynamic array (C++ `BSTArray<T, Allocator>`)
//! - `BSScrapArray<T>` вЂ” type alias for `BSTArray<T, BSScrapArrayAllocator>`
//! - `BSStaticArray<T>` вЂ” non-owning static array (pointer + size)
//! - `BSTSmallSharedArray<T>` вЂ” small shared array with inline storage for 0-1 elements

use bytemuck::Zeroable;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::{ManuallyDrop, MaybeUninit};
use core::ops::{Index, IndexMut};
use core::ptr;

use crate::offsets::offsets_rtti::RTTI_BSTArrayBase__IAllocatorFunctor;
use crate::offsets::offsets_vtable::VTABLE_BSTArrayBase__IAllocatorFunctor;
use crate::re::memory_manager;
use crate::re::scrap_heap::ScrapHeap;
use crate::relocation::{RttiType, VariantID};
use crate::virtual_method;

// в”Ђв”Ђв”Ђ BSTArrayBase в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `RE::BSTArrayBase`.
/// Base class for `BSTArray`, stores the element count.
///
/// Layout: `{ _size: u32 }` вЂ” 0x4 bytes total.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BSTArrayBase {
    size: u32, // 0x00
}

const _: () = assert!(core::mem::size_of::<BSTArrayBase>() == 0x4);

impl BSTArrayBase {
    #[inline(always)]
    pub const fn new() -> Self {
        Self { size: 0 }
    }

    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline(always)]
    pub const fn len(&self) -> u32 {
        self.size
    }

    #[inline(always)]
    pub fn set_size(&mut self, new_size: u32) {
        self.size = new_size;
    }
}

impl Default for BSTArrayBase {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// C++ `RE::BSTArrayBase::IAllocatorFunctor`.
#[repr(C)]
pub struct BSTArrayBaseIAllocatorFunctor {
    pub vtable: *const usize, // 00
}

const _: () = assert!(core::mem::size_of::<BSTArrayBaseIAllocatorFunctor>() == 0x8);
const _: () = assert!(core::mem::offset_of!(BSTArrayBaseIAllocatorFunctor, vtable) == 0x00);

impl RttiType for BSTArrayBaseIAllocatorFunctor {
    const RTTI: VariantID = RTTI_BSTArrayBase__IAllocatorFunctor;
}

impl BSTArrayBaseIAllocatorFunctor {
    pub const RTTI: VariantID = RTTI_BSTArrayBase__IAllocatorFunctor;
    pub const VTABLE: &'static [crate::relocation::VariantID] =
        &VTABLE_BSTArrayBase__IAllocatorFunctor;

    virtual_method! {
        pub const VFUNC_ALLOCATE: usize = 0x00;
        pub fn allocate(num: u32, elem_size: u32) -> bool
    }

    virtual_method! {
        pub const VFUNC_REALLOCATE: usize = 0x01;
        pub fn reallocate(
            min_new_size_in_items: u32,
            front_copy_count: u32,
            shift_count: u32,
            back_copy_count: u32,
            elem_size: u32
        ) -> bool
    }

    virtual_method! {
        pub const VFUNC_DEALLOCATE: usize = 0x02;
        pub fn deallocate()
    }

    virtual_method! {
        pub const VFUNC_DTOR: usize = 0x03;
        pub fn dtor()
    }
}

// в”Ђв”Ђв”Ђ BSTArrayAllocator trait в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// Trait replacing C++ `IAllocatorFunctor` вЂ” defines the allocator interface
/// used by `BSTArray`.
///
/// # Safety
/// Implementations must return memory blocks that are valid for `size` bytes
/// and properly aligned for any `T` that will be stored.
pub unsafe trait BSTArrayAllocator {
    fn data(&self) -> *mut u8;
    fn capacity(&self) -> u32;
    /// Allocate `size` bytes. Returns a zeroed memory block.
    fn allocate(&mut self, size: usize) -> *mut u8;
    /// Deallocate a previously allocated block.
    fn deallocate(&mut self, ptr: *mut u8);
    /// Update the stored data pointer and capacity after a reallocation.
    fn set_allocator_traits(&mut self, data: *mut u8, capacity: u32, elem_size: usize);
}

// в”Ђв”Ђв”Ђ BSTArrayHeapAllocator в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `RE::BSTArrayHeapAllocator`.
/// Default allocator for `BSTArray` вЂ” uses `RE::malloc` / `RE::free`
/// through the C++ bridge, exactly matching the engine's allocation path.
///
/// Layout: `{ _data: *mut u8, _capacity: u32, _pad0c: u32 }` вЂ” 0x10 bytes total.
#[repr(C)]
pub struct BSTArrayHeapAllocator {
    _data: *mut u8, // 0x00
    _capacity: u32, // 0x08
    _pad0c: u32,    // 0x0C
}

const _: () = assert!(core::mem::size_of::<BSTArrayHeapAllocator>() == 0x10);

impl BSTArrayHeapAllocator {
    pub const fn new() -> Self {
        Self {
            _data: ptr::null_mut(),
            _capacity: 0,
            _pad0c: 0,
        }
    }
}

unsafe impl BSTArrayAllocator for BSTArrayHeapAllocator {
    #[inline(always)]
    fn data(&self) -> *mut u8 {
        self._data
    }

    #[inline(always)]
    fn capacity(&self) -> u32 {
        self._capacity
    }

    fn allocate(&mut self, size: usize) -> *mut u8 {
        if size == 0 {
            return ptr::null_mut();
        }
        unsafe {
            // RE::malloc в†’ MemoryManager::GetSingleton()->Allocate(size, 0, false)
            let mem = memory_manager::malloc(size);
            if mem.is_null() {
                panic!("BSTArrayHeapAllocator: out of memory");
            }
            ptr::write_bytes(mem as *mut u8, 0, size);
            mem as *mut u8
        }
    }

    fn deallocate(&mut self, ptr: *mut u8) {
        if !ptr.is_null() {
            // RE::free в†’ MemoryManager::GetSingleton()->Deallocate(ptr, false)
            memory_manager::free(ptr as *mut c_void);
        }
    }

    #[inline(always)]
    fn set_allocator_traits(&mut self, data: *mut u8, capacity: u32, _elem_size: usize) {
        self._data = data;
        self._capacity = capacity;
    }
}

// в”Ђв”Ђв”Ђ BSTSmallArrayHeapAllocator<N> в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `RE::BSTSmallArrayHeapAllocator<N>`.
/// An allocator with `N` bytes of inline storage. If the requested allocation
/// fits within the inline buffer, no heap allocation occurs.
///
/// C++ uses bitfields `_capacity: 31` and `_local: 1` packed into a single `u32`.
/// MSVC packs them as: capacity in bits 0-30, local flag in bit 31.
///
/// Layout: `{ _capacity_and_local: u32, _pad04: u32, _data: union { heap, local[N] } }`
#[repr(C)]
pub struct BSTSmallArrayHeapAllocator<const N: usize> {
    /// Packed bitfield: bits 0-30 = capacity, bit 31 = local flag
    _capacity_and_local: u32, // 0x00
    _pad04: u32,              // 0x04
    _data: SmallArrayData<N>, // 0x08
}

/// Internal union for `BSTSmallArrayHeapAllocator`.
/// `heap` is a pointer to heap-allocated storage.
/// `local` is inline stack storage of `N` bytes.
#[repr(C)]
pub union SmallArrayData<const N: usize> {
    pub heap: *mut u8,
    pub local: [u8; N],
}

/// Bitfield constants for `BSTSmallArrayHeapAllocator`.
const CAPACITY_MASK: u32 = 0x7FFF_FFFF; // bits 0-30
const LOCAL_FLAG: u32 = 0x8000_0000; // bit 31

impl<const N: usize> BSTSmallArrayHeapAllocator<N> {
    pub const fn new() -> Self {
        Self {
            // _capacity = 0, _local = 1 в†’ bit 31 set
            _capacity_and_local: LOCAL_FLAG,
            _pad04: 0,
            _data: SmallArrayData { local: [0u8; N] },
        }
    }

    /// Returns true if using inline (local) storage.
    #[inline(always)]
    fn is_local(&self) -> bool {
        (self._capacity_and_local & LOCAL_FLAG) != 0
    }

    /// Sets the local flag.
    #[inline(always)]
    fn set_local(&mut self, local: bool) {
        if local {
            self._capacity_and_local |= LOCAL_FLAG;
        } else {
            self._capacity_and_local &= CAPACITY_MASK;
        }
    }

    /// Gets capacity from the packed bitfield (bits 0-30).
    #[inline(always)]
    fn get_capacity(&self) -> u32 {
        self._capacity_and_local & CAPACITY_MASK
    }

    /// Sets capacity in the packed bitfield (preserving the local flag).
    #[inline(always)]
    fn set_capacity(&mut self, cap: u32) {
        debug_assert!(cap <= CAPACITY_MASK, "capacity exceeds 31-bit max");
        self._capacity_and_local = (self._capacity_and_local & LOCAL_FLAG) | (cap & CAPACITY_MASK);
    }
}

impl<const N: usize> Default for BSTSmallArrayHeapAllocator<N> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<const N: usize> BSTArrayAllocator for BSTSmallArrayHeapAllocator<N> {
    #[inline(always)]
    fn data(&self) -> *mut u8 {
        if self.is_local() {
            // Return pointer to inline storage
            unsafe { self._data.local.as_ptr() as *mut u8 }
        } else {
            unsafe { self._data.heap }
        }
    }

    #[inline(always)]
    fn capacity(&self) -> u32 {
        self.get_capacity()
    }

    fn allocate(&mut self, size: usize) -> *mut u8 {
        if size > N {
            // Exceeds inline storage вЂ” allocate from engine heap
            unsafe {
                let mem = memory_manager::malloc(size);
                if mem.is_null() {
                    panic!("BSTSmallArrayHeapAllocator: out of memory");
                }
                ptr::write_bytes(mem as *mut u8, 0, size);
                mem as *mut u8
            }
        } else {
            // Fits in inline storage
            unsafe { self._data.local.as_mut_ptr() }
        }
    }

    fn deallocate(&mut self, ptr: *mut u8) {
        // Only free if the pointer is NOT our inline buffer
        let local_ptr = unsafe { self._data.local.as_ptr() as *mut u8 };
        if ptr != local_ptr && !ptr.is_null() {
            memory_manager::free(ptr as *mut c_void);
        }
    }

    fn set_allocator_traits(&mut self, data: *mut u8, capacity: u32, elem_size: usize) {
        self.set_capacity(capacity);
        if (capacity as usize) * elem_size > N {
            // Switched to heap mode
            self.set_local(false);
            self._data = SmallArrayData { heap: data };
        } else {
            // CommonLib #547: (re-)enter local mode, pulling the elements into the
            // inline buffer when they were handed over from elsewhere (e.g. when
            // shrinking back from the heap).
            self.set_local(true);
            let local = unsafe { self._data.local.as_mut_ptr() };
            if !data.is_null() && data != local {
                unsafe { ptr::copy(data, local, (capacity as usize) * elem_size) };
            }
        }
    }
}

// в”Ђв”Ђв”Ђ BSScrapArrayAllocator в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `RE::BSScrapArrayAllocator`.
/// Allocator using the engine's per-thread `ScrapHeap`.
/// All engine calls go through the C++ bridge for exact parity with CommonLib.
///
/// Layout: `{ _allocator: *mut ScrapHeap, _data: *mut u8, _capacity: u32, _pad14: u32 }` вЂ” 0x18 bytes.
#[repr(C)]
pub struct BSScrapArrayAllocator {
    _allocator: *mut ScrapHeap, // 0x00
    _data: *mut u8,             // 0x08
    _capacity: u32,             // 0x10
    _pad14: u32,                // 0x14
}

const _: () = assert!(core::mem::size_of::<BSScrapArrayAllocator>() == 0x18);

impl BSScrapArrayAllocator {
    pub const fn new() -> Self {
        Self {
            _allocator: ptr::null_mut(),
            _data: ptr::null_mut(),
            _capacity: 0,
            _pad14: 0,
        }
    }
}

unsafe impl BSTArrayAllocator for BSScrapArrayAllocator {
    #[inline(always)]
    fn data(&self) -> *mut u8 {
        self._data
    }

    #[inline(always)]
    fn capacity(&self) -> u32 {
        self._capacity
    }

    fn allocate(&mut self, size: usize) -> *mut u8 {
        // Lazy-initialize the scrap heap from MemoryManager (mirrors BSTArray.cpp)
        if self._allocator.is_null() {
            unsafe {
                let mgr = memory_manager::MemoryManager::get_singleton();
                if !mgr.is_null() {
                    self._allocator = (*mgr).get_thread_scrap_heap();
                }
            }
        }
        assert!(
            !self._allocator.is_null(),
            "BSScrapArrayAllocator: no ScrapHeap"
        );

        unsafe {
            let mem = (*self._allocator).allocate(size, core::mem::align_of::<*mut u8>());
            assert!(!mem.is_null(), "BSScrapArrayAllocator: allocation failed");
            ptr::write_bytes(mem as *mut u8, 0, size);
            mem as *mut u8
        }
    }

    fn deallocate(&mut self, ptr: *mut u8) {
        if self._allocator.is_null() {
            assert!(
                ptr.is_null(),
                "BSScrapArrayAllocator: no allocator for dealloc"
            );
            return;
        }
        unsafe {
            (*self._allocator).deallocate(ptr as *mut c_void);
        }
    }

    #[inline(always)]
    fn set_allocator_traits(&mut self, data: *mut u8, capacity: u32, _elem_size: usize) {
        self._data = data;
        self._capacity = capacity;
    }
}

// в”Ђв”Ђв”Ђ BSTArray<T, A> в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `RE::BSTArray<T, Allocator>`.
///
/// A generic dynamic array composed of an allocator (first) and `BSTArrayBase` (second).
/// Memory layout matches C++ multiple inheritance order: `Allocator` then `BSTArrayBase`.
///
/// Default allocator is `BSTArrayHeapAllocator`.
#[repr(C)]
pub struct BSTArray<T, A: BSTArrayAllocator = BSTArrayHeapAllocator> {
    allocator: A,
    base: BSTArrayBase,
    _marker: PhantomData<*mut T>,
}

const _: () = assert!(core::mem::size_of::<BSTArray<u8, BSTArrayHeapAllocator>>() == 0x18);
const _: () =
    assert!(core::mem::offset_of!(BSTArray<u8, BSTArrayHeapAllocator>, allocator) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSTArray<u8, BSTArrayHeapAllocator>, base) == 0x10);
const _: () = assert!(core::mem::size_of::<BSTArray<u8, BSScrapArrayAllocator>>() == 0x20);
const _: () = assert!(core::mem::offset_of!(BSTArray<u8, BSScrapArrayAllocator>, base) == 0x18);
const _: () = assert!(core::mem::size_of::<BSTArray<u8, BSTSmallArrayHeapAllocator<8>>>() == 0x18);
const _: () =
    assert!(core::mem::offset_of!(BSTArray<u8, BSTSmallArrayHeapAllocator<8>>, base) == 0x10);

/// Default capacity for new allocations (Beth default).
const BST_ARRAY_DF_CAP: u32 = 4;

/// Growth factor (2.0 as in the original).
const BST_ARRAY_GROWTH_FACTOR: f32 = 2.0;

// Provide Default implementations for the allocators
impl Default for BSTArrayHeapAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BSScrapArrayAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A: BSTArrayAllocator + Default> BSTArray<T, A> {
    /// Creates a new empty `BSTArray`.
    pub fn new() -> Self {
        Self {
            allocator: A::default(),
            base: BSTArrayBase::new(),
            _marker: PhantomData,
        }
    }
}

impl<T, A: BSTArrayAllocator + Default> Default for BSTArray<T, A> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A> Clone for BSTArray<T, A>
where
    T: Clone,
    A: BSTArrayAllocator + Default,
{
    fn clone(&self) -> Self {
        let mut out = Self::new();
        out.reserve(self.len());
        unsafe {
            for item in self.as_slice() {
                out.push_back(item.clone());
            }
        }
        out
    }
}

impl<T, A: BSTArrayAllocator> BSTArray<T, A> {
    /// Number of elements in the array.
    #[inline(always)]
    pub fn len(&self) -> u32 {
        self.base.len()
    }

    /// C++ `size()`.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        self.len()
    }

    /// Whether the array is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
    }

    /// C++ `empty()`.
    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.is_empty()
    }

    /// Current capacity (in number of elements).
    #[inline(always)]
    pub fn capacity(&self) -> u32 {
        self.allocator.capacity()
    }

    /// Returns a raw pointer to the first element, or null if empty.
    #[inline(always)]
    pub fn data(&self) -> *const T {
        self.allocator.data() as *const T
    }

    /// Returns a mutable raw pointer to the first element.
    #[inline(always)]
    pub fn data_mut(&mut self) -> *mut T {
        self.allocator.data() as *mut T
    }

    /// Raw begin iterator.
    #[inline(always)]
    pub fn begin(&self) -> *const T {
        if self.is_empty() {
            ptr::null()
        } else {
            self.data()
        }
    }

    /// Raw mutable begin iterator.
    #[inline(always)]
    pub fn begin_mut(&mut self) -> *mut T {
        if self.is_empty() {
            ptr::null_mut()
        } else {
            self.data_mut()
        }
    }

    /// Raw end iterator.
    #[inline(always)]
    pub fn end(&self) -> *const T {
        if self.is_empty() {
            ptr::null()
        } else {
            unsafe { self.data().add(self.len() as usize) }
        }
    }

    /// Raw mutable end iterator.
    #[inline(always)]
    pub fn end_mut(&mut self) -> *mut T {
        if self.is_empty() {
            ptr::null_mut()
        } else {
            unsafe { self.data_mut().add(self.len() as usize) }
        }
    }

    /// Raw const begin iterator.
    #[inline(always)]
    pub fn cbegin(&self) -> *const T {
        self.begin()
    }

    /// Raw const end iterator.
    #[inline(always)]
    pub fn cend(&self) -> *const T {
        self.end()
    }

    /// Returns a slice of the array contents.
    ///
    /// # Safety
    /// The caller must ensure the array data pointer is valid and that
    /// `T` is the correct element type matching the engine's array.
    #[inline]
    pub unsafe fn as_slice(&self) -> &[T] {
        unsafe {
            let len = self.len() as usize;
            if len == 0 {
                &[]
            } else {
                let data = self.data();
                if data.is_null() {
                    crate::defensive_sdk_warn!(
                        "BSTArray<{}>::as_slice observed null data with non-zero len={}",
                        core::any::type_name::<T>(),
                        len
                    );
                    &[]
                } else {
                    core::slice::from_raw_parts(data, len)
                }
            }
        }
    }

    /// Returns a mutable slice of the array contents.
    ///
    /// # Safety
    /// Same as `as_slice`, plus exclusive access must be guaranteed.
    #[inline]
    pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            let len = self.len() as usize;
            if len == 0 {
                &mut []
            } else {
                let data = self.data_mut();
                if data.is_null() {
                    crate::defensive_sdk_warn!(
                        "BSTArray<{}>::as_mut_slice observed null data with non-zero len={}",
                        core::any::type_name::<T>(),
                        len
                    );
                    &mut []
                } else {
                    core::slice::from_raw_parts_mut(data, len)
                }
            }
        }
    }

    /// Returns the first element.
    ///
    /// # Safety
    /// The caller must ensure the underlying engine memory is valid.
    #[inline]
    pub unsafe fn front(&self) -> &T {
        assert!(!self.is_empty());
        unsafe { &*self.data() }
    }

    /// Returns the first element mutably.
    ///
    /// # Safety
    /// The caller must ensure the underlying engine memory is valid.
    #[inline]
    pub unsafe fn front_mut(&mut self) -> &mut T {
        assert!(!self.is_empty());
        unsafe { &mut *self.data_mut() }
    }

    /// Returns the last element.
    ///
    /// # Safety
    /// The caller must ensure the underlying engine memory is valid.
    #[inline]
    pub unsafe fn back(&self) -> &T {
        assert!(!self.is_empty());
        unsafe { &*self.data().add(self.len() as usize - 1) }
    }

    /// Returns the last element mutably.
    ///
    /// # Safety
    /// The caller must ensure the underlying engine memory is valid.
    #[inline]
    pub unsafe fn back_mut(&mut self) -> &mut T {
        assert!(!self.is_empty());
        unsafe { &mut *self.data_mut().add(self.len() as usize - 1) }
    }

    /// Reserves capacity for at least `new_cap` elements.
    pub fn reserve(&mut self, new_cap: u32) {
        if new_cap > self.capacity() {
            self.change_capacity(new_cap);
        }
    }

    /// Shrinks capacity to match the current size.
    pub fn shrink_to_fit(&mut self) {
        let new_cap = self.len();
        if new_cap != self.capacity() {
            self.change_capacity(new_cap);
        }
    }

    /// Adds an element to the end of the array.
    ///
    /// # Safety
    /// The caller must ensure the array data pointer is valid.
    pub unsafe fn push(&mut self, value: T) {
        let _ = unsafe { self.push_back(value) };
    }

    /// C++ `push_back`.
    ///
    /// # Safety
    /// The caller must ensure the array data pointer is valid.
    pub unsafe fn push_back(&mut self, value: T) -> &mut T {
        unsafe {
            if self.len() == self.capacity() {
                self.grow_capacity();
            }
            let size = self.len();
            self.base.set_size(size + 1);
            let dst = self.data_mut().add(size as usize);
            ptr::write(dst, value);
            &mut *dst
        }
    }

    /// C++ `push_front`.
    ///
    /// # Safety
    /// The caller must ensure the array data pointer is valid.
    #[inline]
    pub unsafe fn push_front(&mut self, value: T) -> &mut T {
        unsafe { self.insert(0, value) }
    }

    /// Removes and returns the last element.
    ///
    /// # Safety
    /// The caller must ensure the array is non-empty and data pointer is valid.
    pub unsafe fn pop(&mut self) -> T {
        unsafe { self.pop_back() }
    }

    /// C++ `pop_back`.
    ///
    /// # Safety
    /// The caller must ensure the array is non-empty and data pointer is valid.
    pub unsafe fn pop_back(&mut self) -> T {
        unsafe {
            assert!(!self.is_empty());
            let new_size = self.len() - 1;
            let val = ptr::read(self.data().add(new_size as usize));
            self.base.set_size(new_size);
            val
        }
    }

    /// Inserts an element at `index`, shifting the tail to the right.
    ///
    /// # Safety
    /// The caller must ensure the array data pointer is valid.
    pub unsafe fn insert(&mut self, index: u32, value: T) -> &mut T {
        unsafe {
            assert!(index <= self.len());

            if index == self.len() {
                return self.push_back(value);
            }

            if self.len() == self.capacity() {
                self.grow_capacity();
            }

            let old_len = self.len();
            let data = self.data_mut();
            for i in (index as usize..old_len as usize).rev() {
                let src = data.add(i);
                let dst = data.add(i + 1);
                ptr::write(dst, ptr::read(src));
            }
            ptr::write(data.add(index as usize), value);
            self.base.set_size(old_len + 1);
            &mut *data.add(index as usize)
        }
    }

    /// Removes and returns the element at `index`, shifting the tail left.
    ///
    /// # Safety
    /// The caller must ensure the array data pointer is valid.
    pub unsafe fn erase(&mut self, index: u32) -> T {
        unsafe {
            assert!(index < self.len());

            let old_len = self.len();
            let data = self.data_mut();
            let removed = ptr::read(data.add(index as usize));
            for i in index as usize..old_len as usize - 1 {
                let src = data.add(i + 1);
                let dst = data.add(i);
                ptr::write(dst, ptr::read(src));
            }
            self.base.set_size(old_len - 1);
            removed
        }
    }

    /// Clears all elements, calling drop on each.
    ///
    /// # Safety
    /// The caller must ensure the data pointer is valid.
    pub unsafe fn clear(&mut self) {
        unsafe {
            let old_size = self.len();
            if old_size > 0 {
                let data = self.data_mut();
                for i in 0..old_size {
                    ptr::drop_in_place(data.add(i as usize));
                }
                self.base.set_size(0);
            }
        }
    }

    /// Resizes the array to `new_size` elements.
    /// New elements are zero-initialized.
    ///
    /// # Safety
    /// The caller must ensure the data pointer is valid.
    ///
    /// # Compile Error
    /// This will fail to compile if `T` is a complex C++ class (like `TESForm`)
    /// that cannot be safely zero-initialized.
    pub unsafe fn resize(&mut self, new_size: u32)
    where
        T: Zeroable,
    {
        unsafe {
            if new_size != self.len() {
                self.change_size(new_size);
            }
        }
    }

    /// C++ `resize(count)` with Rust `Default` construction instead of zero-fill.
    ///
    /// # Safety
    /// The caller must ensure the data pointer remains valid for the array lifetime.
    pub unsafe fn resize_default(&mut self, new_size: u32)
    where
        T: Default,
    {
        unsafe {
            if new_size > self.capacity() {
                self.grow_capacity_to(new_size);
            }

            let old_size = self.len();
            let data = self.data_mut();
            if new_size > old_size {
                for i in old_size..new_size {
                    ptr::write(data.add(i as usize), T::default());
                }
            } else if new_size < old_size {
                for i in new_size..old_size {
                    ptr::drop_in_place(data.add(i as usize));
                }
            }

            self.base.set_size(new_size);
        }
    }

    /// C++ `resize(count, value)`.
    ///
    /// # Safety
    /// The caller must ensure the data pointer remains valid for the array lifetime.
    pub unsafe fn resize_fill(&mut self, new_size: u32, value: &T)
    where
        T: Clone,
    {
        unsafe {
            if new_size > self.capacity() {
                self.grow_capacity_to(new_size);
            }

            let old_size = self.len();
            let data = self.data_mut();
            if new_size > old_size {
                for i in old_size..new_size {
                    ptr::write(data.add(i as usize), value.clone());
                }
            } else if new_size < old_size {
                for i in new_size..old_size {
                    ptr::drop_in_place(data.add(i as usize));
                }
            }

            self.base.set_size(new_size);
        }
    }

    /// Releases all memory and resets the array.
    ///
    /// # Safety
    /// The caller must ensure the data pointer is valid.
    pub unsafe fn release(&mut self) {
        unsafe {
            self.clear();
            self.change_capacity(0); // change_capacity С‚РѕР¶Рµ РЅРµ РґРѕР»Р¶РµРЅ РёРјРµС‚СЊ where T: Zeroable!
        }
    }

    // в”Ђв”Ђ Private helpers в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

    /// Typed allocation: allocates memory for `num` elements.
    fn allocate_elements(&mut self, num: u32) -> *mut T {
        let size = (num as usize) * core::mem::size_of::<T>();
        self.allocator.allocate(size) as *mut T
    }

    /// Update allocator traits using element size.
    fn set_allocator_traits_typed(&mut self, data: *mut T, capacity: u32) {
        self.allocator
            .set_allocator_traits(data as *mut u8, capacity, core::mem::size_of::<T>());
    }

    /// Change the allocated capacity, relocating the raw storage.
    fn change_capacity(&mut self, new_capacity: u32) {
        let new_data = if new_capacity > 0 {
            self.allocate_elements(new_capacity)
        } else {
            ptr::null_mut()
        };

        let old_data = self.data_mut();
        if !old_data.is_null() {
            let old_capacity = self.capacity();
            if !new_data.is_null() {
                let to_copy = core::cmp::min(old_capacity, new_capacity) as usize;
                unsafe {
                    // Match the engine's capacity-byte relocation while still
                    // tolerating inline allocators that hand back the same buffer.
                    ptr::copy(
                        old_data.cast::<MaybeUninit<T>>(),
                        new_data.cast::<MaybeUninit<T>>(),
                        to_copy,
                    );
                }
            }
            self.allocator.deallocate(old_data as *mut u8);
        }

        self.set_allocator_traits_typed(new_data, new_capacity);
    }

    /// Change the size, constructing or destroying as needed.
    unsafe fn change_size(&mut self, new_size: u32)
    where
        T: Zeroable,
    {
        unsafe {
            if new_size > self.capacity() {
                self.grow_capacity_to(new_size);
            }

            let old_size = self.len();
            if new_size > old_size {
                let data = self.data_mut();
                ptr::write_bytes(
                    data.add(old_size as usize) as *mut u8,
                    0,
                    (new_size - old_size) as usize * core::mem::size_of::<T>(),
                );
            } else if new_size < old_size {
                let data = self.data_mut();
                for i in new_size..old_size {
                    ptr::drop_in_place(data.add(i as usize));
                }
            }

            self.base.set_size(new_size);
        }
    }

    /// Calculate the next capacity value (doubles, or starts at `DF_CAP`).
    fn next_capacity(&self) -> u32 {
        Self::next_capacity_from(self.capacity())
    }

    fn next_capacity_from(hint: u32) -> u32 {
        if hint > 0 {
            // ceil(hint * 2.0) вЂ” matches C++ std::ceil(cap * GROWTH_FACTOR)
            let grown = (hint as f32) * BST_ARRAY_GROWTH_FACTOR;
            let truncated = grown as u32;
            if (truncated as f32) < grown {
                truncated + 1
            } else {
                truncated
            }
        } else {
            BST_ARRAY_DF_CAP
        }
    }

    fn grow_capacity(&mut self) {
        let next = self.next_capacity();
        self.change_capacity(next);
    }

    fn grow_capacity_to(&mut self, hint: u32) {
        let next = Self::next_capacity_from(hint);
        self.change_capacity(next);
    }
}

impl<T, A: BSTArrayAllocator> Drop for BSTArray<T, A> {
    fn drop(&mut self) {
        unsafe {
            self.release();
        }
    }
}

impl<T, A: BSTArrayAllocator> Index<usize> for BSTArray<T, A> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len() as usize);
        unsafe { &*self.data().add(index) }
    }
}

impl<T, A: BSTArrayAllocator> IndexMut<usize> for BSTArray<T, A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len() as usize);
        unsafe { &mut *self.data_mut().add(index) }
    }
}

impl<T, A> core::iter::FromIterator<T> for BSTArray<T, A>
where
    A: BSTArrayAllocator + Default,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut out = Self::new();
        out.extend(iter);
        out
    }
}

impl<T, A: BSTArrayAllocator> Extend<T> for BSTArray<T, A> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter {
            unsafe {
                self.push_back(value);
            }
        }
    }
}

/// Type alias for `BSTArray<T, BSScrapArrayAllocator>`.
/// C++ `RE::BSScrapArray<T>`.
pub type BSScrapArray<T> = BSTArray<T, BSScrapArrayAllocator>;

/// C++ `template <class T, std::uint32_t N = 1> using BSTSmallArray = BSTArray<T, BSTSmallArrayHeapAllocator<sizeof(T) * N>>`.
///
/// Note: In Rust, const generic expressions like `{core::mem::size_of::<T>() * N}` require
/// `#![feature(generic_const_exprs)]` which is unstable. Instead, the caller must specify
/// the byte count directly. For convenience, the default `N` matches `sizeof(T) * 1`
/// for pointer-sized types (8 bytes).
///
/// Usage: `BSTSmallArray<MyType, {core::mem::size_of::<MyType>()}>` for N=1,
/// or manually compute `sizeof(T) * count`.
pub type BSTSmallArray<T, const N: usize> = BSTArray<T, BSTSmallArrayHeapAllocator<N>>;

// в”Ђв”Ђв”Ђ BSStaticArray<T> в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `RE::BSStaticArray<T>`.
/// A non-owning view over a contiguous block of elements.
///
/// Layout: `{ _data: *mut T, _size: u32, _pad: u32 }` вЂ” 0x10 bytes.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BSStaticArray<T> {
    _data: *mut T, // 0x00
    _size: u32,    // 0x08
    _pad0c: u32,   // 0x0C вЂ” padding for alignment
}

// Size assertion: ptr(8) + u32(4) + pad(4) = 0x10
const _: () = assert!(core::mem::size_of::<BSStaticArray<u8>>() == 0x10);

impl<T> BSStaticArray<T> {
    /// Number of elements.
    #[inline(always)]
    pub fn len(&self) -> u32 {
        self._size
    }

    /// C++ `size()`.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        self.len()
    }

    /// Whether the array is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self._size == 0
    }

    /// C++ `empty()`.
    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.is_empty()
    }

    /// Raw pointer to the first element.
    #[inline(always)]
    pub fn data(&self) -> *const T {
        self._data
    }

    /// Mutable raw pointer to the first element.
    #[inline(always)]
    pub fn data_mut(&mut self) -> *mut T {
        self._data
    }

    /// Raw begin iterator.
    #[inline(always)]
    pub fn begin(&self) -> *const T {
        if self.is_empty() {
            ptr::null()
        } else {
            self.data()
        }
    }

    /// Raw mutable begin iterator.
    #[inline(always)]
    pub fn begin_mut(&mut self) -> *mut T {
        if self.is_empty() {
            ptr::null_mut()
        } else {
            self.data_mut()
        }
    }

    /// Raw end iterator.
    #[inline(always)]
    pub fn end(&self) -> *const T {
        if self.is_empty() {
            ptr::null()
        } else {
            unsafe { self.data().add(self.len() as usize) }
        }
    }

    /// Raw mutable end iterator.
    #[inline(always)]
    pub fn end_mut(&mut self) -> *mut T {
        if self.is_empty() {
            ptr::null_mut()
        } else {
            unsafe { self.data_mut().add(self.len() as usize) }
        }
    }

    /// Raw const begin iterator.
    #[inline(always)]
    pub fn cbegin(&self) -> *const T {
        self.begin()
    }

    /// Raw const end iterator.
    #[inline(always)]
    pub fn cend(&self) -> *const T {
        self.end()
    }

    /// Returns a slice of the array contents.
    ///
    /// # Safety
    /// The caller must ensure the data pointer is valid for `len()` elements.
    #[inline]
    pub unsafe fn as_slice(&self) -> &[T] {
        unsafe {
            if self.is_empty() {
                &[]
            } else {
                if self._data.is_null() {
                    crate::defensive_sdk_warn!(
                        "BSStaticArray<{}>::as_slice observed null data with non-zero len={}",
                        core::any::type_name::<T>(),
                        self._size
                    );
                    &[]
                } else {
                    core::slice::from_raw_parts(self._data, self._size as usize)
                }
            }
        }
    }

    /// Returns a mutable slice of the array contents.
    ///
    /// # Safety
    /// Same as `as_slice`, plus exclusive access must be guaranteed.
    #[inline]
    pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            if self.is_empty() {
                &mut []
            } else {
                if self._data.is_null() {
                    crate::defensive_sdk_warn!(
                        "BSStaticArray<{}>::as_mut_slice observed null data with non-zero len={}",
                        core::any::type_name::<T>(),
                        self._size
                    );
                    &mut []
                } else {
                    core::slice::from_raw_parts_mut(self._data, self._size as usize)
                }
            }
        }
    }

    /// Returns the first element.
    ///
    /// # Safety
    /// The caller must ensure the pointed-to memory is valid.
    #[inline]
    pub unsafe fn front(&self) -> &T {
        assert!(!self.is_empty());
        unsafe { &*self._data }
    }

    /// Returns the first element mutably.
    ///
    /// # Safety
    /// The caller must ensure the pointed-to memory is valid.
    #[inline]
    pub unsafe fn front_mut(&mut self) -> &mut T {
        assert!(!self.is_empty());
        unsafe { &mut *self._data }
    }

    /// Returns the last element.
    ///
    /// # Safety
    /// The caller must ensure the pointed-to memory is valid.
    #[inline]
    pub unsafe fn back(&self) -> &T {
        assert!(!self.is_empty());
        unsafe { &*self._data.add(self._size as usize - 1) }
    }

    /// Returns the last element mutably.
    ///
    /// # Safety
    /// The caller must ensure the pointed-to memory is valid.
    #[inline]
    pub unsafe fn back_mut(&mut self) -> &mut T {
        assert!(!self.is_empty());
        unsafe { &mut *self._data.add(self._size as usize - 1) }
    }
}

impl<T> Default for BSStaticArray<T> {
    fn default() -> Self {
        Self {
            _data: ptr::null_mut(),
            _size: 0,
            _pad0c: 0,
        }
    }
}

impl<T> Index<usize> for BSStaticArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self._size as usize);
        unsafe { &*self._data.add(index) }
    }
}

impl<T> IndexMut<usize> for BSStaticArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self._size as usize);
        unsafe { &mut *self._data.add(index) }
    }
}

// в”Ђв”Ђв”Ђ BSTSmallSharedArray<T> в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `RE::BSTSmallSharedArray<T>`.
///
/// An array optimized for 0 or 1 elements (stored inline) that falls back to
/// heap allocation for 2+ elements.
///
/// When `size > 1`, `_data.heap` is a pointer to a heap-allocated array.
/// When `size <= 1`, `_data.local` stores the single element inline.
///
/// # Safety
/// This type uses a union internally. Access methods are `unsafe` because
/// the caller must know whether the data is in inline or heap mode.
#[repr(C)]
pub struct BSTSmallSharedArray<T> {
    _size: u32,                        // 0x00
    _pad04: u32,                       // 0x04
    _data: BSTSmallSharedArrayData<T>, // 0x08
}

const _: () = assert!(core::mem::size_of::<BSTSmallSharedArray<u8>>() == 0x10);

/// Internal union for `BSTSmallSharedArray`.
#[repr(C)]
pub union BSTSmallSharedArrayData<T> {
    pub heap: *mut T,
    pub local: ManuallyDrop<MaybeUninit<T>>,
}

impl<T> BSTSmallSharedArray<T> {
    /// Number of elements.
    #[inline(always)]
    pub fn len(&self) -> u32 {
        self._size
    }

    /// C++ `size()`.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        self.len()
    }

    /// Whether the array is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self._size == 0
    }

    /// C++ `empty()`.
    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.is_empty()
    }

    /// Returns a pointer to the data.
    /// If `size > 1`, returns the heap pointer.
    /// If `size <= 1`, returns the address of the inline local storage.
    ///
    /// # Safety
    /// The caller must not dereference the returned pointer unless the pointed-to
    /// storage is initialized for `size()` elements.
    #[inline]
    pub unsafe fn data(&self) -> *const T {
        unsafe {
            if self._size > 1 {
                self._data.heap
            } else {
                (*self._data.local).as_ptr()
            }
        }
    }

    /// Returns a mutable pointer to the data.
    ///
    /// # Safety
    /// The caller must not dereference the returned pointer unless the pointed-to
    /// storage is initialized for `size()` elements.
    #[inline]
    pub unsafe fn data_mut(&mut self) -> *mut T {
        unsafe {
            if self._size > 1 {
                self._data.heap
            } else {
                (*self._data.local).as_mut_ptr()
            }
        }
    }

    /// Raw begin iterator.
    ///
    /// # Safety
    /// Same contract as `data()`.
    #[inline]
    pub unsafe fn begin(&self) -> *const T {
        unsafe { self.data() }
    }

    /// Raw mutable begin iterator.
    ///
    /// # Safety
    /// Same contract as `data_mut()`.
    #[inline]
    pub unsafe fn begin_mut(&mut self) -> *mut T {
        unsafe { self.data_mut() }
    }

    /// Raw end iterator.
    ///
    /// # Safety
    /// Same contract as `data()`.
    #[inline]
    pub unsafe fn end(&self) -> *const T {
        unsafe { self.data().add(self._size as usize) }
    }

    /// Raw mutable end iterator.
    ///
    /// # Safety
    /// Same contract as `data_mut()`.
    #[inline]
    pub unsafe fn end_mut(&mut self) -> *mut T {
        unsafe { self.data_mut().add(self._size as usize) }
    }

    /// Raw const begin iterator.
    ///
    /// # Safety
    /// Same contract as `data()`.
    #[inline]
    pub unsafe fn cbegin(&self) -> *const T {
        unsafe { self.begin() }
    }

    /// Raw const end iterator.
    ///
    /// # Safety
    /// Same contract as `data()`.
    #[inline]
    pub unsafe fn cend(&self) -> *const T {
        unsafe { self.end() }
    }

    /// Returns a slice of the array contents.
    ///
    /// # Safety
    /// The caller must ensure the data is properly initialized.
    #[inline]
    pub unsafe fn as_slice(&self) -> &[T] {
        unsafe {
            if self.is_empty() {
                &[]
            } else {
                let data = self.data();
                if data.is_null() {
                    crate::defensive_sdk_warn!(
                        "BSTSmallSharedArray<{}>::as_slice observed null data with non-zero len={}",
                        core::any::type_name::<T>(),
                        self._size
                    );
                    &[]
                } else {
                    core::slice::from_raw_parts(data, self._size as usize)
                }
            }
        }
    }

    /// Returns a mutable slice of the array contents.
    ///
    /// # Safety
    /// Same as `as_slice`, plus exclusive access must be guaranteed.
    #[inline]
    pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            if self.is_empty() {
                &mut []
            } else {
                let data = self.data_mut();
                if data.is_null() {
                    crate::defensive_sdk_warn!(
                        "BSTSmallSharedArray<{}>::as_mut_slice observed null data with non-zero len={}",
                        core::any::type_name::<T>(),
                        self._size
                    );
                    &mut []
                } else {
                    core::slice::from_raw_parts_mut(data, self._size as usize)
                }
            }
        }
    }

    /// Returns the first element.
    ///
    /// # Safety
    /// The caller must ensure the pointed-to memory is valid.
    #[inline]
    pub unsafe fn front(&self) -> &T {
        assert!(!self.is_empty());
        unsafe { &*self.data() }
    }

    /// Returns the first element mutably.
    ///
    /// # Safety
    /// The caller must ensure the pointed-to memory is valid.
    #[inline]
    pub unsafe fn front_mut(&mut self) -> &mut T {
        assert!(!self.is_empty());
        unsafe { &mut *self.data_mut() }
    }

    /// Returns the last element.
    ///
    /// # Safety
    /// The caller must ensure the pointed-to memory is valid.
    #[inline]
    pub unsafe fn back(&self) -> &T {
        assert!(!self.is_empty());
        unsafe { &*self.data().add(self._size as usize - 1) }
    }

    /// Returns the last element mutably.
    ///
    /// # Safety
    /// The caller must ensure the pointed-to memory is valid.
    #[inline]
    pub unsafe fn back_mut(&mut self) -> &mut T {
        assert!(!self.is_empty());
        unsafe { &mut *self.data_mut().add(self._size as usize - 1) }
    }
}

impl<T> Default for BSTSmallSharedArray<T> {
    fn default() -> Self {
        Self {
            _size: 0,
            _pad04: 0,
            _data: BSTSmallSharedArrayData {
                local: ManuallyDrop::new(MaybeUninit::uninit()),
            },
        }
    }
}

impl<T> Index<usize> for BSTSmallSharedArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self._size as usize);
        unsafe { &*self.data().add(index) }
    }
}

impl<T> IndexMut<usize> for BSTSmallSharedArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self._size as usize);
        unsafe { &mut *self.data_mut().add(index) }
    }
}
