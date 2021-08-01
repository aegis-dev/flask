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

#version 430 core

in vec2 frag_texture_coords;

out vec4 color;

layout(binding = 0) uniform sampler2D texture_sampler;
layout(binding = 1) uniform sampler2D palette_sampler;

layout(location = 2) uniform int palette_size;
layout(location = 3) uniform int background_color_index;

const int MAX_COLORS = 256;

void main(void) {
    float color_offset = 1.0 / palette_size;

    float color_idx = texture2D(texture_sampler, frag_texture_coords).x;
    if (color_idx <= 0.0) {
        color_idx = background_color_index;
    } else {
        color_idx = color_idx * MAX_COLORS;
    }

    float palette_color = (color_idx - 1) * color_offset;
    color = texture(palette_sampler, vec2(palette_color, 0.0));
}