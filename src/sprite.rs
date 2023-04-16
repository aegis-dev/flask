//
// Copyright © 2020-2023  Egidijus Lileika
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

use std::collections::HashMap;
use png::{ColorType, Transformations};

use crate::byte_buffer_reader::ByteBufferReader;

pub struct Sprite {
    width: u32,
    height: u32,
    pixels: Vec<Vec<u8>>,
}

impl Sprite {
    pub fn from_indexed_8bit_png(png_bytes: &[u8]) -> Result<Sprite, String> {
        let mut decoder = png::Decoder::new(ByteBufferReader::from(png_bytes));
        decoder.set_transformations(Transformations::IDENTITY);
        let mut reader = match decoder.read_info() {
            Ok(reader) => reader.1,
            Err(error) => return Err(error.to_string())
        };


        let mut image_data = vec![0; reader.output_buffer_size()];
        match reader.next_frame(&mut image_data) {
            Err(error) => return Err(error.to_string()),
            _ => { }
        };

        let info = reader.info();

        match info.color_type {
            ColorType::Indexed => {
                let width = info.width as usize;
                let height = info.height as usize;

                if width * height != image_data.len() {
                    return Err(
                        String::from(format!("Unexpected image size or it is corrupted.\nwidth: {0}\
                        , height: {1}, byte_count: {2}", width, height, image_data.len()))
                    );
                }

                let mut pixels = vec![vec![0; height as usize]; width as usize];

                let mut pixel_idx = 0;
                for y in 0..height as usize {
                    for x in 0..width as usize {
                        *pixels.get_mut(x).unwrap().get_mut(height - 1 - y).unwrap() = *image_data.get(pixel_idx).unwrap();
                        pixel_idx += 1;
                    }
                }

                Ok(Sprite { width: width as u32, height: height as u32, pixels })
            }
            _ => {
                Err(String::from("Only indexed png format is supported"))
            }
        }
    }

    pub fn from_pixels(width: u32, height: u32, pixels: Vec<Vec<u8>>) -> Sprite {
        Sprite { width, height, pixels }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn get_color_index_at(&self, x: u32, y: u32) -> u8 {
        if x >= self.width || y >= self.height {
            return 0;
        }
        *self.pixels.get(x as usize).unwrap().get(y as usize).unwrap()
    }
}

#[derive(Clone, Copy, Eq, Hash)]
pub struct SpriteID(pub u32);

impl PartialEq for SpriteID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub struct SpriteBank {
    counter: u32,
    sprites_map: HashMap<SpriteID, Sprite>,
}

impl SpriteBank {
    pub fn new() -> SpriteBank {
        SpriteBank { counter: 0, sprites_map: HashMap::new() }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) -> SpriteID {
        let id = SpriteID(self.counter);
        self.sprites_map.insert(id, sprite);
        self.counter += 1;
        id
    }

    pub fn get_sprite(&self, sprite_id: &SpriteID) -> Option<&Sprite> {
        self.sprites_map.get(sprite_id)
    }
}

pub struct TileSet {
    tileset: Vec<Sprite>,
}

impl TileSet {
    pub fn from_indexed_8bit_png(png_bytes: &[u8], tile_width: u32, tile_height: u32) -> Result<TileSet, String> {
        let sprite = Sprite::from_indexed_8bit_png(png_bytes)?;

        if sprite.get_width() % tile_width != 0 {
            return Err(String::from(format!("Tileset width is not multiple of {}", tile_width)));
        }
        if sprite.get_height() % tile_height != 0 {
            return Err(String::from(format!("Tileset height is not multiple of {}", tile_height)));
        }

        let columns = sprite.get_width() / tile_width;
        let rows = sprite.get_height() / tile_height;

        let mut tileset = vec![];

        for row in (0..rows).rev() {
            for column in 0..columns {
                let tile_x_offset = column * tile_width;
                let tile_y_offset = row * tile_height;

                let mut tile_pixels = vec![vec![0; tile_height as usize]; tile_width as usize];

                for y in 0..tile_height as usize {
                    for x in 0..tile_width as usize {
                        *tile_pixels.get_mut(x).unwrap().get_mut(y).unwrap() = *sprite.pixels.get(tile_x_offset as usize + x).unwrap().get(tile_y_offset as usize + y).unwrap();
                    }
                }
                tileset.push(Sprite::from_pixels(tile_width, tile_height, tile_pixels))
            }
        }

        Ok(TileSet { tileset })
    }

    pub fn get_tile_at_index(&self, index: u32) -> Result<&Sprite, String> {
        if index as usize >= self.tileset.len() {
            return Err(String::from("Tile index out of bounds"));
        }

        return Ok(self.tileset.get(index as usize).unwrap())
    }
}