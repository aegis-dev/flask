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

use web_sys::WebGl2RenderingContext;

use crate::mesh::Mesh;
use crate::texture::{Texture, ImageMode};

pub struct FrameBuffer {
    buffer_width: u32,
    buffer_height: u32,
    frame_buffer_quad: Mesh,
    frame_buffer_texture: Texture,
    frame_buffer: Vec<u8>,
}

impl FrameBuffer {
    pub fn new(gl_context: WebGl2RenderingContext, buffer_width: u32, buffer_height: u32) -> FrameBuffer {
        let vertices = vec![
            -1.0, -1.0, 0.0, // bot left
            -1.0,  1.0, 0.0, // top left
            1.0,  1.0, 0.0,  // top right
            1.0, -1.0, 0.0,  // bot right
        ];
        let texture_coords = vec![
            0.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            1.0, 0.0,
        ];
        let indices = vec![
            0, 1, 2,
            3, 0, 2
        ];

        let frame_buffer_quad = Mesh::from_data(gl_context.clone(), &vertices, &texture_coords, &indices);
        let frame_buffer: Vec<u8> = vec![0; (buffer_width * buffer_height) as usize];
        let frame_buffer_texture = Texture::from_data(gl_context, &frame_buffer, buffer_width, buffer_height, ImageMode::RED);

        FrameBuffer {
            buffer_width,
            buffer_height,
            frame_buffer_quad,
            frame_buffer_texture,
            frame_buffer
        }
    }

    pub fn get_width(&self) -> u32 {
        self.buffer_width
    }

    pub fn get_height(&self) -> u32 {
        self.buffer_height
    }

    pub fn render_to_texture(&self) -> Result<&Texture, String> {
        self.frame_buffer_texture.update_texture_data(&self.frame_buffer, ImageMode::RED);

        Ok(&self.frame_buffer_texture)
    }

    pub fn get_quad(&self) -> &Mesh {
        &self.frame_buffer_quad
    }

    pub fn get_texture(&self) -> &Texture {
        &self.frame_buffer_texture
    }

    pub fn clear_screen(&mut self) {
        // It's much faster to allocate new buffer than iterating it and setting color
        self.frame_buffer = vec![0; (self.buffer_width * self.buffer_height) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color_idx: u8) {
        if x >= self.buffer_width || y >= self.buffer_height {
            return;
        }

        let pixel_offset = y * self.buffer_width + x;
        // I am assuring that it is safe just to unwrap since buffer size never changes
        *self.frame_buffer.get_mut(pixel_offset as usize).unwrap() = color_idx;
    }
}