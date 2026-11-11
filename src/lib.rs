pub mod backend;
pub mod state;
pub mod resource;
pub mod command;
pub mod device;
pub mod ffi;
pub mod allocator;

pub use backend::GraphicsBackend;
pub use resource::{ResourceDesc, ResourceType, GpuResource};
pub use command::CommandList;
pub use device::{GpuDevice, DeviceManager};

 #[cfg(test)]
mod tests {
    use crate::backend::GraphicsBackend;
    use crate::command::CommandList;
    use crate::device::{DeviceManager, GpuDevice};
    use crate::resource::{ResourceDesc, ResourceType};
    use crate::state::{ResourceState, ResourceStateFlags, QuatDigit};
    use crate::state::PackedQuatState64;
    use crate::ffi::*;
    use std::ffi::c_void;
    use std::time::Instant;

    // ==========================================
    // 1. QUATERNARY STATES & BITFLAGS TESTS
    // ==========================================

    #[test]
    fn test_resource_states_4_modes() {
        let iterations = 100_000;
        let start = Instant::now();

        for _ in 0..iterations {
            let state_00 = ResourceState::from(0b00);
            assert_eq!(state_00, ResourceState::State00);
            assert_eq!(state_00.numeric_value(), 0);
            assert!(!state_00.is_writable());

            let state_01 = ResourceState::from(0b01);
            assert_eq!(state_01, ResourceState::State01);
            assert_eq!(state_01.numeric_value(), 1);
            assert!(!state_01.is_writable());

            let state_10 = ResourceState::from(0b10);
            assert_eq!(state_10, ResourceState::State10);
            assert_eq!(state_10.numeric_value(), 2);
            assert!(state_10.is_writable());

            let state_11 = ResourceState::from(0b11);
            assert_eq!(state_11, ResourceState::State11);
            assert_eq!(state_11.numeric_value(), 3);
            assert!(state_11.is_writable());
        }

        let duration = start.elapsed();
        println!("[BENCH] test_resource_states_4_modes ({} iter): {:?}", iterations, duration);
    }

    #[test]
    fn test_quaternary_masking_overflow() {
        let iterations = 100_000;
        let start = Instant::now();

        for _ in 0..iterations {
            // Test that base-4 masking correctly translates values greater than 3
            assert_eq!(ResourceState::from(4), ResourceState::State00); // 4 & 3 = 0
            assert_eq!(ResourceState::from(5), ResourceState::State01); // 5 & 3 = 1
            assert_eq!(ResourceState::from(255), ResourceState::State11); // 255 & 3 = 3
        }

        let duration = start.elapsed();
        println!("[BENCH] test_quaternary_masking_overflow ({} iter): {:?}", iterations, duration);
    }

    #[test]
    fn test_bitflags_2_4_integration() {
        let iterations = 100_000;
        let start = Instant::now();

        for _ in 0..iterations {
            // Testing combination of quaternary flags with bitflags 2.4
            let mut flags = ResourceStateFlags::NONE;
            assert_eq!(flags.bits(), 0b00);

            flags.insert(ResourceStateFlags::SHADER_READ);
            assert!(flags.contains(ResourceStateFlags::SHADER_READ));
            assert_eq!(flags.bits(), 0b01);

            let combined = ResourceStateFlags::SHADER_READ | ResourceStateFlags::COPY_DEST;
            assert_eq!(combined.bits(), 0b11);
        }

        let duration = start.elapsed();
        println!("[BENCH] test_bitflags_2_4_integration ({} iter): {:?}", iterations, duration);
    }

    // ==========================================
    // 2. GRAPHICS BACKENDS & PIPELINE TESTS
    // ==========================================

