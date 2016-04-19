extern crate dwemthys;
extern crate tcod;

use dwemthys::game::Game;
use dwemthys::util::{Bound, Point};
use dwemthys::character::{Character, NPC, Renderable};

use tcod::console::{Root, Console, FontLayout, FontType};
use tcod::input::{Key, KeyCode};


fn main() {
    let mut game = Game {
        is_exit: false,
        window_bounds: Bound {
            min: Point { x: 0, y: 0 },
            max: Point { x: 79, y: 49 },
        },
    };

    let width = game.window_bounds.max.x + 1;
    let height = game.window_bounds.max.y + 1;
    let mut root = Root::initializer()
                       .font("arial10x10.png", FontLayout::Tcod)
                       .font_type(FontType::Greyscale)
                       .size(width, height)
                       .title("Rust/libtcod tutorial")
                       .init();

    let ch = Box::new(Character::new(40, 20, '@'));
    let dog = Box::new(NPC::new(10, 10, 'd'));
    let mut renderables: Vec<Box<Renderable>> = vec![ch, dog];


    render(&mut root, &renderables);
    while !(root.window_closed() || game.is_exit) {
        let key = root.wait_for_keypress(true);
        match key.code {
            KeyCode::Escape => game.is_exit = true,
            _ => {}
        }
        update(&mut renderables, key, &game);
        render(&mut root, &renderables);
    }
}

fn render(root: &mut Root, renderables: &Vec<Box<Renderable>>) {
    root.clear();
    for renderable in renderables.iter() {
        renderable.render(root);
    }
    root.flush();
}

fn update(renderables: &mut Vec<Box<Renderable>>, key: Key, game: &Game) {
    for renderable in renderables.into_iter() {
        renderable.update(key, &game);
    }
}
