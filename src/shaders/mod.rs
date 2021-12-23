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

use gl;
use std;
use std::ffi::{CStr, CString};

pub struct ShaderProgram {
    id: gl::types::GLuint,
}

impl ShaderProgram {
    pub fn load_shaders(vert_shader_source: &CStr, frag_vert_source: &CStr) -> ShaderProgram {
        let vert_shader = match Shader::from_vert_source(vert_shader_source) {
            Ok(shader) => shader,
            Err(error) => panic!("Failed to compile vertex shader: {:?}", error)
        };
        let frag_shader = match Shader::from_frag_source(frag_vert_source) {
            Ok(shader) => shader,
            Err(error) => panic!("Failed to compile fragment shader: {:?}", error)
        };

        match ShaderProgram::from_shaders(&[vert_shader, frag_shader]) {
            Ok(shader_program) => shader_program,
            Err(error) => panic!("Failed to compile shader program: {:?}", error)
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn enable(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn set_uniform_int(&self, location: i32, value: i32) {
        unsafe {
            gl::Uniform1i(location, value);
        }
    }

    fn from_shaders(shaders: &[Shader]) -> Result<ShaderProgram, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = allocate_buffer_for_gl_message(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        unsafe {
            gl::ValidateProgram(program_id);
        }

        success = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::VALIDATE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = allocate_buffer_for_gl_message(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }


        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(ShaderProgram { id: program_id })
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = Shader::shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = allocate_buffer_for_gl_message(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        Ok(id)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn allocate_buffer_for_gl_message(len: usize) -> CString {
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}
