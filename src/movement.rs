extern crate rand;
extern crate tcod;

use self::rand::Rng;
use self::tcod::input::KeyCode;
use util::{Point, Bound, Contains, PointRelationX, PointRelationY, PointEquality};
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

pub struct MovementComponentAggro {
    window_bound: Bound
}

impl MovementComponent for MovementComponentAggro {
    fn new(bound: Bound) -> MovementComponentAggro {
        MovementComponentAggro { window_bound: bound }
    }

    fn update(&self, point: Point) -> Point {
        let char_point = Game::get_character_point();

        let mut offset = Point{x: 0, y: 0};
        match point.compare_x(char_point) {
            PointRelationX::RightOfPoint => offset = offset.offset_x(-1),
            PointRelationX::LeftOfPoint => offset = offset.offset_x(1),
            PointRelationX::OnPointX => {}
        }
        match point.compare_y(char_point) {
            PointRelationY::BelowPoint => offset = offset.offset_y(-1),
            PointRelationY::AbovePoint => offset = offset.offset_y(1),
            PointRelationY::OnPointY => {}
        }
        match point.offset(offset).compare(char_point) {
            PointEquality::Equal => {return point; },
            PointEquality::NotEqual => {
                match self.window_bound.contains(point.offset(offset)) {
                    Contains::DoesContain => point.offset(offset),
                    Contains::DoesNotContain => point
                }
            }
        }
    }
}