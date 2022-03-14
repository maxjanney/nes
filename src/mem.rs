use std::cell::RefCell;

use crate::ppu::Ppu;

const RAM_SIZE: usize = 0x800;

pub struct Memory {
    ram: [u8; RAM_SIZE],
    ppu: RefCell<Ppu>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: [0; RAM_SIZE],
            ppu: RefCell::new(Ppu::new()),
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            // 2 KB internam RAM mirrors
            0x0000..=0x1fff => self.ram[(addr & 0x7ff) as usize],
            // PPU Status register
            0x2002 => self.ppu.borrow_mut().stat(),
            // PPU OAM data
            0x2004 => self.ppu.borrow().read_oam(),
            // PPU Data register
            0x2007 => self.ppu.borrow_mut().read_data(),
            _ => 0,
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        (self.read8(addr) as u16) | ((self.read8(addr + 1) as u16) << 8)
    }

    pub fn write8(&mut self, addr: u16, val: u8) {}
}
