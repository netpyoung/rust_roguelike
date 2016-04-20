extern crate tcod;

use self::tcod::input::{Key};
use util::{Bound, Point};
use character::{Character, Renderable};
use render::{RenderingComponent, RenderingComponentAble};

pub struct Game <'a>{
    pub is_exit: bool,
    pub window_bounds: Bound,
    pub renderer: Box<RenderingComponentAble + 'a>,
}

impl<'a> Game <'a>{
    pub fn new(width: i32, height: i32) -> Game <'a>{
        let bound = Bound {
            min: Point { x: 0, y: 0 },
            max: Point { x: width - 1, y: height - 1 },
        };
        let renderer = Box::new(RenderingComponent::new(&bound));

        return Game {
            is_exit: false,
            window_bounds: bound,
            renderer: renderer
        }
    }

    pub fn render(&mut self, c: &Character, npcs: &Vec<&mut Renderable>) {
        self.renderer.before_render_new_frame();
        for i in npcs.iter() {
            i.render(&mut *self.renderer);
        }
        c.render(&mut *self.renderer);
        self.renderer.after_render_new_frame();
    }

    pub fn update(&mut self, c: &mut Character, npcs: &mut Vec<&mut Renderable>, keypress: Key) {
        c.update(keypress, self);
        for i in npcs.iter_mut() {
            i.update(&self);
        }
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        self.renderer.wait_for_keypress()
    }

    pub fn is_renderable(&mut self) -> bool {
        self.renderer.is_renderable()
    }
}
