//
// Copyright © 2020-2021  Egidijus Lileika
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

use std::io::{Read, Error};

pub struct ByteBufferReader {
    index: usize,
    buffer: Vec<u8>
}

impl ByteBufferReader {
    pub fn from(buffer: &[u8]) -> ByteBufferReader {
        ByteBufferReader { index: 0, buffer: Vec::from(buffer) }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        match self.buffer.get(self.index) {
            Some(byte) => {
                self.index += 1;
                Some(*byte)
            }
            None => None
        }
    }
}

impl Read for ByteBufferReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let mut byte_idx = 0;
        for _i in 0..buf.len() {
            match self.read_byte() {
                None => break,
                Some(byte) => {
                    buf[byte_idx] = byte;
                    byte_idx += 1;
                }
            }
        }

        Ok(byte_idx)
    }
}