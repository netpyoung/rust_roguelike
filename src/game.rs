extern crate tcod;

use self::tcod::input::{Key};
use util::{Bound, Point};
use actor::{Actor};
use render::{RenderingComponent, RenderingComponentAble};


static mut LAST_KEYPRESS: Option<Key> = None;
static mut CHAR_LOCATION: Point = Point{x: 40, y: 25};

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
        let renderer = Box::new(RenderingComponent::new(bound));

        return Game {
            is_exit: false,
            window_bounds: bound,
            renderer: renderer
        }
    }

    pub fn render(&mut self, c: &Actor, npcs: &Vec<Box<Actor>>) {
        self.renderer.before_render_new_frame();
        for i in npcs.iter() {
            i.render(&mut *self.renderer);
        }
        c.render(&mut *self.renderer);
        self.renderer.after_render_new_frame();
    }

    pub fn update(&mut self, c: &mut Actor, npcs: &mut Vec<Box<Actor>>) {
        c.update();
        Game::set_character_point(c.position);
        for i in npcs.iter_mut() {
            i.update();
        }
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        let key = self.renderer.wait_for_keypress();
        Game::set_last_keypress(key);
        key
    }

    pub fn is_renderable(&mut self) -> bool {
        self.renderer.is_renderable()
    }

    pub fn get_last_keypress() -> Option<Key> {
        unsafe { LAST_KEYPRESS }
    }

    pub fn set_last_keypress(key: Key) {
        unsafe { LAST_KEYPRESS = Some(key); }
    }

    pub fn get_character_point() -> Point {
        unsafe { CHAR_LOCATION }
    }

    pub fn set_character_point(p: Point) {
        unsafe { CHAR_LOCATION = p }
    }
}
