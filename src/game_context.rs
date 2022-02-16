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

use crate::scene::Scene;
use crate::color::Color;
use crate::game_status::GameStatus;
use crate::flask_context::FlaskContext;

pub struct GameContext;

impl GameContext {
    const TICK_RATE: u128 = (1.0f64 / 60.0f64 * 1000.0f64) as u128;

    pub fn new() -> GameContext {
        GameContext { }
    }

    // This func is mutable to ensure that this object is not used more than once when game is running
    pub fn run(&mut self, buffer_width: u32, buffer_height: u32, game_name: &str, palette: Vec<Color>, starting_scene: Box<dyn Scene>) -> Result<(), String> {
        let mut flask_context = FlaskContext::new(buffer_width, buffer_height, game_name, palette)?;

        let mut current_scene = starting_scene;

        current_scene.on_start(&mut flask_context.get_renderer_mut());

        let delta_time = FlaskContext::time_now();
        let mut last_frame_time = delta_time;

        let mut game_status = GameStatus::new();
        'main_loop: loop {
            let input = flask_context.poll_input_events();
            if input.should_quit() {
                break 'main_loop;
            }

            let time_now = FlaskContext::time_now();
            if time_now >= last_frame_time + GameContext::TICK_RATE {
                let delta_time = time_now - last_frame_time;
                last_frame_time = time_now;

                // Update scene
                match current_scene.on_update(
                    &mut game_status,
                    flask_context.get_renderer_mut(),
                    &input,
                    delta_time as f64 / 1000.0
                ) {
                    Some(scene) => {
                        current_scene.on_destroy();
                        current_scene = scene;

                        let renderer = flask_context.get_renderer_mut();

                        // Reset renderer
                        renderer.set_camera_x(0);
                        renderer.set_camera_y(0);
                        renderer.set_background_color(1).unwrap();

                        current_scene.on_start(renderer);
                    }
                    _ => {
                        if game_status.should_quit() {
                            break 'main_loop
                        }
                    }
                };

                flask_context.render_and_swap()?;
                flask_context.clear_screen();
            }
        }

        Ok(())
    }
}
