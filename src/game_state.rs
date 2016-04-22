extern crate tcod;

use self::tcod::input::{KeyCode};

use actor::Actor;
use rendering::render::RenderingComponentAble;
use rendering::window::Windows;
use rendering::window::WindowComponent;
use game::Game;


pub trait GameState {
    fn new() -> Self where Self: Sized;

    fn update(&mut self, npcs: &mut Vec<Box<Actor>>, character: &mut Actor, windows: &mut Windows);
    fn render(&mut self, renderer: &mut RenderingComponentAble, npcs: &Vec<Box<Actor>>, character: &Actor, windows: &mut Windows) {
        renderer.before_render_new_frame();
        for window in windows.all_windows() {
            renderer.attach_window(window);
        }
        for npc in npcs {
            npc.render(renderer);
        }
        character.render(renderer);
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

    fn update(&mut self, npcs: &mut Vec<Box<Actor>>, character: &mut Actor, _: &mut Windows) {
        character.update();
        Game::set_character_point(character.position);
        for npc in npcs.iter_mut() {
            npc.update();
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

    fn update(&mut self,  _: &mut Vec<Box<Actor>>, _: &mut Actor, windows: &mut Windows) {

        match Game::get_last_keypress() {
            Some(key) => {
                let mut msg = "You attack ".to_string();

                match key.code {
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
