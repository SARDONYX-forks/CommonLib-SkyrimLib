//! Rust port of `SKSE::Init` / `SKSE::Impl::API`: queries and caches the SKSE
//! interfaces handed to the plugin, and exposes the task helpers.

use core::ffi::c_void;
use core::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

use crate::skse::{
    InterfaceId, LoadInterface, MessagingDispatcher, MessagingInterface, ObjectInterface,
    TaskInterface,
};

macro_rules! api_slots {
    ($($name:ident),* $(,)?) => {
        $(static $name: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());)*
    };
}

api_slots!(
    SCALEFORM,
    PAPYRUS,
    SERIALIZATION,
    TASK,
    MESSAGING,
    OBJECT,
    TRAMPOLINE,
    DELAY_FUNCTOR_MANAGER,
    OBJECT_REGISTRY,
    PERSISTENT_OBJECT_STORAGE,
    MOD_CALLBACK_EVENT_SOURCE,
    CAMERA_EVENT_SOURCE,
    CROSSHAIR_REF_EVENT_SOURCE,
    ACTION_EVENT_SOURCE,
    NI_NODE_UPDATE_EVENT_SOURCE,
);

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// `SKSE::Init(const LoadInterface*)`
pub unsafe fn init_commonlib(skse_interface: *const c_void) {
    unsafe { init_commonlib_with_log(skse_interface, true) }
}

/// `SKSE::Init(const LoadInterface*, bool log)`
///
/// Logging is owned by [`crate::skse::log`], so the flag is accepted only for
/// signature compatibility.
pub unsafe fn init_commonlib_with_log(skse_interface: *const c_void, _log: bool) {
    let Some(intfc) = (unsafe { skse_interface.cast::<LoadInterface>().as_ref() }) else {
        return;
    };
    if INITIALIZED.swap(true, Ordering::AcqRel) {
        return;
    }

    let store = |slot: &AtomicPtr<c_void>, id: InterfaceId| {
        slot.store(intfc.query_interface(id), Ordering::Release);
    };
    store(&SCALEFORM, InterfaceId::Scaleform);
    store(&PAPYRUS, InterfaceId::Papyrus);
    store(&SERIALIZATION, InterfaceId::Serialization);
    store(&TASK, InterfaceId::Task);
    store(&TRAMPOLINE, InterfaceId::Trampoline);
    store(&MESSAGING, InterfaceId::Messaging);
    store(&OBJECT, InterfaceId::Object);

    let messaging = MESSAGING
        .load(Ordering::Acquire)
        .cast::<MessagingInterface>();
    if let Some(messaging) = unsafe { messaging.as_ref() } {
        let dispatch = |slot: &AtomicPtr<c_void>, id: MessagingDispatcher| {
            slot.store(messaging.get_event_dispatcher(id), Ordering::Release);
        };
        dispatch(&MOD_CALLBACK_EVENT_SOURCE, MessagingDispatcher::ModEvent);
        dispatch(&CAMERA_EVENT_SOURCE, MessagingDispatcher::CameraEvent);
        dispatch(
            &CROSSHAIR_REF_EVENT_SOURCE,
            MessagingDispatcher::CrosshairEvent,
        );
        dispatch(&ACTION_EVENT_SOURCE, MessagingDispatcher::ActionEvent);
        dispatch(
            &NI_NODE_UPDATE_EVENT_SOURCE,
            MessagingDispatcher::NiNodeUpdateEvent,
        );
    }

    let object = OBJECT.load(Ordering::Acquire).cast::<ObjectInterface>();
    if let Some(object) = unsafe { object.as_ref() } {
        DELAY_FUNCTOR_MANAGER.store(object.get_delay_functor_manager().cast(), Ordering::Release);
        OBJECT_REGISTRY.store(object.get_object_registry().cast(), Ordering::Release);
        PERSISTENT_OBJECT_STORAGE.store(
            object.get_persistent_object_storage().cast(),
            Ordering::Release,
        );
    }
}

