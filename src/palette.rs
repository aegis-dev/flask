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

use std::fs::File;
use std::io::Read;

use png::{ColorType, Transformations};
use num_derive::FromPrimitive;

use crate::color::Color;
use crate::byte_buffer_reader::ByteBufferReader;


#[derive(Copy, Clone, Eq, PartialEq, FromPrimitive, Hash)]
#[repr(u8)]
pub enum FlaskColor {
    Black = 1,
    Blue,
    Red,
    Yellow,
    Olive,
    Mauve,
    Gray,
    White
}

pub fn flask_default() -> Vec<Color> {
    let mut palette = vec![];

    // https://coolors.co/000000-3c91e6-d56062-f5b700-60992d-dfb2f4-dbe4ee-ffffff
    palette.push(Color::from_hex(0x000000));
    palette.push(Color::from_hex(0x3C91E6));
    palette.push(Color::from_hex(0xD56062));
    palette.push(Color::from_hex(0xF5B700));
    palette.push(Color::from_hex(0x60992D));
    palette.push(Color::from_hex(0xDFB2F4));
    palette.push(Color::from_hex(0xDBE4EE));
    palette.push(Color::from_hex(0xFFFFFF));

    palette
}

pub fn from_png(png_file_path: &str) -> Result<Vec<Color>, String> {
    let mut file = match File::open(png_file_path) {
        Ok(file) => { file }
        Err(_) => {
            return Err(String::from(format!("Failed to open '{}' file", png_file_path)))
        }
    };

    let mut png_bytes = Vec::new();
    match file.read_to_end(&mut png_bytes) {
        Ok(_) => {}
        Err(_) => {
            return Err(String::from(format!("Failed to read whole '{}' file", png_file_path)))
        }
    };
    from_png_bytes(&png_bytes)
}

pub fn from_png_bytes(png_bytes: &[u8]) -> Result<Vec<Color>, String> {
    let mut decoder = png::Decoder::new(ByteBufferReader::from(png_bytes));
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

    let width = info.width as usize;
    let height = info.height as usize;
    let color_size = image_data.len() / (width * height);

    if color_size != 3 && color_size != 4 {
        return Err(
            String::from(format!("Unexpected image size or it is corrupted.\nwidth: {0} \
            , height: {1}, byte_count: {2}", width, height, image_data.len()))
        );
    }

    let mut palette = vec![];

    let mut byte_idx = 0;
    for _color_idx in 0..(width * height) {
        palette.push(
            Color::new(
                *image_data.get(byte_idx).unwrap(),
                *image_data.get(byte_idx + 1).unwrap(),
                *image_data.get(byte_idx + 2).unwrap()
            )
        );
        byte_idx += color_size;
    }

    Ok(palette)
}