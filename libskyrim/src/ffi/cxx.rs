//! Rust implementations of the small C++ classes CommonLib used to synthesize
//! on the C++ side: `BSTEventSink<T>` adapters, `IFunctionArguments`,
//! `NF_util::NativeFunctionBase` descendants and `ButtonEvent::Create`.
//!
//! Each object is laid out `#[repr(C)]` with a vtable pointer at offset 0 that
//! points to a Rust-built table following the MSVC x64 ABI:
//! slot 0 is the scalar deleting destructor `void* (this, unsigned flags)`.

use core::ffi::{c_char, c_void};

use spin::Once;

use super::game::delete_polymorphic;
use super::{
    CommonlibBstEventSinkProcessCallback, CommonlibDestroyCallback,
    CommonlibFunctionArgumentsCollectCallback, CommonlibNativeFunctionMarshallCallback,
};
use crate::offsets::offsets_vtable::{
    VTABLE_BSScript__NF_util__NativeFunctionBase, VTABLE_BSScript__ZeroFunctionArguments,
    VTABLE_ButtonEvent,
};
use crate::re::bs_fixed_string::BSFixedString;
use crate::re::bst_tuple::BSTTuple;
use crate::re::native_function_base::NativeFunctionBase;
use crate::re::type_info::TypeInfo;
use crate::re::v_desc_table::VDescTable;

type DeletingDtor = unsafe extern "C" fn(this: *mut c_void, flags: u32) -> *mut c_void;

const DELETE_FLAG: u32 = 1;

// ---------------------------------------------------------------------------
// BSTEventSink<void*>

#[repr(C)]
struct EventSinkVTable {
    dtor: DeletingDtor,
    process_event: unsafe extern "C" fn(
        this: *mut EventSink,
        event: *const c_void,
        source: *mut c_void,
    ) -> i32,
}

#[repr(C)]
struct EventSink {
    vtable: *const EventSinkVTable,
    ctx: *mut c_void,
    process: Option<CommonlibBstEventSinkProcessCallback>,
    destroy: Option<CommonlibDestroyCallback>,
}

unsafe extern "C" fn event_sink_dtor(this: *mut c_void, flags: u32) -> *mut c_void {
    let sink = this.cast::<EventSink>();
    unsafe {
        if let Some(destroy) = (*sink).destroy
            && !(*sink).ctx.is_null()
        {
            destroy((*sink).ctx);
        }
        if flags & DELETE_FLAG != 0 {
            drop(alloc::boxed::Box::from_raw(sink));
        }
    }
    this
}

unsafe extern "C" fn event_sink_process(
    this: *mut EventSink,
    event: *const c_void,
    source: *mut c_void,
) -> i32 {
    // BSEventNotifyControl::kContinue = 0, kStop = 1
    let Some(sink) = (unsafe { this.as_ref() }) else {
        return 0;
    };
    match sink.process {
        Some(process) => {
            if unsafe { process(sink.ctx, event, source) } == 1 {
                1
            } else {
                0
            }
        }
        None => 0,
    }
}

static EVENT_SINK_VTABLE: EventSinkVTable = EventSinkVTable {
    dtor: event_sink_dtor,
    process_event: event_sink_process,
};

pub unsafe fn commonlib_bst_event_sink_create(
    ctx: *mut c_void,
    process: Option<CommonlibBstEventSinkProcessCallback>,
    destroy: Option<CommonlibDestroyCallback>,
) -> *mut c_void {
    alloc::boxed::Box::into_raw(alloc::boxed::Box::new(EventSink {
        vtable: &EVENT_SINK_VTABLE,
        ctx,
        process,
        destroy,
    }))
    .cast()
}

pub unsafe fn commonlib_bst_event_sink_destroy(sink: *mut c_void) {
    if !sink.is_null() {
        unsafe { event_sink_dtor(sink, DELETE_FLAG) };
    }
}

// ---------------------------------------------------------------------------
// BSScript::IFunctionArguments

