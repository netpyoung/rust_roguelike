extern crate tcod;

use std::rc::Rc;
use std::cell::RefCell;

use input::{Key, KeyCode};

use rendering::render::RenderingComponentAble;
use rendering::window::Windows;
use rendering::window::WindowComponent;
use game::MoveInfo;
use map::Maps;

pub trait GameState {
    fn new() -> Self where Self: Sized;

    fn update(&mut self, windows: &mut Windows, maps: &mut Maps, move_info: &mut Rc<RefCell<MoveInfo>>);
    fn render(&mut self, renderer: &mut RenderingComponentAble, windows: &mut Windows, maps: &mut Maps) {
        renderer.before_render_new_frame();
        for window in windows.all_windows() {
            renderer.attach_window(window);
        }
        maps.render(renderer);
        renderer.after_render_new_frame();
    }

    fn enter(&self, windows: &mut Windows);
    fn exit(&self);
    fn should_update_state(&self) -> bool;
}


pub struct MovementGameState;
impl GameState for MovementGameState {
    fn new() -> MovementGameState {
        MovementGameState
    }

    fn update(&mut self, windows: &mut Windows, maps: &mut Maps, move_info: &mut Rc<RefCell<MoveInfo>>) {
        let keypress = move_info.borrow().last_keypress;
        match keypress {
            Some(ks) => {
                match ks.key {
                    Key::SpecialKey(KeyCode::Shift) => {
                    },
                    _ => {
                        maps.update(windows);
                    }
                }
            },
            _    => {}
        }
    }

    fn enter(&self, _: &mut Windows) {
    }

    fn exit(&self) {
    }

    fn should_update_state(&self) -> bool {
        true
    }
}

pub struct AttackInputGameState {
    should_update_state: bool,
    pub weapon: String
}

impl GameState for AttackInputGameState {
    fn new() -> AttackInputGameState {
        AttackInputGameState {
            should_update_state: false,
            weapon: "".to_string()
        }
    }

    fn should_update_state(&self) -> bool {
        self.should_update_state
    }

    fn enter(&self, windows: &mut Windows) {
        windows.input.flush_buffer();
        let mut msg = "Which direction do you want to attack with ".to_string();
        msg.push_str(self.weapon.as_str());
        msg.push_str("? [Use the arrow keys to answer]");
        windows.input.buffer_message(msg.as_str())
    }

    fn exit(&self) {
    }

    fn update(&mut self,  windows: &mut Windows, _: &mut Maps, move_info: &mut Rc<RefCell<MoveInfo>>) {
        let keypress = move_info.borrow().last_keypress;

        match keypress {
            Some(keyboard_input) => {
                let mut msg = "You attack ".to_string();

                match keyboard_input.key {
                    Key::SpecialKey(special) => {
                        match special {
                            KeyCode::Up => {
                                msg.push_str("up");
                                self.should_update_state = true;
                            },
                            KeyCode::Down => {
                                msg.push_str("down");
                                self.should_update_state = true;
                            },
                            KeyCode::Left => {
                                msg.push_str("left");
                                self.should_update_state = true;
                            },
                            KeyCode::Right => {
                                msg.push_str("right");
                                self.should_update_state = true;
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }

                if self.should_update_state {
                    msg.push_str(" with your ");
                    msg.push_str(self.weapon.as_str());
                    msg.push_str("!");
                    windows.messages.buffer_message(msg.as_str());
                }
            },
            _ => {}
        }
    }
}
