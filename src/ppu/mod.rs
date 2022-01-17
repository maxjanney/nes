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
    oam: u16,
    /// OAM data, $2004
    data: [u8; OAM_SIZE],
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
            oam: 0,
            data: [0u8; OAM_SIZE],
            ram: [0u8; RAM_SIZE],
        }
    }
}
