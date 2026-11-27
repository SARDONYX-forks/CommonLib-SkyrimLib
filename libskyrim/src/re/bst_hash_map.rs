//! Translation of `RE::BSTHashMap.h` вЂ” the scatter table implementation.
//!
//! Contains:
//! - `BSTScatterTableStandardParent` / `BSTScatterTableFixedParent` вЂ” metadata layouts
//! - `BSTScatterTableEntry<V>` вЂ” entry type with value + next pointer for chaining
//! - `BSTScatterTableHeapAllocator` / `BSTScatterTableScrapAllocator` вЂ” table allocators
//! - `BSTScatterTable<K, V, ...>` вЂ” the core hash table
//! - Type aliases: `BSTHashMap`, `BSTSet`, `BSTFixedHashMap`, `BSTScrapHashMap`

use alloc::vec::Vec;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::ptr;

use crate::re::bst_tuple::BSTTuple;
use crate::re::crc::BSTHash;
use crate::re::memory_manager;
use crate::re::scrap_heap::ScrapHeap;

// в”Ђв”Ђв”Ђ Sentinel в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ
//
// The engine uses a sentinel value `{ 0xDE, 0xAD, 0xBE, 0xEF }` to mark the
// end of a chain. The actual sentinel pointer is stored in each table's
// `_sentinel` field and is initialized at construction time (C++ side).
// `entry.next == sentinel` в†’ "end of chain" (vs null в†’ "empty slot").
//
static BST_SCATTER_TABLE_SENTINEL: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];

// в”Ђв”Ђв”Ђ Parent Structs в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `RE::BSTScatterTableStandardParent`.
///
/// Layout:
/// ```text
/// 0x00: _pad00     u64
/// 0x08: _pad08     u32
/// 0x0C: _capacity  u32  вЂ” total slots (power of 2)
/// 0x10: _free      u32  вЂ” free slots remaining
/// 0x14: _good      u32  вЂ” last free index hint
/// ```
/// Size: `0x18`
#[repr(C)]
pub struct BSTScatterTableStandardParent {
    _pad00: u64,        // 0x00
    _pad08: u32,        // 0x08
    pub _capacity: u32, // 0x0C
    pub _free: u32,     // 0x10
    pub _good: u32,     // 0x14
}

const _: () = assert!(core::mem::size_of::<BSTScatterTableStandardParent>() == 0x18);

impl BSTScatterTableStandardParent {
    pub const fn new() -> Self {
        Self {
            _pad00: 0,
            _pad08: 0,
            _capacity: 0,
            _free: 0,
            _good: 0,
        }
    }
}

/// C++ `RE::BSTScatterTableFixedParent`.
///
/// Same fields as Standard but in a different order & smaller pad.
///
/// Layout:
/// ```text
/// 0x00: _pad00     u32
/// 0x04: _free      u32
/// 0x08: _good      u32
/// 0x0C: _capacity  u32
/// ```
/// Size: `0x10`
#[repr(C)]
pub struct BSTScatterTableFixedParent {
    _pad00: u32,        // 0x00
    pub _capacity: u32, // 0x04 - total # of slots, always a power of 2
    pub _free: u32,     // 0x08 - # of free slots
    pub _good: u32,     // 0x0C - last free index
}

const _: () = assert!(core::mem::size_of::<BSTScatterTableFixedParent>() == 0x10);

impl BSTScatterTableFixedParent {
    pub const fn new() -> Self {
        Self {
            _pad00: 0,
            _free: 0,
            _good: 0,
            _capacity: 0,
        }
    }
}

// в”Ђв”Ђв”Ђ Entry Type в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `BSTScatterTable::entry_type`.
///
/// Each slot in the scatter table. Contains an optional value and a `next` pointer:
/// - `next == null` в†’ slot is **empty** (no value stored)
/// - `next == sentinel (0xDEADBEEF)` в†’ slot has value, **end of chain**
/// - `next == some entry*` в†’ slot has value, chain continues at `next`
#[repr(C)]
pub struct BSTScatterTableEntry<V> {
    pub value: core::mem::MaybeUninit<V>,
    pub next: *mut BSTScatterTableEntry<V>,
}

impl<V> BSTScatterTableEntry<V> {
    /// Returns true if this entry has a value (next is non-null).
    #[inline(always)]
    pub fn has_value(&self) -> bool {
        !self.next.is_null()
    }

    /// Destroys the value in this entry and sets next to null.
    ///
    /// # Safety
    /// Caller must ensure the entry has a value.
    pub unsafe fn destroy(&mut self) {
        unsafe {
            if self.has_value() {
                ptr::drop_in_place(self.value.as_mut_ptr());
                self.next = ptr::null_mut();
            }
        }
    }

