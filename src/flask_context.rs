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

use std::time::{SystemTime, UNIX_EPOCH};

use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    EventTarget, MouseEvent, WebGlBuffer, WebGlProgram, WebGl2RenderingContext, WebGlUniformLocation,
};

use crate::gl_renderer::GlRenderer;
use crate::shaders::ShaderProgram;
use crate::color::Color;
use crate::input::Input;
use crate::renderer::Renderer;
use crate::frame_buffer::FrameBuffer;


pub struct FlaskContext {
    buffer_width: u32,
    buffer_height: u32,
    canvas: web_sys::HtmlCanvasElement,
    gl_context: web_sys::WebGl2RenderingContext,
    gl_renderer: GlRenderer,
    renderer: Renderer,
    input: Input,
}

const UNIFORM_PALETTE_SIZE_LOCATION: i32 = 2;
const UNIFORM_BACKGROUND_COLOR_INDEX_LOCATION: i32 = 3;

impl FlaskContext {
    pub fn new(buffer_width: u32, buffer_height: u32, game_name: &str, palette: Vec<Color>) -> Result<FlaskContext, String> {
        if buffer_width % 4 != 0 {
            return Err(String::from("\'buffer_width\' must be 4 byte aligned"));
        }

        if  buffer_width < buffer_height {
            return Err(String::from("\'buffer_width\' can't be smaller than \'buffer_height\'"));
        }

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let gl_context: web_sys::WebGl2RenderingContext = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();

//        // Hide mouse cursor
//        sdl.mouse().show_cursor(false);
//        sdl.mouse().set_relative_mouse_mode(true);
//
//        // Get primary display bounds
//        let current_display = video_subsystem.display_bounds(0)?;
//        let display_width = current_display.width();
//        let display_height = current_display.height();
//
//        let window = video_subsystem
//            .window(game_name, display_width, display_height)
//            .opengl()
//            .borderless()
//            .build()
//            .unwrap();

//        let gl_context = window.gl_create_context().unwrap();
//        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

//        // disable vsync
//        video_subsystem.gl_set_swap_interval(0).unwrap();

        let display_width = canvas.width();
        let display_height = canvas.height();

        let modifier = display_height / buffer_height;
        let viewport_width = (buffer_width * modifier) as i32;
        let viewport_height = display_height as i32;

        gl_context.viewport(
            (display_width as i32 - viewport_width) / 2,
            0,
            viewport_width,
            viewport_height
        );

        // Load shaders
        let shader = {
            ShaderProgram::load_shaders(
                gl_context.clone(),
                &String::from(include_str!("shaders/screen_shader.vert")),
                &String::from(include_str!("shaders/screen_shader.frag"))
            )
        };

        let gl_renderer = GlRenderer::new(gl_context.clone(), shader);
        gl_renderer.set_clear_color(0.0, 0.0, 0.0, 0.0);

        let renderer = Renderer::new(gl_context.clone(), FrameBuffer::new(gl_context.clone(), buffer_width, buffer_height), palette)?;

        let input = Input::new(
            buffer_width as i32,
            buffer_height as i32,
            display_width as i32,
            display_height as i32
        );

        Ok(FlaskContext { buffer_width, buffer_height, canvas, gl_context, gl_renderer, renderer, input })
    }

    pub fn poll_input_events(&mut self) -> Input {
//        let mut event_pump = self.sdl.event_pump().unwrap();
//        for event in event_pump.poll_iter() {
//            match self.input.process_sdl_event(&event) {
//                Err(error) => println!("{}", error),
//                Ok(_) => {}
//            };
//        }
//
//        self.input.clone()
        Input {  }
    }

    pub fn get_renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn clear_screen(&mut self) {
        self.renderer.clear_screen();
    }

    pub fn render_and_swap(&mut self) -> Result<(), String> {
        let frame_buffer = self.renderer.get_frame_buffer();
        self.render_buffer_and_swap(&frame_buffer)
    }

    pub fn render_buffer_and_swap(&self, frame_buffer: &FrameBuffer) -> Result<(), String> {
        let texture = frame_buffer.render_to_texture()?;

        self.gl_renderer.clear_buffer();

        self.gl_renderer.begin_rendering();

        let palette_texture = self.renderer.get_palette_texture();
        self.gl_renderer.set_uniform_int(UNIFORM_PALETTE_SIZE_LOCATION, palette_texture.width() as i32);
        self.gl_renderer.set_uniform_int(UNIFORM_BACKGROUND_COLOR_INDEX_LOCATION, self.renderer.get_background_color() as i32);

        self.gl_renderer.render(frame_buffer.get_quad(), texture, palette_texture);

        self.gl_renderer.end_rendering();

        Ok(())
    }

    pub fn time_now() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }
}

