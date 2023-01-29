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


use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject, WebGlBuffer};

use crate::js_utils::{js_float_32_array, js_uint_32_array};


pub struct Mesh {
    gl_context: WebGl2RenderingContext,
    vao: WebGlVertexArrayObject,
    vbos: Vec<WebGlBuffer>,
    vertices_count: i32,
    indices_count: i32,
}

impl Mesh {
    pub fn from_data(gl_context: WebGl2RenderingContext, vertices: &Vec<f32>, texture_coord: &Vec<f32>, indices: &Vec<u32>) -> Mesh {
        let vao = gl_context.create_vertex_array().unwrap();
        gl_context.bind_vertex_array(Some(&vao));

        let mut vbos: Vec<WebGlBuffer> = vec![];
        vbos.push(Mesh::bind_indices_buffer(&gl_context, indices));
        vbos.push(Mesh::store_data_in_attribute_list(&gl_context, 0, 3, vertices));
        vbos.push(Mesh::store_data_in_attribute_list(&gl_context, 1, 2, texture_coord));

        gl_context.bind_vertex_array(None);

        let vertices_count = vertices.len() as i32;
        let indices_count = indices.len() as i32;
        Mesh { gl_context, vao, vbos, vertices_count, indices_count }
    }

    fn bind_indices_buffer(gl_context: &WebGl2RenderingContext, indices: &Vec<u32>) -> WebGlBuffer {
        let vbo = gl_context.create_buffer().unwrap();

        let indices_js_array = js_uint_32_array(indices.as_slice());
        gl_context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&vbo));
        gl_context.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, &indices_js_array, WebGl2RenderingContext::STATIC_DRAW);
        gl_context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        vbo
    }

    fn store_data_in_attribute_list(gl_context: &WebGl2RenderingContext, attribute_id: u32, attribute_size: i32, data: &Vec<f32>) -> WebGlBuffer {
        let vbo = gl_context.create_buffer().unwrap();

        let data_js_array = js_float_32_array(data.as_slice());
        gl_context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));
        gl_context.buffer_data_with_array_buffer_view(WebGl2RenderingContext::ARRAY_BUFFER, &data_js_array, WebGl2RenderingContext::STATIC_DRAW);
        gl_context.vertex_attrib_pointer_with_i32(attribute_id, attribute_size, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl_context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        vbo
    }

    pub fn vao(&self) -> &WebGlVertexArrayObject {
        &self.vao
    }

    pub fn vertices_count(&self) -> i32 {
        self.vertices_count
    }

    pub fn indices_count(&self) -> i32 {
        self.indices_count
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        for vbo in &self.vbos {
            self.gl_context.delete_buffer(Some(&vbo));
        }
        self.gl_context.delete_vertex_array(Some(&self.vao));
    }
}