    /// Emplaces a value into this entry with the given next pointer.
    ///
    /// # Safety
    /// Caller must ensure this entry is empty or has been destroyed.
    pub unsafe fn emplace(&mut self, value: V, next: *mut BSTScatterTableEntry<V>) {
        unsafe {
            self.destroy();
            self.value.as_mut_ptr().write(value);
            self.next = next;
        }
    }

    /// Moves the value out of this entry, destroying it.
    ///
    /// # Safety
    /// Caller must ensure the entry has a value.
    pub unsafe fn steal(&mut self) -> V {
        unsafe {
            debug_assert!(self.has_value());
            let val = ptr::read(self.value.as_ptr());
            self.next = ptr::null_mut();
            val
        }
    }
}

// в”Ђв”Ђв”Ђ Scatter Table Allocator Trait в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// Trait for scatter table allocators.
///
/// # Safety
/// Implementations must return properly aligned memory for entry arrays.
pub unsafe trait BSTScatterTableAllocatorTrait {
    const MIN_SIZE: u32;
    fn get_entries(&self) -> *mut u8;
    fn set_entries(&mut self, entries: *mut u8);
    fn allocate_bytes(&mut self, bytes: usize) -> *mut u8;
    fn deallocate_bytes(&mut self, ptr: *mut u8);
}

// в”Ђв”Ђв”Ђ BSTScatterTableHeapAllocator в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `RE::BSTScatterTableHeapAllocator<S, A>`.
///
/// Layout: `{ _pad00: u64, _entries: *mut u8 }` вЂ” at offsets 0x20/0x28 relative to the table.
/// (0x00/0x08 relative to the allocator field itself)
#[repr(C)]
pub struct BSTScatterTableHeapAllocator {
    _pad00: u64,       // 0x00
    _entries: *mut u8, // 0x08
}

const _: () = assert!(core::mem::size_of::<BSTScatterTableHeapAllocator>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSTScatterTableHeapAllocator, _entries) == 0x08);

impl Default for BSTScatterTableHeapAllocator {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl BSTScatterTableHeapAllocator {
    pub const fn new() -> Self {
        Self {
            _pad00: 0,
            _entries: ptr::null_mut(),
        }
    }
}

unsafe impl BSTScatterTableAllocatorTrait for BSTScatterTableHeapAllocator {
    const MIN_SIZE: u32 = 1 << 3; // 8

    #[inline(always)]
    fn get_entries(&self) -> *mut u8 {
        self._entries
    }

    #[inline(always)]
    fn set_entries(&mut self, entries: *mut u8) {
        self._entries = entries;
    }

    fn allocate_bytes(&mut self, bytes: usize) -> *mut u8 {
        unsafe {
            let mem = crate::ffi::commonlib_malloc(bytes);
            mem as *mut u8
        }
    }

    fn deallocate_bytes(&mut self, ptr: *mut u8) {
        unsafe {
            crate::ffi::commonlib_free(ptr as *mut c_void);
        }
    }
}

// в”Ђв”Ђв”Ђ BSTScatterTableScrapAllocator в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `RE::BSTScatterTableScrapAllocator<S, A>`.
///
/// Layout: `{ _allocator: *mut ScrapHeap, _entries: *mut u8 }`
#[repr(C)]
pub struct BSTScatterTableScrapAllocator {
    _allocator: *mut ScrapHeap, // 0x00
    _entries: *mut u8,          // 0x08
}

const _: () = assert!(core::mem::size_of::<BSTScatterTableScrapAllocator>() == 0x10);
const _: () = assert!(core::mem::offset_of!(BSTScatterTableScrapAllocator, _allocator) == 0x00);
const _: () = assert!(core::mem::offset_of!(BSTScatterTableScrapAllocator, _entries) == 0x08);

impl BSTScatterTableScrapAllocator {
    pub fn new() -> Self {
        // Initialize with the current thread's scrap heap, like the C++ default
        let allocator = unsafe {
            let mgr = memory_manager::MemoryManager::get_singleton();
            if mgr.is_null() {
                ptr::null_mut()
            } else {
                (*mgr).get_thread_scrap_heap()
            }
        };
        Self {
            _allocator: allocator,
            _entries: ptr::null_mut(),
        }
    }
}

impl Default for BSTScatterTableScrapAllocator {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl BSTScatterTableAllocatorTrait for BSTScatterTableScrapAllocator {
    const MIN_SIZE: u32 = 1 << 3; // 8

    #[inline(always)]
    fn get_entries(&self) -> *mut u8 {
        self._entries
    }

    #[inline(always)]
    fn set_entries(&mut self, entries: *mut u8) {
        self._entries = entries;
    }

    fn allocate_bytes(&mut self, bytes: usize) -> *mut u8 {
        assert!(!self._allocator.is_null());
        unsafe { (*self._allocator).allocate(bytes, 0x10) as *mut u8 }
    }

