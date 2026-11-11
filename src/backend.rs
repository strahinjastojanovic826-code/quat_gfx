#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GraphicsBackend {
    DirectX12 = 0,
    Vulkan = 1,
    Metal = 2,
    WebGPU = 3,
    OpenGL = 4,
}

impl GraphicsBackend {
    pub fn from_u8(val: u8) -> Self {
        match val & 0b11 { // Kvartarno maskiranje
            0 => GraphicsBackend::DirectX12,
            1 => GraphicsBackend::Vulkan,
            2 => GraphicsBackend::Metal,
            3 => GraphicsBackend::WebGPU,
            _ => GraphicsBackend::OpenGL,
        }
    }

    pub fn current_platform_default() -> Self {
        #[cfg(target_os = "windows")]
        {
            GraphicsBackend::DirectX12
        }
        #[cfg(target_os = "linux")]
        {
            GraphicsBackend::Vulkan
        }
        #[cfg(target_os = "macos")]
        {
            GraphicsBackend::Metal
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            GraphicsBackend::Vulkan
        }
    }
}

use crate::state::ResourceState;

/// Hardware-specific barrier descriptor powered by Quaternary Logic validation
#[repr(C)]
pub struct NativeResourceBarrier {
    pub resource_id: u32,
    pub from_quaternary_state: u8, // Base-4 encoded state (00, 01, 10, 11)
    pub to_quaternary_state: u8,   // Base-4 encoded state (00, 01, 10, 11)
}

impl NativeResourceBarrier {
    /// Translates abstract quaternary states into real GPU driver commands (Vulkan / DX12 / Metal)
    pub fn resolve_hardware_barrier(&self) -> Result<(), &'static str> {
        let from_state = ResourceState::from(self.from_quaternary_state);
        let to_state = ResourceState::from(self.to_quaternary_state);

        // Enforce Quaternary Validation Matrix before hitting the driver
        if !ResourceState::validate_transition(from_state, to_state) {
            return Err("Hardware Barrier Error: Invalid Quaternary State Transition!");
        }

        // Match backend and dispatch real driver commands
        // (Example integration stub for Vulkan / DX12 API translation)
        match self.to_quaternary_state {
            0b00 => {
                // TODO: Execute Vulkan VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT / DX12 D3D12_RESOURCE_STATE_COMMON
            }
            0b01 => {
                // TODO: Execute Vulkan VK_ACCESS_SHADER_READ_BIT / DX12 D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE
            }
            0b10 => {
                // TODO: Execute Vulkan VK_ACCESS_TRANSFER_WRITE_BIT / DX12 D3D12_RESOURCE_STATE_COPY_DEST
            }
            0b11 => {
                // TODO: Execute Vulkan VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT / DX12 D3D12_RESOURCE_STATE_RENDER_TARGET
            }
            _ => return Err("Unknown Quaternary Digit in Driver Pipeline"),
        }

        Ok(())
    }
}