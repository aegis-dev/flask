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

use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, Window, HtmlCanvasElement, MouseEvent};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;


lazy_static! {
    static ref CURSOR_X: Mutex<f32> = Mutex::new(0.0);
    static ref CURSOR_Y: Mutex<f32> = Mutex::new(0.0);
    static ref MAX_CURSOR_X: Mutex<f32> = Mutex::new(0.0);
    static ref MAX_CURSOR_Y: Mutex<f32> = Mutex::new(0.0);
    static ref MIN_CURSOR_X: Mutex<f32> = Mutex::new(0.0);
    static ref MIN_CURSOR_Y: Mutex<f32> = Mutex::new(0.0);
    static ref CURSOR_X_SENSITIVITY: Mutex<f32> = Mutex::new(0.0);
    static ref CURSOR_Y_SENSITIVITY: Mutex<f32> = Mutex::new(0.0);
    static ref MOUSE_EVENT_LOCK: Mutex<i64> = Mutex::new(0);
    static ref KEY_STATE: Mutex<HashMap<Key, State>> = Mutex::new(HashMap::new());
    static ref LAST_KEY_STATE: Mutex<HashMap<Key, State>> = Mutex::new(HashMap::new());
    static ref MOUSE_STATE: Mutex<HashMap<Button, State>> = Mutex::new(HashMap::new());
    static ref LAST_MOUSE_STATE: Mutex<HashMap<Button, State>> = Mutex::new(HashMap::new());
}

#[derive(Copy, Clone, Eq, PartialEq, FromPrimitive, Hash)]
#[repr(u8)]
pub enum State {
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq, FromPrimitive, Hash)]
#[repr(u8)]
pub enum Button {
    Left    = 0,
    Middle  = 1,
    Right   = 2,
    X1      = 3,
    X2      = 4,
}

#[derive(Copy, Clone, Eq, PartialEq, FromPrimitive, Hash)]
#[repr(i32)]
pub enum Key {
    Backspace       = 8,
    Tab             = 9,
    Enter           = 13,
    Shift           = 16,
    Ctrl            = 17,
    Alt             = 18,
    Pause           = 19,
    CapsLock        = 20,
    Escape          = 27,
    Space           = 32,
    PageUp          = 33,
    PageDown        = 34,
    End             = 35,
    Home            = 36,
    LeftArrow       = 37,
    UpArrow         = 38,
    RightArrow      = 39,
    DownArrow       = 40,
    PrintScreen     = 44,
    Insert          = 45,
    Delete          = 46,
    Num0            = 48,
    Num1            = 49,
    Num2            = 50,
    Num3            = 51,
    Num4            = 52,
    Num5            = 53,
    Num6            = 54,
    Num7            = 55,
    Num8            = 56,
    Num9            = 57,
    A               = 65,
    B               = 66,
    C               = 67,
    D               = 68,
    E               = 69,
    F               = 70,
    G               = 71,
    H               = 72,
    I               = 73,
    J               = 74,
    K               = 75,
    L               = 76,
    M               = 77,
    N               = 78,
    O               = 79,
    P               = 80,
    Q               = 81,
    R               = 82,
    S               = 83,
    T               = 84,
    U               = 85,
    V               = 86,
    W               = 87,
    X               = 88,
    Y               = 89,
    Z               = 90,
    LeftWindowKey   = 91,
    RightWindowKey  = 92,
    SelectKey       = 93,
    Numpad0         = 96,
    Numpad1         = 97,
    Numpad2         = 98,
    Numpad3         = 99,
    Numpad4         = 100,
    Numpad5         = 101,
    Numpad6         = 102,
    Numpad7         = 103,
    Numpad8         = 104,
    Numpad9         = 105,
    Multiply        = 106,
    Add             = 107,
    Subtract        = 109,
    DecimalPoint    = 110,
    Divide          = 111,
    F1              = 112,
    F2              = 113,
    F3              = 114,
    F4              = 115,
    F5              = 116,
    F6              = 117,
    F7              = 118,
    F8              = 119,
    F9              = 120,
    F10             = 121,
    F11             = 122,
    F12             = 123,
    NumLock         = 144,
    ScrollLock      = 145,
    SemiColon       = 186,
    Equal           = 187,
    Comma           = 188,
    Dash            = 189,
    Period          = 190,
    ForwardSlash    = 191,
    Backquote       = 192,
    OpenBracket     = 219,
    BackSlash       = 220,
    CloseBracket    = 221,
    SingleQuote     = 222,
}

