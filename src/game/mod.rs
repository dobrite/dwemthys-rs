extern crate tcod;

use util::{Point, Bound};
use rendering::{TcodRenderingComponent, RenderingComponent};
use traits::Updates;
use character::Character;

pub struct Game {
    pub exit:           bool,
    pub window_bounds: Bound,
    pub rendering_component: Box<RenderingComponent>
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

    fn render(&mut self, npcs: &Vec<Box<Updates>>, c: Character) {
        self.rendering_component.before_render_new_frame();
        for i in npcs.iter() {
            i.render(self.rendering_component);
        }
        c.render(self.rendering_component);
        self.rendering_component.after_render_new_frame();
    }

    fn update(&mut self, npcs: &mut Vec<Box<Updates>>, c: &mut Character, keypress: tcod::KeyState, game: Game) {
        c.update(keypress, self);
        for i in npcs.mut_iter() {
            i.update(self);
        }
    }
}
