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

use std::collections::HashMap;

use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone)]
pub struct Input {
    cursor_x: f32,
    cursor_y: f32,
    max_cursor_x: f32,
    max_cursor_y: f32,
    min_cursor_x: f32,
    min_cursor_y: f32,
    cursor_x_sensitivity: f32,
    cursor_y_sensitivity: f32,
    key_state: HashMap<Key, State>,
    mouse_button_state: HashMap<Button, State>,
    should_quit: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, FromPrimitive, Hash)]
#[repr(u8)]
pub enum State {
    Up,
    Down,
    Repeating,
}

#[derive(Copy, Clone, Eq, PartialEq, FromPrimitive, Hash)]
#[repr(u8)]
pub enum Button {
    Unknown = 0,
    Left = MouseButton::Left as u8,
    Middle = MouseButton::Middle as u8,
    Right = MouseButton::Right as u8,
    X1 = MouseButton::X1 as u8,
    X2 = MouseButton::X2 as u8,
}

#[derive(Copy, Clone, Eq, PartialEq, FromPrimitive, Hash)]
#[repr(i32)]
pub enum Key {
    Backspace = Keycode::Backspace as i32,
    Tab = Keycode::Tab as i32,
    Return = Keycode::Return as i32,
    Escape = Keycode::Escape as i32,
    Space = Keycode::Space as i32,
    Exclaim = Keycode::Exclaim as i32,
    Quotedbl = Keycode::Quotedbl as i32,
    Hash = Keycode::Hash as i32,
    Dollar = Keycode::Dollar as i32,
    Percent = Keycode::Percent as i32,
    Ampersand = Keycode::Ampersand as i32,
    Quote = Keycode::Quote as i32,
    LeftParen = Keycode::LeftParen as i32,
    RightParen = Keycode::RightParen as i32,
    Asterisk = Keycode::Asterisk as i32,
    Plus = Keycode::Plus as i32,
    Comma = Keycode::Comma as i32,
    Minus = Keycode::Minus as i32,
    Period = Keycode::Period as i32,
    Slash = Keycode::Slash as i32,
    Num0 = Keycode::Num0 as i32,
    Num1 = Keycode::Num1 as i32,
    Num2 = Keycode::Num2 as i32,
    Num3 = Keycode::Num3 as i32,
    Num4 = Keycode::Num4 as i32,
    Num5 = Keycode::Num5 as i32,
    Num6 = Keycode::Num6 as i32,
    Num7 = Keycode::Num7 as i32,
    Num8 = Keycode::Num8 as i32,
    Num9 = Keycode::Num9 as i32,
    Colon = Keycode::Colon as i32,
    Semicolon = Keycode::Semicolon as i32,
    Less = Keycode::Less as i32,
    Equals = Keycode::Equals as i32,
    Greater = Keycode::Greater as i32,
    Question = Keycode::Question as i32,
    At = Keycode::At as i32,
    LeftBracket = Keycode::LeftBracket as i32,
    Backslash = Keycode::Backslash as i32,
    RightBracket = Keycode::RightBracket as i32,
    Caret = Keycode::Caret as i32,
    Underscore = Keycode::Underscore as i32,
    Backquote = Keycode::Backquote as i32,
    A = Keycode::A as i32,
    B = Keycode::B as i32,
    C = Keycode::C as i32,
    D = Keycode::D as i32,
    E = Keycode::E as i32,
    F = Keycode::F as i32,
    G = Keycode::G as i32,
    H = Keycode::H as i32,
    I = Keycode::I as i32,
    J = Keycode::J as i32,
    K = Keycode::K as i32,
    L = Keycode::L as i32,
    M = Keycode::M as i32,
    N = Keycode::N as i32,
    O = Keycode::O as i32,
    P = Keycode::P as i32,
    Q = Keycode::Q as i32,
    R = Keycode::R as i32,
    S = Keycode::S as i32,
    T = Keycode::T as i32,
    U = Keycode::U as i32,
    V = Keycode::V as i32,
    W = Keycode::W as i32,
    X = Keycode::X as i32,
    Y = Keycode::Y as i32,
    Z = Keycode::Z as i32,
    Delete = Keycode::Delete as i32,
    CapsLock = Keycode::CapsLock as i32,
    F1 = Keycode::F1 as i32,
    F2 = Keycode::F2 as i32,
    F3 = Keycode::F3 as i32,
    F4 = Keycode::F4 as i32,
    F5 = Keycode::F5 as i32,
    F6 = Keycode::F6 as i32,
    F7 = Keycode::F7 as i32,
    F8 = Keycode::F8 as i32,
    F9 = Keycode::F9 as i32,
    F10 = Keycode::F10 as i32,
    F11 = Keycode::F11 as i32,
    F12 = Keycode::F12 as i32,
    PrintScreen = Keycode::PrintScreen as i32,
    ScrollLock = Keycode::ScrollLock as i32,
    Pause = Keycode::Pause as i32,
    Insert = Keycode::Insert as i32,
    Home = Keycode::Home as i32,
    PageUp = Keycode::PageUp as i32,
    End = Keycode::End as i32,
    PageDown = Keycode::PageDown as i32,
    Right = Keycode::Right as i32,
    Left = Keycode::Left as i32,
    Down = Keycode::Down as i32,
    Up = Keycode::Up as i32,
    NumLockClear = Keycode::NumLockClear as i32,
    KpDivide = Keycode::KpDivide as i32,
    KpMultiply = Keycode::KpMultiply as i32,
    KpMinus = Keycode::KpMinus as i32,
    KpPlus = Keycode::KpPlus as i32,
    KpEnter = Keycode::KpEnter as i32,
    Kp1 = Keycode::Kp1 as i32,
    Kp2 = Keycode::Kp2 as i32,
    Kp3 = Keycode::Kp3 as i32,
    Kp4 = Keycode::Kp4 as i32,
    Kp5 = Keycode::Kp5 as i32,
    Kp6 = Keycode::Kp6 as i32,
    Kp7 = Keycode::Kp7 as i32,
    Kp8 = Keycode::Kp8 as i32,
    Kp9 = Keycode::Kp9 as i32,
    Kp0 = Keycode::Kp0 as i32,
    KpPeriod = Keycode::KpPeriod as i32,
    Application = Keycode::Application as i32,
    Power = Keycode::Power as i32,
    KpEquals = Keycode::KpEquals as i32,
    F13 = Keycode::F13 as i32,
    F14 = Keycode::F14 as i32,
    F15 = Keycode::F15 as i32,
    F16 = Keycode::F16 as i32,
    F17 = Keycode::F17 as i32,
    F18 = Keycode::F18 as i32,
    F19 = Keycode::F19 as i32,
    F20 = Keycode::F20 as i32,
    F21 = Keycode::F21 as i32,
    F22 = Keycode::F22 as i32,
    F23 = Keycode::F23 as i32,
    F24 = Keycode::F24 as i32,
    Execute = Keycode::Execute as i32,
    Help = Keycode::Help as i32,
    Menu = Keycode::Menu as i32,
    Select = Keycode::Select as i32,
    Stop = Keycode::Stop as i32,
    Again = Keycode::Again as i32,
    Undo = Keycode::Undo as i32,
    Cut = Keycode::Cut as i32,
    Copy = Keycode::Copy as i32,
    Paste = Keycode::Paste as i32,
    Find = Keycode::Find as i32,
    Mute = Keycode::Mute as i32,
    VolumeUp = Keycode::VolumeUp as i32,
    VolumeDown = Keycode::VolumeDown as i32,
    KpComma = Keycode::KpComma as i32,
    KpEqualsAS400 = Keycode::KpEqualsAS400 as i32,
    AltErase = Keycode::AltErase as i32,
    Sysreq = Keycode::Sysreq as i32,
    Cancel = Keycode::Cancel as i32,
    Clear = Keycode::Clear as i32,
    Prior = Keycode::Prior as i32,
    Return2 = Keycode::Return2 as i32,
    Separator = Keycode::Separator as i32,
    Out = Keycode::Out as i32,
    Oper = Keycode::Oper as i32,
    ClearAgain = Keycode::ClearAgain as i32,
    CrSel = Keycode::CrSel as i32,
    ExSel = Keycode::ExSel as i32,
    Kp00 = Keycode::Kp00 as i32,
    Kp000 = Keycode::Kp000 as i32,
    ThousandsSeparator = Keycode::ThousandsSeparator as i32,
    DecimalSeparator = Keycode::DecimalSeparator as i32,
    CurrencyUnit = Keycode::CurrencyUnit as i32,
    CurrencySubUnit = Keycode::CurrencySubUnit as i32,
    KpLeftParen = Keycode::KpLeftParen as i32,
    KpRightParen = Keycode::KpRightParen as i32,
    KpLeftBrace = Keycode::KpLeftBrace as i32,
    KpRightBrace = Keycode::KpRightBrace as i32,
    KpTab = Keycode::KpTab as i32,
    KpBackspace = Keycode::KpBackspace as i32,
    KpA = Keycode::KpA as i32,
    KpB = Keycode::KpB as i32,
    KpC = Keycode::KpC as i32,
    KpD = Keycode::KpD as i32,
    KpE = Keycode::KpE as i32,
    KpF = Keycode::KpF as i32,
    KpXor = Keycode::KpXor as i32,
    KpPower = Keycode::KpPower as i32,
    KpPercent = Keycode::KpPercent as i32,
    KpLess = Keycode::KpLess as i32,
    KpGreater = Keycode::KpGreater as i32,
    KpAmpersand = Keycode::KpAmpersand as i32,
    KpDblAmpersand = Keycode::KpDblAmpersand as i32,
    KpVerticalBar = Keycode::KpVerticalBar as i32,
    KpDblVerticalBar = Keycode::KpDblVerticalBar as i32,
    KpColon = Keycode::KpColon as i32,
    KpHash = Keycode::KpHash as i32,
    KpSpace = Keycode::KpSpace as i32,
    KpAt = Keycode::KpAt as i32,
    KpExclam = Keycode::KpExclam as i32,
    KpMemStore = Keycode::KpMemStore as i32,
    KpMemRecall = Keycode::KpMemRecall as i32,
    KpMemClear = Keycode::KpMemClear as i32,
    KpMemAdd = Keycode::KpMemAdd as i32,
    KpMemSubtract = Keycode::KpMemSubtract as i32,
    KpMemMultiply = Keycode::KpMemMultiply as i32,
    KpMemDivide = Keycode::KpMemDivide as i32,
    KpPlusMinus = Keycode::KpPlusMinus as i32,
    KpClear = Keycode::KpClear as i32,
    KpClearEntry = Keycode::KpClearEntry as i32,
    KpBinary = Keycode::KpBinary as i32,
    KpOctal = Keycode::KpOctal as i32,
    KpDecimal = Keycode::KpDecimal as i32,
    KpHexadecimal = Keycode::KpHexadecimal as i32,
    LCtrl = Keycode::LCtrl as i32,
    LShift = Keycode::LShift as i32,
    LAlt = Keycode::LAlt as i32,
    LGui = Keycode::LGui as i32,
    RCtrl = Keycode::RCtrl as i32,
    RShift = Keycode::RShift as i32,
    RAlt = Keycode::RAlt as i32,
    RGui = Keycode::RGui as i32,
    Mode = Keycode::Mode as i32,
    AudioNext = Keycode::AudioNext as i32,
    AudioPrev = Keycode::AudioPrev as i32,
    AudioStop = Keycode::AudioStop as i32,
    AudioPlay = Keycode::AudioPlay as i32,
    AudioMute = Keycode::AudioMute as i32,
    MediaSelect = Keycode::MediaSelect as i32,
    Www = Keycode::Www as i32,
    Mail = Keycode::Mail as i32,
    Calculator = Keycode::Calculator as i32,
    Computer = Keycode::Computer as i32,
    AcSearch = Keycode::AcSearch as i32,
    AcHome = Keycode::AcHome as i32,
    AcBack = Keycode::AcBack as i32,
    AcForward = Keycode::AcForward as i32,
    AcStop = Keycode::AcStop as i32,
    AcRefresh = Keycode::AcRefresh as i32,
    AcBookmarks = Keycode::AcBookmarks as i32,
    BrightnessDown = Keycode::BrightnessDown as i32,
    BrightnessUp = Keycode::BrightnessUp as i32,
    DisplaySwitch = Keycode::DisplaySwitch as i32,
    KbdIllumToggle = Keycode::KbdIllumToggle as i32,
    KbdIllumDown = Keycode::KbdIllumDown as i32,
    KbdIllumUp = Keycode::KbdIllumUp as i32,
    Eject = Keycode::Eject as i32,
    Sleep = Keycode::Sleep as i32,
}

