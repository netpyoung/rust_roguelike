extern crate rand;
extern crate tcod;

use util::{Point, Bound};
use rendering::render::{RenderingComponentAble};
use game::Game;
use movement::MovementComponent;
use movement::MovementComponentRandom;
use movement::MovementComponentUser;
use movement::MovementComponentAggro;


pub struct Actor {
    pub position: Point,
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

    pub fn kobold(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<MovementComponentAggro> = Box::new(MovementComponent::new(bound));
        Actor::new(x, y, 'k', mc)
    }

    pub fn heroine(bound: Bound) -> Actor {
        let mc: Box<MovementComponentUser> = Box::new(MovementComponent::new(bound));
        let p = Game::get_character_point();
        Actor::new(p.x, p.y, '@', mc)
    }

    pub fn update(&mut self) {
        self.position = self.movement_component.update(self.position);
    }

    pub fn render(&self, rendering_component: &mut RenderingComponentAble) {
        rendering_component.render_object(self.position, self.display_char);
    }
}
