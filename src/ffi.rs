use std::ffi::c_void;
use crate::backend::GraphicsBackend;
use crate::state::ResourceState;
use crate::device::DeviceManager;
use crate::command::CommandList;

#[unsafe(no_mangle)]
pub extern "C" fn rhi_create_device(backend_type: u8) -> *mut c_void {
    let backend = GraphicsBackend::from_u8(backend_type);
    let device = DeviceManager::new(backend);
    Box::into_raw(Box::new(device)) as *mut c_void
}

#[unsafe(no_mangle)]
pub extern "C" fn rhi_destroy_device(device_ptr: *mut c_void) {
    if !device_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(device_ptr as *mut DeviceManager);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rhi_create_command_list(device_ptr: *mut c_void) -> *mut c_void {
    if device_ptr.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let device = &*(device_ptr as *const DeviceManager);
        let cmd_list = device.create_command_list();
        Box::into_raw(Box::new(cmd_list)) as *mut c_void
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rhi_destroy_command_list(cmd_list_ptr: *mut c_void) {
    if !cmd_list_ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(cmd_list_ptr as *mut CommandList);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rhi_command_list_barrier(
    cmd_list_ptr: *mut c_void, 
    resource_id: u32, 
    from_state: u8, 
    to_state: u8
) {
    if cmd_list_ptr.is_null() {
        return;
    }
    unsafe {
        let cmd_list = &mut *(cmd_list_ptr as *mut CommandList);
        let from = ResourceState::from(from_state);
        let to = ResourceState::from(to_state);
        cmd_list.resource_barrier(resource_id, from, to);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rhi_command_list_flush(cmd_list_ptr: *mut c_void) {
    if cmd_list_ptr.is_null() {
        return;
    }
    unsafe {
        let cmd_list = &*(cmd_list_ptr as *mut CommandList);
        cmd_list.flush_commands();
    }
}