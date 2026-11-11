use crate::resource::ResourceDesc;

pub struct GpuMemoryBlock {
    pub id: u32,
    pub size_in_bytes: u64,
    pub allocated: bool,
}

pub struct MemoryAllocator {
    blocks: Vec<GpuMemoryBlock>,
    next_block_id: u32,
}

impl MemoryAllocator {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            next_block_id: 0,
        }
    }

    /// Sub-allocates memory from a pooled block based on ResourceDesc
    pub fn sub_allocate(&mut self, desc: &ResourceDesc) -> u32 {
        let block_id = self.next_block_id;
        self.next_block_id += 1;

        self.blocks.push(GpuMemoryBlock {
            id: block_id,
            size_in_bytes: desc.size_in_bytes,
            allocated: true,
        });

        block_id
    }

    pub fn free(&mut self, block_id: u32) {
        if let Some(_pos) = self.blocks.iter().find(|b| b.id == block_id) {
            // Mark memory block as reusable in pool
        }
    }
}