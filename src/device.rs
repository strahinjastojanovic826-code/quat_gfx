use crate::backend::GraphicsBackend;
use crate::command::CommandList;
use crate::resource::{GpuResource, NativeGpuHandle, ResourceDesc};
use crate::state::ResourceState;

pub trait GpuDevice {
    fn backend_type(&self) -> GraphicsBackend;
    fn create_resource(&mut self, desc: ResourceDesc) -> GpuResource;
    fn transition_resource(
        &self,
        cmd_list: &mut CommandList,
        resource: &mut GpuResource,
        new_state: ResourceState,
    );
}

pub struct DeviceManager {
    backend: GraphicsBackend,
    next_resource_id: u32,
}

impl DeviceManager {
    pub fn new(backend: GraphicsBackend) -> Self {
        Self {
            backend,
            next_resource_id: 1,
        }
    }

    pub fn create_command_list(&self) -> CommandList {
        CommandList::new(self.backend)
    }
    pub fn safe_transition_resource(
        &mut self,
        cmd_list: &mut crate::command::CommandList,
        resource: &mut crate::resource::GpuResource,
        new_state: ResourceState,
    ) -> Result<(), &'static str> {
        let old_state = resource.current_state();

        if old_state == new_state {
            return Ok(()); // Nema potrebe za tranzicijom
        }

        // Kvartarna validacija
        if !old_state.is_transition_valid(new_state) {
            return Err("Invalid resource state transition detected by Quat validation matrix!");
        }

        cmd_list.resource_barrier(resource.id(), old_state, new_state);
        resource.set_state(new_state);
        Ok(())
    }

}

impl GpuDevice for DeviceManager {
    fn backend_type(&self) -> GraphicsBackend {
        self.backend
    }

    fn create_resource(&mut self, desc: ResourceDesc) -> GpuResource {
        let id = self.next_resource_id;
        self.next_resource_id += 1;

        // Kreiranje nativnih ručki za izabrani backend
        let handle = match self.backend {
            GraphicsBackend::DirectX12 => NativeGpuHandle::Dx12Resource(std::ptr::null_mut()),
            GraphicsBackend::Vulkan => NativeGpuHandle::VulkanImage(0),
            GraphicsBackend::Metal => NativeGpuHandle::MetalTexture(std::ptr::null_mut()),
            GraphicsBackend::WebGPU => NativeGpuHandle::WebGpuBuffer(std::ptr::null_mut()),
            GraphicsBackend::OpenGL => NativeGpuHandle::OpenGlTexture(0),
        };

        GpuResource::new(id, desc, handle)
    }

    fn transition_resource(
        &self,
        cmd_list: &mut CommandList,
        resource: &mut GpuResource,
        new_state: ResourceState,
    ) {
        let old_state = resource.current_state();
        if old_state == new_state {
            return;
        }

        cmd_list.resource_barrier(resource.id(), old_state, new_state);
        resource.set_state(new_state);
    }
}