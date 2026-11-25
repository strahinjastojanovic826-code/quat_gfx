//! # FFI Safety & Ownership Overview (`unsafe` Contract)
//!
//! All `unsafe` blocks and `pub unsafe extern "C"` functions in this module cross the C/C++/C#
//! FFI boundary and uphold the following invariants:
//! - **Pointer validity & alignment:** Any non-null raw pointer passed from external callers must point
//!   to a valid, properly aligned, and initialized Rust instance (`DeviceManager`, `CommandList`,
//!   `ThreadSafeDeviceContext`, or `PackedQuatState64`) for the duration of the call.
//! - **Ownership & lifetime (`Box::into_raw` / `Box::from_raw`):** Functions returning raw pointers
//!   transfer heap ownership to the caller, and each object must be freed once using its matching
//!   `destroy_*` function (`rhi_destroy_device`, `rhi_destroy_command_list`,
//!   `rhi_destroy_thread_safe_device`, or `quat_packed_state_destroy64`) to avoid leaks or double-free.
//! - **Null & bounds checks:** Callers must check returned pointers for null and respect index/value
//!   bounds (or use `quat_packed_state_set_checked` for explicit error status codes).
//! - **Concurrency:** `ThreadSafeDeviceContext` wraps `DeviceManager` in `Arc<Mutex<...>>` for
//!   synchronized access across threads while keeping the underlying pointer valid until destroyed.

use std::ffi::c_void;
use crate::backend::GraphicsBackend;
use crate::command::CommandList;
use crate::device::DeviceManager;
use crate::state::ResourceState;
use std::sync::{Arc, Mutex};
use crate::state::{PackedQuatState64, QuatDigit};
use std::os::raw::c_int;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhi_create_device(backend_type: u8) -> *mut c_void {
    let backend = GraphicsBackend::from_u8(backend_type);
    let device = DeviceManager::new(backend);
    Box::into_raw(Box::new(device)) as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhi_destroy_device(device_ptr: *mut c_void) {
    if !device_ptr.is_null() {
        let _ = unsafe { Box::from_raw(device_ptr as *mut DeviceManager) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhi_create_command_list(device_ptr: *mut c_void) -> *mut c_void {
    let device = match unsafe { (device_ptr as *const DeviceManager).as_ref() } {
        Some(d) => d,
        None => return std::ptr::null_mut(),
    };

    let cmd_list = device.create_command_list();
    Box::into_raw(Box::new(cmd_list)) as *mut c_void
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhi_destroy_command_list(cmd_list_ptr: *mut c_void) {
    if !cmd_list_ptr.is_null() {
        let _ = unsafe { Box::from_raw(cmd_list_ptr as *mut CommandList) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhi_command_list_barrier(
    cmd_list_ptr: *mut c_void,
    resource_id: u32,
    from_state: u8,
    to_state: u8,
) {
    // Maskiranje na kvartarnu logiku (2 bita: values 0..3)
    let from = ResourceState::from(from_state);
    let to = ResourceState::from(to_state);

    if let Some(cmd_list) = unsafe { (cmd_list_ptr as *mut CommandList).as_mut() } {
        cmd_list.resource_barrier(resource_id, from, to);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhi_command_list_flush(cmd_list_ptr: *mut c_void) {
    if let Some(cmd_list) = unsafe { (cmd_list_ptr as *mut CommandList).as_mut() } {
        cmd_list.flush_commands();
    }
}

pub struct ThreadSafeDeviceContext {
    pub device: Arc<Mutex<DeviceManager>>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhi_create_thread_safe_device(
    backend_type: u8,
) -> *mut ThreadSafeDeviceContext {
    let backend = crate::backend::GraphicsBackend::from_u8(backend_type);
    let manager = DeviceManager::new(backend);
    
    let context = ThreadSafeDeviceContext {
        device: Arc::new(Mutex::new(manager)),
    };

    Box::into_raw(Box::new(context))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhi_destroy_thread_safe_device(
    ctx_ptr: *mut ThreadSafeDeviceContext,
) { unsafe {
    if !ctx_ptr.is_null() {
        let _ = Box::from_raw(ctx_ptr);
    }
}}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rhi_thread_safe_create_command_list(
    ctx_ptr: *mut ThreadSafeDeviceContext,
) -> *mut CommandList { unsafe {
    if ctx_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let ctx = &*ctx_ptr;
    if let Ok(device) = ctx.device.lock() {
        let cmd_list = device.create_command_list();
        Box::into_raw(Box::new(cmd_list))
    } else {
        std::ptr::null_mut()
    }
}}

/// Creates a new 64-bit Packed Quaternary State container for C++ clients
#[unsafe(no_mangle)]
pub extern "C" fn quat_packed_state_create64() -> *mut PackedQuatState64 {
    let state_box = Box::new(PackedQuatState64::new());
    Box::into_raw(state_box)
}

/// Destroys a C++ allocated PackedQuatState64 container
#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_packed_state_destroy64(state_ptr: *mut PackedQuatState64) { unsafe {
    if !state_ptr.is_null() {
        let _ = Box::from_raw(state_ptr);
    }
}}

/// Sets a Quaternary Digit (0-3) at a specific index (0-31) from C++
#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_packed_state_set(
    state_ptr: *mut PackedQuatState64,
    index: usize,
    digit_val: u8,
) -> c_int { unsafe {
    if state_ptr.is_null() || index >= 32 || digit_val > 3 {
        return -1; // Error code
    }
    
    let state = &mut *state_ptr;
    let digit = QuatDigit::from_bits(digit_val & 0b11);
    state.set_quat(index, digit);
    
    0 // Success
}}

/// Gets a Quaternary Digit (0-3) from a specific index (0-31) for C++
#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_packed_state_get(
    state_ptr: *const PackedQuatState64,
    index: usize,
) -> u8 { unsafe {
    if state_ptr.is_null() || index >= 32 {
        return 0;
    }
    
    let state = &*state_ptr;
    state.get_quat(index) as u8
}}

/// Validates a resource transition using the Quaternary Matrix from C++
#[unsafe(no_mangle)]
pub extern "C" fn quat_validate_transition_ffi(from_val: u8, to_val: u8) -> bool {
    let from_state = ResourceState::from(from_val & 0b11);
    let to_state = ResourceState::from(to_val & 0b11);
    ResourceState::validate_transition(from_state, to_state)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn quat_packed_state_set_checked(
    state_ptr: *mut PackedQuatState64,
    index: usize,
    digit_val: u8,
) -> c_int {
    if state_ptr.is_null() {
        return -1;
    }
    if index >= PackedQuatState64::CAPACITY {
        return -2;
    }
    if digit_val > 3 {
        return -3;
    }
    unsafe { quat_packed_state_set(state_ptr, index, digit_val) }
}