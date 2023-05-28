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

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

use crate::input::Input;
use crate::palette::{FlaskColor, FlaskPalette};
use crate::scene::Scene;
use crate::flask_context::FlaskContext;
use crate::log;

pub struct GameContext {
    flask_context: FlaskContext,
    current_scene: Box<dyn Scene>,
    last_frame_time: i128,
}

impl GameContext {
    const TICK_RATE: u128 = (1.0f64 / 60.0f64 * 1000.0f64) as u128;

    fn new(buffer_width: u32, buffer_height: u32, fullscreen: bool, palette: FlaskPalette, starting_scene: Box<dyn Scene>) -> Result<GameContext, String> {
        log("Creating GameContext...");

        let flask_context = FlaskContext::new(buffer_width, buffer_height, fullscreen, palette)?;

        let current_scene = starting_scene;
        let last_frame_time = FlaskContext::time_now();

        log("GameContext created");

        Ok(GameContext {
            flask_context,
            current_scene,
            last_frame_time,
        })
    }

    // This func is mutable to ensure that this object is not used more than once when game is running
    pub fn run(buffer_width: u32, buffer_height: u32, fullscreen: bool, palette: FlaskPalette, starting_scene: Box<dyn Scene>) -> Result<(), String> {
        let mut game_context = GameContext::new(buffer_width, buffer_height, fullscreen, palette, starting_scene)?;

        log("Initialization successful. Running...");

        game_context.current_scene.on_start(&mut game_context.flask_context.get_renderer_mut());

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            game_context.on_frame_update();
            GameContext::request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));

        GameContext::request_animation_frame(g.borrow().as_ref().unwrap());

        Ok(())
    }

    fn on_frame_update(&mut self) {
        let time_now = FlaskContext::time_now();
        let delta_time = std::cmp::max(time_now - self.last_frame_time, 0);

        self.last_frame_time = time_now;

        // Update scene
        if let Some(scene) = self.current_scene.on_update(
                self.flask_context.get_renderer_mut(),
                &Input::new(),
                delta_time as f64 / 1000.0
        ) {
            self.current_scene.on_destroy();
            self.current_scene = scene;

            let renderer = self.flask_context.get_renderer_mut();

            // Reset renderer
            renderer.set_camera_x(0);
            renderer.set_camera_y(0);
            renderer.set_background_color(FlaskColor::Purple);

            self.current_scene.on_start(renderer);
        };

        self.flask_context.render_frame();
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        FlaskContext::get_window().request_animation_frame(f.as_ref().unchecked_ref()).expect("Failed to request animation frame");
    }
}
