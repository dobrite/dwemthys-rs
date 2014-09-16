extern crate tcod;
use tcod::{Console, background_flag, key_code, Special};
use std::rand::Rng;

struct Point {
    x: int,
    y: int
}

impl Point {
    fn offset_x(&self, offset: int) -> Point {
        Point { x: self.x + offset, y : self.y }
    }

    fn offset_y(&self, offset: int) -> Point {
        Point { x: self.x, y : self.y + offset }
    }

    fn offset(&self, offset: Point) -> Point {
        Point { x: self.x + offset.x, y : self.y + offset.y }
    }
}

enum Contains {
    DoesContain,
    DoesNotContain
}

struct Bound {
    min: Point,
    max: Point
}

impl Bound {
    fn contains(&self, point: Point) -> Contains {
        if
            point.x >= self.min.x &&
            point.x <= self.max.x &&
            point.y >= self.min.y &&
            point.y <= self.max.y
        {
            DoesContain
        } else {
            DoesNotContain
        }
    }
}



fn render(con: &mut Console, c_point: Point, d_point: Point) {
    con.clear();
    con.put_char(c_point.x, c_point.y, '@', background_flag::Set);
    con.put_char(d_point.x, d_point.y, 'd', background_flag::Set);
    con.flush();
}

fn main() {
    let window_bounds = Bound { min: Point { x: 0, y: 0 }, max: Point { x: 79, y: 49 } };
    let mut con = Console::init_root(window_bounds.max.x, window_bounds.max.y, "libtcod Rust tutorial", false);

    let mut exit = false;

    let mut char_point = Point { x: 40i, y: 25i };
    let mut dog_point  = Point { x: 10i, y: 10i };

    render(&mut con, char_point, dog_point);

    while !(Console::window_closed() || exit) {
        let keypress = con.wait_for_keypress(true);
        let mut offset = Point { x: 0, y: 0 };

        match keypress.key {
            Special(key_code::Escape) => exit = true,
            Special(key_code::Up) => {
                offset.y = -1;
            },
            Special(key_code::Down) => {
                offset.y = 1;
            },
            Special(key_code::Left) => {
                offset.x = -1;
            },
            Special(key_code::Right) => {
                offset.x = 1;
            },
            _ => {}
        }

        match window_bounds.contains(char_point.offset(offset)) {
            DoesContain    => char_point = char_point.offset(offset),
            DoesNotContain => {}
        }

        let offset_x = std::rand::task_rng().gen_range(0, 3i) - 1;
        match window_bounds.contains(dog_point.offset_x(offset_x)) {
            DoesContain    => dog_point = dog_point.offset_x(offset_x),
            DoesNotContain => {}
        }

        let offset_y = std::rand::task_rng().gen_range(0, 3i) - 1;
        match window_bounds.contains(dog_point.offset_y(offset_y)) {
            DoesContain    => dog_point = dog_point.offset_y(offset_y),
            DoesNotContain => {}
        }

        render(&mut con, char_point, dog_point);
    }
}
