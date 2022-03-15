mod regs;

use regs::*;

const OAM_SIZE: usize = 64 * 4;
const VRAM_SIZE: usize = 2048;

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
    vram: [u8; RAM_SIZE],
    /// Character rom
    char_rom: Vec<u8>,
    /// NMI Interrupt flag
    nmi: bool,
    /// Internal data buf
    data_buf: u8,
}

impl Ppu {
    pub fn new(char_rom: Vec<u8>) -> Self {
        Self {
            ctrl: Control::empty(),
            mask: Mask::empty(),
            stat: Status::empty(),
            scroll: Scroll::default(),
            addr: Addr::new(),
            oam_addr: 0,
            oam_data: [0u8; OAM_SIZE],
            vram: [0u8; RAM_SIZE],
            char_rom,
            nmi: false,
            data_buf: 0,
        }
    }

    // Read the status register
    pub fn read_stat(&mut self) -> u8 {
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

    // Read from vram
    pub fn read_vram(&mut self) -> u8 {
        let addr = self.addr.raw as usize;
        self.addr.increment(self.ctrl.increment_amt());
        match addr {
            // All reads in range 0 - $3eff will return the contents of an internal read buffer
            // this read buffer is updated after the read operation with the current vram address

            // Character rom/pattern tables
            0x0000..=0x1fff => {
                let res = self.data_buf;
                self.data_buf = self.char_rom[addr];
                res
            }
            // Internal vram/nametables
            0x2000..=0x2fff => {
                let res = self.data_buf;
                self.data_buf = self.vram[addr];
                res
            }
            _ => 0,
        }
    }

    // Write to the control register
    pub fn write_ctrl(&mut self, val: u8) {
        let nmi = self.ctrl.nmi();
        self.ctrl.update(val);
        if !nmi && self.ctrl.nmi() && self.stat.in_vblank() {
            self.nmi = true;
        }
    }

    // Write to the mask register
    pub fn write_mask(&mut self, val: u8) {
        self.mask.update(val);
    }

    // Write to the oam address
    pub fn write_oam_addr(&mut self, val: u8) {
        self.oam_addr = val as u16;
    }

    // Write oam data
    pub fn write_oam_data(&mut self, val: u8) {
        self.oam_data[self.oam_addr as usize] = val;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    // Write to the scroll register
    pub fn write_scroll(&mut self, val: u8) {
        self.scroll.update(val);
    }

    // Write to the address register
    pub fn write_address(&mut self, val: u8) {
        self.addr.update(val);
    }

    // Write to the data register
    pub fn write_vram(&mut self, val: u8) {
        let addr = self.addr.raw;
        match addr {
            _ => {}
        }
        self.addr.increment(self.ctrl.increment_amt());
    }
}
