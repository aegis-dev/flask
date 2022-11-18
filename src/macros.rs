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

#[macro_export]
macro_rules! js_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[macro_export]
macro_rules! js_float_32_array {
    ($arr:expr) => {{
        let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>().unwrap()
        .buffer();
        let arr_location = $arr.as_ptr() as u32 / 4;
        let array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(arr_location, arr_location + $arr.len() as u32);
        array
    }};
}

#[macro_export]
macro_rules! js_uint_32_array {
    ($arr:expr) => {{
        let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>().unwrap()
        .buffer();
        let arr_location = $arr.as_ptr() as u32 / 4;
        let array = js_sys::Uint32Array::new(&memory_buffer)
        .subarray(arr_location, arr_location + $arr.len() as u32);
        array
    }};
}