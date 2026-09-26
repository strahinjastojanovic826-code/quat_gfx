#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GraphicsBackend {
    DirectX12 = 0,
    Vulkan = 1,
    Metal = 2,
}

impl GraphicsBackend {
    pub fn from_u8(val: u8) -> Self {
        match val {
            0 => GraphicsBackend::DirectX12,
            1 => GraphicsBackend::Vulkan,
            _ => GraphicsBackend::Metal,
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