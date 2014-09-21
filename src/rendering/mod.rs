extern crate tcod;

use util::{Point};
use self::tcod::{Console, background_flag, KeyState};

pub struct TcodRenderingComponent {
    pub console: Console
}

impl TcodRenderingComponent {
    pub fn before_render_new_frame(&mut self) {
        self.console.clear();
    }

    pub fn render_object(&mut self, position: Point, symbol: char) {
        self.console.put_char(position.x as int, position.y as int, symbol, background_flag::Set);
    }

    pub fn after_render_new_frame(&mut self) {
        self.console.flush();
    }

    pub fn wait_for_keypress(&self) -> KeyState {
        self.console.wait_for_keypress(true)
    }
}
