extern crate dwemthys;
extern crate tcod;

use dwemthys::game::Game;
use dwemthys::rendering::render::{RenderingComponentAble};
use dwemthys::rendering::window::WindowComponent;
use dwemthys::actor::{Actor};
use dwemthys::movement::{MovementComponent};


use tcod::input::{KeyCode};


fn main() {
    let mut game = Game::new();

    let map_bounds = game.windows.map.get_bounds();
    let mut ch = Actor::heroine(map_bounds);
    let mut renderables: Vec<Box<Actor>> = vec![
        Box::new(Actor::dog(10, 10, map_bounds)),
        Box::new(Actor::cat(40, 25, map_bounds)),
        Box::new(Actor::kobold(20, 20, map_bounds)),
    ];

    game.render(&ch, &renderables);
    while !(game.is_renderable() || game.is_exit) {

        let key = game.wait_for_keypress();
        match key.code {
            KeyCode::Escape => game.is_exit = true,
            _ => {}
        }
        game.update(&mut ch, &mut renderables);
        game.render(&ch, &renderables);
    }
}
