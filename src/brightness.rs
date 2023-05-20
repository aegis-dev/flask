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

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Brightness {
    Normal = 0,
    Dim,
    Dark,
    VeryDark,
}

impl Brightness {
    #[inline(always)]
    pub fn is_lighter(&self, other: Brightness) -> bool {
        (*self as u8) < (other as u8)
    }
    
    #[inline(always)]
    pub fn is_darker(&self, other: Brightness) -> bool {
        !self.is_lighter(other)
    }
}