    fn deallocate_bytes(&mut self, ptr: *mut u8) {
        assert!(!self._allocator.is_null());
        unsafe {
            (*self._allocator).deallocate(ptr as *mut c_void);
        }
    }
}

// в”Ђв”Ђв”Ђ BSTStaticHashMapAllocator в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// Aligned buffer wrapper for inline storage.
/// Uses 8-byte alignment (pointer alignment on x64), which covers all
/// entry types since `BSTScatterTableEntry<V>` always contains a pointer.
#[repr(C, align(8))]
pub struct AlignedEntryBuffer<const BUF: usize> {
    data: [u8; BUF],
}

impl<const BUF: usize> AlignedEntryBuffer<BUF> {
    pub const fn new() -> Self {
        Self { data: [0u8; BUF] }
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
}

/// C++ `RE::BSTStaticHashMapBase<N>::Allocator<S, A>`.
///
/// An allocator with a fixed inline buffer. All entries live in the inline
/// storage вЂ” no heap allocation is performed. If the requested allocation
/// exceeds the buffer, `allocate_bytes` returns null.
///
/// Const generics:
/// - `N`: number of entries (must be a power of 2) вЂ” used as `min_size()`
/// - `BUF`: total buffer size in bytes (`N * sizeof(entry_type)`)
///
/// Layout: `{ _buffer: [u8; BUF] (align 8), _entries: *mut u8 }`
#[repr(C)]
pub struct BSTStaticHashMapAllocator<const N: u32, const BUF: usize> {
    _buffer: AlignedEntryBuffer<BUF>, // 0x00
    _entries: *mut u8,                // BUF (aligned)
}

impl<const N: u32, const BUF: usize> BSTStaticHashMapAllocator<N, BUF> {
    pub const fn new() -> Self {
        Self {
            _buffer: AlignedEntryBuffer::new(),
            _entries: ptr::null_mut(),
        }
    }
}

impl<const N: u32, const BUF: usize> Default for BSTStaticHashMapAllocator<N, BUF> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<const N: u32, const BUF: usize> BSTScatterTableAllocatorTrait
    for BSTStaticHashMapAllocator<N, BUF>
{
    const MIN_SIZE: u32 = N;

    #[inline(always)]
    fn get_entries(&self) -> *mut u8 {
        self._entries
    }

    #[inline(always)]
    fn set_entries(&mut self, entries: *mut u8) {
        self._entries = entries;
    }

    fn allocate_bytes(&mut self, bytes: usize) -> *mut u8 {
        if bytes <= BUF {
            self._buffer.as_mut_ptr()
        } else {
            ptr::null_mut()
        }
    }

    fn deallocate_bytes(&mut self, _ptr: *mut u8) {
        // Static buffer вЂ” nothing to free.
        // In C++ this asserts ptr == _buffer, but we skip since
        // the buffer is embedded in the struct.
    }
}

// в”Ђв”Ђв”Ђ BSTScatterTable Traits в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// Trait for extracting the key from a value in the scatter table.
/// Maps: value = BSTTuple<K, V>, key = K.
/// Sets: value = K, key = K.
pub trait BSTScatterTableTraits {
    type Key;
    type Value;
    fn unwrap_key(value: &Self::Value) -> &Self::Key;
}

/// Map traits: `BSTScatterTableTraits<Key, T>` вЂ” value is `BSTTuple<Key, T>`.
pub struct BSTMapTraits<K, V>(PhantomData<(K, V)>);

impl<K, V> BSTScatterTableTraits for BSTMapTraits<K, V> {
    type Key = K;
    type Value = BSTTuple<K, V>;

    #[inline(always)]
    fn unwrap_key(value: &BSTTuple<K, V>) -> &K {
        &value.first
    }
}

/// Set traits: `BSTSetTraits<Key>` вЂ” value IS the key.
pub struct BSTSetTraits<K>(PhantomData<K>);

impl<K> BSTScatterTableTraits for BSTSetTraits<K> {
    type Key = K;
    type Value = K;

