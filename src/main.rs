extern crate dwemthys;
extern crate tcod;

use dwemthys::game::Game;
use dwemthys::rendering::render::RenderingComponentAble;
use dwemthys::rendering::window::WindowComponent;
use dwemthys::movement::MovementComponent;
use dwemthys::input::{Key, KeyCode};


fn main() {
    let mut game = Game::new();

    game.render();
    while !(game.is_renderable() || game.is_exit) {

        let keyboard_input = game.wait_for_keypress();
        match keyboard_input.key {
            Key::SpecialKey(KeyCode::Escape) => game.is_exit = true,
            _ => {}
        }
        game.update();
        game.render();
    }
}
