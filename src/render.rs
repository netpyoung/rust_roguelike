extern crate tcod;

use util::{Point};
use self::tcod::console::{Root, Console, BackgroundFlag};
use self::tcod::input::{Key};


pub struct RenderingComponent {
    pub root: Root,
}

pub trait RenderingComponentAble {
    fn new(Root) -> Self where Self: Sized;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn is_renderable(&mut self) -> bool;
}

impl RenderingComponentAble for RenderingComponent {
    fn new(root: Root) -> Self {
        RenderingComponent{root: root}
    }

    fn before_render_new_frame(&mut self) {
        self.root.clear();
    }

    fn render_object(&mut self, position: Point, symbol: char) {
        self.root.put_char(position.x, position.y, symbol, BackgroundFlag::Set);
    }

    fn after_render_new_frame(&mut self) {
        self.root.flush();
    }

    fn wait_for_keypress(&mut self) -> Key {
        self.root.wait_for_keypress(true)
    }

    fn is_renderable(&mut self) -> bool {
        self.root.window_closed()
    }
}