    #[inline(always)]
    fn unwrap_key(value: &K) -> &K {
        value
    }
}

// в”Ђв”Ђв”Ђ Parent Trait в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// Trait to abstract over parent struct field access.
pub trait BSTScatterTableParent {
    fn capacity(&self) -> u32;
    fn free(&self) -> u32;
    fn good(&self) -> u32;
    fn set_capacity(&mut self, v: u32);
    fn set_free(&mut self, v: u32);
    fn set_good(&mut self, v: u32);
}

impl BSTScatterTableParent for BSTScatterTableStandardParent {
    #[inline(always)]
    fn capacity(&self) -> u32 {
        self._capacity
    }
    #[inline(always)]
    fn free(&self) -> u32 {
        self._free
    }
    #[inline(always)]
    fn good(&self) -> u32 {
        self._good
    }
    #[inline(always)]
    fn set_capacity(&mut self, v: u32) {
        self._capacity = v;
    }
    #[inline(always)]
    fn set_free(&mut self, v: u32) {
        self._free = v;
    }
    #[inline(always)]
    fn set_good(&mut self, v: u32) {
        self._good = v;
    }
}

impl BSTScatterTableParent for BSTScatterTableFixedParent {
    #[inline(always)]
    fn capacity(&self) -> u32 {
        self._capacity
    }
    #[inline(always)]
    fn free(&self) -> u32 {
        self._free
    }
    #[inline(always)]
    fn good(&self) -> u32 {
        self._good
    }
    #[inline(always)]
    fn set_capacity(&mut self, v: u32) {
        self._capacity = v;
    }
    #[inline(always)]
    fn set_free(&mut self, v: u32) {
        self._free = v;
    }
    #[inline(always)]
    fn set_good(&mut self, v: u32) {
        self._good = v;
    }
}

// в”Ђв”Ђв”Ђ BSTScatterTable в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `RE::BSTScatterTable<Hash, KeyEqual, Traits, Allocator, Parent>`.
///
/// An open-addressing hash table with chaining for collision resolution.
/// Uses a sentinel pointer (`0xDEADBEEF`) to mark the end of chains.
///
/// Memory layout: `{ parent_fields..., _sentinel: *const entry, _allocator: Alloc }`
#[repr(C)]
pub struct BSTScatterTable<
    T: BSTScatterTableTraits,
    A: BSTScatterTableAllocatorTrait,
    P: BSTScatterTableParent,
> {
    _parent: P,
    _sentinel: *const BSTScatterTableEntry<T::Value>,
    _allocator: A,
}

impl<T, A, P> BSTScatterTable<T, A, P>
where
    T: BSTScatterTableTraits,
    A: BSTScatterTableAllocatorTrait + Default,
    P: BSTScatterTableParent + Default,
{
    // в”Ђв”Ђ Public API в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

    /// Creates a new empty scatter table.
    pub fn new() -> Self {
        Self {
            _parent: P::default(),
            // РЇРІРЅС‹Р№ РєР°СЃС‚: Р±РµСЂРµРј СЃС‹СЂРѕР№ СѓРєР°Р·Р°С‚РµР»СЊ РЅР° u8 РёР· РјР°СЃСЃРёРІР°,
            // Р° Р·Р°С‚РµРј РїСЂРёРІРѕРґРёРј РµРіРѕ Рє С‚РёРїРёР·РёСЂРѕРІР°РЅРЅРѕРјСѓ СѓРєР°Р·Р°С‚РµР»СЋ РЅР° Entry.
            _sentinel: BST_SCATTER_TABLE_SENTINEL.as_ptr() as *const BSTScatterTableEntry<T::Value>,
            _allocator: A::default(),
        }
    }
    /// Returns the number of elements in the table.
    #[inline(always)]
    pub fn size(&self) -> u32 {
        self._parent.capacity().wrapping_sub(self._parent.free())
    }

    /// Returns the total capacity (number of slots).
    #[inline(always)]
    pub fn capacity(&self) -> u32 {
        self._parent.capacity()
    }

    /// Returns true if the table is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.is_empty()
    }

    pub fn get(&self, key: &T::Key) -> Option<&T::Value>
    where
        T::Key: BSTHash + PartialEq,
    {
        let value = self.find(key);
        if value.is_null() {
            None
        } else {
            Some(unsafe { &*value })
        }
    }

    pub fn get_mut(&mut self, key: &T::Key) -> Option<&mut T::Value>
    where
        T::Key: BSTHash + PartialEq,
    {
        let value = self.find_mut(key);
        if value.is_null() {
            None
        } else {
            Some(unsafe { &mut *value })
        }
    }

    /// Looks up a key in the table. Returns a pointer to the value, or null if not found.
    pub fn find(&self, key: &T::Key) -> *const T::Value
    where
        T::Key: BSTHash + PartialEq,
    {
        if self.is_empty() {
            return ptr::null();
        }

        let entries = self.get_entries();
        if entries.is_null() {
            return ptr::null();
        }

        unsafe {
            let entry = self.get_entry_for(key);
            if !(*entry).has_value() {
                return ptr::null();
            }

            let mut current = entry;
            loop {
                if T::unwrap_key(&*(*current).value.as_ptr()) == key {
                    return (*current).value.as_ptr();
                }
                let next = (*current).next;
                if next == self._sentinel as *mut _ {
                    return ptr::null();
                }
                current = next;
            }
        }
    }

