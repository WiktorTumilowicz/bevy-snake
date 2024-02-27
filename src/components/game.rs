use bevy::prelude::*;

use crate::components::background::{Coordinates, Direction};

pub const SNAKE_START_HEAD: Coordinates = Coordinates::new(4, 4);
pub const SNAKE_START_BODY: Coordinates = Coordinates::new(3, 4);
pub const SNAKE_START_TAIL: Coordinates = Coordinates::new(2, 4);
pub const APPLE_START: Coordinates = Coordinates::new(8, 8);

#[derive(Component)]
pub struct SnakeSegment {}

#[derive(Component)]
pub struct Apple {}

// #[derive(Resource, Default)]
// pub struct Snake {
//     pub snake: Vec<Vec<(f32, f32)>>,
// }

#[derive(Resource, Default)]
pub struct SnakeDirection {
    pub direction: Direction,
}
