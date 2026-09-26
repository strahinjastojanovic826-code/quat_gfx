pub mod backend;
pub mod state;
pub mod resource;
pub mod command;
pub mod device;
pub mod ffi;

pub use backend::GraphicsBackend;
pub use state::ResourceState;
pub use resource::{ResourceDesc, ResourceType, GpuResource};
pub use command::CommandList;
pub use device::{GpuDevice, DeviceManager};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ResourceState;
    use crate::backend::GraphicsBackend;
    use crate::device::{DeviceManager, GpuDevice};
    use crate::resource::{ResourceDesc, ResourceType};

    #[test]
    fn test_resource_states_4_modes() {
        // Test State 00 (0b00) -> Undefined / Common
        let state_00 = ResourceState::from(0b00);
        assert_eq!(state_00, ResourceState::State00);
        assert_eq!(state_00.numeric_value(), 0);
        assert!(!state_00.is_writable());

        // Test State 01 (0b01) -> Shader Resource / Read-Only
        let state_01 = ResourceState::from(0b01);
        assert_eq!(state_01, ResourceState::State01);
        assert_eq!(state_01.numeric_value(), 1);
        assert!(!state_01.is_writable());

        // Test State 10 (0b10) -> Copy Destination / Storage Write
        let state_10 = ResourceState::from(0b10);
        assert_eq!(state_10, ResourceState::State10);
        assert_eq!(state_10.numeric_value(), 2);
        assert!(state_10.is_writable());

        // Test State 11 (0b11) -> Render Target / Present
        let state_11 = ResourceState::from(0b11);
        assert_eq!(state_11, ResourceState::State11);
        assert_eq!(state_11.numeric_value(), 3);
        assert!(state_11.is_writable());
    }

    #[test]
    fn test_backends_creation() {
        // Test backend selection initialization for DX12, Vulkan, and Metal
        let dx12_device = DeviceManager::new(GraphicsBackend::DirectX12);
        assert_eq!(dx12_device.backend_type(), GraphicsBackend::DirectX12);

        let vulkan_device = DeviceManager::new(GraphicsBackend::Vulkan);
        assert_eq!(vulkan_device.backend_type(), GraphicsBackend::Vulkan);

        let metal_device = DeviceManager::new(GraphicsBackend::Metal);
        assert_eq!(metal_device.backend_type(), GraphicsBackend::Metal);
    }

    #[test]
    fn test_resource_transition_pipeline() {
        let mut device = DeviceManager::new(GraphicsBackend::Vulkan);
        let mut cmd_list = device.create_command_list();

        let desc = ResourceDesc {
            resource_type: ResourceType::Buffer,
            size_in_bytes: 1024,
            width: 0,
            height: 0,
            initial_state: ResourceState::State00,
        };

        let mut resource = device.create_resource(desc);
        assert_eq!(resource.current_state(), ResourceState::State00);

        // Transition: State 00 -> State 01
        device.transition_resource(&mut cmd_list, &mut resource, ResourceState::State01);
        assert_eq!(resource.current_state(), ResourceState::State01);

        // Transition: State 01 -> State 10
        device.transition_resource(&mut cmd_list, &mut resource, ResourceState::State10);
        assert_eq!(resource.current_state(), ResourceState::State10);

        // Transition: State 10 -> State 11
        device.transition_resource(&mut cmd_list, &mut resource, ResourceState::State11);
        assert_eq!(resource.current_state(), ResourceState::State11);
    }
}