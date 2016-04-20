extern crate rand;
extern crate tcod;

use self::rand::Rng;
use self::tcod::input::KeyCode;
use util::{Point, Bound, Contains};
use game::Game;

pub trait MovementComponent {
    fn new(Bound) -> Self where Self: Sized;
    fn update(&self, Point) -> Point;
}

pub struct MovementComponentRandom {
    window_bound: Bound,
}

impl MovementComponent for MovementComponentRandom {
    fn new(bound: Bound) -> MovementComponentRandom {
        MovementComponentRandom { window_bound: bound }
    }

    fn update(&self, point: Point) -> Point {
        let mut offset = Point{x:point.x, y: point.y};

        let offset_x =rand::thread_rng().gen_range(0, 3) - 1;
        match self.window_bound.contains(offset.offset_x(offset_x)) {
            Contains::DoesNotContain => {return point;},
            Contains::DoesContain => offset = offset.offset_x(offset_x)
        }

        let offset_y =rand::thread_rng().gen_range(0, 3) - 1;
        match self.window_bound.contains(offset.offset_y(offset_y)) {
            Contains::DoesNotContain => {return point;},
            Contains::DoesContain => offset = offset.offset_y(offset_y)
        }
        offset
    }
}

pub struct MovementComponentUser {
    window_bound: Bound,
}

impl MovementComponent for MovementComponentUser {
    fn new(bound: Bound) -> MovementComponentUser {
        MovementComponentUser { window_bound: bound }
    }

    fn update(&self, point: Point) -> Point {
        let key = Game::get_last_keypress().unwrap();
        let mut offset = Point { x: point.x, y: point.y };

        offset = match key.code {
            KeyCode::Up => offset.offset_y(-1),
            KeyCode::Down => offset.offset_y(1),
            KeyCode::Left => offset.offset_x(-1),
            KeyCode::Right => offset.offset_x(1),
            _ => offset
        };

        match self.window_bound.contains(offset) {
            Contains::DoesContain => offset,
            Contains::DoesNotContain => point
        }
    }
}
