use std::cell::RefCell;

use crate::ppu::{Mirroring, Ppu};

const RAM_SIZE: usize = 0x800;

pub struct Memory {
    ram: [u8; RAM_SIZE],
    ppu: RefCell<Ppu>,
}

impl Memory {
    pub fn new(char_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Self {
            ram: [0; RAM_SIZE],
            ppu: RefCell::new(Ppu::new(char_rom, mirroring)),
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            // 2 KB internam RAM mirrors
            0x0000..=0x1fff => self.ram[(addr & 0x7ff) as usize],
            // PPU Status register
            0x2002 => self.ppu.borrow_mut().read_stat(),
            // PPU OAM data
            0x2004 => self.ppu.borrow().read_oam(),
            // PPU Data register
            0x2007 => self.ppu.borrow_mut().read_vram(),
            _ => 0,
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        (self.read8(addr) as u16) | ((self.read8(addr + 1) as u16) << 8)
    }

    pub fn write8(&mut self, addr: u16, val: u8) {
        let mut ppu = self.ppu.borrow_mut();
        match addr {
            // PPU Controller register
            0x2000 => ppu.write_ctrl(val),
            // PPU Mask register
            0x2001 => ppu.write_mask(val),
            // PPU OAM address
            0x2003 => ppu.write_oam_addr(val),
            // PPU OAM data
            0x2004 => ppu.write_oam_data(val),
            // PPU Scroll register
            0x2005 => ppu.write_scroll(val),
            // PPU Address register
            0x2006 => ppu.write_address(val),
            // PPU Data register
            0x2007 => ppu.write_vram(val),
            _ => {}
        }
    }
}
