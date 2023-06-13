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

use wasm_bindgen::JsCast;
use js_sys::Date;
use web_sys::{Window, HtmlCanvasElement};
use web_sys::{
    WebGl2RenderingContext, WebGlUniformLocation,
};

use crate::gl_renderer::GlRenderer;
use crate::input;
use crate::palette::FlaskPalette;
use crate::shaders::ShaderProgram;
use crate::renderer::Renderer;
use crate::frame_buffer::FrameBuffer;
use crate::{log, palette};


pub struct FlaskContext {
    buffer_width: u32,
    buffer_height: u32,
    window: web_sys::Window,
    canvas: web_sys::HtmlCanvasElement,
    gl_context: web_sys::WebGl2RenderingContext,
    gl_renderer: GlRenderer,
    uniform_palette_size_location: WebGlUniformLocation,
    uniform_background_color_index_location: WebGlUniformLocation,
    renderer: Renderer,
}

impl FlaskContext {
    pub fn new(buffer_width: u32, buffer_height: u32, fullscreen: bool, palette: FlaskPalette) -> Result<FlaskContext, String> {
        log("Creating FlaskContext...");

        if buffer_width % 4 != 0 {
            return Err(String::from("\'buffer_width\' must be 4 byte aligned"));
        }

//        if  buffer_width < buffer_height {
//            return Err(String::from("\'buffer_width\' can't be smaller than \'buffer_height\'"));
//        }

        log("Resolving canvas...");

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        log("Creating webgl2 context...");

        let gl_context: web_sys::WebGl2RenderingContext = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();

        let display_width = canvas.width();
        let display_height = canvas.height();

        let viewport_width = (buffer_width * (display_width / buffer_width)) as i32;
        let viewport_height = (buffer_height * (display_height / buffer_height)) as i32;

        log("Setting viewport...");

        gl_context.viewport(
            (display_width as i32 - viewport_width) / 2,
            0,
            viewport_width,
            viewport_height
        );

        log("Loading shader...");

        // Load shaders
        let shader = ShaderProgram::load_shaders(
                gl_context.clone(),
                &String::from(include_str!("shaders/screen_shader.vert")),
                &String::from(include_str!("shaders/screen_shader.frag"))
        )?;

        log("Resolving uniform locations...");

        let uniform_palette_size_location = shader.get_uniform_location("palette_size").unwrap();
        let uniform_background_color_index_location = shader.get_uniform_location("background_color_index").unwrap();

        let uniform_texture_sampler_location = shader.get_uniform_location("texture_sampler").unwrap();
        let uniform_palette_sampler_location = shader.get_uniform_location("palette_sampler").unwrap();

        log("Creating GlRenderer...");

        let gl_renderer = GlRenderer::new(gl_context.clone(), shader);
        gl_renderer.set_clear_color(0.0, 0.0, 0.0, 0.0);

        gl_renderer.begin_rendering();
        gl_renderer.set_uniform_int(&uniform_texture_sampler_location, 0);
        gl_renderer.set_uniform_int(&uniform_palette_sampler_location, 1);
        gl_renderer.end_rendering();
        
        log("Loading palette...");
        
        let palette = palette::load_palette(palette);

        log("Creating Renderer...");

        let renderer = Renderer::new(gl_context.clone(), FrameBuffer::new(gl_context.clone(), buffer_width, buffer_height), palette)?;

        input::init_input_handlers(
                &window,
                &canvas,
                buffer_width as i32,
                buffer_height as i32,
                display_width as i32,
                display_height as i32,
                fullscreen
        );

        log("FlaskContext created");

        Ok(FlaskContext {
            buffer_width,
            buffer_height,
            window,
            canvas,
            gl_context,
            gl_renderer,
            uniform_palette_size_location,
            uniform_background_color_index_location,
            renderer,
        })
    }

    pub fn get_window() -> Window {
        web_sys::window().expect("Global `window` doesn't exists")
    }

    pub fn get_canvas(&self) -> HtmlCanvasElement {
        self.canvas.clone()
    }

    pub fn get_gl_context(&self) -> web_sys::WebGl2RenderingContext {
        self.gl_context.clone()
    }

    pub fn get_renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn render_frame(&mut self) {
        let frame_buffer = self.renderer.get_frame_buffer();
        self.render_frame_buffer(&frame_buffer)
    }

    pub fn render_frame_buffer(&self, frame_buffer: &FrameBuffer) {
        let texture = frame_buffer.render_to_texture();

        self.gl_renderer.clear_buffer();

        self.gl_renderer.begin_rendering();

        let palette_texture = self.renderer.get_palette_texture();
        self.gl_renderer.set_uniform_int(&self.uniform_palette_size_location, palette_texture.width() as i32);
        self.gl_renderer.set_uniform_int(&self.uniform_background_color_index_location, self.renderer.get_background_color() as i32);

        self.gl_renderer.render(frame_buffer.get_quad(), texture, palette_texture);

        self.gl_renderer.end_rendering();
    }

    pub fn time_now() -> i128 {
        Date::new_0().get_milliseconds() as i128
    }
        
    pub fn update_input(&self) {
        input::update_last_states();
    }
}

