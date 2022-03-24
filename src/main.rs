use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use std::{env, fs, vec};

const WIDTH: usize = 2 * 256;
const HEIGHT: usize = 240;

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

    let mut canvas = window.into_canvas().build()?;
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

    // Get the left or right bank
    let bank = tile_bank(&char_rom, Bank::Right);

    texture.update(None, &bank, 256 * 3)?;
    canvas.copy(&texture, None, None)?;
    canvas.present();

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
    let mut tiles = vec![0; 3 * WIDTH * HEIGHT];
    let bank = bank as usize * 0x1000;
    let mut x = 0;
    let mut y = 0;

    for tilen in 0..256 {
        if tilen != 0 && tilen % 25 == 0 {
            x = 0;
            y += 10;
        }

        let tilei = 16 * tilen + bank;
        let tile = &rom[tilei..(tilei + 16)];

        for i in 0..8 {
            let (mut lo, mut hi) = (tile[i], tile[i + 8]);
            for j in (0..8).rev() {
                let v = ((hi & 1) << 1) | (lo & 1);
                lo >>= 1;
                hi >>= 1;
                // just randomly choose the pixel's color
                let color = match v {
                    0 => SYSTEM_PALETTE[0x01],
                    1 => SYSTEM_PALETTE[0x23],
                    2 => SYSTEM_PALETTE[0x27],
                    3 => SYSTEM_PALETTE[0x30],
                    _ => panic!("{v} is too large!"),
                };
                set_pixels(x + j, y + i, color, &mut tiles);
            }
        }
        x += 10;
    }

    tiles
}

fn set_pixels(x: usize, y: usize, color: (u8, u8, u8), tiles: &mut [u8]) {
    let index = 3 * (WIDTH * y + x);
    tiles[index + 0] = color.0;
    tiles[index + 1] = color.1;
    tiles[index + 2] = color.2;
}

#[rustfmt::skip]
static SYSTEM_PALETTE: [(u8, u8, u8); 64] = [
   (0x80, 0x80, 0x80), (0x00, 0x3D, 0xA6), (0x00, 0x12, 0xB0), (0x44, 0x00, 0x96), (0xA1, 0x00, 0x5E),
   (0xC7, 0x00, 0x28), (0xBA, 0x06, 0x00), (0x8C, 0x17, 0x00), (0x5C, 0x2F, 0x00), (0x10, 0x45, 0x00),
   (0x05, 0x4A, 0x00), (0x00, 0x47, 0x2E), (0x00, 0x41, 0x66), (0x00, 0x00, 0x00), (0x05, 0x05, 0x05),
   (0x05, 0x05, 0x05), (0xC7, 0xC7, 0xC7), (0x00, 0x77, 0xFF), (0x21, 0x55, 0xFF), (0x82, 0x37, 0xFA),
   (0xEB, 0x2F, 0xB5), (0xFF, 0x29, 0x50), (0xFF, 0x22, 0x00), (0xD6, 0x32, 0x00), (0xC4, 0x62, 0x00),
   (0x35, 0x80, 0x00), (0x05, 0x8F, 0x00), (0x00, 0x8A, 0x55), (0x00, 0x99, 0xCC), (0x21, 0x21, 0x21),
   (0x09, 0x09, 0x09), (0x09, 0x09, 0x09), (0xFF, 0xFF, 0xFF), (0x0F, 0xD7, 0xFF), (0x69, 0xA2, 0xFF),
   (0xD4, 0x80, 0xFF), (0xFF, 0x45, 0xF3), (0xFF, 0x61, 0x8B), (0xFF, 0x88, 0x33), (0xFF, 0x9C, 0x12),
   (0xFA, 0xBC, 0x20), (0x9F, 0xE3, 0x0E), (0x2B, 0xF0, 0x35), (0x0C, 0xF0, 0xA4), (0x05, 0xFB, 0xFF),
   (0x5E, 0x5E, 0x5E), (0x0D, 0x0D, 0x0D), (0x0D, 0x0D, 0x0D), (0xFF, 0xFF, 0xFF), (0xA6, 0xFC, 0xFF),
   (0xB3, 0xEC, 0xFF), (0xDA, 0xAB, 0xEB), (0xFF, 0xA8, 0xF9), (0xFF, 0xAB, 0xB3), (0xFF, 0xD2, 0xB0),
   (0xFF, 0xEF, 0xA6), (0xFF, 0xF7, 0x9C), (0xD7, 0xE8, 0x95), (0xA6, 0xED, 0xAF), (0xA2, 0xF2, 0xDA),
   (0x99, 0xFF, 0xFC), (0xDD, 0xDD, 0xDD), (0x11, 0x11, 0x11), (0x11, 0x11, 0x11),
];
