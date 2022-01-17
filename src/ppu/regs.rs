use bitflags::bitflags;

// Write only Ppu Control register, mapped to $2000
bitflags! {
    pub(super) struct Control: u8 {
        /// Base nametable address
        /// (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
        const N1 = 0b00000001;
        const N2 = 0b00000010;
        /// VRAM address increment per CPU read/write of PPUDATA
        /// (0: add 1, going across; 1: add 32, going down)
        const I = 0b00000100;
        /// Sprite pattern table address for 8x8 sprites
        /// (0: $0000; 1: $1000; ignored in 8x16 mode)
        const S = 0b00001000;
        /// Background pattern table address
        /// (0: $0000; 1: $1000)
        const B = 0b00010000;
        /// Sprite size
        /// (0: 8x8 pixels; 1: 8x16 pixels)
        const H = 0b00100000;
        /// Ppu master/slave select
        /// (0: read backdrop from EXT pins; 1: output color on EXT pins)
        const P = 0b01000000;
        /// Generate an NMI at the start of the vertical blanking interval
        /// (0: off; 1: on)
        const V = 0b10000000;
    }
}

impl Control {
    pub(super) fn increment_amt(&self) -> u16 {
        if self.contains(Control::I) {
            32
        } else {
            1
        }
    }
}

// Write only Ppu Mask register, mapped to $2001
bitflags! {
    pub(super) struct Mask: u8 {
        /// Greyscale
        /// (0: normal color, 1: produce a greyscale display)
        const GS = 0b00000001;
        /// 1: Show background in leftmost 8 pixels of screen, 0: Hide
        const RC1 = 0b00000010;
        /// 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
        const RC2 = 0b00000100;
        /// 1: Show background
        const RC3 = 0b00001000;
        /// 1: Show sprites
        const RC4 = 0b00010000;
        /// Emphasize red (green on PAL/Dendy)
        const R = 0b00100000;
        /// Emphasize green (red on PAL/Dendy)
        const G = 0b01000000;
        /// Emphasize blue
        const B = 0b10000000;
    }
}

// Read only Ppu Status register, mapped to $2002
bitflags! {
    pub(super) struct Status: u8 {
        /// Least significant bits previously written into a Ppu register
        /// I have no idea if these are actually used
        const U1 = 0b00000001;
        const U2 = 0b00000010;
        const U3 = 0b00000100;
        const U4 = 0b00001000;
        const U5 = 0b00010000;
        /// Sprite overflow
        const O = 0b00100000;
        /// Sprite 0 Hit
        const S = 0b01000000;
        /// Vertical blank has started
        /// (0: not in vblank; 1: in vblank).
        const V = 0b10000000;
    }
}

// Write only Ppu Scrolling position register, mapped to $2005
#[derive(Default)]
pub(super) struct Scroll {
    /// scroll-x
    x: u8,
    /// scroll-y
    y: u8,
    /// alternate writes
    latched: bool,
}

impl Scroll {
    pub(super) fn set(&mut self, val: u8) {
        if self.latched {
            self.y = val;
        } else {
            self.x = val;
        }
        self.latched = !self.latched;
    }
}

const ADDR_MAX: u16 = 0x3fff;

// Write only Ppu Address register, mapped to $2006
pub(super) struct Addr {
    /// Raw addr
    raw: u16,
    /// Alternate between hi and lo
    hi: bool,
}

impl Addr {
    pub(super) fn new() -> Self {
        Self { raw: 0, hi: true }
    }

    pub(super) fn set(&mut self, val: u8) {
        self.raw = if self.hi {
            (self.raw & 0x00ff) | ((val as u16) << 8)
        } else {
            (self.raw & 0xff00) | (val as u16)
        } & ADDR_MAX;
        self.hi = !self.hi
    }

    pub(super) fn increment(&mut self, amt: u16) {
        self.raw = self.raw.wrapping_add(amt) & ADDR_MAX;
    }
}
