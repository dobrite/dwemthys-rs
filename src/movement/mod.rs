extern crate tcod;

use std;
use std::rand::Rng;

use self::tcod::{Special, key_code};

use util::{Bound, Point, DoesContain, DoesNotContain};
use game::Game;

pub trait MovementComponent {
    fn new(Bound) -> Self;
    fn update(&self, Point) -> Point;
}

pub struct RandomMovementComponent {
    window_bounds: Bound
}

pub struct TcodUserMovementComponent {
    window_bounds: Bound
}

pub struct AggroMovementComponent {
    window_bounds: Bound
}

impl MovementComponent for RandomMovementComponent {
    fn new(bound: Bound) -> RandomMovementComponent {
        RandomMovementComponent { window_bounds: bound }
    }

    fn update(&self, point: Point) -> Point {
        let mut offset = Point { x: point.x, y: point.y };
        let offset_x = std::rand::task_rng().gen_range(0, 3i32) - 1;
        match self.window_bounds.contains(offset.offset_x(offset_x)) {
            DoesContain => offset = offset.offset_x(offset_x),
            DoesNotContain => { return point; }
        }

        let offset_y = std::rand::task_rng().gen_range(0, 3i32) - 1;
        match self.window_bounds.contains(offset.offset_y(offset_y)) {
            DoesContain => offset = offset.offset_y(offset_y),
            DoesNotContain => { return point; }
        }

        offset
    }
}

impl MovementComponent for TcodUserMovementComponent {
    fn new(bound: Bound) -> TcodUserMovementComponent {
        TcodUserMovementComponent { window_bounds: bound }
    }

    fn update(&self, point: Point) -> Point {
        let mut offset = Point { x: point.x, y: point.y };
        offset = match Game::get_last_keypress() {
            Some(keypress) => {
                match keypress.key {
                    Special(key_code::Up) => {
                        offset.offset_y(-1)
                    },
                    Special(key_code::Down) => {
                        offset.offset_y(1)
                    },
                    Special(key_code::Left) => {
                        offset.offset_x(-1)
                    },
                    Special(key_code::Right) => {
                        offset.offset_x(1)
                    },
                    _ => { offset }
                }
            },
            None => { offset }
        };

        match self.window_bounds.contains(offset) {
            DoesContain => { offset }
            DoesNotContain => { point }
        }
    }
}

impl MovementComponent for AggroMovementComponent {
    fn new(bound: Bound) -> AggroMovementComponent {
        AggroMovementComponent { window_bounds: bound }
    }

    fn update(&self, point: Point) -> Point {
        point
    }
}
