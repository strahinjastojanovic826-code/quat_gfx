use std::ffi::c_void;
use crate::backend::GraphicsBackend;
use crate::command::CommandList;
use crate::device::{DeviceManager, GpuDevice};
use crate::state::ResourceState;

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