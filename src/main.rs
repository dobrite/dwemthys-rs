extern crate tcod;
extern crate dwemthys;

use dwemthys::util::{Point, Bound};
use dwemthys::game::Game;
use dwemthys::traits::{Updates, RenderingComponent};
use dwemthys::character::Character;
use dwemthys::npc::NPC;
use dwemthys::rendering::TcodRenderingComponent;

use tcod::{Console, key_code, Special};

fn main() {
    let mut game = Game::new();
    let mut c = Character::new(40i32, 25i32, '@');
    let d  = box NPC::new(10i32, 10i32, 'd') as Box<Updates>;
    let ct = box NPC::new(40, 25, 'c') as Box<Updates>;

    let mut npcs: Vec<Box<Updates>> = vec![d, ct];

    render(&mut rendering_component, &npcs, c);

    while !(Console::window_closed() || game.exit) {
        let keypress = rendering_component.wait_for_keypress();
        match keypress.key {
            Special(key_code::Escape) => game.exit = true,
            _                         => {}
        }

        update(&mut npcs, &mut c, keypress, game);

        render(&mut rendering_component, &npcs, c);
    }
}
