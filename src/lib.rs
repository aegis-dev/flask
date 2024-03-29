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

#![allow(dead_code)]

extern crate lazy_static;
extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

mod shaders;
mod mesh;
mod texture;
mod gl_renderer;
mod byte_buffer_reader;
mod js_utils;
pub mod game_context;
pub mod scene;
pub mod frame_buffer;
pub mod color;
pub mod input;
pub mod renderer;
pub mod brightness;
pub mod sprite;
pub mod font;
pub mod palette;
pub mod flask_context;
pub mod colliders;
pub mod rand;
pub mod lights;

pub fn log(message: &str) {
    crate::js_utils::js_log(format!("[flask] {}", message).as_str());
}