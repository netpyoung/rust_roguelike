use std::cell::RefCell;
use std::rc::Rc;


use util::Bound;
use util::Point;
use actor::Actor;
use rendering::window::Windows;
use rendering::render::RenderingComponentAble;
use game::MoveInfo;

pub struct Map {
    pub content: Vec<Vec<Vec<Box<Actor>>>>,
    pub size: Bound,
    pub move_info: Rc<RefCell<MoveInfo>>,
}

pub struct Maps {
    pub terrain: Box<Map>,
    pub enemies: Box<Map>,
    pub friends: Box<Map>,
    pub pcs: Box<Map>,
}

impl Map {
    pub fn new(move_info: Rc<RefCell<MoveInfo>>) -> Map {
        let size = move_info.borrow().bounds;
        let content = Map::init_contents(size);
        Map {
            content: content,
            size: size,
            move_info: move_info,
        }
    }

    pub fn init_contents(size: Bound) -> Vec<Vec<Vec<Box<Actor>>>> {
        let mut contents: Vec<Vec<Vec<Box<Actor>>>> = vec![];
        for _ in 0..size.max.x {
            let mut x_vec: Vec<Vec<Box<Actor>>> = vec![];
            for _ in 0..size.max.y {
                let y_vec: Vec<Box<Actor>> = vec![];
                x_vec.push(y_vec);
            }
            contents.push(x_vec);
        }
        return contents;
    }

    pub fn push_actor(&mut self, point: Point, actor: Box<Actor>) {
        self.content[point.x as usize][point.y as usize].push(actor);
    }

    pub fn update(&mut self, windows: &mut Windows) {
        let mut new_content = Map::init_contents(self.size);
        for x_iter in self.content.iter_mut() {
            for y_iter in x_iter.iter_mut() {
                for actor in y_iter.iter_mut() {
                    actor.update(windows);
                    if actor.is_pc {
                        self.move_info.borrow_mut().char_location = actor.position;
                    }
                    let point = actor.position;
                    let new_actor = actor.clone();
                    new_content[point.x as usize][point.y as usize].push(new_actor);
                }
            }
        }
        self.content = new_content;
    }

    pub fn render(&mut self, renderer: &mut RenderingComponentAble) {
        for (x, x_iter) in self.content.iter_mut().enumerate() {
            for (y, y_iter) in x_iter.iter_mut().enumerate() {
                for actor in y_iter.iter_mut() {
                    let point = Point::new(x as i32, y as i32);
                    renderer.render_object(point, actor.display_char);
                }
            }
        }
    }
}

impl Maps {
    pub fn new(move_info: Rc<RefCell<MoveInfo>>) -> Maps {
        let terrain = Box::new(Map::new(move_info.clone()));
        let enemies = Box::new(Map::new(move_info.clone()));
        let friends = Box::new(Map::new(move_info.clone()));
        let pcs = Box::new(Map::new(move_info.clone()));

        Maps {
            friends: friends,
            enemies: enemies,
            terrain: terrain,
            pcs: pcs,
        }
    }

    pub fn update(&mut self, windows: &mut Windows) {
        self.pcs.update(windows);
        self.terrain.update(windows);
        self.friends.update(windows);
        self.enemies.update(windows);
    }

    pub fn render(&mut self, renderer: &mut RenderingComponentAble) {
        self.terrain.render(renderer);
        self.friends.render(renderer);
        self.enemies.render(renderer);
        self.pcs.render(renderer);
    }

    pub fn enemy_at(&self, point: Point) -> Option<&Box<Actor>> {
        let enemies_at_point = &self.enemies.content[point.x as usize][point.y as usize];
        if enemies_at_point.len() > 0 {
            Some(&enemies_at_point[0])
        } else {
            None
        }
    }
}
