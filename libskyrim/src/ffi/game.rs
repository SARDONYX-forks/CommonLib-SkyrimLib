//! Ports of CommonLib helpers that call into engine code: handle resolution,
//! by-value-handle virtual calls, form/UI lookups, ref-counting and small
//! factories.

use core::ffi::{c_char, c_void};
use core::sync::atomic::{AtomicU32, Ordering};

use crate::re::bs_atomic::{BSReadLockGuard, BSReadWriteLock};
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bst_hash_map::BSTHashMap;
use crate::re::tes_form::TESForm;
use crate::relocation::{RelocationID, VariantOffset};

// ---------------------------------------------------------------------------
// Virtual-call helpers

/// Reads slot `index` of the vtable of `this`.
#[inline(always)]
unsafe fn vfunc(this: *const c_void, index: usize) -> usize {
    unsafe {
        let vtable = *(this as *const *const usize);
        *vtable.add(index)
    }
}

/// Invokes the MSVC scalar deleting destructor (`vtable[0](this, 1)`), i.e.
/// `delete static_cast<Base*>(ptr)` for a polymorphic engine object.
pub(crate) unsafe fn delete_polymorphic(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let dtor: unsafe extern "C" fn(*mut c_void, u32) -> *mut c_void =
            core::mem::transmute(vfunc(ptr, 0));
        dtor(ptr, 1);
    }
}

// ---------------------------------------------------------------------------
// Actor / TESObjectREFR wrappers
//
// `ObjectRefHandle` and `ActorHandle` are non-trivial C++ classes (user
// provided move operations), so MSVC passes them by hidden reference and
// returns them through a hidden out pointer placed right after `this`.

pub unsafe fn commonlib_actor_get_gold_amount(actor: *mut c_void, no_init: bool) -> i32 {
    match unsafe { actor.cast::<crate::re::actor::Actor>().as_mut() } {
        Some(actor) => actor.get_gold_amount(no_init),
        None => 0,
    }
}

pub unsafe fn commonlib_actor_drop_object(
    actor: *mut c_void,
    out: *mut c_void,
    object: *const c_void,
    extra_list: *mut c_void,
    count: i32,
    drop_loc: *const c_void,
    rotate: *const c_void,
) {
    if out.is_null() {
        return;
    }
    unsafe {
        out.cast::<u32>().write(0);
        if actor.is_null() {
            return;
        }
        let index = VariantOffset::new_se_ae(0x0CB, 0x0CD).offset();
        let func: unsafe extern "C" fn(
            *mut c_void,
            *mut c_void,
            *const c_void,
            *mut c_void,
            i32,
            *const c_void,
            *const c_void,
        ) -> *mut c_void = core::mem::transmute(vfunc(actor, index));
        func(actor, out, object, extra_list, count, drop_loc, rotate);
    }
}

pub unsafe fn commonlib_actor_set_last_ridden_mount(actor: *mut c_void, mount: *const c_void) {
    if actor.is_null() {
        return;
    }
    unsafe {
        let handle: u32 = mount.cast::<u32>().as_ref().copied().unwrap_or(0);
        let index = VariantOffset::new_se_ae(0x113, 0x115).offset();
        let func: unsafe extern "C" fn(*mut c_void, *const u32) =
            core::mem::transmute(vfunc(actor, index));
        func(actor, &handle);
    }
}

pub unsafe fn commonlib_actor_q_last_ridden_mount(actor: *const c_void, out: *mut c_void) {
    if out.is_null() {
        return;
    }
    unsafe {
        out.cast::<u32>().write(0);
        if actor.is_null() {
            return;
        }
        let index = VariantOffset::new_se_ae(0x114, 0x116).offset();
        let func: unsafe extern "C" fn(*const c_void, *mut c_void) -> *mut c_void =
            core::mem::transmute(vfunc(actor, index));
        func(actor, out);
    }
}

pub unsafe fn commonlib_tes_object_refr_remove_item(
    refr: *mut c_void,
    out: *mut c_void,
    item: *mut c_void,
    count: i32,
    reason: i32,
    extra_list: *mut c_void,
    move_to_ref: *mut c_void,
    drop_loc: *const c_void,
    rotate: *const c_void,
) {
    if out.is_null() {
        return;
    }
    unsafe {
        out.cast::<u32>().write(0);
        if refr.is_null() {
            return;
        }
        let func: unsafe extern "C" fn(
            *mut c_void,
            *mut c_void,
            *mut c_void,
            i32,
            i32,
            *mut c_void,
            *mut c_void,
            *const c_void,
            *const c_void,
        ) -> *mut c_void = core::mem::transmute(vfunc(refr, 0x56));
        func(
            refr,
            out,
            item,
            count,
            reason,
            extra_list,
            move_to_ref,
            drop_loc,
            rotate,
        );
    }
}

