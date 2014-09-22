extern crate tcod;

use game::Game;
use rendering::TcodRenderingComponent;
use util::{Point};
use self::tcod::{Console, background_flag, KeyState};

pub trait Updates {
    fn update(&mut self, Game);
    fn render(&self, &mut TcodRenderingComponent);
}

pub trait RenderingComponent {
    fn new(Console) -> Self;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, Point, char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> KeyState;
}

impl RenderingComponent for TcodRenderingComponent {
    fn new(console: Console) -> TcodRenderingComponent {
        TcodRenderingComponent {
            console: console
        }
    }

    pub fn before_render_new_frame(&mut self) {
        self.console.clear();
    }

    pub fn render_object(&mut self, position: Point, symbol: char) {
        self.console.put_char(position.x as int, position.y as int, symbol, background_flag::Set);
    }

    pub fn after_render_new_frame(&mut self) {
        self.console.flush();
    }

    pub fn wait_for_keypress(&mut self) -> KeyState {
        self.console.wait_for_keypress(true)
    }
}
