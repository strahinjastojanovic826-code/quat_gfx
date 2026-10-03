pub mod backend;
pub mod state;
pub mod resource;
pub mod command;
pub mod device;
pub mod ffi;

pub use backend::GraphicsBackend;
use crate::state::ResourceState;
pub use resource::{ResourceDesc, ResourceType, GpuResource};
pub use command::CommandList;
pub use device::{GpuDevice, DeviceManager};

 #[cfg(test)]
mod tests {
    use crate::backend::GraphicsBackend;
    use crate::command::CommandList;
    use crate::device::{DeviceManager, GpuDevice};
    use crate::resource::{ResourceDesc, ResourceType};
    use crate::state::{ResourceState, ResourceStateFlags, QuatDigit, PackedQuatState};
    use crate::ffi::*;
    use std::ffi::c_void;

    // ==========================================
    // 1. TESTOVI KVARTARNIH STANJA I BITFLAGS 2.4
    // ==========================================

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
    fn test_quaternary_masking_overflow() {
        // Test da li maskiranje sa 0b11 (baza 4) ispravno prevodi vrijednosti veće od 3
        assert_eq!(ResourceState::from(4), ResourceState::State00); // 4 & 3 = 0
        assert_eq!(ResourceState::from(5), ResourceState::State01); // 5 & 3 = 1
        assert_eq!(ResourceState::from(255), ResourceState::State11); // 255 & 3 = 3
    }

    #[test]
    fn test_bitflags_2_4_integration() {
        // Testiranje kombinovanja kvartarnih flegova sa bitflags 2.4
        let mut flags = ResourceStateFlags::NONE;
        assert_eq!(flags.bits(), 0b00);

        flags.insert(ResourceStateFlags::SHADER_READ);
        assert!(flags.contains(ResourceStateFlags::SHADER_READ));
        assert_eq!(flags.bits(), 0b01);

        let combined = ResourceStateFlags::SHADER_READ | ResourceStateFlags::COPY_DEST;
        assert_eq!(combined.bits(), 0b11);
    }

    // ==========================================
    // 2. TESTOVI GRAFIČKIH BEKENDA I TR
    // ==========================================

    #[test]
    fn test_backends_creation() {
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

    // ==========================================
    // 3. NOVI TESTOVI: PAKOVANI KVATI I LOGIKA
    // ==========================================

    #[test]
    fn test_packed_quat_state_packing() {
        // Pakovanje 4 kvat stanja u samo 1 bajt (u8)
        let mut packed = PackedQuatState(0);

        packed.set_quat(0, QuatDigit::Q0); // 0b00 na poziciju 0
        packed.set_quat(1, QuatDigit::Q1); // 0b01 na poziciju 1
        packed.set_quat(2, QuatDigit::Q2); // 0b10 na poziciju 2
        packed.set_quat(3, QuatDigit::Q3); // 0b11 na poziciju 3

        // Očekivani bajt u binarnom zapisu: 0b11_10_01_00 = 228
        assert_eq!(packed.0, 0b11100100);

        // Provjera čitanja spakovanih stanja
        assert_eq!(packed.get_quat(0), QuatDigit::Q0);
        assert_eq!(packed.get_quat(1), QuatDigit::Q1);
        assert_eq!(packed.get_quat(2), QuatDigit::Q2);
        assert_eq!(packed.get_quat(3), QuatDigit::Q3);
    }

    // ==========================================
    // 4. NOVI TESTOVI: C-FFI SIGURNOST (NULL POINTERS)
    // ==========================================

    #[test]
    fn test_ffi_safe_lifecycle() {
        unsafe {
            // Kreiranje uređaja preko FFI
            let dev_ptr = rhi_create_device(1); // 1 = Vulkan
            assert!(!dev_ptr.is_null());

            // Kreiranje command list-e preko FFI
            let cmd_ptr = rhi_create_command_list(dev_ptr);
            assert!(!cmd_ptr.is_null());

            // Pozivanje barijere sa kvartarnim stanjima
            rhi_command_list_barrier(cmd_ptr, 100, 0b00, 0b11);
            rhi_command_list_flush(cmd_ptr);

            // Čišćenje memorije
            rhi_destroy_command_list(cmd_ptr);
            rhi_destroy_device(dev_ptr);
        }
    }

    #[test]
    fn test_ffi_null_pointer_handling() {
        // Testiranje da FFI ne ruši aplikaciju (no panic) ako dobije NULL pokazivače
        unsafe {
            let null_ptr: *mut c_void = std::ptr::null_mut();

            // Ne bi trebalo doći do crasha (UB)
            rhi_destroy_device(null_ptr);
            rhi_destroy_command_list(null_ptr);
            rhi_command_list_barrier(null_ptr, 1, 0, 1);
            rhi_command_list_flush(null_ptr);

            let res = rhi_create_command_list(null_ptr);
            assert!(res.is_null());
        }
    }
}