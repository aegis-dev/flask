//
// Copyright © 2020-2021  Egidijus Lileika
//
// This file is part of Flask - Framework for 2D game development
//
// Flask is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Flask is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Flask. If not, see <https://www.gnu.org/licenses/>.
//

use crate::frame_buffer::FrameBuffer;
use crate::color::Color;
use crate::sprite::Sprite;
use crate::font::Font;
use crate::texture::{Texture, ImageMode};

pub struct Renderer {
    frame_buffer: FrameBuffer,
    palette_texture: Texture,
    palette_color_count: u8,
    camera_x: i64,
    camera_y: i64,
    camera_origin_x: i64,
    camera_origin_y: i64,
    background_color_index: u8
}

impl Renderer {
    pub fn new(frame_buffer: FrameBuffer, palette: Vec<Color>) -> Result<Renderer, String> {
        if palette.len() == 0 {
            return Err(String::from("Palette doesn't contain any colors"));
        }
        if palette.len() % 4 != 0 {
            return Err(String::from("Palette must be 4 color aligned"));
        }

        let camera_origin_x = frame_buffer.get_width() as i64 / 2;
        let camera_origin_y = frame_buffer.get_height() as i64 / 2;

        let mut palette_texture_data: Vec<u8> = vec![0; 0];
        palette_texture_data.reserve_exact((palette.len() * 3) as usize);
        for color in &palette {
            palette_texture_data.push(color.r);
            palette_texture_data.push(color.g);
            palette_texture_data.push(color.b);
        }

        let palette_texture = Texture::from_data(&palette_texture_data, palette.len() as u32, 1, ImageMode::RGB);

        Ok(Renderer {
            frame_buffer,
            palette_texture,
            palette_color_count: palette.len() as u8,
            camera_x: 0,
            camera_y: 0,
            camera_origin_x,
            camera_origin_y,
            background_color_index: 1
        })
    }

    pub fn swap(&mut self) -> Result<(), String> {
        self.frame_buffer.swap_buffers();
        self.frame_buffer.clear_screen();
        self.frame_buffer.update_frame_texture()
    }

    pub fn get_frame_buffer(&self) -> &FrameBuffer {
        &self.frame_buffer
    }

    pub fn get_palette_texture(&self) -> &Texture {
        &self.palette_texture
    }

    pub fn get_camera_x(&mut self) -> i64 {
        self.camera_x
    }

    pub fn get_camera_y(&mut self) -> i64 {
        self.camera_y
    }

    pub fn set_camera_x(&mut self, x: i64) {
        self.camera_x = x;
    }

    pub fn set_camera_y(&mut self, y: i64) {
        self.camera_y = y;
    }

    pub fn set_background_color(&mut self, background_color_index: u8) -> Result<(), String> {
        match background_color_index {
            0 => Err(String::from("Background color index can't be 0")),
            index => {
                if index > self.palette_color_count {
                    return Err(String::from(format!("Background color index {} out of bounds", index)));
                }
                self.background_color_index = index;
                Ok(())
            }
        }
    }

    pub fn get_background_color(&self) -> u8 {
        self.background_color_index as u8
    }

    pub fn get_palette_color_count(&self) -> u8 {
        self.palette_color_count as u8
    }

    pub fn point(&mut self, x: i64, y: i64, color_idx: u8) {
        let x_real = x - self.camera_x as i64 + self.camera_origin_x;
        let y_real = y - self.camera_y as i64 + self.camera_origin_y;
        if x_real < 0 || y_real < 0 {
            return;
        }

        self.frame_buffer.set_pixel(x_real as u32, y_real as u32, color_idx);
    }

    pub fn circle(&mut self, origin_x: i64, origin_y: i64, radius: u32, color_idx: u8) {
        let mut x: i64 = 0;
        let mut y: i64 = radius as i64;
        let mut p: i64 = ((5 - radius as i64 * 4) / 4) as i64;

        self.circle_points(origin_x, origin_y, x, y, color_idx);
        while x < y {
            x += 1;
            if p < 0 {
                p += 2 * x + 1;
            } else {
                y -= 1;
                p += 2 * (x - y) + 1;
            }
            self.circle_points(origin_x, origin_y, x, y, color_idx);
        }
    }

    pub fn circle_filled(&mut self, origin_x: i64, origin_y: i64, radius: u32, color_idx: u8) {
        let mut x: i64 = 0;
        let mut y: i64 = radius as i64;
        let mut p: i64 = ((5 - radius * 4) / 4) as i64;

        self.circle_points_fill(origin_x, origin_y, x, y, color_idx);
        while x < y {
            x += 1;
            if p < 0 {
                p += 2 * x + 1;
            } else {
                y -= 1;
                p += 2 * (x - y) + 1;
            }
            self.circle_points_fill(origin_x, origin_y, x, y, color_idx);
        }
    }

