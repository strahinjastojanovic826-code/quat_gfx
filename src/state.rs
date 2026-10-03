use bitflags::bitflags;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResourceState {
    /// Kvartarno 0 (0b00): Undefined / Common
    State00 = 0b00,
    /// Kvartarno 1 (0b01): Shader Resource / Read-Only
    State01 = 0b01,
    /// Kvartarno 2 (0b10): Copy Destination / Storage Write
    State10 = 0b10,
    /// Kvartarno 3 (0b11): Render Target / Depth Write / Present
    State11 = 0b11,
}

impl From<u8> for ResourceState {
    fn from(val: u8) -> Self {
        // Maskiranje na najniža 2 bita (Baza 4: vrednosti 0, 1, 2, 3)
        match val & 0b11 {
            0b00 => ResourceState::State00,
            0b01 => ResourceState::State01,
            0b10 => ResourceState::State10,
            _ => ResourceState::State11,
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

// Bitflags 2.4 integracija za kombinovana stanja
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ResourceStateFlags: u8 {
        const NONE           = 0b00;
        const SHADER_READ    = 0b01;
        const COPY_DEST      = 0b10;
        const RENDER_TARGET  = 0b11;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum QuatDigit {
    Q0 = 0b00, // 0 u bazi 4
    Q1 = 0b01, // 1 u bazi 4
    Q2 = 0b10, // 2 u bazi 4
    Q3 = 0b11, // 3 u bazi 4
}

impl QuatDigit {
    #[inline(always)]
    pub fn from_bits(val: u8) -> Self {
        match val & 0b11 {
            0b00 => QuatDigit::Q0,
            0b01 => QuatDigit::Q1,
            0b10 => QuatDigit::Q2,
            _    => QuatDigit::Q3,
        }
    }
}

/// Pakuje 4 Kvat stanja u samo 1 bajt (u8)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackedQuatState(pub u8);

impl PackedQuatState {
    pub fn get_quat(&self, index: usize) -> QuatDigit {
        let shift = (index & 0b11) * 2;
        QuatDigit::from_bits((self.0 >> shift) & 0b11)
    }

    pub fn set_quat(&mut self, index: usize, quat: QuatDigit) {
        let shift = (index & 0b11) * 2;
        let mask = !(0b11 << shift);
        self.0 = (self.0 & mask) | ((quat as u8) << shift);
    }
}