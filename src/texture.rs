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

use web_sys::{WebGl2RenderingContext, WebGlTexture};

use crate::js_utils::js_uint_8_array;

pub struct Texture {
    gl_context: WebGl2RenderingContext,
    texture: WebGlTexture,
    texture_id: u32,
    width: u32,
    height: u32,
}

#[derive(Clone, Copy)]
pub enum ImageMode {
    RED = WebGl2RenderingContext::RED as isize,
    RGB = WebGl2RenderingContext::RGB as isize,
    RGBA = WebGl2RenderingContext::RGBA as isize
}

impl Texture {
    pub fn from_data(gl_context: WebGl2RenderingContext, data: &Vec<u8>, width: u32, height: u32, mode: ImageMode, texture_id: u32) -> Texture {
        let texture = gl_context.create_texture().unwrap();

        gl_context.active_texture(texture_id);
        gl_context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
        gl_context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::REPEAT as i32);
        gl_context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::REPEAT as i32);
        gl_context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
        gl_context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);

        let internal_format = match mode {
            ImageMode::RED => WebGl2RenderingContext::R8,
            ImageMode::RGB => WebGl2RenderingContext::RGB,
            ImageMode::RGBA => WebGl2RenderingContext::RGBA,
        };

        let data_js_array = js_uint_8_array(data.as_slice());
        gl_context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                internal_format as i32,
                width as i32,
                height as i32,
                0,
                mode as u32,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                &data_js_array,
                0
        ).unwrap();

        gl_context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        Texture { gl_context, texture, texture_id, width, height }
    }

    pub fn texture_id(&self) -> u32 {
        self.texture_id
    }

    pub fn texture(&self) -> &WebGlTexture {
        &self.texture
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn update_texture_data(&self, data: &Vec<u8>, mode: ImageMode) {
        let internal_format = match mode {
            ImageMode::RED => WebGl2RenderingContext::R8,
            ImageMode::RGB => WebGl2RenderingContext::RGB,
            ImageMode::RGBA => WebGl2RenderingContext::RGBA,
        };

        self.gl_context.active_texture(self.texture_id);
        self.gl_context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&self.texture));
        let data_js_array = js_uint_8_array(data.as_slice());
        self.gl_context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                internal_format as i32,
                self.width as i32,
                self.height as i32,
                0,
                mode as u32,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                &data_js_array,
                0
         ).unwrap();

        self.gl_context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.gl_context.delete_texture(Some(&self.texture))
    }
}