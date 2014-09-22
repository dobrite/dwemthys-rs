use util::Bound;
use traits::RenderingComponent;

pub struct Game {
    pub exit:           bool,
    pub window_bounds: Bound,
    pub rendering_component: Box<RenderingComponent>
}

impl Game {
    pub fn new() -> Game {
        let bound = Bound {
            min: Point { x: 0, y: 0 },
            max: Point { x: 79, y: 49 },
        };

        let rc : Box<TcodRenderingComponent> = box RenderingComponent::new(bound);

        Game {
            exit: false,
            window_bounds: bound,
            rendering_component: rc,
        }
    }
}
