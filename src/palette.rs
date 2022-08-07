//
// Copyright © 2020-2022  Egidijus Lileika
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

use num_derive::FromPrimitive;

use crate::color::Color;
use crate::byte_buffer_reader::ByteBufferReader;


#[derive(Copy, Clone, Eq, PartialEq, FromPrimitive, Hash)]
#[repr(u8)]
pub enum FlaskColor {
    Purple = 1,
    Green,
    Brown,
    Red,
    Salad,
    Teal,
    Yellow,
    White
}

// https://lospec.com/palette-list/retrocal-8
// Sorter variant https://coolors.co/2f142f-2a584f-774448-c6505a-74a33f-6eb8a8-ee9c5d-fcffc0
pub fn flask_default() -> Vec<Color> {
    let mut palette = vec![];

    palette.push(Color::from_hex(0x2f142f));
    palette.push(Color::from_hex(0x2a584f));
    palette.push(Color::from_hex(0x774448));
    palette.push(Color::from_hex(0xc6505a));
    palette.push(Color::from_hex(0x74a33f));
    palette.push(Color::from_hex(0x6eb8a8));
    palette.push(Color::from_hex(0xee9c5d));
    palette.push(Color::from_hex(0xfcffc0));

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
    let decoder = png::Decoder::new(ByteBufferReader::from(png_bytes));
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