fn get_key_state(key: Key) -> State {
    match KEY_STATE.lock().unwrap().get(&key) {
        None => State::Up,
        Some(state) => *state
    }
}

fn get_last_key_state(key: Key) -> State {
    match LAST_KEY_STATE.lock().unwrap().get(&key) {
        None => State::Up,
        Some(state) => *state
    }
}

fn get_button_state(button: Button) -> State {
    match MOUSE_STATE.lock().unwrap().get(&button) {
        None => State::Up,
        Some(state) => *state
    }
}

fn get_last_button_state(button: Button) -> State {
    match LAST_MOUSE_STATE.lock().unwrap().get(&button) {
        None => State::Up,
        Some(state) => *state
    }
}

fn get_cursor_position_x() -> i64  {
    *CURSOR_X.lock().unwrap() as i64
}

fn get_cursor_position_y() -> i64  {
    *CURSOR_Y.lock().unwrap() as i64
}

fn get_cursor_position() -> (i64, i64)  {
    (*(CURSOR_X.lock().unwrap()) as i64, *(CURSOR_Y.lock().unwrap()) as i64)
}

pub(crate) fn init_input_handlers(
        window: &Window,
        canvas: &HtmlCanvasElement,
        screen_buffer_width: i32,
        screen_buffer_height: i32,
        screen_width: i32,
        screen_height: i32,
        fullscreen: bool,
) {
    *MAX_CURSOR_X.lock().unwrap() = (screen_buffer_width / 2) as f32;
    *MAX_CURSOR_Y.lock().unwrap() = (screen_buffer_height / 2) as f32;
    *MIN_CURSOR_X.lock().unwrap() = (-screen_buffer_width / 2) as f32;
    *MIN_CURSOR_Y.lock().unwrap() = (-screen_buffer_height / 2) as f32;
    *CURSOR_X_SENSITIVITY.lock().unwrap() = screen_buffer_width as f32 / screen_width as f32;
    *CURSOR_Y_SENSITIVITY.lock().unwrap() = screen_buffer_height as f32 / screen_height as f32;

    {
        let callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let key = Key::from_u32(event.key_code()).unwrap();
            KEY_STATE.lock().unwrap().insert(key, State::Down);
        }) as Box<dyn FnMut(KeyboardEvent)>);

        window.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref()).unwrap();
        callback.forget();
    }
    {
        let callback = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let key = Key::from_u32(event.key_code()).unwrap();
            KEY_STATE.lock().unwrap().insert(key, State::Up);
        }) as Box<dyn FnMut(KeyboardEvent)>);

        window.add_event_listener_with_callback("keyup", callback.as_ref().unchecked_ref()).unwrap();
        callback.forget();
    }
    {
        let callback = Closure::wrap(Box::new(move |event: MouseEvent| {
            let button = Button::from_i16(event.button()).unwrap();
            MOUSE_STATE.lock().unwrap().insert(button, State::Down);
        }) as Box<dyn FnMut(MouseEvent)>);

        canvas.add_event_listener_with_callback("mousedown", callback.as_ref().unchecked_ref()).unwrap();
        callback.forget();
    }
    {
        let callback = Closure::wrap(Box::new(move |event: MouseEvent| {
            let button = Button::from_i16(event.button()).unwrap();
            MOUSE_STATE.lock().unwrap().insert(button, State::Up);
        }) as Box<dyn FnMut(MouseEvent)>);

        canvas.add_event_listener_with_callback("mouseup", callback.as_ref().unchecked_ref()).unwrap();
        callback.forget();
    }
    {
        let callback = Closure::wrap(Box::new(move |event: MouseEvent| {
            let mouse_move_x = event.movement_x();
            let mouse_move_y = event.movement_y();

            let adjusted_x_rel = mouse_move_x as f32 * *CURSOR_X_SENSITIVITY.lock().unwrap();
            let adjusted_y_rel = mouse_move_y as f32 * *CURSOR_Y_SENSITIVITY.lock().unwrap();

            let mut cursor_x = {*CURSOR_X.lock().unwrap()};
            let mut cursor_y = {*CURSOR_Y.lock().unwrap()};

            let max_cursor_x = *MAX_CURSOR_X.lock().unwrap();
            let max_cursor_y = *MAX_CURSOR_Y.lock().unwrap();

            let min_cursor_x = *MIN_CURSOR_X.lock().unwrap();
            let min_cursor_y = *MIN_CURSOR_Y.lock().unwrap();

            cursor_x += adjusted_x_rel;
            if cursor_x > max_cursor_x {
                cursor_x = max_cursor_x;
            } else if cursor_x < min_cursor_x {
                cursor_x = min_cursor_x ;
            }
            cursor_y -= adjusted_y_rel; // subtract to invert y value
            if cursor_y > max_cursor_y {
                cursor_y = max_cursor_y;
            } else if cursor_y < min_cursor_y {
                cursor_y = min_cursor_y;
            }
            *CURSOR_X.lock().unwrap() = cursor_x;
            *CURSOR_Y.lock().unwrap() = cursor_y;
        }) as Box<dyn FnMut(MouseEvent)>);

        canvas.add_event_listener_with_callback("mousemove", callback.as_ref().unchecked_ref()).unwrap();
        callback.forget();
    }

    // Lock mouse in canvas and enter fullscreen
    {
        let callback = Closure::wrap(Box::new(move || {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let canvas = document.get_element_by_id("canvas").unwrap();
            let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
            canvas.request_pointer_lock();
            if fullscreen {
                canvas.request_fullscreen().unwrap();
            }
        }) as Box<dyn FnMut()>);

        canvas.set_onclick(Some(callback.as_ref().unchecked_ref()));
        callback.forget();
    }
}

