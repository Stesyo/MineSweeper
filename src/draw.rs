use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;
use std::ops::Range;
use std::{u32, usize};

use crate::core::{Board, Tile};

pub struct Drawer<'a> {
    rects: Vec<Rect>,
    texture: Texture<'a>,
    pub viewport: Rect,
    pub tile_size: u32,
    pub focused_tile: Option<usize>,
}

impl<'a> Drawer<'a> {
    pub fn new(texture_creator: &TextureCreator<WindowContext>) -> Drawer {
        let texture = texture_creator.load_texture("texture.png").unwrap();
        let mut rects: Vec<Rect> = Vec::new();
        for i in 0..16 {
            rects.push(Rect::new(i % 4 * 16, i / 4 * 16, 16, 16));
        }

        let viewport = Rect::new(0, 0, 0, 0);

        Drawer {
            texture,
            rects,
            viewport,
            tile_size: 0,
            focused_tile: None,
        }
    }

    pub fn adjust(&mut self, window: (u32, u32), board: &Board) {
        self.viewport.resize(window.0, window.1);
        self.tile_size = (window.0 / board.width as u32)
            .min(window.1 / board.height as u32)
            .max(1);
        let mut content = Rect::new(
            0,
            0,
            self.tile_size * board.width as u32,
            self.tile_size * board.height as u32,
        );
        content.center_on(self.viewport.center());
        self.viewport.reposition(content.top_left());
    }

    fn get_visible(&self, board: &Board) -> (Range<usize>, Range<usize>) {
        let width_s = (-self.viewport.x / self.tile_size as i32).max(0) as usize;
        let width_e =
            (((-self.viewport.x + (self.viewport.width() as i32)) / self.tile_size as i32) + 1)
                .min(board.width as i32)
                .max(0) as usize;
        let height_s = (-self.viewport.y / self.tile_size as i32).max(0) as usize;
        let height_e =
            (((-self.viewport.y + (self.viewport.height() as i32)) / self.tile_size as i32) + 1)
                .min(board.height as i32)
                .max(0) as usize;
        (width_s..width_e, height_s..height_e)
    }

    pub fn paint(&self, window: &mut WindowCanvas, board: &Board) {
        let mut dst = Rect::new(0, 0, self.tile_size, self.tile_size);
        let (width, height) = self.get_visible(board);
        for y in height {
            for x in width.clone() {
                let src: Rect = match (board.alive, &board.tiles[y][x]) {
                    (
                        false,
                        Tile {
                            flagged: true,
                            mine: true,
                            ..
                        },
                    ) => self.rects[10],
                    (
                        false,
                        Tile {
                            digged: true,
                            mine: true,
                            ..
                        },
                    ) => self.rects[12],
                    (false, Tile { mine: true, .. }) => self.rects[11],
                    (.., Tile { flagged: true, .. }) => self.rects[9],
                    (.., Tile { digged: false, .. }) => self.rects[8],
                    (.., Tile { count: 0, .. }) => self.rects[15],
                    (.., Tile { count: d, .. }) => self.rects[(d - 1) as usize],
                };
                dst.set_x(x as i32 * self.tile_size as i32 + self.viewport.x);
                dst.set_y(y as i32 * self.tile_size as i32 + self.viewport.y);
                let _ = window.copy(&self.texture, src, dst);
            }
        }
        if let Some(tile) = self.focused_tile {
            let tile_x = (tile % board.width) as i32;
            let tile_y = (tile / board.width) as i32;
            let focused = Rect::new(
                self.viewport.x + tile_x * self.tile_size as i32,
                self.viewport.y + tile_y * self.tile_size as i32,
                self.tile_size,
                self.tile_size,
            );
            window.set_draw_color(Color::RGB(255, 255, 0));
            let _ = window.draw_rect(focused);
        }
        window.set_draw_color(Color::RGB(0, 0, 0));
    }
}
