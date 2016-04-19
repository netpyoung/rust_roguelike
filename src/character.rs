extern crate rand;
extern crate tcod;

use game::Game;
use util::{Point, Contains};
use render::{RenderingComponentAble};
use self::rand::Rng;
use self::tcod::input::{Key, KeyCode};




#[derive(Debug, Copy, Clone)]
pub struct Character {
    position: Point,
    display_char: char,
}

impl Character {
    pub fn new(x: i32, y: i32, dc: char) -> Character {
        Character {
            position: Point { x: x, y: y },
            display_char: dc,
        }
    }
}

pub struct NPC {
    position: Point,
    display_char: char,
}

impl NPC {
    pub fn new(x: i32, y: i32, dc: char) -> NPC {
        NPC {
            position: Point { x: x, y: y },
            display_char: dc,
        }
    }
}

pub trait Renderable {
    fn update(&mut self, &Game);
    fn render(&self, &mut RenderingComponentAble);
}

impl Character {
    pub fn update(&mut self, key: Key, game: &Game) {
        let mut offset = Point { x: 0, y: 0 };
        match key.code {
            KeyCode::Up => offset.y = -1,
            KeyCode::Down => offset.y = 1,
            KeyCode::Left => offset.x = -1,
            KeyCode::Right => offset.x = 1,
            _ => {}
        }
        match game.window_bounds.contains(self.position.offset(offset)) {
            Contains::DoesContain => self.position = self.position.offset(offset),
            Contains::DoesNotContain => {}
        }
    }

    pub fn render(&self, rendering_component: &mut RenderingComponentAble) {
        rendering_component.render_object(self.position, self.display_char);
    }
}


impl Renderable for NPC {
    fn update(&mut self, game: &Game) {
        let offset_x = rand::thread_rng().gen_range(0, 3) - 1;
        match game.window_bounds.contains(self.position.offset_x(offset_x)) {
            Contains::DoesContain => self.position = self.position.offset_x(offset_x),
            Contains::DoesNotContain => {}
        }

        let offset_y = rand::thread_rng().gen_range(0, 3) - 1;
        match game.window_bounds.contains(self.position.offset_y(offset_y)) {
            Contains::DoesContain => self.position = self.position.offset_y(offset_y),
            Contains::DoesNotContain => {}
        }
    }

    fn render(&self, rendering_component: &mut RenderingComponentAble) {
        rendering_component.render_object(self.position, self.display_char);
    }
}
