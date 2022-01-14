mod isa;

use bitflags::bitflags;

#[derive(Debug)]
pub struct Registers {
    /// Accumulator
    a: u8,
    /// X index register
    x: u8,
    // Y index register
    y: u8,
    // Program counter
    pc: u16,
    // Stack pointer
    sp: u16,
    /// Processor status register
    psr: Psr,
}

bitflags! {
    struct Psr: u8 {
        /// Carry flag
        const C = 0b00000001;
        /// Zero flag
        const Z = 0b00000010;
        /// IRQ Disable flag
        const I = 0b00000100;
        /// Decimal mode flag
        const D = 0b00001000;
        /// BRK Command flag
        const B = 0b00010000;
        /// Unused
        const U = 0b00100000;
        /// Overflow flag
        const V = 0b01000000;
        /// Negative flag
        const N = 0b10000000;
    }
}

impl Psr {
    fn new() -> Self {
        Self::from_bits_truncate(0x34)
    }
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0x01fd,
            psr: Psr::new(),
        }
    }

    fn bump(&mut self) -> u16 {
        let pc = self.pc;
        self.pc = self.pc.wrapping_add(1);
        pc
    }
}
