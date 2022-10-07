extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

const WINDOW_WIDTH: u32 = 680;
const WINDOW_HEIGHT: u32 = 480;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("An SDL2 window", WINDOW_WIDTH, WINDOW_HEIGHT)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'is_running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'is_running,
                _ => {}
            }
        }

        {
            // Game loop..
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            // Present the backbuffer
            canvas.present();
        }
    }

    Ok(())
}
