extern crate tcod;


use self::tcod::console::{Root, Console, Offscreen, BackgroundFlag, FontLayout, FontType};
use self::tcod::Color;
use self::tcod::input::{Key};
use util::{Point, Bound};


pub struct TcodRenderingComponent {
    pub root: Root,
}

pub trait RenderingComponentAble {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
    fn is_renderable(&mut self) -> bool;

    fn attach_window(&mut self, &mut Box<WindowComponent>);
}


impl TcodRenderingComponent {
    pub fn new(bound: Bound) ->  Self {
        let w = bound.max.x + 1;
        let h = bound.max.y + 1;
        let root = Root::initializer()
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(w, h)
            .title("Rust/libtcod tutorial")
            .init();
        TcodRenderingComponent{root: root}
    }
}

impl RenderingComponentAble for TcodRenderingComponent {

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

    fn attach_window(&mut self, window: &mut Box<WindowComponent>) {
        window.clear();
        let bound = window.get_bounds();
        let console = window.get_console();
        let messages = window.get_messages();
        tcod::console::blit(&console, (0, 0), ((bound.max.x + 1), (bound.max.y + 1)), &mut self.root, (bound.min.x, bound.min.y), 1.0, 1.0);
    }
}

pub trait WindowComponent {
    fn new(Bound) -> Self where Self: Sized;
    fn get_bounds(&self) -> Bound;
    fn get_bg_color(&self) -> Color;
    fn get_console(&mut self) -> &Box<Console>;

    fn clear(&mut self) {
        let color = self.get_bg_color();
        let mut console = self.get_console();
        console.set_default_background(color);
        console.clear();
    }

    // messages
    fn print_message(&mut self, x: i32, y: i32, alignment: tcod::TextAlignment, text: &str) {
        let mut console = self.get_console();
        console.print_ex(x, y, BackgroundFlag::Set, alignment, text);
    }

    fn get_mut_messages(&mut self) -> &mut Vec<String>;
    fn get_messages(&self) -> &Vec<String>;
    fn get_max_messages(&self) -> usize;

    fn buffer_message(&mut self, text: &str) {
        let max = self.get_max_messages();
        let message = String::from(text);
        let messages = self.get_mut_messages();
        messages.insert(0, message);
        messages.truncate(max);
    }
}

pub struct TcodStatsWindowComponent {
    pub console: Box<Console>,
    pub background_color: Color,
    bound: Bound,

    messages: Vec<String>,
    max_messages: usize,
}

impl WindowComponent for TcodStatsWindowComponent {
    fn new(bound: Bound) -> TcodStatsWindowComponent {
        let w = bound.max.x - bound.min.x + 1;
        let h = bound.max.y - bound.min.y + 1;
        let color = Color::new(255, 0, 0);
        let console: Box<Console> = Box::new(Offscreen::new(w, h));
        let messages: Vec<String> = vec![];
        let max_messages = 32;

        TcodStatsWindowComponent {
            console: console,
            background_color: color,
            bound: bound,
            messages: messages,
            max_messages: max_messages,

        }
    }

    fn get_bounds(&self) -> Bound {
        self.bound
    }
    fn get_bg_color(&self) -> Color {
        self.background_color
    }
    fn get_console(&mut self) -> &Box<Console> {
        &self.console
    }

    fn get_mut_messages(&mut self) -> &mut Vec<String> {
        &mut self.messages
    }

    fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }

    fn get_max_messages(&self) -> usize{
        self.max_messages
    }

}

pub struct TcodInputWindowComponent {
    pub console: Box<Console>,
    pub background_color: Color,
    bound: Bound,

    messages: Vec<String>,
    max_messages: usize,
}

impl WindowComponent for TcodInputWindowComponent {
    fn new(bound: Bound) -> TcodInputWindowComponent {
        let w = bound.max.x - bound.min.x + 1;
        let h = bound.max.y - bound.min.y + 1;
        let color = Color::new(255, 0, 255);
        let console: Box<Console> = Box::new(Offscreen::new(w, h));
        let messages: Vec<String> = vec![];
        let max_messages = 32;

        TcodInputWindowComponent {
            console: console,
            background_color: color,
            bound: bound,
            messages: messages,
            max_messages: max_messages,
        }
    }

    fn get_bounds(&self) -> Bound {
        self.bound
    }
    fn get_bg_color(&self) -> Color {
        self.background_color
    }
    fn get_console(&mut self) -> &Box<Console> {
        &self.console
    }
        fn get_mut_messages(&mut self) -> &mut Vec<String> {
        &mut self.messages
    }

    fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }

    fn get_max_messages(&self) -> usize{
        self.max_messages
    }

}

struct TcodMessageWindowComponent;

pub struct TcodMapWindowComponent {
    pub console: Box<Console>,
    pub background_color: Color,
    bound: Bound,
    messages: Vec<String>,
    max_messages: usize,
}


impl WindowComponent for TcodMapWindowComponent {
    fn new(bound: Bound) -> TcodMapWindowComponent {
        let w = bound.max.x - bound.min.x + 1;
        let h = bound.max.y - bound.min.y + 1;
        let color = Color::new(255, 255, 255);
        let console: Box<Console> = Box::new(Offscreen::new(w, h));
        let messages: Vec<String> = vec![];
        let max_messages = 32;

        TcodMapWindowComponent {
            console: console,
            background_color: color,
            bound: bound,
            messages: messages,
            max_messages: max_messages,

        }
    }

    fn get_bounds(&self) -> Bound {
        self.bound
    }
    fn get_bg_color(&self) -> Color {
        self.background_color
    }
    fn get_console(&mut self) -> &Box<Console> {
        &self.console
    }

    fn get_mut_messages(&mut self) -> &mut Vec<String> {
        &mut self.messages
    }

    fn get_messages(&self) -> &Vec<String> {
        &self.messages
    }

    fn get_max_messages(&self) -> usize{
        self.max_messages
    }

}
