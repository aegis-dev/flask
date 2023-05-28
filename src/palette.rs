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

use num_enum::TryFromPrimitive;

use crate::color::Color;

#[derive(Copy, Clone, Eq, PartialEq, TryFromPrimitive, Hash)]
#[repr(u8)]
pub enum FlaskColor {
    None = 0,
    Purple,
    Green,
    Brown,
    Red,
    Salad,
    Teal,
    Yellow,
    White
}

impl FlaskColor {
    #[inline(always)]
    pub fn count() -> u8 {
        FlaskColor::White as u8
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum FlaskPalette {
    Default
}

pub(crate) fn get_main_colors(palette: FlaskPalette) -> Vec<Color> {
    match palette {
        FlaskPalette::Default => {
            // https://lospec.com/palette-list/retrocal-8
            // Sorter variant https://coolors.co/2f142f-2a584f-774448-c6505a-74a33f-6eb8a8-ee9c5d-fcffc0
            let mut main_colors = vec![];
            main_colors.push(Color::from_hex(0x2f142f));
            main_colors.push(Color::from_hex(0x2a584f));
            main_colors.push(Color::from_hex(0x774448));
            main_colors.push(Color::from_hex(0xc6505a));
            main_colors.push(Color::from_hex(0x74a33f));
            main_colors.push(Color::from_hex(0x6eb8a8));
            main_colors.push(Color::from_hex(0xee9c5d));
            main_colors.push(Color::from_hex(0xfcffc0));
            main_colors
        },
    }
}


pub(crate) fn load_palette(palette: FlaskPalette) -> Vec<Color> {
    let mut colors = vec![];
    
    let main_colors = get_main_colors(palette);
    
    // Normal colors
    for color in &main_colors {
        colors.push(color.clone());
    }
    
    // Dim colors
    for color in &main_colors {
        let dim_color = Color::new(
            (color.r as f32 * 0.7) as u8,
            (color.g as f32 * 0.7) as u8,
            (color.b as f32 * 0.7) as u8
        );
        
        colors.push(dim_color);
    }
    
    // Dark colors
    for color in &main_colors {
        let dim_color = Color::new(
            (color.r as f32 * 0.2) as u8,
            (color.g as f32 * 0.2) as u8,
            (color.b as f32 * 0.2) as u8
        );

        colors.push(dim_color);
    }
    
    // Black colors
    for color in &main_colors {
        let dim_color = Color::new(
            (color.r as f32 * 0.1) as u8,
            (color.g as f32 * 0.1) as u8,
            (color.b as f32 * 0.1) as u8
        );

        colors.push(dim_color);
    }

    colors
}