// ---------------------------------------------------------------------------
// BSPointerHandleManagerInterface<T>::GetSmartPointer
//
// The engine functions are shared by every handle type (Actor, Projectile,
// TESObjectREFR): they only touch the 4-byte handle and the NiPointer slot.

crate::relocation_func! {
    fn get_smart_pointer_mut(handle: *mut u32, out: *mut *mut c_void) -> bool
        => RelocationID::new(12785, 12922)
}

crate::relocation_func! {
    fn get_smart_pointer_const(handle: *const u32, out: *mut *mut c_void) -> bool
        => RelocationID::new(12204, 12332)
}

unsafe fn handle_get_const(handle: *const c_void, out: *mut c_void) -> bool {
    if handle.is_null() || out.is_null() {
        return false;
    }
    unsafe {
        out.cast::<*mut c_void>().write(core::ptr::null_mut());
        get_smart_pointer_const(handle.cast(), out.cast())
    }
}

unsafe fn handle_get_mut(handle: *mut c_void, out: *mut c_void) -> bool {
    if handle.is_null() || out.is_null() {
        return false;
    }
    unsafe {
        out.cast::<*mut c_void>().write(core::ptr::null_mut());
        get_smart_pointer_mut(handle.cast(), out.cast())
    }
}

pub unsafe fn commonlib_actor_handle_get_smart_pointer_const(
    handle: *const c_void,
    out: *mut c_void,
) -> bool {
    unsafe { handle_get_const(handle, out) }
}

pub unsafe fn commonlib_actor_handle_get_smart_pointer_mut(
    handle: *mut c_void,
    out: *mut c_void,
) -> bool {
    unsafe { handle_get_mut(handle, out) }
}

pub unsafe fn commonlib_object_ref_handle_get_smart_pointer_const(
    handle: *const c_void,
    out: *mut c_void,
) -> bool {
    unsafe { handle_get_const(handle, out) }
}

pub unsafe fn commonlib_object_ref_handle_get_smart_pointer_mut(
    handle: *mut c_void,
    out: *mut c_void,
) -> bool {
    unsafe { handle_get_mut(handle, out) }
}

pub unsafe fn commonlib_projectile_handle_get_smart_pointer_const(
    handle: *const c_void,
    out: *mut c_void,
) -> bool {
    unsafe { handle_get_const(handle, out) }
}

pub unsafe fn commonlib_projectile_handle_get_smart_pointer_mut(
    handle: *mut c_void,
    out: *mut c_void,
) -> bool {
    unsafe { handle_get_mut(handle, out) }
}

// ---------------------------------------------------------------------------
// GRefCountImpl::AddRef / Release
//
// Every Scaleform ref-counted base keeps `_refCount` right after the vtable.

#[inline(always)]
unsafe fn gref_count(ptr: *mut c_void) -> &'static AtomicU32 {
    unsafe { &*ptr.cast::<u8>().add(8).cast::<AtomicU32>() }
}

unsafe fn gref_add_ref(ptr: *mut c_void) {
    if !ptr.is_null() {
        unsafe { gref_count(ptr).fetch_add(1, Ordering::AcqRel) };
    }
}

unsafe fn gref_release(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        if gref_count(ptr).fetch_sub(1, Ordering::AcqRel) == 1 {
            delete_polymorphic(ptr);
        }
    }
}

pub unsafe fn commonlib_gfx_movie_view_add_ref(movie_view: *mut c_void) {
    unsafe { gref_add_ref(movie_view) }
}

pub unsafe fn commonlib_gfx_movie_view_release(movie_view: *mut c_void) {
    unsafe { gref_release(movie_view) }
}

pub unsafe fn commonlib_fx_delegate_add_ref(delegate: *mut c_void) {
    unsafe { gref_add_ref(delegate) }
}

pub unsafe fn commonlib_fx_delegate_release(delegate: *mut c_void) {
    unsafe { gref_release(delegate) }
}

pub unsafe fn commonlib_fx_delegate_handler_add_ref(handler: *mut c_void) {
    unsafe { gref_add_ref(handler) }
}

pub unsafe fn commonlib_fx_delegate_handler_release(handler: *mut c_void) {
    unsafe { gref_release(handler) }
}

pub unsafe fn commonlib_imenu_add_ref(menu: *mut c_void) {
    unsafe { gref_add_ref(menu) }
}

pub unsafe fn commonlib_imenu_release(menu: *mut c_void) {
    unsafe { gref_release(menu) }
}

