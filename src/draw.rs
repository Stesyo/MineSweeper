use sdl2::image::LoadTexture;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, CanvasBuilder, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use crate::core::Board;

pub struct Display {
    pub rects: Vec<Rect>,
}

impl Display {
    pub fn new(canvas: Canvas<Window>) -> Display {
        Display {}
    }
}

pub fn paint(canvas: &mut Canvas<Window>, board: &mut Board, texture: &Texture) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
}
