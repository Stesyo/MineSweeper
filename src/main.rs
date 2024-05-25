use core::Board;
use draw::Drawer;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::f64;

pub mod core;
pub mod draw;

fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Saper", 800, 800)
        .position_centered()
        .resizable()
        // .maximized()
        // .borderless()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut board = Board::new(30, 16, 99);
    let mut drawer = Drawer::new(&texture_creator);
    drawer.adjust(canvas.window().size(), &board);

    let mut moved = false;
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
                Event::Window {
                    win_event: WindowEvent::Resized(x, y),
                    ..
                } => {
                    drawer.viewport.set_width(x as u32);
                    drawer.viewport.set_height(y as u32);
                }
                Event::MouseMotion {
                    mousestate,
                    x,
                    y,
                    xrel,
                    yrel,
                    ..
                } => {
                    moved = true;
                    if mousestate.left() {
                        drawer.viewport.x += xrel;
                        drawer.viewport.y += yrel;
                    } else {
                        let n_x = (x - drawer.viewport.x) / drawer.tile_size as i32;
                        let n_y = (y - drawer.viewport.y) / drawer.tile_size as i32;

                        drawer.focused_tile = if board.width as i32 > n_x
                            && n_x >= 0
                            && board.height as i32 > n_y
                            && n_y >= 0
                        {
                            Some(n_y as usize * board.width + n_x as usize)
                        } else {
                            None
                        };
                    }
                }
                Event::MouseButtonDown { .. } => moved = false,

                Event::MouseButtonUp { mouse_btn, .. } => {
                    if !moved {
                        match mouse_btn {
                            MouseButton::Left => {
                                if let Some(index) = drawer.focused_tile {
                                    board.action(index, core::Action::Dig)?;
                                }
                            }
                            MouseButton::Right => {
                                if let Some(index) = drawer.focused_tile {
                                    board.action(index, core::Action::Flag)?;
                                }
                            }
                            _ => {}
                        }
                    }
                }

                Event::MouseWheel { y, .. } => {
                    let center_x = (drawer.viewport.x - (drawer.viewport.width() / 2) as i32)
                        as f64
                        / drawer.tile_size as f64;
                    let center_y = (drawer.viewport.y - (drawer.viewport.height() / 2) as i32)
                        as f64
                        / drawer.tile_size as f64;
                    let tile_size = (drawer.tile_size as i32
                        + 1.max(drawer.tile_size / 10) as i32 * y)
                        .max(8)
                        .min(256) as u32;

                    drawer.viewport.x =
                        (center_x * tile_size as f64) as i32 + (drawer.viewport.width() / 2) as i32;
                    drawer.viewport.y = (center_y * tile_size as f64) as i32
                        + (drawer.viewport.height() / 2) as i32;

                    drawer.tile_size = tile_size;
                }
                _ => {}
            }
        }

        board.tick();

        canvas.clear();
        drawer.paint(&mut canvas, &board);

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
