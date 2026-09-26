#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ResourceState {
    /// State 00 (0): Undefined / Common State (Initial or General Access)
    State00 = 0b00,
    /// State 01 (1): Shader Resource / Read-Only / Copy Source
    State01 = 0b01,
    /// State 10 (2): Copy Destination / Storage Buffer Write / UAV
    State10 = 0b10,
    /// State 11 (3): Render Target / Depth-Stencil Write / Present
    State11 = 0b11,
}

impl From<u8> for ResourceState {
    fn from(val: u8) -> Self {
        match val & 0b11 {
            0b00 => ResourceState::State00,
            0b01 => ResourceState::State01,
            0b10 => ResourceState::State10,
            _    => ResourceState::State11,
        }
    }
}

impl ResourceState {
    pub fn is_writable(&self) -> bool {
        matches!(self, ResourceState::State10 | ResourceState::State11)
    }

    pub fn numeric_value(&self) -> u8 {
        *self as u8
    }
}