use super::Rect;
use rltk::{RandomNumberGenerator, Rltk, RGB};
use std::cmp::{max, min};

// Used to enumerate the types of tiles used
// PartialEq allows for == to be used
// Clone allows for copies to be made
// Copy ensures that assigment makes a copy rather than moving values
#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

// This will convert a position to an array index (for width 80 screen)
// Lack of ; at the end makes this statement a return statement implicitly (equiv to saying return)
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

// Having 3/ makes the comopiler use the comment as a function description usable to IDEs
/// Makes a map with solid boundaries and 400 randomly placed walls. For code testing
pub fn new_map_test() -> Vec<TileType> {
    // Vectors are arrays that can change size at runtime
    // vec! takes params in [] and is a macro for: for _i in 0..4000 {map.push(TileType::Floor);}
    let mut map = vec![TileType::Floor; 80 * 50];

    // Make the boundaries walls
    for x in 0..80 {
        // iterates 0 to 79
        // These calls will index the vector map[idx] and set the upper and lower boundaries to walls
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        // These do the same for the right and left boundaries
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        // _i means that i won't be used
        // Get x and y from 1 to 79 or 49 inclusive in this case
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        // Index the values into the map vector
        let idx = xy_idx(x, y);
        // Assure that it isn't wear the player is starting
        if idx != xy_idx(40, 25) {
            // Turn that space into a wall
            map[idx] = TileType::Wall;
        }
    }

    map
}

/// Utilizes the Rect class of the size of a desired room then changes that area to floor tiles
/// Create a new Rect of the correct size then feed that into this function
fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy_idx(x, y)] = TileType::Floor;
        }
    }
}

// Horizontal and Vertical Corridors
// &[TileType] rather than &Vec<TileType> allows for slices rather than full vector
fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    // Use min and max to find the appropriate range from lowest to highest x
    for x in min(x1, x2)..=max(x1, x2) {
        // Index into the map array
        let idx = xy_idx(x, y);
        // Confirm that it's a valid index before changing index into a floor tile
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map[idx as usize] = TileType::Floor;
        }
    }
}

/// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
/// This gives a handful of random rooms and corridors joining them together.
/// This gets called from the main module in the main function
pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80 * 50];

    //// EARLY CODE!!! - Make a couple of rooms with a corridor then return the map
    // let room1 = Rect::new(20, 15, 10, 15);
    // let room2 = Rect::new(35, 15, 10, 15);
    // apply_room_to_map(&room1, &mut map);
    // apply_room_to_map(&room2, &mut map);
    // apply_horizontal_tunnel(&mut map, 25, 40, 23);
    // map

    // Create some Random rooms!
    let mut rooms: Vec<Rect> = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _i in 0..MAX_ROOMS {
        // roll_dice and range seem equivalent?
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        // let x = rng.roll_dice(1, 80 - w - 1) - 1;
        // let y = rng.roll_dice(1, 50 - h - 1) - 1;
        let x = rng.range(1, 80 - w - 1) - 1;
        let y = rng.range(1, 50 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;
        // Assure that the new room doesn't intersect any other rooms before adding to map
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            apply_room_to_map(&new_room, &mut map);

            // Connect the new room with the previous room (if there is a previous room) with a corridor
            if !rooms.is_empty() {
                // Get the centers of this new room and the previous room
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                // This will randomly decide whether to draw the horizontal or vertical corridor first
                if rng.range(0, 2) == 1 {
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                    apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                } else {
                    apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                    apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                }
            }

            rooms.push(new_room);
        }
    }
    (rooms, map)

    //     }
    // }

    // (rooms, map)
}

// DRAW THE MAP ////////////////////////////////////////////////////////
// Note &[TileType] rather than &Vec<TileType> allows for slices rather than full vector
pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;
    // Iterate through the map vector
    for tile in map.iter() {
        // Render a tile depending upon the tile type
        match tile {
            // How to draw a TileType::Floor
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            // How to draw a TileType::Wall
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
