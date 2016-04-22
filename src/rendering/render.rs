extern crate tcod;


use self::tcod::console::{Root, Console, BackgroundFlag, FontLayout, FontType};
use input::{TcodInputKey, InputComponent, TcodInputComponent, KeyboardInput};
use util::{Point, Bound};
use rendering::window::{
    WindowComponent,
};

pub struct TcodRenderingComponent {
    pub root: Root,
    pub input_component: Box<InputComponent<TcodInputKey>>
}

pub trait RenderingComponentAble {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> KeyboardInput;
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
        let ic  = Box::new(TcodInputComponent::new());
        TcodRenderingComponent{
            root: root,
            input_component: ic,
        }
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

    fn wait_for_keypress(&mut self) -> KeyboardInput {
        self.input_component.translate_input(self.root.wait_for_keypress(true))
    }

    fn is_renderable(&mut self) -> bool {
        self.root.window_closed()
    }

    fn attach_window(&mut self, window: &mut Box<WindowComponent>) {
        window.clear();
        let bound = window.get_bounds();
        let messages = window.get_messages();

        for (i, message) in messages.iter().enumerate() {
            window.print_message(0, i as i32, tcod::TextAlignment::Left, message);
        }

        {
            let console = window.get_console();

            tcod::console::blit(&console, (0, 0), ((bound.max.x + 1), (bound.max.y + 1)), &mut self.root, (bound.min.x, bound.min.y), 1.0, 1.0);
        }
    }
}
