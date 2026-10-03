use crate::backend::GraphicsBackend;
use crate::state::ResourceState;

pub struct CommandList {
    backend: GraphicsBackend,
    // Ovde u produkciji ide npr. ID3D12GraphicsCommandList ili VkCommandBuffer
    commands_count: usize,
}

impl CommandList {
    pub fn new(backend: GraphicsBackend) -> Self {
        Self {
            backend,
            commands_count: 0,
        }
    }

    pub fn resource_barrier(
        &mut self,
        resource_id: u32,
        from_state: ResourceState,
        to_state: ResourceState,
    ) {
        match self.backend {
            GraphicsBackend::DirectX12 => {
                // Implementirati D3D12_RESOURCE_BARRIER tranziciju
                self.commands_count += 1;
            }
            GraphicsBackend::Vulkan => {
                // Implementirati VkImageMemoryBarrier / VkBufferMemoryBarrier
                self.commands_count += 1;
            }
            GraphicsBackend::Metal => {
                // Implementirati MTLRenderPassDescriptor / MTLBlitCommandEncoder
                self.commands_count += 1;
            }
        }
    }

    pub fn draw(&mut self, _vertex_count: u32) {
        self.commands_count += 1;
    }

    pub fn flush_commands(&mut self) {
        // Submit komandne liste na GPU queue
        self.commands_count = 0;
    }
}