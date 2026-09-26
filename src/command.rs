use crate::state::ResourceState;
use crate::backend::GraphicsBackend;

pub struct CommandList {
    backend: GraphicsBackend,
    commands: Vec<String>,
}

impl CommandList {
    pub fn new(backend: GraphicsBackend) -> Self {
        Self {
            backend,
            commands: Vec::new(),
        }
    }

    pub fn resource_barrier(&mut self, resource_id: u32, from_state: ResourceState, to_state: ResourceState) {
        match self.backend {
            GraphicsBackend::DirectX12 => {
                let cmd = format!(
                    "[DX12] D3D12_RESOURCE_BARRIER for Resource {} : State {:?} (0b{:02b}) -> State {:?} (0b{:02b})",
                    resource_id, from_state, from_state.numeric_value(), to_state, to_state.numeric_value()
                );
                self.commands.push(cmd);
            }
            GraphicsBackend::Vulkan => {
                let cmd = format!(
                    "[Vulkan] VkImageMemoryBarrier / VkBufferMemoryBarrier for Resource {} : State {:?} (0b{:02b}) -> State {:?} (0b{:02b})",
                    resource_id, from_state, from_state.numeric_value(), to_state, to_state.numeric_value()
                );
                self.commands.push(cmd);
            }
            GraphicsBackend::Metal => {
                let cmd = format!(
                    "[Metal] MTLBlitCommandEncoder / RenderPass Barrier for Resource {} : State {:?} (0b{:02b}) -> State {:?} (0b{:02b})",
                    resource_id, from_state, from_state.numeric_value(), to_state, to_state.numeric_value()
                );
                self.commands.push(cmd);
            }
        }
    }

    pub fn draw(&mut self, vertex_count: u32) {
        self.commands.push(format!("DrawPrimitives(vertex_count: {})", vertex_count));
    }

    pub fn flush_commands(&self) {
        for cmd in &self.commands {
            println!("{}", cmd);
        }
    }
}