    /// Looks up a key and returns a mutable pointer to the value.
    pub fn find_mut(&mut self, key: &T::Key) -> *mut T::Value
    where
        T::Key: BSTHash + PartialEq,
    {
        self.find(key) as *mut T::Value
    }

    /// Returns true if the table contains the given key.
    pub fn contains(&self, key: &T::Key) -> bool
    where
        T::Key: BSTHash + PartialEq,
    {
        !self.find(key).is_null()
    }

    /// Inserts a value into the table. Returns true if inserted, false if key already existed.
    ///
    /// # Safety
    /// The table must be in a valid state.
    pub unsafe fn insert(&mut self, value: T::Value) -> bool
    where
        T::Key: BSTHash + PartialEq,
    {
        unsafe {
            let key = T::unwrap_key(&value);

            // Check if already exists
            if !self.find(key).is_null() {
                return false;
            }

            // Grow if no free entries
            if self._parent.free() == 0 {
                self.reserve(self._parent.capacity() + 1);
                assert!(self._parent.free() > 0);
            }

            let sentinel = self._sentinel as *mut BSTScatterTableEntry<T::Value>;
            let entry = self.get_entry_for(key);

            self._parent.set_free(self._parent.free() - 1);

            if (*entry).has_value() {
                // Slot is taken вЂ” resolve conflict
                let free = self.get_free_entry();

                let existing_key = T::unwrap_key(&*(*entry).value.as_ptr());
                let wouldve = self.get_entry_for(existing_key);

                if wouldve == entry {
                    // Hash collision вЂ” chain the new entry
                    let old_next = (*entry).next;
                    (*entry).next = free;
                    (*free).emplace(value, old_next);
                } else {
                    // Robin Hood: evict current to free, insert new at home position
                    // Find the previous entry in the chain that points to `entry`
                    let mut prev = wouldve;
                    while (*prev).next != entry {
                        prev = (*prev).next;
                    }

                    // Move current entry to free slot
                    let old_next = (*entry).next;
                    let stolen_value = (*entry).steal();
                    (*free).emplace(stolen_value, old_next);
                    (*prev).next = free;

                    // Place new value at home position
                    (*entry).emplace(value, sentinel);
                }
            } else {
                // Empty slot вЂ” just place it
                (*entry).emplace(value, sentinel);
            }

            true
        }
    }

    /// Removes a key from the table. Returns true if the key was found and removed.
    ///
    /// # Safety
    /// The table must be in a valid state.
    pub unsafe fn erase(&mut self, key: &T::Key) -> bool
    where
        T::Key: BSTHash + PartialEq,
    {
        unsafe {
            if self.is_empty() {
                return false;
            }

            let entries = self.get_entries();
            if entries.is_null() {
                return false;
            }

            let entry = self.get_entry_for(key);
            if !(*entry).has_value() {
                return false;
            }

            // Find the entry with matching key in the chain
            let sentinel = self._sentinel as *mut BSTScatterTableEntry<T::Value>;

            // Check if the head of chain matches
            if T::unwrap_key(&*(*entry).value.as_ptr()) == key {
                if (*entry).next == sentinel {
                    // Only entry in chain вЂ” just destroy
                    (*entry).destroy();
                } else {
                    // Move next entry into current slot
                    let next = (*entry).next;
                    let next_next = (*next).next;
                    let next_val = (*next).steal();
                    (*entry).destroy();
                    (*entry).emplace(next_val, next_next);
                }
                self._parent.set_free(self._parent.free() + 1);
                return true;
            }

            // Search the chain for the key
            let mut prev = entry;
            let mut current = (*entry).next;
            while current != sentinel {
                if T::unwrap_key(&*(*current).value.as_ptr()) == key {
                    // Found it вЂ” unlink from chain
                    (*prev).next = (*current).next;
                    (*current).destroy();
                    self._parent.set_free(self._parent.free() + 1);
                    return true;
                }
                prev = current;
                current = (*current).next;
            }

            false
        }
    }

    /// Removes all entries but keeps the allocated storage.
    ///
    /// # Safety
    /// The table must be in a valid state.
    pub unsafe fn clear(&mut self) {
        unsafe {
            if self.size() > 0 {
                let entries = self.get_entries() as *mut BSTScatterTableEntry<T::Value>;
                assert!(!entries.is_null());
                let cap = self._parent.capacity();
                for i in 0..cap {
                    (*entries.add(i as usize)).destroy();
                }
                self._parent.set_free(cap);
                self._parent.set_good(0);
            }
        }
    }

