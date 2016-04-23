extern crate rand;
extern crate tcod;


use std::rc::Rc;
use std::cell::RefCell;

use self::rand::Rng;
use input::{Key, KeyCode};
use util::{Point, Contains, PointRelationX, PointRelationY, PointEquality};
use game::MoveInfo;


pub trait MovementComponent {
    fn new(move_info: Rc<RefCell<MoveInfo>>) -> Self where Self: Sized;
    fn update(&self, Point) -> Point;
    fn box_clone(&self) -> Box<MovementComponent>;
}

pub struct MovementComponentRandom {
    move_info: Rc<RefCell<MoveInfo>>,
}

impl MovementComponent for MovementComponentRandom {
    fn new(move_info: Rc<RefCell<MoveInfo>>) -> MovementComponentRandom {
        MovementComponentRandom { move_info: move_info }
    }

    fn update(&self, point: Point) -> Point {
        let mut offset = Point{x:point.x, y: point.y};

        let offset_x =rand::thread_rng().gen_range(0, 3) - 1;
        match self.move_info.borrow().bounds.contains(offset.offset_x(offset_x)) {
            Contains::DoesNotContain => {return point;},
            Contains::DoesContain => offset = offset.offset_x(offset_x)
        }

        let offset_y =rand::thread_rng().gen_range(0, 3) - 1;
        match self.move_info.borrow().bounds.contains(offset.offset_y(offset_y)) {
            Contains::DoesNotContain => {return point;},
            Contains::DoesContain => offset = offset.offset_y(offset_y)
        }
        offset
    }

    fn box_clone(&self) -> Box<MovementComponent> {
        Box::new(MovementComponentRandom{move_info: self.move_info.clone()})
    }
}

pub struct MovementComponentUser {
    move_info: Rc<RefCell<MoveInfo>>,
}

impl MovementComponent for MovementComponentUser {
    fn new(move_info: Rc<RefCell<MoveInfo>>) -> MovementComponentUser {
        MovementComponentUser { move_info: move_info }
    }

    fn update(&self, point: Point) -> Point {
        let keypress = self.move_info.borrow().last_keypress;
        let keyboard_input = keypress.unwrap();
        let mut offset = Point { x: point.x, y: point.y };

        offset = match keyboard_input.key {
            Key::SpecialKey(special) => {
                match special {
                    KeyCode::Up => offset.offset_y(-1),
                    KeyCode::Down => offset.offset_y(1),
                    KeyCode::Left => offset.offset_x(-1),
                    KeyCode::Right => offset.offset_x(1),
                    _ => offset
                }
            },
            _ => offset
        };

        match self.move_info.borrow().bounds.contains(offset) {
            Contains::DoesContain => offset,
            Contains::DoesNotContain => point
        }
    }

    fn box_clone(&self) -> Box<MovementComponent> {
        Box::new(MovementComponentUser{move_info: self.move_info.clone()})
    }
}

pub struct MovementComponentAggro {
    move_info: Rc<RefCell<MoveInfo>>
}

impl MovementComponent for MovementComponentAggro {
    fn new(move_info: Rc<RefCell<MoveInfo>>) -> MovementComponentAggro {
        MovementComponentAggro { move_info: move_info }
    }

    fn update(&self, point: Point) -> Point {
        let char_point = self.move_info.borrow().char_location;

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
                match self.move_info.borrow().bounds.contains(point.offset(offset)) {
                    Contains::DoesContain => point.offset(offset),
                    Contains::DoesNotContain => point
                }
            }
        }
    }

    fn box_clone(&self) -> Box<MovementComponent> {
        Box::new(MovementComponentAggro{move_info: self.move_info.clone()})
    }
}
