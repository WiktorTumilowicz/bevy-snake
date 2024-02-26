use bevy::prelude::*;

pub const BOARDSIZE: i32 = 10;

#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
}
