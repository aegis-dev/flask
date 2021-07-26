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

use crate::mesh::Mesh;
use crate::texture::{Texture, ImageMode};

pub struct FrameBuffer {
    buffer_width: u32,
    buffer_height: u32,
    frame_buffer_quad: Mesh,
    frame_buffer_texture: Texture,
    current_frame_buffer: u8,
    frame_buffers: [Vec<u8>; 2],
}

impl FrameBuffer {
    pub fn new(buffer_width: u32, buffer_height: u32) -> FrameBuffer {
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

        let frame_buffer_quad = Mesh::from_data(&vertices, &texture_coords, &indices);
        let front_frame_buffer: Vec<u8> = vec![0; (buffer_width * buffer_height) as usize];
        let back_frame_buffer: Vec<u8> = vec![0; (buffer_width * buffer_height) as usize];
        let frame_buffer_texture = Texture::from_data(&front_frame_buffer, buffer_width, buffer_height, ImageMode::RED);

        FrameBuffer {
            buffer_width,
            buffer_height,
            frame_buffer_quad,
            frame_buffer_texture,
            current_frame_buffer: 0,
            frame_buffers: [ front_frame_buffer, back_frame_buffer ]
        }
    }

    pub fn get_width(&self) -> u32 {
        self.buffer_width
    }

    pub fn get_height(&self) -> u32 {
        self.buffer_height
    }

    pub fn update_frame_texture(&mut self) -> Result<(), String> {
        let back_buffer_idx = self.current_frame_buffer ^ 1;
        let back_buffer = &self.frame_buffers[back_buffer_idx as usize];

        self.frame_buffer_texture.update_texture_data(&back_buffer, self.buffer_width, self.buffer_height, ImageMode::RED);

        Ok(())
    }

    pub fn get_quad(&self) -> &Mesh {
        &self.frame_buffer_quad
    }

    pub fn get_texture(&self) -> &Texture {
        &self.frame_buffer_texture
    }

    pub fn swap_buffers(&mut self) {
        self.current_frame_buffer ^= 1;
    }

    pub fn clear_screen(&mut self) {
        // It's much faster to allocate new buffer than iterating it and setting color
        let buffer = &mut self.frame_buffers[self.current_frame_buffer as usize];
        *buffer = vec![0; (self.buffer_width * self.buffer_height) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color_idx: u8) {
        if x >= self.buffer_width || y >= self.buffer_height {
            return;
        }

        let pixel_offset = y * self.buffer_width + x;
        // I am assuring that it is safe just to unwrap since buffer size never changes
        *self.frame_buffers[self.current_frame_buffer as usize].get_mut(pixel_offset as usize).unwrap() = color_idx;
    }
}