#[repr(C)]
struct FunctionArgumentsVTable {
    dtor: DeletingDtor,
    call: unsafe extern "C" fn(this: *const FunctionArguments, dst: *mut c_void) -> bool,
}

#[repr(C)]
struct FunctionArguments {
    vtable: *const FunctionArgumentsVTable,
    ctx: *mut c_void,
    collect: Option<CommonlibFunctionArgumentsCollectCallback>,
    destroy: Option<CommonlibDestroyCallback>,
}

unsafe extern "C" fn function_arguments_dtor(this: *mut c_void, flags: u32) -> *mut c_void {
    let args = this.cast::<FunctionArguments>();
    unsafe {
        if let Some(destroy) = (*args).destroy
            && !(*args).ctx.is_null()
        {
            destroy((*args).ctx);
        }
        if flags & DELETE_FLAG != 0 {
            super::commonlib_free(this);
        }
    }
    this
}

unsafe extern "C" fn function_arguments_call(
    this: *const FunctionArguments,
    dst: *mut c_void,
) -> bool {
    let Some(args) = (unsafe { this.as_ref() }) else {
        return false;
    };
    match args.collect {
        Some(collect) => unsafe { collect(args.ctx, dst) },
        None => false,
    }
}

static FUNCTION_ARGUMENTS_VTABLE: FunctionArgumentsVTable = FunctionArgumentsVTable {
    dtor: function_arguments_dtor,
    call: function_arguments_call,
};

/// Allocates on the engine heap: `IFunctionArguments` declares
/// `TES_HEAP_REDEFINE_NEW()` and the VM may delete it.
pub unsafe fn commonlib_function_arguments_create(
    ctx: *mut c_void,
    collect: Option<CommonlibFunctionArgumentsCollectCallback>,
    destroy: Option<CommonlibDestroyCallback>,
) -> *mut c_void {
    unsafe {
        let mem = super::commonlib_malloc(core::mem::size_of::<FunctionArguments>())
            .cast::<FunctionArguments>();
        if mem.is_null() {
            return core::ptr::null_mut();
        }
        mem.write(FunctionArguments {
            vtable: &FUNCTION_ARGUMENTS_VTABLE,
            ctx,
            collect,
            destroy,
        });
        mem.cast()
    }
}

/// `new RE::BSScript::ZeroFunctionArguments()` using the engine's own vtable.
pub unsafe fn commonlib_function_arguments_create_zero() -> *mut c_void {
    let vtable = VTABLE_BSScript__ZeroFunctionArguments[0].address();
    if vtable == 0 {
        return core::ptr::null_mut();
    }
    unsafe {
        let mem = super::commonlib_malloc(core::mem::size_of::<usize>()).cast::<usize>();
        if !mem.is_null() {
            mem.write(vtable);
        }
        mem.cast()
    }
}

/// `delete static_cast<RE::BSScript::IFunctionArguments*>(args)`
pub unsafe fn commonlib_function_arguments_destroy(args: *mut c_void) {
    unsafe { delete_polymorphic(args) }
}

// ---------------------------------------------------------------------------
// BSScript::NF_util::NativeFunctionBase

/// Number of virtual slots of `NativeFunctionBase` (00..=16).
const NATIVE_FUNCTION_SLOTS: usize = 0x17;
const VFUNC_HAS_STUB: usize = 0x15;
const VFUNC_MARSHALL_AND_DISPATCH: usize = 0x16;

#[repr(C)]
struct NativeFunction {
    base: NativeFunctionBase,
    ctx: *mut c_void,
    marshall: Option<CommonlibNativeFunctionMarshallCallback>,
    destroy: Option<CommonlibDestroyCallback>,
}

unsafe fn drop_desc_table(table: &mut VDescTable) {
    let count = table.entries.size();
    let data = table.entries.data();
    if data.is_null() {
        return;
    }
    unsafe {
        for i in 0..count {
            core::ptr::drop_in_place(core::ptr::addr_of_mut!((*data.add(i)).first));
        }
        super::commonlib_free(table.entries.get_head());
    }
}

