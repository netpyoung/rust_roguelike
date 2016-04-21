extern crate tcod;

use self::tcod::console::{Console, Offscreen, BackgroundFlag};
use self::tcod::Color;
use util::{Bound};

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
    fn get_messages(&self) -> Vec<String>;
    fn get_max_messages(&self) -> usize;

    fn buffer_message(&mut self, text: &str) {
        let max = self.get_max_messages();
        let message = String::from(text);
        let messages = self.get_mut_messages();
        messages.insert(0, message);
        messages.truncate(max);
    }

    fn flush_buffer(&mut self) {
        let max      = self.get_max_messages();
        let messages = self.get_mut_messages();

        for _ in 0..max {
            messages.insert(0, String::from(""));
        }
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

    fn get_messages(&self) -> Vec<String> {
        self.messages.clone()
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
        let max_messages = 1;

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

    fn get_messages(&self) -> Vec<String> {
        self.messages.clone()
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

    fn get_messages(&self) -> Vec<String> {
        self.messages.clone()
    }

    fn get_max_messages(&self) -> usize{
        self.max_messages
    }

}
