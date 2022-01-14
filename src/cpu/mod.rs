mod mos6502;

use crate::mem::Memory;
use mos6502::Registers;

pub struct Cpu {
    regs: Registers,
    ticks: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: Registers::new(),
            ticks: 0,
        }
    }

    pub fn step(&mut self, mem: &mut Memory) -> u8 {
        0
    }
}
