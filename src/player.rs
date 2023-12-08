use super::{xy_idx, Player, Position, State, TileType};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

// TAKE IN USER INPUT ////////////////////////////////////////////////////////////////////////
pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    // This will find all entities in ecs storage that have Position and Player Components
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>(); // Get the map

    // For loop only applies to entities that have both Components thanks to .join(), which returns an iterator
    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        // min() and max() used to clamp the movements into a certain area
        // These functions come from the std::cmp library
        if map[destination_idx] != TileType::Wall {
            // Ensure not moving into wall
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

// Get key inputs from arrows, numpad, or Vi standard
pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        // Either has a key or not (Optional)
        None => {} // Nothing happened
        Some(key) => match key {
            // If has a key then match which key
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, &mut gs.ecs)
            }

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, &mut gs.ecs)
            }

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, &mut gs.ecs)
            }

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, &mut gs.ecs)
            }
            // Default
            _ => {}
        },
    }
}
