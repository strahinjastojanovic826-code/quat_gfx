use crate::resource::ResourceDesc;

pub struct GpuMemoryBlock {
    pub id: u32,
    pub size_in_bytes: u64,
    pub allocated: bool,
    pub offset: u64,
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

        let offset: u64 = self.blocks.iter().map(|b| b.size_in_bytes).sum();

        self.blocks.push(GpuMemoryBlock {
            id: block_id,
            size_in_bytes: desc.size_in_bytes,
            allocated: true,
            offset,
        });

        block_id
    }

    pub fn free(&mut self, block_id: u32) -> bool {
    if let Some(block) = self.blocks.iter_mut().find(|b| b.id == block_id) {
        block.allocated = false;
        true
    } else {
        false
    }
}

pub fn allocate_reusable(&mut self, size_in_bytes: u64) -> Option<u32> {
    if let Some(block) = self
        .blocks
        .iter_mut()
        .find(|b| !b.allocated && b.size_in_bytes >= size_in_bytes)
    {
        block.allocated = true;
        Some(block.id)
    } else {
        None
    }
}

}