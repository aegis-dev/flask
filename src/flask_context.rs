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

use std::time::{SystemTime, UNIX_EPOCH};

use sdl2::Sdl;
use sdl2::VideoSubsystem;
use sdl2::video::Window;
use sdl2::video::GLContext;

use crate::gl_renderer::GlRenderer;
use crate::shaders::ShaderProgram;
use crate::color::Color;
use crate::input::Input;
use crate::renderer::Renderer;
use crate::frame_buffer::FrameBuffer;

pub struct FlaskContext {
    buffer_width: u32,
    buffer_height: u32,
    sdl: Sdl,
    video_subsystem: VideoSubsystem,
    window: Window,
    gl_context: GLContext,
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

        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        // Hide mouse cursor
        sdl.mouse().show_cursor(false);
        sdl.mouse().set_relative_mouse_mode(true);

        // Get primary display bounds
        let current_display = video_subsystem.display_bounds(0)?;
        let display_width = current_display.width();
        let display_height = current_display.height();

        let window = video_subsystem
            .window(game_name, display_width, display_height)
            .opengl()
            .borderless()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        // disable vsync
        video_subsystem.gl_set_swap_interval(0).unwrap();

        unsafe {
            let modifier = display_height / buffer_height;
            let viewport_width = (buffer_width * modifier) as i32;
            let viewport_height = display_height as i32;
            gl::Viewport(
                (display_width as i32 - viewport_width) / 2,
                0,
                viewport_width,
                viewport_height
            );
        }

        // Load shaders
        let shader = {
            use std::ffi::CString;
            ShaderProgram::load_shaders(
                &CString::new(include_str!("shaders/screen_shader.vert")).unwrap(),
                &CString::new(include_str!("shaders/screen_shader.frag")).unwrap(),
            )
        };

        let gl_renderer = GlRenderer::new(shader);
        gl_renderer.set_clear_color(0.0, 0.0, 0.0, 0.0);

        let renderer = Renderer::new(FrameBuffer::new(buffer_width, buffer_height), palette)?;

        let input = Input::new(
            buffer_width as i32,
            buffer_height as i32,
            display_width as i32,
            display_height as i32
        );

        Ok(FlaskContext {
            buffer_width,
            buffer_height,
            sdl,
            video_subsystem,
            window,
            gl_context,
            gl_renderer,
            renderer,
            input
        })
    }

    pub fn poll_input_events(&mut self) -> Input {
        let mut event_pump = self.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match self.input.process_sdl_event(&event) {
                Err(error) => println!("{}", error),
                Ok(_) => {}
            };
        }

        self.input.clone()
    }

    pub fn get_renderer_mut(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    pub fn render_and_swap(&mut self) -> Result<(), String> {
        // Update render texture
        self.renderer.swap()?;
        //println!("update: {0}, + swap: {1}", update_time - time_now, GameContext::time_now() - time_now);

        self.gl_renderer.clear_buffer();

        self.gl_renderer.begin_rendering();

        let palette_texture = self.renderer.get_palette_texture();
        self.gl_renderer.set_uniform_int(UNIFORM_PALETTE_SIZE_LOCATION, palette_texture.width() as i32);
        self.gl_renderer.set_uniform_int(UNIFORM_BACKGROUND_COLOR_INDEX_LOCATION, self.renderer.get_background_color() as i32);

        let frame_buffer = self.renderer.get_frame_buffer();
        self.gl_renderer.render(frame_buffer.get_quad(), frame_buffer.get_texture(), palette_texture);

        self.gl_renderer.end_rendering();

        self.window.gl_swap_window();

        Ok(())
    }

    pub fn time_now() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }
}

