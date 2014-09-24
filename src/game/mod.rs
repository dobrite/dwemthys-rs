extern crate tcod;

use self::tcod::{KeyState};

use util::{Point, Bound};
use rendering::{TcodRenderingComponent, RenderingComponent};
use traits::Updates;
use character::Character;

pub struct Game {
    pub exit:           bool,
    pub window_bounds: Bound,
    pub rendering_component: Box<RenderingComponent + 'static>
}

impl Game {
    pub fn new() -> Game {
        let bounds = Bound {
            min: Point { x: 0, y: 0 },
            max: Point { x: 79, y: 49 },
        };

        let rc : Box<TcodRenderingComponent> = box RenderingComponent::new(bounds);

        Game {
            exit: false,
            window_bounds: bounds,
            rendering_component: rc,
        }
    }

    pub fn render(&mut self, npcs: &Vec<Box<Updates>>, c: Character) {
        self.rendering_component.before_render_new_frame();
        for i in npcs.iter() {
            i.render(&mut *self.rendering_component);
        }
        c.render(&mut *self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    pub fn update(&self, npcs: &mut Vec<Box<Updates>>, c: &mut Character, keypress: tcod::KeyState) {
        c.update(keypress, self);
        for i in npcs.mut_iter() {
            i.update(self);
        }
    }

    pub fn wait_for_keypress(&mut self) -> KeyState {
        self.rendering_component.wait_for_keypress()
    }
}
