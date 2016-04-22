extern crate tcod;

use input::{Key, KeyboardInput};
use util::{Bound, Point};
use actor::{Actor};
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


static mut LAST_KEYPRESS: Option<KeyboardInput> = None;
static mut CHAR_LOCATION: Point = Point{x: 40, y: 25};

pub struct Game <'a> {
    pub is_exit: bool,
    pub window_bounds: Bound,
    pub renderer: Box<RenderingComponentAble + 'a>,
    pub windows: Windows,
    pub game_state: Box<GameState>,
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
        return Game {
            is_exit: false,
            window_bounds: total_bound,
            renderer: renderer,
            windows: windows,
            game_state: gs,
        }
    }

    pub fn render(&mut self, c: &Actor, npcs: &Vec<Box<Actor>>) {
        self.game_state.render(&mut *self.renderer, npcs, c, &mut self.windows);
    }

    pub fn update(&mut self, c: &mut Actor, npcs: &mut Vec<Box<Actor>>) {
        if self.game_state.should_update_state() {
            self.game_state.exit();
            self.update_state();
            self.game_state.enter(&mut self.windows);
        }

        self.game_state.update(npcs, c, &mut self.windows);
    }

    pub fn wait_for_keypress(&mut self) -> KeyboardInput {
        let key = self.renderer.wait_for_keypress();
        Game::set_last_keypress(key);
        key
    }

    pub fn is_renderable(&mut self) -> bool {
        self.renderer.is_renderable()
    }

    pub fn get_last_keypress() -> Option<KeyboardInput> {
        unsafe { LAST_KEYPRESS }
    }

    pub fn set_last_keypress(keyboard_input: KeyboardInput) {
        unsafe { LAST_KEYPRESS = Some(keyboard_input); }
    }

    pub fn get_character_point() -> Point {
        unsafe { CHAR_LOCATION }
    }

    pub fn set_character_point(p: Point) {
        unsafe { CHAR_LOCATION = p }
    }

    fn update_state(&mut self) {
        let keyboard_input = Game::get_last_keypress().unwrap();
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
