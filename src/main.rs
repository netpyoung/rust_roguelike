extern crate dwemthys;
extern crate tcod;

use dwemthys::game::Game;
use dwemthys::render::{RenderingComponentAble};
use dwemthys::character::{Character, NPC, Renderable};

use tcod::input::{KeyCode};


fn main() {
    let width = 80;
    let height = 50;

    let mut game = Game::new(width, height);

    let mut ch = Character::new(40, 24, '@');
    let mut dog = NPC::new(10, 10, 'd');
    let mut cat = NPC::new(40, 25, 'c');
    let mut renderables: Vec<&mut Renderable> = vec![&mut dog, &mut cat];


    game.render(&ch, &renderables);
    while !(game.is_renderable() || game.is_exit) {

        let key = game.wait_for_keypress();
        match key.code {
            KeyCode::Escape => game.is_exit = true,
            _ => {}
        }
        game.update(&mut ch, &mut renderables, key);
        game.render(&ch, &renderables);
    }
}