unsafe extern "C" fn native_function_dtor(this: *mut c_void, flags: u32) -> *mut c_void {
    let func = this.cast::<NativeFunction>();
    unsafe {
        let base = &mut (*func).base;
        core::ptr::drop_in_place(&mut base.name);
        core::ptr::drop_in_place(&mut base.obj_name);
        core::ptr::drop_in_place(&mut base.state_name);
        core::ptr::drop_in_place(&mut base.doc_string);
        drop_desc_table(&mut base.desc_table);

        if let Some(destroy) = (*func).destroy
            && !(*func).ctx.is_null()
        {
            destroy((*func).ctx);
        }
        if flags & DELETE_FLAG != 0 {
            super::commonlib_free(this);
        }
    }
    this
}

unsafe extern "C" fn native_function_has_stub(this: *const NativeFunction) -> bool {
    unsafe { this.as_ref().is_some_and(|f| f.marshall.is_some()) }
}

unsafe extern "C" fn native_function_marshall_and_dispatch(
    this: *const NativeFunction,
    base_value: *mut c_void,
    vm: *mut c_void,
    stack_id: u32,
    result_value: *mut c_void,
    frame: *const c_void,
) -> bool {
    let Some(func) = (unsafe { this.as_ref() }) else {
        return false;
    };
    match func.marshall {
        Some(marshall) => unsafe {
            marshall(func.ctx, base_value, vm, stack_id, result_value, frame)
        },
        None => false,
    }
}

/// Builds (once) a vtable whose `IFunction` slots 01..=14 come from the engine's
/// `NF_util::NativeFunctionBase` vtable, with the destructor and the two pure
/// virtuals (`HasStub`, `MarshallAndDispatch`) provided by Rust.
fn native_function_vtable() -> Option<&'static [usize; NATIVE_FUNCTION_SLOTS]> {
    static VTABLE: Once<Option<[usize; NATIVE_FUNCTION_SLOTS]>> = Once::new();
    VTABLE
        .call_once(|| {
            let game = VTABLE_BSScript__NF_util__NativeFunctionBase[0].address() as *const usize;
            if game.is_null() {
                return None;
            }
            let mut table = [0usize; NATIVE_FUNCTION_SLOTS];
            for (i, slot) in table.iter_mut().enumerate().take(VFUNC_HAS_STUB) {
                *slot = unsafe { *game.add(i) };
            }
            table[0] = native_function_dtor as *const () as usize;
            table[VFUNC_HAS_STUB] = native_function_has_stub as *const () as usize;
            table[VFUNC_MARSHALL_AND_DISPATCH] =
                native_function_marshall_and_dispatch as *const () as usize;
            Some(table)
        })
        .as_ref()
}

fn param_name(index: usize) -> BSFixedString {
    // "param" + (index + 1), NUL terminated, without allocating.
    let mut buf = *b"param\0\0\0\0\0\0\0";
    let mut n = index + 1;
    let mut digits = [0u8; 6];
    let mut count = 0;
    loop {
        digits[count] = b'0' + (n % 10) as u8;
        count += 1;
        n /= 10;
        if n == 0 || count == digits.len() {
            break;
        }
    }
    for i in 0..count {
        buf[5 + i] = digits[count - 1 - i];
    }
    BSFixedString::new(buf.as_ptr().cast::<c_char>())
}

