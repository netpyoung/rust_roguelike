extern crate dwemthys;
extern crate tcod;

use dwemthys::game::Game;
use dwemthys::render::{RenderingComponentAble};
use dwemthys::character::{Actor};
use dwemthys::movement::{MovementComponent};

use tcod::input::{KeyCode};


fn main() {
    let width = 80;
    let height = 50;

    let mut game = Game::new(width, height);

    let mut ch = Actor::heroine(40, 24, game.window_bounds);
    let mut renderables: Vec<Box<Actor>> = vec![
        Box::new(Actor::dog(10, 10, game.window_bounds)),
        Box::new(Actor::cat(40, 25, game.window_bounds)),
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
