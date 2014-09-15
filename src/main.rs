extern crate tcod;
use tcod::{Console, background_flag, key_code, Special};
use std::rand::Rng;

fn render(con: &mut Console, x: int, y: int, dog_x: int, dog_y: int) {
    con.clear();
    con.put_char(x, y, '@', background_flag::Set);
    con.put_char(dog_x, dog_y, 'd', background_flag::Set);
    con.flush();
}

fn main() {
    let con_x = 80i;
    let con_y = 50i;

    let mut con = Console::init_root(con_x, con_y, "libtcod Rust tutorial", false);
    let mut exit = false;

    let mut char_x = 40i;
    let mut char_y = 25i;
    let mut dog_x = 10i;
    let mut dog_y = 10i;

    render(&mut con, char_x, char_y, dog_x, dog_y);

    while !(Console::window_closed() || exit) {
        let keypress = con.wait_for_keypress(true);

        match keypress.key {
            Special(key_code::Escape) => exit = true,
            Special(key_code::Up) => {
                if char_y >= 1 {
                    char_y -= 1;
                }
            },
            Special(key_code::Down) => {
                if char_y < (con_y - 1) {
                    char_y += 1;
                }
            },
            Special(key_code::Left) => {
                if char_x >= 1 {
                    char_x -= 1;
                }
            },
            Special(key_code::Right) => {
                if char_x < (con_x - 1) {
                    char_x += 1;
                }
            },
            _ => {}
        }

        let offset_x = std::rand::task_rng().gen_range(0, 3i) - 1;
        if (dog_x + offset_x) > 0 && (dog_x + offset_x) < (con_x - 1) {
            dog_x += offset_x;
        }

        let offset_y = std::rand::task_rng().gen_range(0, 3i) - 1;
        if (dog_y + offset_y) > 0 && (dog_y + offset_y) < (con_y - 1) {
            dog_y += offset_y;
        }

        render(&mut con, char_x, char_y, dog_x, dog_y);
    }
}
