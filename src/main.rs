extern crate dwemthys;
extern crate tcod;

use dwemthys::game::Game;
use dwemthys::render::{RenderingComponent, RenderingComponentAble};
use dwemthys::util::{Bound, Point};
use dwemthys::character::{Character, NPC, Renderable};

use tcod::console::{Root, FontLayout, FontType};
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
    let root = Root::initializer()
                       .font("arial10x10.png", FontLayout::Tcod)
                       .font_type(FontType::Greyscale)
                       .size(width, height)
                       .title("Rust/libtcod tutorial")
                       .init();

    let mut rendering_component = RenderingComponent::new(root);
    // let mut rendering_component = RenderingComponent{root: root};

    let mut ch = Character::new(40, 24, '@');
    let mut dog = NPC::new(10, 10, 'd');
    let mut cat = NPC::new(40, 25, 'c');
    let mut renderables: Vec<&mut Renderable> = vec![&mut dog, &mut cat];


    render(&mut rendering_component, &ch, &renderables);
    while !(rendering_component.is_renderable() || game.is_exit) {

        let key = rendering_component.wait_for_keypress();
        match key.code {
            KeyCode::Escape => game.is_exit = true,
            _ => {}
        }
        update(&mut ch, &mut renderables, key, &game);
        render(&mut rendering_component, &ch, &renderables);
    }
}

fn render(render: &mut RenderingComponent, ch: &Character, npcs: &Vec<&mut Renderable>) {
    render.before_render_new_frame();
    for npc in npcs.iter() {
        npc.render(render);
    }
    ch.render(render);
    render.after_render_new_frame();
}

fn update(ch: &mut Character, npcs: &mut Vec<&mut Renderable>, key: Key, game: &Game) {
    ch.update(key, &game);
    for npc in npcs.iter_mut() {
        npc.update(&game);
    }
}
