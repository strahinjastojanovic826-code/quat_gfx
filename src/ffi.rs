use std::ffi::c_void;
use crate::backend::GraphicsBackend;
use crate::command::CommandList;
use crate::device::DeviceManager;
use crate::state::ResourceState;
use std::sync::{Arc, Mutex};

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