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

pub struct Rand {
    state: u64,
}

impl Rand {
    pub fn new() -> Rand {
        Rand { state: 0 }
    }

    pub fn new_with_seed(seed: u64) -> Rand {
        Rand { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state += 0x9E3779B97F4A7C15;
        let mut z: u64 = self.state;
        z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9;
        z = (z ^ (z >> 27)) * 0x94D049BB133111EB;
        z ^ (z >> 31)
    }

    pub fn next_bool(&mut self) -> bool {
        self.next_i64_in_range(-2, 2) > 0
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    pub fn next_f32(&mut self) -> f32 {
        self.next_u32() as f32 / u32::MAX as f32
    }

    pub fn next_f64(&mut self) -> f64 {
        self.next_u32() as f64 / u32::MAX as f64
    }

    // Min inclusive and max exclusive
    pub fn next_i64_in_range(&mut self, min: i64, max: i64) -> i64 {
        (self.next_f64() * (max - min) as f64) as i64 + min
    }
}