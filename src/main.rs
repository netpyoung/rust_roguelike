extern crate rand;
extern crate tcod;

use rand::Rng;
use tcod::console::{Root, Console, FontLayout, FontType, BackgroundFlag};
use tcod::input::{Key, KeyCode};

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32
}

struct Bound {
    min: Point,
    max: Point,
}

impl Point {
    fn offset_x(&self, offset: i32) -> Point {
        Point{x: self.x + offset, y: self.y}
    }
    fn offset_y(&self, offset: i32) -> Point {
        Point{x: self.x, y: self.y + offset}
    }
    fn offset(&self, offset: Point) -> Point {
        Point{x: self.x + offset.x, y: self.y + offset.y}
    }
}

enum Contains {
    DoesContain,
    DoesNotContain
}

impl Bound {
    fn contains(&self, point: Point) -> Contains {
        if point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
        {
            Contains::DoesContain
        } else {
            Contains::DoesNotContain
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Character {
    position:     Point,
    display_char: char
}

impl Character {
    fn new(x: i32, y: i32, dc: char) -> Character {
        Character{
            position: Point { x: x, y: y },
            display_char: dc
        }
    }
}

struct NPC {
    position:     Point,
    display_char: char
}

impl NPC {
    fn new(x: i32, y: i32, dc: char) -> NPC {
        NPC{
            position: Point { x: x, y: y },
            display_char: dc
        }
    }
}

struct Game {
    is_exit: bool,
    window_bounds: Bound
}

trait Renderable {
    fn update(&mut self, Key, &Game);
    fn render(&self, &mut Root);
}

impl Renderable for Character {
    fn update(&mut self, key: Key, game: &Game) {
        let mut offset = Point { x: 0, y: 0 };
        match key.code {
            KeyCode::Up     => offset.y = -1,
            KeyCode::Down   => offset.y =  1,
            KeyCode::Left   => offset.x = -1,
            KeyCode::Right  => offset.x =  1,
            _ => {}
        }
        match game.window_bounds.contains(self.position.offset(offset)) {
            Contains::DoesContain    => self.position = self.position.offset(offset),
            Contains::DoesNotContain => {}
        }
    }

    fn render(&self, root: &mut Root) {
        root.put_char(self.position.x, self.position.y, self.display_char, BackgroundFlag::Set);
    }
}


impl Renderable for NPC {
    fn update(&mut self, _: Key, game: &Game) {
        let offset_x = rand::thread_rng().gen_range(0, 3) - 1;
        match game.window_bounds.contains(self.position.offset_x(offset_x)) {
            Contains::DoesContain    => self.position = self.position.offset_x(offset_x),
            Contains::DoesNotContain => {}
        }

        let offset_y = rand::thread_rng().gen_range(0, 3) - 1;
        match game.window_bounds.contains(self.position.offset_y(offset_y)) {
            Contains::DoesContain    => self.position = self.position.offset_y(offset_y),
            Contains::DoesNotContain => {}
        }
    }

    fn render(&self, root: &mut Root) {
        root.put_char(self.position.x, self.position.y, self.display_char, BackgroundFlag::Set);
    }
}


fn main() {
    let mut game = Game {
        is_exit: false,
        window_bounds: Bound {
            min: Point { x: 0, y: 0 },
            max: Point { x: 79, y: 49 }
        }
    };

    let width = game.window_bounds.max.x + 1;
    let height = game.window_bounds.max.y + 1;
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(width, height)
        .title("Rust/libtcod tutorial")
        .init();

    let ch  = Box::new(Character::new(40, 20, '@'));
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
