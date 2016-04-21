extern crate tcod;

use self::tcod::input::{Key};
use util::{Bound, Point};
use actor::{Actor};
use game_state::GameState;
use game_state::MovementGameState;
use rendering::render::{
    TcodRenderingComponent,
    RenderingComponentAble,
};
use rendering::window::{
    WindowComponent,
    TcodStatsWindowComponent,
    TcodInputWindowComponent,
    TcodMapWindowComponent,
    TcodMessageWindowComponent,
};




static mut LAST_KEYPRESS: Option<Key> = None;
static mut CHAR_LOCATION: Point = Point{x: 40, y: 25};

pub struct Game <'a>{
    pub is_exit: bool,
    pub window_bounds: Bound,
    pub renderer: Box<RenderingComponentAble + 'a>,
    pub stats_window: Box<WindowComponent>,
    pub input_window: Box<WindowComponent>,
    pub map_window: Box<WindowComponent>,
    pub message_window: Box<WindowComponent>,
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

        let gs: Box<GameState>  = Box::new(MovementGameState::new());
        return Game {
            is_exit: false,
            window_bounds: total_bound,
            renderer: renderer,
            stats_window: sw,
            input_window: iw,
            map_window: maw,
            message_window: mw,
            game_state: gs,
        }
    }

    pub fn render(&mut self, c: &Actor, npcs: &Vec<Box<Actor>>) {
        let mut windows = vec![
            &mut self.stats_window,
            &mut self.map_window,
            &mut self.input_window,
            &mut self.message_window,
        ];
        self.game_state.render(&mut *self.renderer, npcs, c, &mut windows);
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
        match key.printable {
            '/' => self.input_window.buffer_message("Wich direction would you like to attack with your heoric sword? [Press an arrow key]"),
            _  => self.input_window.flush_buffer()
        }


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
