use bevy::prelude::*;

pub const BOARDSIZE: usize = 10;

#[derive(Component)]
pub struct Tile {}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    UP,
    #[default]
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    pub fn to_oposite(&self) -> Direction {
        match &self {
            Direction::DOWN => Direction::UP,
            Direction::UP => Direction::DOWN,
            Direction::RIGHT => Direction::LEFT,
            Direction::LEFT => Direction::RIGHT,
        }
    }
}

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, Hash)]
pub struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    pub const fn new(x: usize, y: usize) -> Coordinates {
        Coordinates { x, y }
    }

    pub fn clone_in_direction(&self, dir: &Direction) -> Coordinates {
        match dir {
            Direction::UP => Coordinates {
                x: self.x,
                y: (self.y + 1) % BOARDSIZE,
            },
            Direction::RIGHT => Coordinates {
                x: (self.x + 1) % BOARDSIZE,
                y: self.y,
            },
            Direction::DOWN => Coordinates {
                x: self.x,
                y: if self.y == 0 {
                    BOARDSIZE - 1
                } else {
                    self.y - 1
                },
            },
            Direction::LEFT => Coordinates {
                x: if self.x == 0 {
                    BOARDSIZE - 1
                } else {
                    self.x - 1
                },
                y: self.y,
            },
        }
    }

    pub fn get_xy(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn is_adjacent(&self, other: Coordinates) -> Option<Direction> {
        // Check if the coordinates are adjacent horizontally or vertically
        // Considering the wrapping around of coordinates
        if self.x == other.x {
            if (self.y + 1) % BOARDSIZE == other.y {
                return Some(Direction::UP);
            } else if (other.y + 1) % BOARDSIZE == self.y {
                return Some(Direction::DOWN);
            }
        } else if self.y == other.y {
            if (self.x + 1) % BOARDSIZE == other.x {
                return Some(Direction::RIGHT);
            } else if (other.x + 1) % BOARDSIZE == self.x {
                return Some(Direction::LEFT);
            }
        }
        None
    }
}

#[derive(Resource, Default)]
pub struct Board {
    pub translation_grid: Vec<Vec<(f32, f32)>>,
    gap_size: f32,
}

impl Board {
    // Create a new TranslationGrid instance
    pub fn new(window_size: f32) -> Self {
        let mut grid = Vec::new();
        for x in 0..BOARDSIZE {
            let mut row = Vec::new();
            for y in 0..BOARDSIZE {
                let translation_x =
                    -window_size / 2.0 + (x as f32 + 0.5) / BOARDSIZE as f32 * window_size;
                let translation_y =
                    -window_size / 2.0 + (y as f32 + 0.5) / BOARDSIZE as f32 * window_size;
                row.push((translation_x, translation_y));
            }
            grid.push(row);
        }
        Board {
            translation_grid: grid,
            gap_size: window_size / BOARDSIZE as f32,
        }
    }

    // Function to get translation for a given x, y coordinate
    pub fn get_translation(&self, coordinates: Coordinates) -> (f32, f32) {
        let (x, y) = coordinates.get_xy();
        // Check if the provided coordinates are within bounds
        if x < self.translation_grid.len() && y < self.translation_grid[0].len() {
            self.translation_grid[x][y]
        } else {
            panic!("Something went wrong making the snake!")
        }
    }

    pub fn gap_size(&self) -> f32 {
        self.gap_size
    }
}

pub fn get_snake_image(
    direction_in: Option<Direction>,
    direction_out: Option<Direction>,
) -> &'static str {
    match (direction_in, direction_out) {
        (Some(Direction::UP), Some(Direction::DOWN)) => "body_vertical.png",
        (Some(Direction::DOWN), Some(Direction::UP)) => "body_vertical.png",
        (Some(Direction::LEFT), Some(Direction::RIGHT)) => "body_horizontal.png",
        (Some(Direction::RIGHT), Some(Direction::LEFT)) => "body_horizontal.png",
        (None, Some(Direction::DOWN)) => "head_up.png",
        (None, Some(Direction::UP)) => "head_down.png",
        (None, Some(Direction::RIGHT)) => "head_left.png",
        (None, Some(Direction::LEFT)) => "head_right.png",
        (Some(Direction::DOWN), None) => "tail_up.png",
        (Some(Direction::UP), None) => "tail_down.png",
        (Some(Direction::RIGHT), None) => "tail_left.png",
        (Some(Direction::LEFT), None) => "tail_right.png",
        (Some(Direction::UP), Some(Direction::LEFT)) => "body_topleft.png",
        (Some(Direction::UP), Some(Direction::RIGHT)) => "body_topright.png",
        (Some(Direction::DOWN), Some(Direction::LEFT)) => "body_bottomleft.png",
        (Some(Direction::DOWN), Some(Direction::RIGHT)) => "body_bottomright.png",
        (Some(Direction::LEFT), Some(Direction::UP)) => "body_topleft.png",
        (Some(Direction::RIGHT), Some(Direction::UP)) => "body_topright.png",
        (Some(Direction::LEFT), Some(Direction::DOWN)) => "body_bottomleft.png",
        (Some(Direction::RIGHT), Some(Direction::DOWN)) => "body_bottomright.png",
        _ => panic!("Invalid direction combination"),
    }
}
