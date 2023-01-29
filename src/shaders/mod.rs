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

use web_sys::{WebGl2RenderingContext, WebGlShader, WebGlProgram, WebGlUniformLocation};


pub struct ShaderProgram {
    gl_context: WebGl2RenderingContext,
    program: WebGlProgram,
}

impl ShaderProgram {
    pub fn load_shaders(gl_context: WebGl2RenderingContext, vert_shader_source: &String, frag_vert_source: &String) -> Result<ShaderProgram, String> {
        let vert_shader = Shader::from_vert_source(&gl_context, vert_shader_source)?;
        let frag_shader = Shader::from_frag_source(&gl_context, frag_vert_source)?;
        let program = ShaderProgram::link_program(&gl_context, &vert_shader, &frag_shader)?;
        Ok(ShaderProgram { gl_context, program })
    }

    pub fn enable(&self) {
        self.gl_context.use_program(Some(&self.program));
    }

    pub fn disable(&self) {
        self.gl_context.use_program(None);
    }

    pub fn get_uniform_location(&self, uniform_name: &str) -> Option<WebGlUniformLocation> {
        self.gl_context.get_uniform_location(&self.program, uniform_name)
    }

    pub fn set_uniform_int(&self, location: &WebGlUniformLocation, value: i32) {
        self.gl_context.uniform1i(Some(location), value);
    }

    fn link_program(
            gl_context: &WebGl2RenderingContext,
            vert_shader: &Shader,
            frag_shader: &Shader,
    ) -> Result<WebGlProgram, String> {
        let program = gl_context.create_program().ok_or_else(|| String::from("Unable to create shader object"))?;

        gl_context.attach_shader(&program, vert_shader.get_shader());
        gl_context.attach_shader(&program, frag_shader.get_shader());
        gl_context.link_program(&program);

        if gl_context.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
            Ok(program)
        } else {
            Err(gl_context.get_program_info_log(&program).unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.gl_context.delete_program(Some(&self.program))
    }
}

struct Shader {
    gl_context: WebGl2RenderingContext,
    shader: WebGlShader,
}

impl Shader {
    pub fn get_shader(&self) -> &WebGlShader {
        &self.shader
    }

    pub fn from_vert_source(gl_context: &WebGl2RenderingContext, source: &String) -> Result<Shader, String> {
        Shader::from_source(gl_context, source, WebGl2RenderingContext::VERTEX_SHADER)
    }

    pub fn from_frag_source(gl_context: &WebGl2RenderingContext, source: &String) -> Result<Shader, String> {
        Shader::from_source(gl_context, source, WebGl2RenderingContext::FRAGMENT_SHADER)
    }

    fn from_source(gl_context: &WebGl2RenderingContext, source: &String, kind: u32) -> Result<Shader, String> {
        let shader = Shader::compile_shader(gl_context, source, kind)?;
        Ok(Shader { gl_context: gl_context.clone(), shader })
    }

    pub fn compile_shader(gl_context: &WebGl2RenderingContext,  source: &String, shader_type: u32) -> Result<WebGlShader, String> {
        let shader = gl_context.create_shader(shader_type).ok_or_else(|| String::from("Unable to create shader object"))?;
        gl_context.shader_source(&shader, source.as_str());
        gl_context.compile_shader(&shader);

        if gl_context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
            Ok(shader)
        } else {
            Err(gl_context.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.gl_context.delete_shader(Some(&self.shader))
    }
}
