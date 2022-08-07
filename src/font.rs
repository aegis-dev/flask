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

use std::collections::HashMap;

use crate::sprite::Sprite;

pub struct Font {
    null_glyph: Sprite,
    glyphs: HashMap<u8, Sprite>
}

impl Font {
    pub fn load_3x5() -> Result<Font, String> {
        let mut glyphs = HashMap::new();

        glyphs.insert('a' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_1.png"))?);
        glyphs.insert('b' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_2.png"))?);
        glyphs.insert('c' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_3.png"))?);
        glyphs.insert('d' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_4.png"))?);
        glyphs.insert('e' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_5.png"))?);
        glyphs.insert('f' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_6.png"))?);
        glyphs.insert('g' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_7.png"))?);
        glyphs.insert('h' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_8.png"))?);
        glyphs.insert('i' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_9.png"))?);
        glyphs.insert('j' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_10.png"))?);
        glyphs.insert('k' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_11.png"))?);
        glyphs.insert('l' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_12.png"))?);
        glyphs.insert('m' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_13.png"))?);
        glyphs.insert('n' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_14.png"))?);
        glyphs.insert('o' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_15.png"))?);
        glyphs.insert('p' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_16.png"))?);
        glyphs.insert('q' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_17.png"))?);
        glyphs.insert('r' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_18.png"))?);
        glyphs.insert('s' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_19.png"))?);
        glyphs.insert('t' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_20.png"))?);
        glyphs.insert('u' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_21.png"))?);
        glyphs.insert('v' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_22.png"))?);
        glyphs.insert('w' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_23.png"))?);
        glyphs.insert('x' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_24.png"))?);
        glyphs.insert('y' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_25.png"))?);
        glyphs.insert('z' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_26.png"))?);

        glyphs.insert('A' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_27.png"))?);
        glyphs.insert('B' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_28.png"))?);
        glyphs.insert('C' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_29.png"))?);
        glyphs.insert('D' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_30.png"))?);
        glyphs.insert('E' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_31.png"))?);
        glyphs.insert('F' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_32.png"))?);
        glyphs.insert('G' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_33.png"))?);
        glyphs.insert('H' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_34.png"))?);
        glyphs.insert('I' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_35.png"))?);
        glyphs.insert('J' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_36.png"))?);
        glyphs.insert('K' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_37.png"))?);
        glyphs.insert('L' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_38.png"))?);
        glyphs.insert('M' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_39.png"))?);
        glyphs.insert('N' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_40.png"))?);
        glyphs.insert('O' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_41.png"))?);
        glyphs.insert('P' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_42.png"))?);
        glyphs.insert('Q' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_43.png"))?);
        glyphs.insert('R' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_44.png"))?);
        glyphs.insert('S' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_45.png"))?);
        glyphs.insert('T' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_46.png"))?);
        glyphs.insert('U' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_47.png"))?);
        glyphs.insert('V' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_48.png"))?);
        glyphs.insert('W' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_49.png"))?);
        glyphs.insert('X' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_50.png"))?);
        glyphs.insert('Y' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_51.png"))?);
        glyphs.insert('Z' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_52.png"))?);

        glyphs.insert('0' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_53.png"))?);
        glyphs.insert('1' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_54.png"))?);
        glyphs.insert('2' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_55.png"))?);
        glyphs.insert('3' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_56.png"))?);
        glyphs.insert('4' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_57.png"))?);
        glyphs.insert('5' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_58.png"))?);
        glyphs.insert('6' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_59.png"))?);
        glyphs.insert('7' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_60.png"))?);
        glyphs.insert('8' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_61.png"))?);
        glyphs.insert('9' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_62.png"))?);

        glyphs.insert('.' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_63.png"))?);
        glyphs.insert(',' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_64.png"))?);
        glyphs.insert('!' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_65.png"))?);
        glyphs.insert('?' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_66.png"))?);
        glyphs.insert(':' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_67.png"))?);
        glyphs.insert(';' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_68.png"))?);
        glyphs.insert('<' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_69.png"))?);
        glyphs.insert('>' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_70.png"))?);
        glyphs.insert('=' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_71.png"))?);
        glyphs.insert('(' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_72.png"))?);
        glyphs.insert(')' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_73.png"))?);
        glyphs.insert('\'' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_74.png"))?);
        glyphs.insert('%' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_75.png"))?);
        glyphs.insert('$' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_76.png"))?);
        glyphs.insert('&' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_77.png"))?);
        glyphs.insert('#' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_78.png"))?);
        glyphs.insert('"' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_79.png"))?);
        glyphs.insert('-' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_80.png"))?);
        glyphs.insert('+' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_81.png"))?);
        glyphs.insert('_' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_82.png"))?);
        glyphs.insert('{' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_83.png"))?);
        glyphs.insert('}' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_84.png"))?);
        glyphs.insert('*' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_85.png"))?);
        glyphs.insert('/' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_86.png"))?);
        glyphs.insert('`' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_87.png"))?);
        glyphs.insert('^' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_88.png"))?);
        glyphs.insert('|' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_89.png"))?);
        glyphs.insert('~' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_90.png"))?);
        glyphs.insert('[' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_91.png"))?);
        glyphs.insert(']' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_92.png"))?);
        glyphs.insert('@' as u8, Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_94.png"))?);

        let null_glyph = Sprite::from_indexed_8bit_png(include_bytes!("assets/font/3x5/font_93.png"))?;

        Ok(Font { null_glyph, glyphs })
    }

    pub fn get_glyph(&self, char: &u8) -> &Sprite {
        match self.glyphs.get(char) {
            Some(glyph) => glyph,
            None => &self.null_glyph
        }
    }

    pub fn get_glyph_width(&self) -> u32 {
        self.null_glyph.get_width()
    }

    pub fn get_glyph_height(&self) -> u32 {
        self.null_glyph.get_height()
    }
}