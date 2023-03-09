// Note that the white code is autogenerated by VSCode to help with types / variables
// This Roguelike in Rust is ECS based (Entity Component System) - Different from OOP approach

use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*; // ECS (Entity componenet system - assoc components with entities)
use specs_derive::Component;
use std::cmp::{max, min};

// COMPONENTS /////////////////////////////////

// Position of an entity
#[derive(Component)] // From specs-derive, automatically implements the Component trait
struct Position {
    x: i32,
    y: i32,
}

/* WOULD BE NEEDED WITHOUT SPECS-DERIVE
impl Component for Position {
    type Storage = VecStorage<Self>;
}
 */

// How to draw an entity
#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType, // What character to draw
    fg: RGB,                   // Foreground, RGB from RLTK
    bg: RGB,                   // Background color, RGB from RLTK
}

// GAME STATE ////////////////////////////////

struct State {
    ecs: World, // Registers all components at startup
}

impl GameState for State {
    // Required trait for RLTK
    fn tick(&mut self, ctx: &mut Rltk) {
        // mutable reference to RLTK context (the screen), received from the main
        ctx.cls(); // Clear screen
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        // Loops through positions and renderables and joins them, guarantees that returned tuples are from entities with both
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
        // ctx.print(1, 1, "Hello Rust World");
    }
}

// MAIN FUNCITON ///////////////////////////////////////////////////////////////////////
fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50() // Returns a context (screen)
        .with_title("Roguelike Tutorial")
        .build()?; // The ? unwraps a Result or Option type (in this case a Result)
    let mut gs = State { ecs: World::new() }; // gs is a State struct defined above

    // REGISTER COMPONENTS WITH THE WORLD
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    // CREATE SOME ENTITIES
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'), // See http://dwarffortresswiki.org/index.php/Character_table
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('☺'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }

    rltk::main_loop(context, gs) // Calls the main_loop in rltk, this will call tick in the State
}