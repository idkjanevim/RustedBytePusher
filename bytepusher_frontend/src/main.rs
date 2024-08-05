use std::{fs::File, io::Read};

use bytepusher_core::*;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

const SCALE: u32 = 3;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut loaded = false;

    let window = video_subsystem
        .window("Rusted BytePusher Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut bytepusher = Emulator::new();

    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. } => break 'gameloop,
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key2btn(key) {
                        bytepusher.keypress(k, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = key2btn(key) {
                        bytepusher.keypress(k, false);
                    }
                }
                Event::DropFile { filename, .. } => {
                    bytepusher.reset();
                    let mut rom = File::open(filename).expect("Unable to open file");
                    let mut buffer = Vec::new();
                    rom.read_to_end(&mut buffer).unwrap();
                    bytepusher.load(&buffer);
                    loaded = true;
                }
                _ => (),
            }
        }

        if !loaded {
            continue;
        }

        bytepusher.cycle();

        draw_screen(&bytepusher, &mut canvas);
    }
}

fn draw_screen(emu: &Emulator, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    let screen_buf = emu.get_screen();

    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let value = screen_buf[y][x];
            if value > 216 {
                canvas.set_draw_color(Color::BLACK);
            } else {
                let b = (value % 6) * 51;
                let g = ((value / 6) % 6) * 51;
                let r = ((value / 36) % 6) * 51;

                canvas.set_draw_color(Color::RGB(r, g, b));
            }

            let rect = Rect::new(
                (x as u32 * SCALE) as i32,
                (y as u32 * SCALE) as i32,
                SCALE,
                SCALE,
            );
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0x4),
        Keycode::Num5 => Some(0x5),
        Keycode::Num6 => Some(0x6),
        Keycode::Num7 => Some(0x7),
        Keycode::Num8 => Some(0x8),
        Keycode::Num9 => Some(0x9),
        Keycode::Num0 => Some(0x0),
        Keycode::A => Some(0xA),
        Keycode::B => Some(0xB),
        Keycode::C => Some(0xC),
        Keycode::D => Some(0xD),
        Keycode::E => Some(0xE),
        Keycode::F => Some(0xF),

        _ => None,
    }
}