impl Input {
    pub fn new(screen_buffer_width: i32, screen_buffer_height: i32, screen_width: i32, screen_height: i32) -> Input {
        Input {
            cursor_x: 0.0,
            cursor_y: 0.0,
            max_cursor_x: (screen_buffer_width / 2) as f32,
            max_cursor_y: (screen_buffer_height / 2) as f32,
            min_cursor_x: (-screen_buffer_width / 2) as f32,
            min_cursor_y: (-screen_buffer_height / 2) as f32,
            cursor_x_sensitivity: screen_buffer_width as f32 / screen_width as f32,
            cursor_y_sensitivity: screen_buffer_height as f32 / screen_height as f32,
            key_state: HashMap::new(),
            mouse_button_state: HashMap::new(),
            should_quit: false,
        }
    }

    #[allow(unused_variables)]
    pub fn process_sdl_event(&mut self, event:& Event) -> Result<(), String> {
        match event {
            Event::KeyDown {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat
            } => {
               self.key_state.insert(Key::from_i32(keycode.unwrap() as i32).ok_or_else(|| "unknown keycode")?, State::Down);
            }
            Event::KeyUp {
                timestamp,
                window_id,
                keycode,
                scancode,
                keymod,
                repeat
            } => {
                self.key_state.insert(Key::from_i32(keycode.unwrap() as i32).ok_or_else(|| "unknown keycode")?, State::Up);
            }
            Event::MouseMotion {
                timestamp,
                window_id,
                which,
                mousestate,
                x,
                y,
                xrel,
                yrel,
            } => {
                let adjusted_x_rel = *xrel as f32 * self.cursor_x_sensitivity;
                let adjusted_y_rel = *yrel as f32 * self.cursor_y_sensitivity;

                self.cursor_x += adjusted_x_rel;
                if self.cursor_x > self.max_cursor_x {
                    self.cursor_x = self.max_cursor_x;
                } else if self.cursor_x < self.min_cursor_x {
                    self.cursor_x = self.min_cursor_x ;
                }
                self.cursor_y -= adjusted_y_rel; // subtract to invert y value
                if self.cursor_y > self.max_cursor_y {
                    self.cursor_y = self.max_cursor_y;
                } else if self.cursor_y < self.min_cursor_y {
                    self.cursor_y = self.min_cursor_y;
                }
            }
            Event::MouseButtonDown {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => {
                self.mouse_button_state.insert(Button::from_i32(*mouse_btn as i32).ok_or_else(|| "unknown button")?, State::Down);
            }
            Event::MouseButtonUp {
                timestamp,
                window_id,
                which,
                mouse_btn,
                clicks,
                x,
                y,
            } => {
                self.mouse_button_state.insert(Button::from_i32(*mouse_btn as i32).ok_or_else(|| "unknown button")?, State::Down);

            }
            Event::Quit {
                timestamp
            } => {
                self.should_quit = true
            }
            _ => { }
        };

        Ok(())
    }

    pub fn get_key_state(&self, key: Key) -> State {
        match self.key_state.get(&key) {
            None => State::Up,
            Some(state) => *state
        }
    }

    pub fn get_cursor_position(&self) -> (i64, i64)  {
        (self.cursor_x as i64, self.cursor_y as i64)
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}