    /// Reserves capacity for at least `count` entries (rehashes if needed).
    ///
    /// # Safety
    /// The table must be in a valid state.
    pub unsafe fn reserve(&mut self, count: u32)
    where
        T::Key: BSTHash + PartialEq,
    {
        unsafe {
            if count <= self._parent.capacity() {
                return;
            }

            let old_cap = self._parent.capacity();
            let old_entries = self.get_entries();
            let old_size = self.size();

            // Calculate new capacity: next power of 2 >= max(count, MIN_SIZE)
            let min = A::MIN_SIZE as u64;
            let mut new_cap = if (count as u64) < min {
                min
            } else {
                count as u64
            };
            new_cap = new_cap.next_power_of_two();
            if new_cap > (1u64 << 31) {
                panic!("BSTScatterTable: buffer grew too large");
            }
            let new_cap = new_cap as u32;

            // Allocate new entries
            let entry_size = core::mem::size_of::<BSTScatterTableEntry<T::Value>>();
            let new_entries_raw = self
                ._allocator
                .allocate_bytes(entry_size * new_cap as usize);
            if new_entries_raw.is_null() {
                panic!("BSTScatterTable: allocation failed");
            }
            let new_entries = new_entries_raw as *mut BSTScatterTableEntry<T::Value>;

            // Zero-initialize new entries (next = null в†’ all empty)
            if !old_entries.is_null() && ptr::eq(new_entries_raw, old_entries as *mut u8) {
                let old_typed = old_entries as *mut BSTScatterTableEntry<T::Value>;
                if new_cap > old_cap {
                    ptr::write_bytes(
                        old_typed.add(old_cap as usize),
                        0,
                        (new_cap - old_cap) as usize,
                    );
                }

                let mut todo = Vec::with_capacity(old_size as usize);
                for i in 0..old_cap {
                    let entry = &mut *old_typed.add(i as usize);
                    if entry.has_value() {
                        todo.push(entry.steal());
                    }
                }

                self._parent.set_capacity(new_cap);
                self._parent.set_free(new_cap);
                self._parent.set_good(0);
                self._allocator.set_entries(new_entries_raw);

                for value in todo {
                    debug_assert!(self.insert(value));
                }
            } else {
                ptr::write_bytes(new_entries, 0, new_cap as usize);

                self._parent.set_capacity(new_cap);
                self._parent.set_free(new_cap);
                self._parent.set_good(0);
                self._allocator.set_entries(new_entries_raw);

                if !old_entries.is_null() {
                    let old_typed = old_entries as *mut BSTScatterTableEntry<T::Value>;
                    for i in 0..old_cap {
                        let entry = &mut *old_typed.add(i as usize);
                        if entry.has_value() {
                            let val = entry.steal();
                            debug_assert!(self.insert(val));
                        }
                    }
                    self._allocator.deallocate_bytes(old_entries as *mut u8);
                }
            }
        }
    }

    // в”Ђв”Ђ Iteration в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

    /// Returns an iterator over immutable references to values.
    pub fn iter(&self) -> BSTScatterTableIter<'_, T::Value> {
        let entries = self.get_entries();
        if entries.is_null() || self._parent.capacity() == 0 {
            BSTScatterTableIter {
                current: ptr::null(),
                end: ptr::null(),
                _marker: PhantomData,
            }
        } else {
            let typed = entries as *const BSTScatterTableEntry<T::Value>;
            unsafe {
                BSTScatterTableIter {
                    current: typed,
                    end: typed.add(self._parent.capacity() as usize),
                    _marker: PhantomData,
                }
            }
        }
    }

    /// Returns an iterator over mutable references to values.
    pub fn iter_mut(&mut self) -> BSTScatterTableIterMut<'_, T::Value> {
        let entries = self.get_entries();
        if entries.is_null() || self._parent.capacity() == 0 {
            BSTScatterTableIterMut {
                current: ptr::null_mut(),
                end: ptr::null_mut(),
                _marker: PhantomData,
            }
        } else {
            let typed = entries as *mut BSTScatterTableEntry<T::Value>;
            unsafe {
                BSTScatterTableIterMut {
                    current: typed,
                    end: typed.add(self._parent.capacity() as usize),
                    _marker: PhantomData,
                }
            }
        }
    }

    // в”Ђв”Ђ Private helpers в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

    #[inline(always)]
    fn get_entries(&self) -> *mut u8 {
        self._allocator.get_entries()
    }

    /// Gets the home entry for a key (hash & (capacity - 1)).
    #[inline]
    unsafe fn get_entry_for(&self, key: &T::Key) -> *mut BSTScatterTableEntry<T::Value>
    where
        T::Key: BSTHash + PartialEq,
    {
        unsafe {
            let entries = self.get_entries() as *mut BSTScatterTableEntry<T::Value>;
            let hash = key.bst_hash();
            let idx = hash & (self._parent.capacity() - 1);
            entries.add(idx as usize)
        }
    }