/// `delete static_cast<RE::GFxResource*>(resource)`
pub unsafe fn commonlib_gfx_resource_delete(resource: *mut c_void) {
    unsafe { delete_polymorphic(resource) }
}

/// `delete static_cast<RE::BSIInputDevice*>(device)`
pub unsafe fn commonlib_destroy_bsi_input_device(device: *mut c_void) {
    unsafe { delete_polymorphic(device) }
}

// ---------------------------------------------------------------------------
// Factories

crate::relocation_func! {
    fn bgs_attack_data_ctor(this: *mut c_void) -> *mut c_void => RelocationID::new(26718, 27397)
}

/// `RE::BGSAttackData::Create()`
pub unsafe fn commonlib_bgs_attack_data_create() -> *mut c_void {
    let size = core::mem::size_of::<crate::re::bgs_attack_data::BGSAttackData>();
    unsafe {
        let mem = super::commonlib_malloc(size);
        if mem.is_null() {
            return mem;
        }
        core::ptr::write_bytes(mem.cast::<u8>(), 0, size);
        bgs_attack_data_ctor(mem);
        mem
    }
}

/// `RE::make_hkref<RE::hkReferencedObject>()`
///
/// Allocates an `hkReferencedObject` on the engine heap and installs the
/// engine's own vtable, matching what `new hkReferencedObject()` produces.
pub unsafe fn commonlib_make_hkref_hk_referenced_object(out: *mut c_void) -> bool {
    use crate::re::hk_referenced_object::hkReferencedObject;

    if out.is_null() {
        return false;
    }
    unsafe {
        out.cast::<*mut c_void>().write(core::ptr::null_mut());
        let size = core::mem::size_of::<hkReferencedObject>();
        let mem = super::commonlib_malloc(size);
        if mem.is_null() {
            return false;
        }
        core::ptr::write_bytes(mem.cast::<u8>(), 0, size);
        let vtable = hkReferencedObject::VTABLE[0].address();
        mem.cast::<usize>().write(vtable);
        // hkReferencedObject(): memSizeAndFlags = 0, referenceCount = 1, then
        // `hkRefPtr(T*)` calls AddReference() -> 2. This matches C++ exactly.
        // (layout: vtbl 00, memSizeAndFlags u16 08, referenceCount i16 0A)
        mem.cast::<u8>().add(0x0A).cast::<i16>().write(2);
        out.cast::<*mut c_void>().write(mem);
        true
    }
}

/// `RE::make_nismart<RE::NiRefObject>()`
pub unsafe fn commonlib_make_nismart_ni_ref_object(out: *mut c_void) -> bool {
    use crate::re::ni_ref_object::NiRefObject;

    if out.is_null() {
        return false;
    }
    unsafe {
        out.cast::<*mut c_void>().write(core::ptr::null_mut());
        let size = core::mem::size_of::<NiRefObject>();
        let mem = super::commonlib_malloc(size);
        if mem.is_null() {
            return false;
        }
        core::ptr::write_bytes(mem.cast::<u8>(), 0, size);
        mem.cast::<usize>().write(NiRefObject::VTABLE[0].address());
        // NiRefObject(): ++objects; _refCount = 0. NiPointer then IncRefCount()s.
        let objects = RelocationID::new(523912, 410493).address() as *const AtomicU32;
        (*objects).fetch_add(1, Ordering::AcqRel);
        let ref_count = &*mem.cast::<u8>().add(8).cast::<AtomicU32>();
        ref_count.fetch_add(1, Ordering::AcqRel);
        out.cast::<*mut c_void>().write(mem);
        true
    }
}

// ---------------------------------------------------------------------------
// UI helpers

type FactoryEntry = crate::re::bst_hash_map::BSTScatterTableEntry<
    crate::re::bst_tuple::BSTTuple<BSFixedString, *const c_void>,
>;

/// C++ `RE::MessageDataFactoryManager`
/// (`BSTCreateFactoryManager<BSFixedString, IUIMessageData, 16>`).
// TODO: move into `re/bst_create_factory_manager.rs` once that CommonLib file is
// translated; only the `factories` map is needed by this helper.
#[repr(C)]
struct MessageDataFactoryManager {
    factories: crate::re::bst_hash_map::BSTStaticHashMap<
        BSFixedString,
        *const c_void,
        16,
        { 16 * core::mem::size_of::<FactoryEntry>() },
    >,
}

const _: () = assert!(core::mem::size_of::<MessageDataFactoryManager>() == 0x1A8);

crate::relocation_func! {
    fn message_data_factory_manager_get_singleton() -> *mut MessageDataFactoryManager
        => RelocationID::new(22843, 52875)
}

