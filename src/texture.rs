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

use std::ffi::c_void;

use gl;

pub struct Texture {
    texture_id: gl::types::GLuint,
    width: gl::types::GLuint,
    height: gl::types::GLuint,
}

#[derive(Clone, Copy)]
pub enum ImageMode {
    RED = gl::RED as isize,
    RGB = gl::RGB as isize,
    RGBA = gl::RGBA as isize
}

impl Texture {
    pub fn from_data(data: &Vec<u8>, width: gl::types::GLuint, height: gl::types::GLuint, mode: ImageMode) -> Texture {
        let texture_id: gl::types::GLuint = {
            let mut texture_ids = vec![0];
            unsafe {
                gl::GenTextures(texture_ids.len() as gl::types::GLsizei, texture_ids.as_mut_ptr());
            }
            texture_ids[0]
        };

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as gl::types::GLint);

            let internal_format = match mode {
                //ImageMode::RED => { 1 }
                _ => { mode as gl::types::GLint }
            };

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal_format,
                width as gl::types::GLint,
                height as gl::types::GLint,
                0,
                mode as gl::types::GLuint,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void
            );
        }

        Texture { texture_id, width, height }
    }

    pub fn texture_id(&self) -> gl::types::GLuint {
        self.texture_id
    }

    pub fn width(&self) -> gl::types::GLuint {
        self.width
    }

    pub fn height(&self) -> gl::types::GLuint {
        self.height
    }

    pub fn update_texture_data(&mut self, data: &Vec<u8>, width: gl::types::GLuint, height: gl::types::GLuint, mode: ImageMode) {
        self.width = width;
        self.height = height;

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);

            let internal_format = match mode {
                //ImageMode::ColorIndex => { 1 }
                _ => { mode as gl::types::GLint }
            };

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal_format,
                width as gl::types::GLint,
                height as gl::types::GLint,
                0,
                mode as gl::types::GLuint,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void
            );
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            let temp_vec = vec![self.texture_id];
            gl::DeleteTextures(temp_vec.len() as gl::types::GLsizei, temp_vec.as_ptr());
        }
    }
}