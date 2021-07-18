# Flask
Simple framework for 2D game development written in pure Rust. Heavily inspired by Pico-8

## Example "game"

main.rs
```rust
mod game;

use crate::game::Game;

use flask::game_context::GameContext;
use flask::color::Color;

fn main() {
    let game_context = GameContext::new();

    let game_scene = Box::new(Game::new());

    // Pixel buffer is of size 460 x 256
    match game_context.run(460, 256, "game", get_darkseed_16_palette(), game_scene) {
        Ok(_) => { }
        Err(error) => println!("{}", error)
    };
}

pub fn get_darkseed_16_palette() -> Vec<Color> {
    let mut palette = vec![];

    palette.push(Color::from_hex(0x000000));
    palette.push(Color::from_hex(0x001418));
    palette.push(Color::from_hex(0x002024));
    palette.push(Color::from_hex(0x002c38));
    palette.push(Color::from_hex(0x143444));
    palette.push(Color::from_hex(0x443444));
    palette.push(Color::from_hex(0x583c48));
    palette.push(Color::from_hex(0x6c4c44));

    palette.push(Color::from_hex(0x806058));
    palette.push(Color::from_hex(0x6c706c));
    palette.push(Color::from_hex(0x888078));
    palette.push(Color::from_hex(0xa49484));
    palette.push(Color::from_hex(0xc4ac9c));
    palette.push(Color::from_hex(0xd8b0a8));
    palette.push(Color::from_hex(0xecd4d0));
    palette.push(Color::from_hex(0xfcfcfc));

    palette
}

```

game.rs

```rust
use flask::scene::Scene;
use flask::input::{Input, Key, State};
use flask::renderer::Renderer;
use flask::sprite::Sprite;
use flask::font::Font;

pub struct Game {
    color: u8,
    char_x : i64,
    char_y : i64,
    flip: bool,
    char_sprite: Sprite,
    font: Font
}

impl Game {
    pub fn new() -> Game {
        // Engine supports only indexed color sprites
        let char_sprite_bytes = include_bytes!("assets/test_char.png");
        let char_sprite = Sprite::from_indexed_8bit_png(char_sprite_bytes).unwrap();

        Game { color: 0, char_x: 64, char_y: 64, flip: false, char_sprite, font: Font::load_3x5().unwrap() }
    }
}

impl Scene for Game {
    fn on_start(&mut self) {

    }

    fn on_update(&mut self, renderer: &mut Renderer, input: &Input, _delta_time: f64) -> Option<Box<dyn Scene>> {
        // Reset background with color indexed 5
        renderer.set_background_color(5);

        // Update character movement
        // In normal circumstances you should also use delta time which is also available
        // as a parameter of on_update function
        if input.get_key_state(Key::D) == State::Down {
            self.char_x += 1;
            self.flip = true;
        } else if input.get_key_state(Key::A) == State::Down {
            self.char_x -= 1;
            self.flip = false;
        }

        if input.get_key_state(Key::W) == State::Down {
            self.char_y += 1;
        } else if input.get_key_state(Key::S) == State::Down {
            self.char_y -= 1;
        }

        // Center camera on character position
        renderer.set_camera_x(self.char_x + self.char_sprite.get_width() as i64 / 2);
        renderer.set_camera_y(self.char_y + self.char_sprite.get_height() as i64 / 2);

        // Draw primitives
        renderer.circle(64, 100, 2, 6);
        renderer.line(50, 50, 0, 0, 7);
        renderer.rectangle(0, 0, 50, 50, 8);
        renderer.rectangle_filled(10, 10, 30, 30, 8);

        // Draw some test text
        let text = String::from("Aa Bb Cc Dd Ee Ff Gg Hh Ii Jj Kk Ll Mm Nn Oo Pp Qq Rr Ss Tt Uu Vv Ww Xx Yy Zz \n0123456789\n~`!?,.<>/|!@#$%^&*()_+-=\'\"");
        renderer.text(&text, &self.font, 30, 30, 10);

        // Render our character sprite
        renderer.sprite(&self.char_sprite, self.char_x, self.char_y, self.flip);

        // Return other boxed scene trait object to load new scene.
        // In this case we don't load any new scenes so we return None
        None
    }

    fn on_destroy(&mut self) {

    }
}
```

## License

Flask is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.