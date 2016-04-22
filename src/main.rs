extern crate dwemthys;
extern crate tcod;

use dwemthys::util::Point;
use dwemthys::game::Game;
use dwemthys::rendering::render::{RenderingComponentAble};
use dwemthys::rendering::window::WindowComponent;
use dwemthys::actor::{Actor};
use dwemthys::movement::{MovementComponent};
use dwemthys::input::{Key, KeyCode};


fn main() {
    let mut game = Game::new();

    let map_bounds = game.windows.map.get_bounds();

    game.maps.friends.push_actor(Game::get_character_point(), Box::new(Actor::heroine(map_bounds)));
    game.maps.friends.push_actor(Point::new(10, 10), Box::new(Actor::cat(40, 25, map_bounds)));
    game.maps.enemies.push_actor(Point::new(10, 10), Box::new(Actor::dog(10, 10, map_bounds)));
    game.maps.enemies.push_actor(Point::new(10, 10), Box::new(Actor::kobold(20, 20, map_bounds)));

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
