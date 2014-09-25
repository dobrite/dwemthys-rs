extern crate tcod;

use game::Game;
use rendering::RenderingComponent;
use util::{Point};
use self::tcod::{Console, background_flag, KeyState};

pub trait Updates {
    fn update(&mut self);
    fn render(&self, &mut RenderingComponent);
}

