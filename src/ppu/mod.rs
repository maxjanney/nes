mod regs;

use regs::*;

const OAM_SIZE: usize = 64 * 4;
const RAM_SIZE: usize = 2048;

pub struct Ppu {
    /// Control register
    ctrl: Control,
    /// Mask register
    mask: Mask,
    /// Status register
    stat: Status,
    /// Scroll register
    scroll: Scroll,
    /// Address register
    addr: Addr,
    /// OAM address, $2003
    oam_addr: u16,
    /// OAM data, $2004
    oam_data: [u8; OAM_SIZE],
    /// Ppu's ram, $2007
    ram: [u8; RAM_SIZE],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            ctrl: Control::empty(),
            mask: Mask::empty(),
            stat: Status::empty(),
            scroll: Scroll::default(),
            addr: Addr::new(),
            oam_addr: 0,
            oam_data: [0u8; OAM_SIZE],
            ram: [0u8; RAM_SIZE],
        }
    }

    // Read the status register
    pub fn stat(&mut self) -> u8 {
        let bits = self.stat.bits();
        self.stat.remove(Status::V);
        self.scroll.latched = false;
        self.addr.hi = true;
        bits
    }

    // Read a byte from the OAM
    pub fn read_oam(&self) -> u8 {
        self.oam_data[self.oam_addr as usize]
    }

    // Read the data register
    pub fn read_data(&mut self) -> u8 {
        let addr = self.addr.raw;
        self.addr.increment(self.ctrl.increment_amt());
        0
    }
}