pub(crate) fn update_last_states() {
    for (key, state) in KEY_STATE.lock().unwrap().iter() {
        LAST_KEY_STATE.lock().unwrap().insert(key.clone(), state.clone());
    }
    
    for (button, state) in MOUSE_STATE.lock().unwrap().iter() {
        LAST_MOUSE_STATE.lock().unwrap().insert(button.clone(), state.clone());
    }
}

pub struct Input;

impl Input {
    pub fn new() -> Input {
        Input { }
    }

    pub fn get_key_state(&self, key: Key) -> State {
        get_key_state(key)
    }
    
    pub fn get_last_key_state(&self, key: Key) -> State {
        get_last_key_state(key)
    }
    
    pub fn key_pressed(&self, key: Key) -> bool {
        let last_state = get_last_key_state(key);
        let state = get_key_state(key);
        state == State::Down && last_state != state
    }

    pub fn get_button_state(&self, button: Button) -> State {
        get_button_state(button)
    }
    
    pub fn get_last_button_state(&self, button: Button) -> State {
        get_last_button_state(button)
    }
    
    pub fn button_pressed(&self, button: Button) -> bool {
        let last_state = get_last_button_state(button);
        let state = get_button_state(button);
        state == State::Down && last_state != state
    }

    pub fn get_cursor_position_x(&self) -> i64  {
        get_cursor_position_x()
    }

    pub  fn get_cursor_position_y(&self) -> i64  {
        get_cursor_position_y()
    }

    pub fn get_cursor_position(&self) -> (i64, i64)  {
        get_cursor_position()
    }
}
