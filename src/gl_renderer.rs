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

use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

use crate::shaders::ShaderProgram;
use crate::mesh::Mesh;
use crate::texture::Texture;

pub struct GlRenderer {
    gl_context: WebGl2RenderingContext,
    shader: ShaderProgram,
}

impl GlRenderer {
    pub fn new(gl_context: WebGl2RenderingContext, shader: ShaderProgram) -> GlRenderer {
        gl_context.disable(WebGl2RenderingContext::DEPTH_TEST);
        gl_context.enable(WebGl2RenderingContext::BLEND);
        gl_context.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

        GlRenderer { gl_context, shader }
    }

    pub fn clear_buffer(&self) {
        self.gl_context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }

    pub fn begin_rendering(&self) {
        self.shader.enable();
    }

    pub fn end_rendering(&self) {
        self.shader.disable();
    }

    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.gl_context.clear_color(r, g, b, a);
    }

    pub fn set_uniform_int(&self, location: &WebGlUniformLocation, value: i32) {
        self.shader.set_uniform_int(location, value);
    }

    pub fn render(&self, mesh: &Mesh, texture: &Texture, palette: &Texture) {
        self.gl_context.bind_vertex_array(Some(mesh.vao()));
        self.gl_context.enable_vertex_attrib_array(0);
        self.gl_context.enable_vertex_attrib_array(1);

        self.gl_context.active_texture(texture.texture_id());
        self.gl_context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture.texture()));

        self.gl_context.active_texture(palette.texture_id());
        self.gl_context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&palette.texture()));

        self.gl_context.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, mesh.indices_count(), WebGl2RenderingContext::UNSIGNED_INT, 0);

        self.gl_context.disable_vertex_attrib_array(0);
        self.gl_context.disable_vertex_attrib_array(1);
        self.gl_context.bind_vertex_array(None);
    }
}