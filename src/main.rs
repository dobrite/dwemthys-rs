extern crate tcod;
extern crate dwemthys;

use dwemthys::util::{Point, Bound, DoesContain, DoesNotContain};
use dwemthys::game::Game;
use dwemthys::traits::Updates;
use dwemthys::character::Character;
use dwemthys::npc::NPC;

use tcod::{Console, background_flag, key_code, Special};
use std::rand::Rng;


fn render(con: &mut Console, objs: &Vec<Box<Updates>>) {
    con.clear();
    for i in objs.iter() {
        i.render(con)
    }
    con.flush();
}

fn update(objs: &mut Vec<Box<Updates>>, keypress: tcod::KeyState, game: Game) {
    for i in objs.mut_iter() {
        i.update(keypress, game);
    }
}

fn main() {
    let mut game = Game { exit: false, window_bounds: Bound { min: Point { x: 0, y: 0 }, max: Point { x: 79, y: 49 } } };
    let mut con = Console::init_root(
        (game.window_bounds.max.x + 1) as int,
        (game.window_bounds.max.y + 1) as int,
        "libtcod Rust tutorial",
        false
    );

    let mut c = box Character::new(40i32, 25i32, '@') as Box<Updates>;
    let mut d = box NPC::new(10i32, 10i32, 'd') as Box<Updates>;

    let mut objs: Vec<Box<Updates>> = vec![c, d];

    render(&mut con, &objs);

    while !(Console::window_closed() || game.exit) {
        let keypress = con.wait_for_keypress(true);
        match keypress.key {
            Special(key_code::Escape) => game.exit = true,
            _                         => {}
        }

        update(&mut objs, keypress, game);

        render(&mut con, &objs);
    }
}
