extern crate tcod;
extern crate dwemthys;

use tcod::{Console, key_code, Special};

use dwemthys::game::Game;
use dwemthys::traits::Updates;
use dwemthys::character::Character;
use dwemthys::npc::NPC;
use dwemthys::rendering::RenderingComponent;
use dwemthys::movement::{MovementComponent, RandomMovementComponent};

fn main() {
    let mut game = Game::new();
    let mut c = Character::new(40i32, 25i32, '@');
    let cmc : Box<RandomMovementComponent> = box MovementComponent::new(game.window_bounds);
    let dmc : Box<RandomMovementComponent> = box MovementComponent::new(game.window_bounds);
    let mut npcs: Vec<Box<Updates>> = vec![
        box NPC::new(10i32, 10i32, 'd', dmc) as Box<Updates>,
        box NPC::new(40i32, 25i32, 'c', cmc) as Box<Updates>,
    ];

    game.render(&npcs, c);

    while !(Console::window_closed() || game.exit) {
        let keypress = game.wait_for_keypress();

        match keypress.key {
            Special(key_code::Escape) => game.exit = true,
            _                         => {}
        }

        game.update(&mut npcs, &mut c, keypress);

        game.render(&npcs, c);
    }
}
