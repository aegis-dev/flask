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

use crate::gl_renderer::GlRenderer;
use crate::shaders::ShaderProgram;
use crate::scene::Scene;
use crate::color::Color;
use crate::input::Input;
use crate::renderer::Renderer;
use crate::frame_buffer::FrameBuffer;

pub struct GameContext;

impl GameContext {
    const TICK_RATE: u128 = (1.0f64 / 60.0f64 * 1000.0f64) as u128;

    pub fn new() -> GameContext {
        GameContext { }
    }

    pub fn run(&self, buffer_width: u32, buffer_height: u32, game_name: &str, palette: Vec<Color>, starting_scene: Box<dyn Scene>) -> Result<(), String> {
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
            //.fullscreen()
            .borderless()
            .build()
            .unwrap();

        let _gl_context = window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

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

        // Initialize palette size uniform value
        gl_renderer.begin_rendering();
        gl_renderer.set_uniform_int("palette_size", palette.len() as i32);
        gl_renderer.end_rendering();

        let mut renderer = Renderer::new(FrameBuffer::new(buffer_width, buffer_height), palette)?;

        let mut input = Input::new(
            buffer_width as i32,
            buffer_height as i32,
            display_width as i32,
            display_height as i32
        );

        let mut event_pump = sdl.event_pump().unwrap();

        let mut current_scene = starting_scene;

        current_scene.on_start();

        let delta_time = GameContext::time_now();
        let mut last_frame_time = delta_time;

        'main_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => break 'main_loop,
                    _ => {},
                }

                match input.process_sdl_event(&event) {
                    Err(error) => println!("{}", error),
                    Ok(_) => {}
                };
            }

            let time_now = GameContext::time_now();
            if time_now >= last_frame_time + GameContext::TICK_RATE {
                let delta_time = time_now - last_frame_time;
                last_frame_time = time_now;

                // Update scene
                match current_scene.on_update(&mut renderer, &input, (delta_time / 1000) as f64) {
                    Some(scene) => {
                        current_scene.on_destroy();
                        current_scene = scene;
                        current_scene.on_start();
                    }
                    _ => { }
                };

                // TODO: add proper debug mode
                //let update_time = GameContext::time_now();

                // Update render texture
                renderer.swap()?;
                //println!("update: {0}, + swap: {1}", update_time - time_now, GameContext::time_now() - time_now);
            }

            // TODO limit frames per second by monitor refresh rate?
            gl_renderer.clear_buffer();

            gl_renderer.begin_rendering();

            gl_renderer.set_uniform_int("background_color_index", renderer.get_background_color() as i32);

            let frame_buffer = renderer.get_frame_buffer();
            gl_renderer.render(frame_buffer.get_quad(), frame_buffer.get_texture(), renderer.get_palette_texture());

            gl_renderer.end_rendering();

            window.gl_swap_window();
        }

        Ok(())
    }

    pub fn time_now() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }
}