    /// Finds the next free entry, starting from `_good`.
    unsafe fn get_free_entry(&mut self) -> *mut BSTScatterTableEntry<T::Value> {
        unsafe {
            let entries = self.get_entries() as *mut BSTScatterTableEntry<T::Value>;
            let cap = self._parent.capacity();
            let mut good = self._parent.good();

            while (*entries.add(good as usize)).has_value() {
                good = (good + 1) & (cap - 1);
            }

            self._parent.set_good(good);
            entries.add(good as usize)
        }
    }
}

impl<T, A, P> Drop for BSTScatterTable<T, A, P>
where
    T: BSTScatterTableTraits,
    A: BSTScatterTableAllocatorTrait,
    P: BSTScatterTableParent,
{
    fn drop(&mut self) {
        let entries_raw = self._allocator.get_entries();
        if !entries_raw.is_null() && self._parent.capacity() > 0 {
            unsafe {
                let typed = entries_raw as *mut BSTScatterTableEntry<T::Value>;
                for i in 0..self._parent.capacity() {
                    (*typed.add(i as usize)).destroy();
                }
                self._allocator.deallocate_bytes(entries_raw);
            }
            self._allocator.set_entries(ptr::null_mut());
            self._parent.set_capacity(0);
            self._parent.set_free(0);
            self._parent.set_good(0);
        }
    }
}

// в”Ђв”Ђв”Ђ Iterators в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

impl<T, A, P> Default for BSTScatterTable<T, A, P>
where
    T: BSTScatterTableTraits,
    A: BSTScatterTableAllocatorTrait + Default,
    P: BSTScatterTableParent + Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A, P> Clone for BSTScatterTable<T, A, P>
where
    T: BSTScatterTableTraits,
    T::Value: Clone,
    T::Key: BSTHash + PartialEq,
    A: BSTScatterTableAllocatorTrait + Default,
    P: BSTScatterTableParent + Default,
{
    fn clone(&self) -> Self {
        let mut clone = Self::new();
        unsafe {
            clone.reserve(self.size());
            for value in self.iter() {
                debug_assert!(clone.insert(value.clone()));
            }
        }
        clone
    }
}

impl<T, A, P> core::iter::FromIterator<T::Value> for BSTScatterTable<T, A, P>
where
    T: BSTScatterTableTraits,
    T::Key: BSTHash + PartialEq,
    A: BSTScatterTableAllocatorTrait + Default,
    P: BSTScatterTableParent + Default,
{
    fn from_iter<I: IntoIterator<Item = T::Value>>(iter: I) -> Self {
        let mut table = Self::new();
        table.extend(iter);
        table
    }
}

impl<T, A, P> Extend<T::Value> for BSTScatterTable<T, A, P>
where
    T: BSTScatterTableTraits,
    T::Key: BSTHash + PartialEq,
    A: BSTScatterTableAllocatorTrait + Default,
    P: BSTScatterTableParent + Default,
{
    fn extend<I: IntoIterator<Item = T::Value>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        let (_, upper) = iter.size_hint();
        if let Some(upper) = upper {
            let additional = upper.min(u32::MAX as usize) as u32;
            unsafe {
                self.reserve(self.size().saturating_add(additional));
            }
        }

        for value in iter {
            unsafe {
                let _ = self.insert(value);
            }
        }
    }
}

impl<'a, T, A, P> IntoIterator for &'a BSTScatterTable<T, A, P>
where
    T: BSTScatterTableTraits,
    A: BSTScatterTableAllocatorTrait + Default,
    P: BSTScatterTableParent + Default,
{
    type Item = &'a T::Value;
    type IntoIter = BSTScatterTableIter<'a, T::Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, A, P> IntoIterator for &'a mut BSTScatterTable<T, A, P>
where
    T: BSTScatterTableTraits,
    A: BSTScatterTableAllocatorTrait + Default,
    P: BSTScatterTableParent + Default,
{
    type Item = &'a mut T::Value;
    type IntoIter = BSTScatterTableIterMut<'a, T::Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// Immutable iterator over scatter table values.
pub struct BSTScatterTableIter<'a, V> {
    current: *const BSTScatterTableEntry<V>,
    end: *const BSTScatterTableEntry<V>,
    _marker: PhantomData<&'a V>,
}

impl<'a, V> Iterator for BSTScatterTableIter<'a, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.current != self.end {
                let entry = &*self.current;
                self.current = self.current.add(1);
                if entry.has_value() {
                    return Some(&*entry.value.as_ptr());
                }
            }
            None
        }
    }
}

/// Mutable iterator over scatter table values.
pub struct BSTScatterTableIterMut<'a, V> {
    current: *mut BSTScatterTableEntry<V>,
    end: *mut BSTScatterTableEntry<V>,
    _marker: PhantomData<&'a mut V>,
}

