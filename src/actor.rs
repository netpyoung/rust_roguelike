extern crate rand;
extern crate tcod;

use util::{Point, Bound};
use rendering::render::{RenderingComponentAble};
use rendering::window::Windows;
use game::Game;
use movement::MovementComponent;
use movement::MovementComponentRandom;
use movement::MovementComponentUser;
use movement::MovementComponentAggro;


pub struct Actor {
    pub position: Point,
    pub display_char: char,
    movement_component: Box<MovementComponent>,
    pub is_pc: bool,
}

impl Actor {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent>, is_pc: bool) -> Actor {
        Actor {
            position: Point { x: x, y: y },
            display_char: dc,
            movement_component: mc,
            is_pc: is_pc,
        }
    }

    pub fn dog(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<MovementComponentRandom> = Box::new(MovementComponent::new(bound));
        Actor::new(x, y, 'd', mc, false)
    }

    pub fn cat(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<MovementComponentRandom> = Box::new(MovementComponent::new(bound));
        Actor::new(x, y, 'c', mc, false)
    }

    pub fn kobold(x: i32, y: i32, bound: Bound) -> Actor {
        let mc: Box<MovementComponentAggro> = Box::new(MovementComponent::new(bound));
        Actor::new(x, y, 'k', mc, false)
    }

    pub fn heroine(bound: Bound) -> Actor {
        let mc: Box<MovementComponentUser> = Box::new(MovementComponent::new(bound));
        let p = Game::get_character_point();
        Actor::new(p.x, p.y, '@', mc, true)
    }

    pub fn update(&mut self, _: &mut Windows) {
        self.position = self.movement_component.update(self.position);
    }

    pub fn render(&self, rendering_component: &mut RenderingComponentAble) {
        rendering_component.render_object(self.position, self.display_char);
    }
}

impl Clone for Actor {
    fn clone(&self) -> Actor {
        let mc = self.movement_component.box_clone();
        Actor::new(self.position.x, self.position.y, self.display_char, mc, self.is_pc)
    }
}