    #[test]
    fn test_backends_creation() {
        let iterations = 50_000;
        let start = Instant::now();

        for _ in 0..iterations {
            let dx12_device = DeviceManager::new(GraphicsBackend::DirectX12);
            assert_eq!(dx12_device.backend_type(), GraphicsBackend::DirectX12);

            let vulkan_device = DeviceManager::new(GraphicsBackend::Vulkan);
            assert_eq!(vulkan_device.backend_type(), GraphicsBackend::Vulkan);

            let metal_device = DeviceManager::new(GraphicsBackend::Metal);
            assert_eq!(metal_device.backend_type(), GraphicsBackend::Metal);

            let webgpu_device = DeviceManager::new(GraphicsBackend::WebGPU);
            assert_eq!(webgpu_device.backend_type(), GraphicsBackend::WebGPU);

            let opengl_device = DeviceManager::new(GraphicsBackend::OpenGL);
            assert_eq!(opengl_device.backend_type(), GraphicsBackend::OpenGL);
        }

        let duration = start.elapsed();
        println!("[BENCH] test_backends_creation ({} iter): {:?}", iterations, duration);
    }

    #[test]
    fn test_resource_transition_pipeline() {
        let iterations = 50_000;
        let start = Instant::now();

        for _ in 0..iterations {
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
            device.safe_transition_resource(&mut cmd_list, &mut resource, ResourceState::State01).unwrap();
            assert_eq!(resource.current_state(), ResourceState::State01);

            // Transition: State 01 -> State 10
            device.safe_transition_resource(&mut cmd_list, &mut resource, ResourceState::State10).unwrap();
            assert_eq!(resource.current_state(), ResourceState::State10);

            // Transition: State 10 -> State 11
            device.safe_transition_resource(&mut cmd_list, &mut resource, ResourceState::State11).unwrap();
            assert_eq!(resource.current_state(), ResourceState::State11);
        }

        let duration = start.elapsed();
        println!("[BENCH] test_resource_transition_pipeline ({} iter): {:?}", iterations, duration);
    }

    // ==========================================
    // 3. PACKED QUATERNARY STATES & LOGIC
    // ==========================================

    #[test]
    fn test_packed_quat_state_packing() {
        let iterations = 100_000;
        let start = Instant::now();

        for _ in 0..iterations {
            let mut packed = PackedQuatState64::new();

            packed.set_quat(0, QuatDigit::Q0); // 0b00 at position 0
            packed.set_quat(1, QuatDigit::Q1); // 0b01 at position 1
            packed.set_quat(2, QuatDigit::Q2); // 0b10 at position 2
            packed.set_quat(3, QuatDigit::Q3); // 0b11 at position 3

            // Expected byte in binary format: 0b11_10_01_00 = 228
            assert_eq!(packed.0, 228);

            // Verify reading packed states
            assert_eq!(packed.get_quat(0), QuatDigit::Q0);
            assert_eq!(packed.get_quat(1), QuatDigit::Q1);
            assert_eq!(packed.get_quat(2), QuatDigit::Q2);
            assert_eq!(packed.get_quat(3), QuatDigit::Q3);
        }

        let duration = start.elapsed();
        println!("[BENCH] test_packed_quat_state_packing ({} iter): {:?}", iterations, duration);
    }

    #[test]
    fn test_packed_quat_state_64() {
        let iterations = 100_000;
        let start = Instant::now();

        for _ in 0..iterations {
            let mut packed = PackedQuatState64::new();

            packed.set_quat(0, QuatDigit::Q3);
            packed.set_quat(31, QuatDigit::Q1);

            assert_eq!(packed.get_quat(0), QuatDigit::Q3);
            assert_eq!(packed.get_quat(31), QuatDigit::Q1);
            // Verify remaining states remain zero (Q0)
            assert_eq!(packed.get_quat(15), QuatDigit::Q0);
        }

        let duration = start.elapsed();
        println!("[BENCH] test_packed_quat_state_64 ({} iter): {:?}", iterations, duration);
    }

    // ==========================================
    // 4. C-FFI SAFETY & NULL POINTER TESTS
    // ==========================================

