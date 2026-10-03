use crate::state::ResourceState;

#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Buffer = 0,
    Texture2D = 1,
    Texture3D = 2,
}

#[derive(Debug, Clone)]
pub struct ResourceDesc {
    pub resource_type: ResourceType,
    pub size_in_bytes: u64,
    pub width: u32,
    pub height: u32,
    pub initial_state: ResourceState,
}

/// Nativne ručke za drajvere umjesto simulacije sa u32
#[derive(Debug)]
pub enum NativeGpuHandle {
    Null,
    Dx12Resource(*mut std::ffi::c_void),
    VulkanImage(u64),
    MetalTexture(*mut std::ffi::c_void),
}

pub struct GpuResource {
    id: u32,
    desc: ResourceDesc,
    current_state: ResourceState,
    pub handle: NativeGpuHandle,
}

impl GpuResource {
    pub fn new(id: u32, desc: ResourceDesc, handle: NativeGpuHandle) -> Self {
        let initial_state = desc.initial_state;
        Self {
            id,
            desc,
            current_state: initial_state,
            handle,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn current_state(&self) -> ResourceState {
        self.current_state
    }

    pub fn set_state(&mut self, new_state: ResourceState) {
        self.current_state = new_state;
    }

    pub fn desc(&self) -> &ResourceDesc {
        &self.desc
    }
}