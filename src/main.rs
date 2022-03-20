use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::perror;
use sdl2::pixels::PixelFormatEnum;

use std::{env, fs};

#[derive(Debug, Clone, Copy)]
enum Bank {
    Left = 0,
    Right = 1,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SDL init
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Tile viewer", 256 * 3, 240 * 3)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().present_vsync().build()?;
    let mut event_pump = sdl_context.event_pump()?;
    canvas.set_scale(3.0, 3.0)?;

    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture_target(PixelFormatEnum::RGB24, 256, 240)?;

    // Load the ROM file
    let rom_file = env::args().nth(1).expect("Missing ROM file");
    let rom = fs::read(rom_file)?;

    if rom[7] & 0x80 != 0 {
        return Err("NES2.0 is not supported :(".into());
    }

    // Extract char rom
    let prg_size = rom[4] as usize * 0x4000;
    let prg_start = 16 + if rom[6] & 0x4 != 0 { 512 } else { 0 };

    let char_size = rom[5] as usize * 0x2000;
    let char_start = prg_start + prg_size;
    let char_rom = &rom[char_start..(char_start + char_size)];

    let mut left_bank = tile_bank(&char_rom, Bank::Left);

    // Main loop
    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                _ => {}
            }
        }
    }
}

fn tile_bank(rom: &[u8], bank: Bank) -> Vec<u8> {
    let mut tiles = Vec::with_capacity(2 * 256 * 240 * 3);
    let bank = bank as usize * 0x1000;

    for i in 0..255 {
        let start = 16 * i + bank;
        let tile = &rom[start..(start + 16)];
        for i in 0..8 {
            let (mut lo, mut hi) = (tile[i], tile[i + 8]);
            for j in (0..8).rev() {
                let v = ((hi & 1) << 1) | (lo & 1);
                lo >>= 1;
                hi >>= 1;
                // just randomly choose the pixel's color
                let color = match v {
                    0 => SYSTEM_PALETTE[0x09],
                    1 => SYSTEM_PALETTE[0x06],
                    2 => SYSTEM_PALETTE[0x1c],
                    3 => SYSTEM_PALETTE[0x33],
                    _ => panic!("{v} is too large!"),
                };
            }
        }
    }

    tiles
}

const SYSTEM_PALETTE: [(u8, u8, u8); 64] = [
    (92, 92, 92),
    (0, 34, 103),
    (19, 18, 128),
    (46, 6, 126),
    (70, 0, 96),
    (83, 2, 49),
    (81, 10, 2),
    (65, 25, 0),
    (40, 41, 0),
    (13, 55, 0),
    (0, 62, 0),
    (0, 60, 10),
    (0, 49, 59),
    (0, 0, 0),
    (0, 0, 0),
    (0, 0, 0),
    (167, 167, 167),
    (30, 85, 183),
    (63, 61, 218),
    (102, 43, 214),
    (136, 34, 172),
    (154, 36, 107),
    (152, 50, 37),
    (129, 71, 0),
    (93, 95, 0),
    (54, 115, 0),
    (24, 125, 0),
    (9, 122, 50),
    (11, 107, 121),
    (0, 0, 0),
    (0, 0, 0),
    (0, 0, 0),
    (254, 255, 255),
    (106, 167, 255),
    (143, 141, 255),
    (185, 121, 255),
    (221, 111, 255),
    (241, 114, 190),
    (238, 129, 115),
    (214, 152, 55),
    (176, 178, 24),
    (134, 199, 28),
    (100, 209, 65),
    (82, 206, 129),
    (84, 190, 205),
    (69, 69, 69),
    (0, 0, 0),
    (0, 0, 0),
    (254, 255, 255),
    (192, 218, 255),
    (208, 207, 255),
    (226, 198, 255),
    (241, 194, 255),
    (249, 195, 228),
    (248, 202, 196),
    (238, 212, 169),
    (222, 223, 155),
    (204, 231, 157),
    (189, 236, 174),
    (181, 234, 202),
    (182, 228, 234),
    (176, 176, 176),
    (0, 0, 0),
    (0, 0, 0),
];
