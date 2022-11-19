#version 300 es

precision mediump float;

in vec2 frag_texture_coords;

out vec4 color;

uniform sampler2D texture_sampler;
uniform sampler2D palette_sampler;

uniform int palette_size;
uniform int background_color_index;

const float MAX_COLORS = 256.0;

void main(void) {
    float color_offset = 1.0 / float(palette_size);

    float color_idx = texture(texture_sampler, frag_texture_coords).x;
    if (color_idx <= 0.0) {
        color_idx = float(background_color_index);
    } else {
        color_idx = color_idx * MAX_COLORS;
    }

    float palette_color = (color_idx - 1.0) * color_offset;
    color = texture(palette_sampler, vec2(palette_color, 0.0));
}