/// `RE::MessageDataFactoryManager::GetSingleton()->GetCreator(name)->Create()`
pub unsafe fn commonlib_create_ui_message_data(class_name: *const c_char) -> *mut c_void {
    if class_name.is_null() {
        return core::ptr::null_mut();
    }
    let Some(manager) = (unsafe { message_data_factory_manager_get_singleton().as_ref() }) else {
        return core::ptr::null_mut();
    };
    let name = BSFixedString::new(class_name);
    let creator = match unsafe { manager.factories.find(&name).as_ref() } {
        Some(entry) => entry.second,
        None => return core::ptr::null_mut(),
    };
    if creator.is_null() {
        return core::ptr::null_mut();
    }
    // IBSTCreator<T>: 00 dtor, 01 CreateImpl() const, 02 Destroy(const T*) const
    unsafe {
        let create: unsafe extern "C" fn(*const c_void) -> *mut c_void =
            core::mem::transmute(vfunc(creator, 1));
        create(creator)
    }
}

/// `RE::UI::Register(name, creator)`: `menuMap.insert({ name, { nullptr, creator } })`
pub unsafe fn commonlib_ui_register_menu(
    ui: *mut c_void,
    menu_name: *const c_char,
    creator: *mut c_void,
) -> bool {
    use crate::re::bst_tuple::BSTTuple;
    use crate::re::ui::{UI, UICreateFn, UIMenuEntry};

    let Some(ui) = (unsafe { ui.cast::<UI>().as_mut() }) else {
        return false;
    };
    if menu_name.is_null() {
        return false;
    }

    let create: Option<UICreateFn> = if creator.is_null() {
        None
    } else {
        Some(unsafe { core::mem::transmute::<*mut c_void, UICreateFn>(creator) })
    };
    let entry = BSTTuple {
        first: BSFixedString::new(menu_name),
        second: UIMenuEntry {
            menu: Default::default(),
            create,
        },
    };
    unsafe { ui.menu_map.insert(entry) };
    true
}

crate::relocation_func! {
    fn ui_message_queue_add_message(
        this: *mut c_void,
        menu: *const BSFixedString,
        message_type: i32,
        data: *mut c_void
    ) => RelocationID::new(13530, 13631)
}

/// `RE::UIMessageQueue::GetSingleton()->AddMessage(name, type, data)`
pub unsafe fn commonlib_ui_message_queue_add_message(
    menu_name: *const c_char,
    message_type: i32,
    data: *mut c_void,
) -> bool {
    if menu_name.is_null() {
        return false;
    }
    let queue = crate::re::ui_message_queue::UIMessageQueue::get_singleton();
    if queue.is_null() {
        return false;
    }
    let name = BSFixedString::new(menu_name);
    ui_message_queue_add_message(queue.cast(), &name, message_type, data);
    true
}

/// `RE::TESForm::LookupByEditorID(editor_id)`
pub unsafe fn commonlib_tes_form_lookup_by_editor_id(editor_id: *const c_char) -> *mut c_void {
    if editor_id.is_null() {
        return core::ptr::null_mut();
    }

    // C++: `REL::Relocation<BSTHashMap<...>**> allForms{514352, 400509}` and
    //      `REL::Relocation<BSReadWriteLock*> lock{514361, 400518}` where the
    //      lock lives *at* the relocated address.
    let map = unsafe {
        *(RelocationID::new(514352, 400509).address()
            as *const *mut BSTHashMap<BSFixedString, *mut TESForm>)
    };
    let lock = RelocationID::new(514361, 400518).address() as *mut BSReadWriteLock;

    let _guard = unsafe { BSReadLockGuard::from_ptr(lock) };
    let Some(map) = (unsafe { map.as_ref() }) else {
        return core::ptr::null_mut();
    };

    let key = BSFixedString::new(editor_id);
    match unsafe { map.find(&key).as_ref() } {
        Some(entry) => entry.second.cast(),
        None => core::ptr::null_mut(),
    }
}

/// `RE::NiControllerManager::GetSequenceByName(name)`: `stringMap.find(name)`
pub unsafe fn commonlib_ni_controller_manager_get_sequence_by_name(
    manager: *mut c_void,
    name: *const c_char,
) -> *mut c_void {
    use crate::re::ni_controller_manager::NiControllerManager;

    let Some(manager) = (unsafe { manager.cast::<NiControllerManager>().as_ref() }) else {
        return core::ptr::null_mut();
    };
    if name.is_null() {
        return core::ptr::null_mut();
    }
    let key = BSFixedString::new(name);
    match unsafe { manager.string_map.find(&key).as_ref() } {
        Some(entry) => entry.second.cast(),
        None => core::ptr::null_mut(),
    }
}
