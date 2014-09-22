extern crate tcod;

use util::{Point};
use self::tcod::{Console, background_flag, KeyState};

pub struct TcodRenderingComponent {
    pub console: Console
}