    #[test]
    fn test_ffi_safe_lifecycle() {
        let iterations = 50_000;
        let start = Instant::now();

        for _ in 0..iterations {
            unsafe {
                // Create device via FFI (1 = Vulkan)
                let dev_ptr = rhi_create_device(1);
                assert!(!dev_ptr.is_null());

                // Create command list via FFI
                let cmd_ptr = rhi_create_command_list(dev_ptr);
                assert!(!cmd_ptr.is_null());

                // Invoke barrier with quaternary states and flush
                rhi_command_list_barrier(cmd_ptr, 100, 0b00, 0b11);
                rhi_command_list_flush(cmd_ptr);

                // Cleanup memory
                rhi_destroy_command_list(cmd_ptr);
                rhi_destroy_device(dev_ptr);
            }
        }

        let duration = start.elapsed();
        println!("[BENCH] test_ffi_safe_lifecycle ({} iter): {:?}", iterations, duration);
    }

    #[test]
    fn test_ffi_null_pointer_handling() {
        let iterations = 100_000;
        let start = Instant::now();

        for _ in 0..iterations {
            unsafe {
                let null_ptr: *mut std::ffi::c_void = std::ptr::null_mut();

                // Should not trigger panic or crash on null pointers (UB prevention)
                rhi_destroy_device(null_ptr);
                rhi_destroy_command_list(null_ptr);
                rhi_command_list_barrier(null_ptr, 1, 0, 1);
                rhi_command_list_flush(null_ptr);

                let res = rhi_create_command_list(null_ptr);
                assert!(res.is_null());
            }
        }

        let duration = start.elapsed();
        println!("[BENCH] test_ffi_null_pointer_handling ({} iter): {:?}", iterations, duration);
    }

    #[test]
    fn test_invalid_resource_transition() {
        let iterations = 100_000;
        let start = Instant::now();

        for _ in 0..iterations {
            let mut device = DeviceManager::new(GraphicsBackend::Vulkan);
            let mut cmd_list = device.create_command_list();

            let desc = ResourceDesc {
                resource_type: ResourceType::Buffer,
                size_in_bytes: 512,
                width: 0,
                height: 0,
                initial_state: ResourceState::State00,
            };

            let mut resource = device.create_resource(desc);
            let _ = device.safe_transition_resource(&mut cmd_list, &mut resource, ResourceState::State01);
        }

        let duration = start.elapsed();
        println!("[BENCH] test_invalid_resource_transition ({} iter): {:?}", iterations, duration);
    }

#[test]
fn bench_packed_quat_operations_native() {
    let mut packed = PackedQuatState64::new();
    let digit = QuatDigit::Q3;
    let iterations = 1_000_000;

    let start = Instant::now();
    for _ in 0..iterations {
        for i in 0..32 {
            packed.set_quat(i, digit);
            let _ = packed.get_quat(i);
        }
    }
    let duration = start.elapsed();
    println!(
        "\n[BENCH] PackedQuatState64 (1M iterations, all 32 quats): Total: {:?}, Average per loop: {:?}", 
        duration, duration / iterations
    );
}

#[test]
fn bench_resource_transition_validation_native() {
    let iterations = 10_000_000;
    let from = ResourceState::State00;
    let to = ResourceState::State11;

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = ResourceState::validate_transition(from, to);
    }
    let duration = start.elapsed();
    println!(
        "\n[BENCH] Matrix Validation (10M iterations): Total: {:?}, Average per call: {:?}", 
        duration, duration / iterations
    );
}

#[test]
fn bench_ffi_boundary_native() {
    unsafe {
        let state_ptr = quat_packed_state_create64();
        let iterations = 1_000_000;

        // Measure FFI state setter performance
        let start = Instant::now();
        for _ in 0..iterations {
            quat_packed_state_set(state_ptr, 15, 0b11);
        }
        let duration = start.elapsed();
        println!(
            "\n[BENCH] FFI State Set (1M iterations): Total: {:?}, Average per call: {:?}", 
            duration, duration / iterations
        );

        // Measure FFI transition validation performance
        let start_val = Instant::now();
        for _ in 0..iterations {
            quat_validate_transition_ffi(0b00, 0b11);
        }
        let duration_val = start_val.elapsed();
        println!(
            "\n[BENCH] FFI Validation (1M iterations): Total: {:?}, Average per call: {:?}", 
            duration_val, duration_val / iterations
        );

        quat_packed_state_destroy64(state_ptr);
    }
}

}