impl<'a, V> Iterator for BSTScatterTableIterMut<'a, V> {
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.current != self.end {
                let entry = &mut *self.current;
                self.current = self.current.add(1);
                if entry.has_value() {
                    return Some(&mut *entry.value.as_mut_ptr());
                }
            }
            None
        }
    }
}

// в”Ђв”Ђв”Ђ Default for Parents в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

impl Default for BSTScatterTableStandardParent {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BSTScatterTableFixedParent {
    fn default() -> Self {
        Self::new()
    }
}

// в”Ђв”Ђв”Ђ Type Aliases в”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђв”Ђ

/// C++ `RE::BSTHashMap<Key, T>`.
/// Standard heap-allocated hash map.
pub type BSTHashMap<K, V> = BSTScatterTable<
    BSTMapTraits<K, V>,
    BSTScatterTableHeapAllocator,
    BSTScatterTableStandardParent,
>;

/// C++ `RE::BSTSet<Key>`.
/// Standard heap-allocated hash set.
pub type BSTSet<K> =
    BSTScatterTable<BSTSetTraits<K>, BSTScatterTableHeapAllocator, BSTScatterTableStandardParent>;

/// C++ `RE::BSTFixedHashMap<Key, T>`.
/// Heap-allocated hash map with fixed parent layout.
pub type BSTFixedHashMap<K, V> =
    BSTScatterTable<BSTMapTraits<K, V>, BSTScatterTableHeapAllocator, BSTScatterTableFixedParent>;

/// C++ `RE::BSTScrapHashMap<Key, T>`.
/// Scrap-heap-allocated hash map.
pub type BSTScrapHashMap<K, V> = BSTScatterTable<
    BSTMapTraits<K, V>,
    BSTScatterTableScrapAllocator,
    BSTScatterTableStandardParent,
>;

/// C++ `RE::BSTStaticHashMap<Key, T, N>`.
/// Hash map with a fixed inline buffer of `N` entries (no heap allocation).
///
/// Const generics:
/// - `N`: number of entries (must be a power of 2)
/// - `BUF`: buffer size in bytes = `N * core::mem::size_of::<BSTScatterTableEntry<BSTTuple<K, V>>>()`
///
/// Usage example for 8 entries of `(u32, u32)`:
/// ```ignore
/// type Entry = BSTScatterTableEntry<BSTTuple<u32, u32>>;
/// type MyMap = BSTStaticHashMap<u32, u32, 8, {8 * core::mem::size_of::<Entry>()}>;
/// ```
pub type BSTStaticHashMap<K, V, const N: u32, const BUF: usize> = BSTScatterTable<
    BSTMapTraits<K, V>,
    BSTStaticHashMapAllocator<N, BUF>,
    BSTScatterTableStandardParent,
>;

/// C++ type aliases for unknown key/value.
pub type UnkKey = usize;
pub type UnkValue = usize;

type BSTHashMapRepresentative = BSTHashMap<u32, u32>;
type BSTFixedHashMapRepresentative = BSTFixedHashMap<u32, u32>;
type BSTScrapHashMapRepresentative = BSTScrapHashMap<u32, u32>;
type BSTScatterTableRepresentativeEntry = BSTScatterTableEntry<BSTTuple<u32, u32>>;
type BSTStaticHashMapAllocatorRepresentative = BSTStaticHashMapAllocator<
    8,
    { 8 * core::mem::size_of::<BSTScatterTableRepresentativeEntry>() },
>;

const _: () = assert!(core::mem::size_of::<BSTScatterTableRepresentativeEntry>() == 0x10);
const _: () = assert!(core::mem::size_of::<BSTHashMapRepresentative>() == 0x30);
const _: () = assert!(core::mem::offset_of!(BSTHashMapRepresentative, _sentinel) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSTHashMapRepresentative, _allocator) == 0x20);
const _: () = assert!(core::mem::size_of::<BSTFixedHashMapRepresentative>() == 0x28);
const _: () = assert!(core::mem::offset_of!(BSTFixedHashMapRepresentative, _sentinel) == 0x10);
const _: () = assert!(core::mem::offset_of!(BSTFixedHashMapRepresentative, _allocator) == 0x18);
const _: () = assert!(core::mem::size_of::<BSTScrapHashMapRepresentative>() == 0x30);
const _: () = assert!(core::mem::offset_of!(BSTScrapHashMapRepresentative, _sentinel) == 0x18);
const _: () = assert!(core::mem::offset_of!(BSTScrapHashMapRepresentative, _allocator) == 0x20);
const _: () = assert!(core::mem::size_of::<BSTStaticHashMapAllocatorRepresentative>() == 0x88);
const _: () =
    assert!(core::mem::offset_of!(BSTStaticHashMapAllocatorRepresentative, _entries) == 0x80);
