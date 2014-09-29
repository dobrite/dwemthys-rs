extern crate tcod;

use game::Game;

use util::{
    Point,
    Bound,
};

use rendering::RenderingComponent;

use movement::{
    MovementComponent,
    TcodUserMovementComponent,
    RandomMovementComponent,
};

pub struct Actor {
    pub position:     Point,
    pub display_char: char,
    pub movement_component: Box<MovementComponent+'static>,
}

impl Actor {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<MovementComponent+'static>) -> Actor {
        Actor {
            position: Point { x: x, y: y },
            display_char: dc,
            movement_component: mc,
        }
    }

    pub fn update(&mut self) {
        self.position = self.movement_component.update(self.position);
    }

    pub fn render(&self, rendering_component: &mut RenderingComponent) {
        rendering_component.render_object(self.position, self.display_char);
    }

    pub fn dog(x: i32, y: i32, bound: Bound) -> Actor {
        let mc : Box<RandomMovementComponent> = box MovementComponent::new(bound);
        Actor::new(x, y, 'd', mc)
    }

    pub fn cat(x: i32, y: i32, bound: Bound) -> Actor {
        let mc : Box<RandomMovementComponent> = box MovementComponent::new(bound);
        Actor::new(x, y, 'c', mc)
    }

    pub fn kobold(x: i32, y: i32, bound: Bound) -> Actor {
        let mc : Box<RandomMovementComponent> = box MovementComponent::new(bound);
        Actor::new(x, y, 'k', mc)
    }

    pub fn heroine(bound: Bound) -> Actor {
        let point = Game::get_character_point();
        let mc : Box<TcodUserMovementComponent> = box MovementComponent::new(bound);
        Actor::new(point.x, point.y, '@', mc)
    }
}
