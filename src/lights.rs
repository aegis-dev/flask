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

use crate::brightness::Brightness;

pub struct PointLight {
    x: i64,
    y: i64,
    radius: u16,
}

impl PointLight {
    pub fn new(x: i64, y: i64, radius: u16) -> PointLight {
        PointLight { x, y, radius }
    }
    
    pub fn get_brightness(&self, x: i64, y: i64) -> Brightness {
        let distance = self.distance(x, y);
        let ratio = distance / self.radius as f64;
        if ratio > 1.0 {
            return Brightness::VeryDark;
        } else if ratio > 0.8 {
            return Brightness::Dark;
        } else if ratio > 0.6 {
            return Brightness::Dim;
        }
        return Brightness::Normal;
    }
    
    #[inline(always)]
    pub fn distance(&self, x: i64, y: i64) -> f64 {
        f64::sqrt(((self.x - x).pow(2) + (self.y - y).pow(2)) as f64)
    }
}