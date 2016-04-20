extern crate rand;
extern crate tcod;

use util::{Point, Bound};
use render::{RenderingComponentAble};
use movement::MovementComponent;
use movement::MovementComponentRandom;
use movement::MovementComponentUser;


pub struct Actor {
    position: Point,
    display_char: char,
    movement_component: Box<MovementComponent>,
}

impl Actor {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent>) -> Actor {
        Actor {
            position: Point { x: x, y: y },
            display_char: dc,
            movement_component: mc,
        }
    }

    pub fn dog(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<MovementComponentRandom> = Box::new(MovementComponent::new(bound));
        Actor::new(x, y, 'd', mc)
    }

    pub fn cat(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<MovementComponentRandom> = Box::new(MovementComponent::new(bound));
        Actor::new(x, y, 'c', mc)
    }

    pub fn heroine(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<MovementComponentUser> = Box::new(MovementComponent::new(bound));
        Actor::new(x, y, '@', mc)
    }

    pub fn update(&mut self) {
        self.position = self.movement_component.update(self.position);
    }

    pub fn render(&self, rendering_component: &mut RenderingComponentAble) {
        rendering_component.render_object(self.position, self.display_char);
    }
}
