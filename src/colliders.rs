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

pub struct BoxCollider {
    position_x: i64,
    position_y: i64,
    width: i64,
    height: i64
}

impl BoxCollider {
    pub fn new(position_x: i64, position_y: i64, width: i64, height: i64) -> BoxCollider {
        BoxCollider { position_x, position_y, width, height }
    }

    pub fn set_position(&mut self, position_x: i64, position_y: i64) {
        self.position_x = position_x;
        self.position_y = position_y;
    }

    pub fn set_size(&mut self, width: i64, height: i64) {
        self.width = width;
        self.height = height;
    }

    pub fn is_point_in_box(&self, point_x: i64, point_y: i64) -> bool {
        (
            point_x >= self.position_x &&
            point_x <= self.position_x + self.width &&
            point_y >= self.position_y && point_y <= self.position_y + self.height
        )
    }
}
