use rltk::RGB;
use specs::prelude::*;
use specs_derive::*;

// COMPONENTS //////////////////////////////////////////////////////////////////////////

// Position of an entity
#[derive(Component)] // From specs-derive, automatically implements the Component trait
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/* WOULD BE NEEDED WITHOUT SPECS-DERIVE
impl Component for Position {
    type Storage = VecStorage<Self>;
}
 */

// How to draw an entity
#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType, // What character to draw
    pub fg: RGB,                   // Foreground, RGB from RLTK
    pub bg: RGB,                   // Background color, RGB from RLTK
}

// Example component to attach to an entity that will move left with a system
// #[derive(Component)]
// struct LeftMover {}

// Used to tag the playable character
#[derive(Component, Debug)]
pub struct Player {}
