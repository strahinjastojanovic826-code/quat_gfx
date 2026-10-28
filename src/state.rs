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
    pub fn is_transition_valid(self, next: Self) -> bool {
        // Matrica 4x4 bazirana na kvartarnim indeksima (0..3)
        // Red = trenutno stanje, Kolona = novo stanje
        #[rustfmt::skip]
        const VALIDATION_MATRIX: [[bool; 4]; 4] = [
            // State00 (Undefined) -> Može u bilo koje stanje
            [true,  true,  true,  true],
            // State01 (Read-Only) -> Može u Read ili Write, ali ne direktno u Present
            [true,  true,  true,  false],
            // State10 (Storage Write) -> Mora ići u Read ili Render Target pre čitanja
            [true,  true,  true,  true],
            // State11 (Render Target / Present) -> Tranzicija dozvoljena
            [true,  true,  true,  true],
        ];

        let from_idx = self as usize;
        let to_idx = next as usize;

        VALIDATION_MATRIX[from_idx][to_idx]
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
    pub fn invert(self) -> Self {
        match self {
            QuatDigit::Q0 => QuatDigit::Q3,
            QuatDigit::Q1 => QuatDigit::Q2,
            QuatDigit::Q2 => QuatDigit::Q1,
            QuatDigit::Q3 => QuatDigit::Q0,
        }
    }

    /// Kvartarno MIN (ekvivalent AND u viševrednosnoj logici)
    pub fn min(self, other: Self) -> Self {
        QuatDigit::from_bits((self as u8).min(other as u8))
    }

    /// Kvartarno MAX (ekvivalent OR u viševrednosnoj logici)
    pub fn max(self, other: Self) -> Self {
        QuatDigit::from_bits((self as u8).max(other as u8))
    }

    /// Sabiranje dve kvartarne cifre sa prenosom (Modularna aritmetika u bazi 4)
    /// Vraća (rezultat, prenos)
    pub fn add_with_carry(self, other: Self, carry_in: Self) -> (Self, Self) {
        let sum = (self as u8) + (other as u8) + (carry_in as u8);
        let result_digit = QuatDigit::from_bits(sum & 0b11); // mod 4 preko maske 0b11
        let carry_out = QuatDigit::from_bits((sum >> 2) & 0b11); // div 4
        (result_digit, carry_out)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PackedQuatState64(pub u64);

impl PackedQuatState64 {
    pub const CAPACITY: usize = 32;

    pub fn new() -> Self {
        Self(0)
    }

    /// Preuzima kvat na zadatom indeksu (0..31)
    #[inline(always)]
    pub fn get_quat(&self, index: usize) -> QuatDigit {
        assert!(index < Self::CAPACITY, "Index out of bounds");
        let shift = (index & 0b11111) * 2; // Maska 31 za opseg 0..31
        let bits = ((self.0 >> shift) & 0b11) as u8;
        QuatDigit::from_bits(bits)
    }

    /// Postavlja kvat na zadatom indeksu (0..31)
    #[inline(always)]
    pub fn set_quat(&mut self, index: usize, quat: QuatDigit) {
        assert!(index < Self::CAPACITY, "Index out of bounds");
        let shift = (index & 0b11111) * 2;
        let mask = !(0b11u64 << shift);
        self.0 = (self.0 & mask) | ((quat as u64) << shift);
    }

    /// Iterator kroz sva 32 kvat stanja u registru
    pub fn iter(&self) -> PackedQuatIterator {
        PackedQuatIterator {
            packed: *self,
            current_index: 0,
        }
    }
}

pub struct PackedQuatIterator {
    packed: PackedQuatState64,
    current_index: usize,
}

impl Iterator for PackedQuatIterator {
    type Item = QuatDigit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < PackedQuatState64::CAPACITY {
            let digit = self.packed.get_quat(self.current_index);
            self.current_index += 1;
            Some(digit)
        } else {
            None
        }
    }
}