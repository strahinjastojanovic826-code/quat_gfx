use crate::backend::GraphicsBackend;
use crate::state::ResourceState;
use crate::resource::{ResourceDesc, GpuResource};
use crate::command::CommandList;

pub trait GpuDevice {
    fn backend_type(&self) -> GraphicsBackend;
    fn create_resource(&mut self, desc: ResourceDesc) -> GpuResource;
    fn transition_resource(&self, cmd_list: &mut CommandList, resource: &mut GpuResource, new_state: ResourceState);
}

pub struct DeviceManager {
    backend: GraphicsBackend,
    next_resource_id: u32,
}

impl DeviceManager {
    pub fn new(backend: GraphicsBackend) -> Self {
        match backend {
            GraphicsBackend::DirectX12 => {
                println!("Initializing DirectX 12 Device, Command Queue, and Swapchain...");
            }
            GraphicsBackend::Vulkan => {
                println!("Initializing Vulkan Instance, Physical Device, Logical Device, and Queues...");
            }
            GraphicsBackend::Metal => {
                println!("Initializing Metal MTLDevice and Command Queues...");
            }
        }
        Self {
            backend,
            next_resource_id: 1,
        }
    }

    pub fn create_command_list(&self) -> CommandList {
        CommandList::new(self.backend)
    }
}

impl GpuDevice for DeviceManager {
    fn backend_type(&self) -> GraphicsBackend {
        self.backend
    }

    fn create_resource(&mut self, desc: ResourceDesc) -> GpuResource {
        let id = self.next_resource_id;
        self.next_resource_id += 1;
        
        match self.backend {
            GraphicsBackend::DirectX12 => {
                println!("[DX12] Creating committed resource ID {} in state {:?}", id, desc.initial_state);
            }
            GraphicsBackend::Vulkan => {
                println!("[Vulkan] Creating VkImage/VkBuffer ID {} in state {:?}", id, desc.initial_state);
            }
            GraphicsBackend::Metal => {
                println!("[Metal] Creating MTLTexture/MTLBuffer ID {} in state {:?}", id, desc.initial_state);
            }
        }

        GpuResource::new(id, desc)
    }

    fn transition_resource(&self, cmd_list: &mut CommandList, resource: &mut GpuResource, new_state: ResourceState) {
        let old_state = resource.current_state();
        if old_state == new_state {
            return;
        }

        cmd_list.resource_barrier(resource.id(), old_state, new_state);
        resource.set_state(new_state);
    }
}