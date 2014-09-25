extern crate tcod;

use traits::Updates;
use util::{Point, DoesContain, DoesNotContain};
use game::Game;
use rendering::RenderingComponent;
use movement::MovementComponent;

pub struct NPC {
    position:     Point,
    display_char: char,
    movement_component: Box<MovementComponent + 'static>
}

impl NPC {
    pub fn new(x: i32, y: i32, dc: char, movement_component: Box<MovementComponent + 'static>) -> NPC {
        NPC {
            position: Point { x: x, y: y },
            display_char: dc,
            movement_component: movement_component
        }
    }
}

impl Updates for NPC {
    fn update(&mut self) {
        self.position = self.movement_component.update(self.position);
    }

    fn render(&self, rendering_component: &mut RenderingComponent) {
        rendering_component.render_object(self.position, self.display_char);
    }
}