macro_rules! getters {
    ($($fn_name:ident => $slot:ident),* $(,)?) => {
        $(
            #[inline]
            pub unsafe fn $fn_name() -> *mut c_void {
                $slot.load(Ordering::Acquire)
            }
        )*
    };
}

getters!(
    commonlib_skse_get_scaleform_interface => SCALEFORM,
    commonlib_skse_get_serialization_interface => SERIALIZATION,
    commonlib_skse_get_papyrus_interface => PAPYRUS,
    commonlib_skse_get_task_interface => TASK,
    commonlib_skse_get_messaging_interface => MESSAGING,
    commonlib_skse_get_object_interface => OBJECT,
    commonlib_skse_get_trampoline_interface => TRAMPOLINE,
    commonlib_skse_get_delay_functor_manager => DELAY_FUNCTOR_MANAGER,
    commonlib_skse_get_object_registry => OBJECT_REGISTRY,
    commonlib_skse_get_persistent_object_storage => PERSISTENT_OBJECT_STORAGE,
    commonlib_skse_get_mod_callback_event_source => MOD_CALLBACK_EVENT_SOURCE,
    commonlib_skse_get_camera_event_source => CAMERA_EVENT_SOURCE,
    commonlib_skse_get_crosshair_ref_event_source => CROSSHAIR_REF_EVENT_SOURCE,
    commonlib_skse_get_action_event_source => ACTION_EVENT_SOURCE,
    commonlib_skse_get_ni_node_update_event_source => NI_NODE_UPDATE_EVENT_SOURCE,
);

// ---------------------------------------------------------------------------
// Tasks

/// SKSE `TaskDelegate` / `UIDelegate_v1` ABI:
/// `virtual void Run() = 0; virtual void Dispose() = 0;`
#[repr(C)]
struct DelegateVTable {
    run: unsafe extern "C" fn(this: *mut TaskDelegate),
    dispose: unsafe extern "C" fn(this: *mut TaskDelegate),
}

#[repr(C)]
struct TaskDelegate {
    vtable: *const DelegateVTable,
    callback: extern "C" fn(*mut c_void),
    data: *mut c_void,
}

unsafe extern "C" fn task_run(this: *mut TaskDelegate) {
    if let Some(task) = unsafe { this.as_ref() } {
        (task.callback)(task.data);
    }
}

unsafe extern "C" fn task_dispose(this: *mut TaskDelegate) {
    if !this.is_null() {
        drop(unsafe { alloc::boxed::Box::from_raw(this) });
    }
}

static TASK_VTABLE: DelegateVTable = DelegateVTable {
    run: task_run,
    dispose: task_dispose,
};

fn make_delegate(cb: extern "C" fn(*mut c_void), data: *mut c_void) -> *mut c_void {
    alloc::boxed::Box::into_raw(alloc::boxed::Box::new(TaskDelegate {
        vtable: &TASK_VTABLE,
        callback: cb,
        data,
    }))
    .cast()
}

/// `SKSE::GetTaskInterface()->AddTask(...)`
pub unsafe fn commonlib_add_task(cb: extern "C" fn(*mut c_void), data: *mut c_void) {
    let task = TASK.load(Ordering::Acquire).cast::<TaskInterface>();
    match unsafe { task.as_ref() } {
        Some(task) => task.add_task_raw(make_delegate(cb, data)),
        None => {
            crate::skse_error!(log, "AddTask: SKSE task interface is unavailable");
        }
    }
}

/// `SKSE::GetTaskInterface()->AddUITask(...)`
pub unsafe fn commonlib_add_ui_task(cb: extern "C" fn(*mut c_void), data: *mut c_void) {
    let task = TASK.load(Ordering::Acquire).cast::<TaskInterface>();
    match unsafe { task.as_ref() } {
        Some(task) => task.add_ui_task_raw(make_delegate(cb, data)),
        None => {
            crate::skse_error!(log, "AddUITask: SKSE task interface is unavailable");
        }
    }
}
