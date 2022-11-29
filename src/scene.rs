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

use crate::input::Input;
use crate::renderer::Renderer;

pub trait Scene {
    // It is useful to pass renderer on_start to do basic setup such as background color or set palette
    fn on_start(&mut self, renderer: &mut Renderer);

    fn on_update(&mut self, renderer: &mut Renderer, input: &Input, delta_time: f64) -> Option<Box<dyn Scene>>;

    fn on_destroy(&mut self);
}