#[allow(clippy::too_many_arguments)]
pub unsafe fn commonlib_native_function_create(
    ctx: *mut c_void,
    marshall: Option<CommonlibNativeFunctionMarshallCallback>,
    destroy: Option<CommonlibDestroyCallback>,
    fn_name: *const c_char,
    class_name: *const c_char,
    is_static: bool,
    return_type: *const c_void,
    param_types: *const c_void,
    param_count: usize,
    is_latent: bool,
) -> *mut c_void {
    if fn_name.is_null()
        || class_name.is_null()
        || return_type.is_null()
        || param_count > 0xFFFF
        || (param_count > 0 && param_types.is_null())
    {
        return core::ptr::null_mut();
    }
    let Some(vtable) = native_function_vtable() else {
        return core::ptr::null_mut();
    };

    unsafe {
        let mem = super::commonlib_malloc(core::mem::size_of::<NativeFunction>())
            .cast::<NativeFunction>();
        if mem.is_null() {
            return core::ptr::null_mut();
        }
        core::ptr::write_bytes(mem.cast::<u8>(), 0, core::mem::size_of::<NativeFunction>());

        let base = &mut (*mem).base;
        base.base.vtable = vtable.as_ptr();
        core::ptr::write(&mut base.name, BSFixedString::new(fn_name));
        core::ptr::write(&mut base.obj_name, BSFixedString::new(class_name));
        core::ptr::write(&mut base.state_name, BSFixedString::new(c"".as_ptr()));
        core::ptr::write(
            &mut base.ret_type,
            core::ptr::read(return_type.cast::<TypeInfo>()),
        );
        core::ptr::write(&mut base.desc_table, VDescTable::new(param_count as u16, 0));
        core::ptr::write(&mut base.doc_string, BSFixedString::empty());
        base.is_static = is_static;
        base.is_callable_from_tasklet = false;
        base.is_latent = is_latent;
        base.user_flags = 0;

        let entries = base.desc_table.entries.data();
        if !entries.is_null() {
            let types = param_types.cast::<TypeInfo>();
            for i in 0..param_count {
                let entry: *mut BSTTuple<BSFixedString, TypeInfo> = entries.add(i);
                (*entry).first = param_name(i);
                core::ptr::write(&mut (*entry).second, core::ptr::read(types.add(i)));
            }
        }

        (*mem).ctx = ctx;
        (*mem).marshall = marshall;
        (*mem).destroy = destroy;
        mem.cast()
    }
}

pub unsafe fn commonlib_native_function_destroy(function: *mut c_void) {
    unsafe { delete_polymorphic(function) }
}

// ---------------------------------------------------------------------------
// ButtonEvent::Create

/// `RE::ButtonEvent::Create(device, userEvent, idCode, value, heldDownSecs)`
pub unsafe fn commonlib_button_event_create(
    input_device: i32,
    user_event: *const c_void,
    id_code: u32,
    value: f32,
    held_down_secs: f32,
) -> *mut c_void {
    use crate::re::button_event::{ButtonEvent, ButtonEventRuntimeData};

    let runtime_offset = ButtonEvent::RUNTIME_DATA_OFFSET.offset();
    let full_size = runtime_offset + core::mem::size_of::<ButtonEventRuntimeData>();
    let vtable = VTABLE_ButtonEvent[0].address();
    if vtable == 0 {
        return core::ptr::null_mut();
    }

    unsafe {
        let mem = super::commonlib_malloc(full_size);
        if mem.is_null() {
            return mem;
        }
        core::ptr::write_bytes(mem.cast::<u8>(), 0, full_size);
        mem.cast::<usize>().write(vtable);

        let event = &mut *mem.cast::<ButtonEvent>();
        event.base.device = core_util::EnumSet::from_underlying(input_device as u32);
        event.base.event_type =
            core_util::EnumSet::from_underlying(crate::re::INPUT_EVENT_TYPE::kButton as u32);
        event.base.next = core::ptr::null_mut();

        let user_event = user_event
            .cast::<BSFixedString>()
            .as_ref()
            .cloned()
            .unwrap_or_default();
        // The storage is zeroed, so overwrite without dropping the old value.
        core::ptr::write(&mut event.as_id_event_mut().user_event, user_event);
        event.set_id_code(id_code);
        event.runtime_data_mut().value = value;
        event.runtime_data_mut().held_down_secs = held_down_secs;

        mem
    }
}