    fn circle_points(&mut self, cx: i64, cy: i64, x: i64, y: i64, color_idx: u8) {
        if x == 0 {
            self.point(cx, cy + y, color_idx);
            self.point(cx, cy - y, color_idx);
            self.point(cx + y, cy, color_idx);
            self.point(cx - y, cy, color_idx);
        } else if x == y {
            self.point(cx + x, cy + y, color_idx);
            self.point(cx - x, cy + y, color_idx);
            self.point(cx + x, cy - y, color_idx);
            self.point(cx - x, cy - y, color_idx);
        } else if x < y {
            self.point(cx + x, cy + y, color_idx);
            self.point(cx - x, cy + y, color_idx);
            self.point(cx + x, cy - y, color_idx);
            self.point(cx - x, cy - y, color_idx);

            self.point(cx + y, cy + x, color_idx);
            self.point(cx - y, cy + x, color_idx);
            self.point(cx + y, cy - x, color_idx);
            self.point(cx - y, cy - x, color_idx);
        }
    }

    fn circle_points_fill(&mut self, cx: i64, cy: i64, x: i64, y: i64, color_idx: u8) {
        if x == 0 {
            for i in cy - y..=cy + y {
                self.point(cx, i, color_idx);
            }
            for i in cx - y..=cx + y {
                self.point(i, cy, color_idx);
            }
        } else if x == y {
            for a in cx - x..=cx + x {
                for b in cy - y..=cy + y {
                    self.point(a, b, color_idx);
                }
            }
        } else if x < y {
            for a in cx - x..=cx + x {
                for b in cy - y..=cy + y {
                    self.point(a, b, color_idx);
                }
            }
            for a in cx - y..=cx + y {
                for b in cy - x..=cy + x {
                    self.point(a, b, color_idx);
                }
            }
        }
    }

    pub fn line(&mut self, x1: i64, y1: i64, x2: i64, y2: i64, color_idx: u8) {
        let mut x = 0;
        let mut y = 0;
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dx1 = dx.abs();
        let dy1 = dy.abs();
        let mut px = 2 * dy1 - dx1;
        let mut py = 2 * dx1 - dy1;
        if dy1 <= dx1 {
            let mut xe = 0;
            if dx >= 0 {
                x = x1;
                y = y1;
                xe = x2;
            } else {
                x = x2;
                y = y2;
                xe = x1;
            }
            self.point(x, y, color_idx);

            while x < xe {
                x += 1;
                if px < 0 {
                    px = px + 2 * dy1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        y = y + 1;
                    } else {
                        y = y - 1;
                    }
                    px = px + 2 * (dy1 - dx1);
                }
                self.point(x, y, color_idx);
            }
        } else {
            let mut ye = 0;
            if dy >= 0 {
                x = x1;
                y = y1;
                ye = y2;
            } else {
                x = x2;
                y = y2;
                ye = y1;
            }
            self.point(x, y, color_idx);
            while y < ye {
                y += 1;
                if py <= 0 {
                    py = py + 2 * dx1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        x = x + 1;
                    } else {
                        x = x - 1;
                    }
                    py = py + 2 * (dx1 - dy1);
                }
                self.point(x, y, color_idx);
            }
        }
    }

    pub fn rectangle(&mut self, x1: i64, y1: i64, x2: i64, y2: i64, color_idx: u8) {
        for x in x1..=x2 {
            self.point(x, y1, color_idx);
            self.point(x, y2, color_idx);
        }

        for y in y1..=y2 {
            self.point(x1, y, color_idx);
            self.point(x2, y, color_idx);
        }
    }

    pub fn rectangle_filled(&mut self, x1: i64, y1: i64, x2: i64, y2: i64, color_idx: u8) {
        for x in x1..=x2 {
            for y in y1..=y2 {
                self.point(x, y, color_idx);
            }
        }
    }

    pub fn sprite(&mut self, sprite: &Sprite, x: i64, y: i64, flip: bool) {
        let sprite_width = sprite.get_width();
        let sprite_height = sprite.get_height();
        for sprite_x in 0..sprite_width {
            for sprite_y in 0..sprite_height {
                let color_idx = sprite.get_color_index_at(sprite_x, sprite_y);
                if color_idx == 0 {
                    continue;
                }
                if flip {
                    self.point(x + (sprite_width - sprite_x) as i64, y + sprite_y as i64, color_idx);
                } else {
                    self.point(x + sprite_x as i64, y + sprite_y as i64, color_idx);
                }
            }
        }
    }

    pub fn text(&mut self, text: &String, font: &Font, x: i64, y: i64, color_idx: u8) {
        let mut x_offset = x;
        let mut y_offset = y;
        let space_width = font.get_glyph_width() as i64;
        let space_height = font.get_glyph_height() as i64;
        for ch in text.as_bytes() {
            if *ch == ' ' as u8 {
                x_offset += space_width + 1;
                continue;
            } else if *ch == '\n' as u8 {
                x_offset = x;
                y_offset -= space_height + 1;
                continue;
            }

            let glyph = font.get_glyph(ch);

            let glyph_width = glyph.get_width();
            let glyph_height = glyph.get_height();
            for glyph_x in 0..glyph_width {
                for glyph_y in 0..glyph_height {
                    if glyph.get_color_index_at(glyph_x, glyph_y) == 0 {
                        continue;
                    }

                    self.point(x_offset + glyph_x as i64, y_offset + glyph_y as i64, color_idx);
                }
            }

            x_offset += glyph_width as i64 + 1;
        }
    }
}