use std::collections::HashSet;

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

#[derive(Resource, Debug)]
pub struct Snake {
    pub array: Vec<Coordinates>,
}

impl Snake {
    pub fn has_duplicates(&self) -> bool {
        let mut set = HashSet::new();
        for coord in &self.array {
            if !set.insert(coord) {
                return true; // Duplicate found
            }
        }
        false // No duplicates found
    }
}

impl Default for Snake {
    fn default() -> Self {
        let mut array = Vec::new();
        // Add tail, body, and head in order
        array.push(SNAKE_START_HEAD);
        array.push(SNAKE_START_BODY);
        array.push(SNAKE_START_TAIL);

        Snake { array }
    }
}

#[derive(Resource, Default, Debug, PartialEq)]
pub struct SnakeDirection {
    pub prev_direction: Direction,
    pub direction: Direction,
}

#[derive(Resource, Default)]
pub struct AppleEaten {
    pub value: bool,
}
