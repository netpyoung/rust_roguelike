extern crate tcod;


use self::tcod::console::{Root, Console, BackgroundFlag, FontLayout, FontType};
use self::tcod::input::{Key};
use util::{Point, Bound};


pub struct RenderingComponent {
    pub root: Root,
}

pub trait RenderingComponentAble {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn is_renderable(&mut self) -> bool;
}

impl RenderingComponent {
    pub fn new(bound: Bound) ->  Self {
        let w = bound.max.x + 1;
        let h = bound.max.y + 1;
        let root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(w, h)
            .title("Rust/libtcod tutorial")
            .init();
        RenderingComponent{root: root}
    }
}

impl RenderingComponentAble for RenderingComponent {

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
