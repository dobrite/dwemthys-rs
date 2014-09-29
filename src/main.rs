extern crate tcod;
extern crate dwemthys;

use tcod::{Console, key_code, Special};

use dwemthys::game::Game;
use dwemthys::actor::Actor;
use dwemthys::rendering::RenderingComponent;
use dwemthys::movement::MovementComponent;

fn main() {
    let mut game = Game::new();
    let mut c = Actor::heroine(game.window_bounds);
    let mut npcs: Vec<Box<Actor>> = vec![
        box Actor::dog(10i32, 10i32, game.window_bounds),
        box Actor::cat(40i32, 25i32, game.window_bounds),
    ];

    game.render(&npcs, &c);

    while !(Console::window_closed() || game.exit) {
        let keypress = game.wait_for_keypress();

        match keypress.key {
            Special(key_code::Escape) => game.exit = true,
            _                         => {}
        }

        game.update(&mut npcs, &mut c);

        game.render(&npcs, &c);
    }
}
