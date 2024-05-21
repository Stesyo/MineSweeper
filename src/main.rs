use core::Board;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, CanvasBuilder};
use sdl2::surface::Surface;
use std::fmt::Display;

pub mod core;
pub mod draw;

fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Saper", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut board = Board::new(10, 10, 0);

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(()),
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("Escaping!");
                    return Ok(());
                }
                _ => {}
            }
        }

        board.tick();

        canvas.present();
    }
}

fn main() {
    let result = run();
    match result {
        Ok(_) => {}
        Err(e) => {
            println!("Program finished with error: {}", e);
        }
    }
}
