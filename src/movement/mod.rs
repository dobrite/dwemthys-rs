extern crate tcod;

use util::{Bound, Point, DoesContain, DoesNotContain};

use std;
use std::rand::Rng;

pub trait MovementComponent {
    fn new(Bound) -> Self;
    fn update(&self, Point) -> Point;
}

pub struct RandomMovementComponent {
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
