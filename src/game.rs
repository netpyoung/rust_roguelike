extern crate tcod;

use std::rc::Rc;
use std::cell::RefCell;

use input::{Key, KeyboardInput};
use util::{Bound, Point};
use game_state::GameState;
use game_state::{
    MovementGameState,
    AttackInputGameState,
};
use rendering::render::{
    TcodRenderingComponent,
    RenderingComponentAble,
};
use rendering::window::{
    Windows,
    WindowComponent,
    TcodStatsWindowComponent,
    TcodInputWindowComponent,
    TcodMapWindowComponent,
    TcodMessageWindowComponent,
};
use map::{Maps};
use actor::{Actor};


pub struct Game <'a> {
    pub is_exit: bool,
    pub window_bounds: Bound,
    pub renderer: Box<RenderingComponentAble + 'a>,
    pub windows: Windows,
    pub game_state: Box<GameState>,
    pub maps: Maps,
    move_info: Rc<RefCell<MoveInfo>>,
}

impl<'a> Game <'a> {
    pub fn new() -> Game <'a>{
        let total_bound = Bound::new(0, 0, 99, 61);
        let stats_bound = Bound::new(79, 0, 99, 49);
        let map_bound = Bound::new(0, 0, 78, 49);
        let input_bound = Bound::new(0, 50, 99, 52);
        let message_bound = Bound::new(0, 52, 99, 61);

        let renderer = Box::new(TcodRenderingComponent::new(total_bound));
        let sw: Box<TcodStatsWindowComponent> = Box::new(WindowComponent::new(stats_bound));
        let iw: Box<TcodInputWindowComponent> = Box::new(WindowComponent::new(input_bound));
        let mw: Box<TcodMessageWindowComponent> = Box::new(WindowComponent::new(message_bound));
        let maw: Box<TcodMapWindowComponent> = Box::new(WindowComponent::new(map_bound));
        let windows = Windows {
            stats: sw,
            input: iw,
            map: maw,
            messages: mw,
        };

        let gs: Box<GameState>  = Box::new(MovementGameState::new());

        let move_info = Rc::new(RefCell::new(MoveInfo::new(map_bound)));
        let mut maps = Maps::new(move_info.clone());

        maps.pcs.push_actor(move_info.borrow().char_location, Box::new(Actor::heroine(move_info.clone())));
        maps.friends.push_actor(Point::new(10, 10), Box::new(Actor::cat(40, 25, move_info.clone())));
        maps.friends.push_actor(Point::new(10, 10), Box::new(Actor::dog(10, 10, move_info.clone())));
        maps.enemies.push_actor(Point::new(10, 10), Box::new(Actor::kobold(20, 20, move_info.clone())));

        return Game {
            is_exit: false,
            window_bounds: total_bound,
            renderer: renderer,
            windows: windows,
            game_state: gs,
            maps: maps,
            move_info: move_info,
        }
    }

    pub fn render(&mut self) {
        self.game_state.render(&mut *self.renderer, &mut self.windows, &mut self.maps);
    }

    pub fn update(&mut self) {
        if self.game_state.should_update_state() {
            self.game_state.exit();
            self.update_state();
            self.game_state.enter(&mut self.windows);
        }

        self.game_state.update(&mut self.windows, &mut self.maps, &mut self.move_info);
    }

    pub fn wait_for_keypress(&mut self) -> KeyboardInput {
        let keyboard_input = self.renderer.wait_for_keypress();
        self.move_info.borrow_mut().last_keypress = Some(keyboard_input);
        keyboard_input
    }

    pub fn is_renderable(&mut self) -> bool {
        self.renderer.is_renderable()
    }

    fn update_state(&mut self) {
        let keyboard_input = self.move_info.borrow().last_keypress.unwrap();
        match keyboard_input.key {

            Key::Printable('/') => {
                let mut is : Box<AttackInputGameState> = Box::new(GameState::new());
                is.weapon = "Heroic Sword".to_string();
                self.game_state = is as Box<GameState>;
            },
            Key::Printable('^') => {
                let mut is : Box<AttackInputGameState> = Box::new(GameState::new());
                is.weapon = "Boomerang".to_string();
                self.game_state = is as Box<GameState>;
            },
            Key::Printable('&') => {
                let mut is : Box<AttackInputGameState> = Box::new(GameState::new());
                is.weapon = "Deadly Bomb".to_string();
                self.game_state = is as Box<GameState>;
            },
            Key::Printable('%') => {
                let mut is : Box<AttackInputGameState> = Box::new(GameState::new());
                is.weapon = "Delicious Lettuce".to_string();
                self.game_state = is as Box<GameState>;
            },
            _ => {
                let ms : Box<MovementGameState> = Box::new(GameState::new());
                self.game_state = ms as Box<GameState>;
            }
        }
    }
}

pub struct MoveInfo {
    pub last_keypress: Option<KeyboardInput>,
    pub char_location: Point,
    pub bounds: Bound
}

impl MoveInfo {
    pub fn new(bound: Bound) -> MoveInfo {
        MoveInfo {
            last_keypress: None,
            char_location: Point::new(40, 25),
            bounds: bound
        }
    }
}
