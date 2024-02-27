use bevy::prelude::*;

pub const BOARDSIZE: usize = 10;

#[derive(Component)]
pub struct Tile {}

#[derive(Default)]
pub enum Direction {
    UP,
    #[default]
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    pub const fn new(x: usize, y: usize) -> Coordinates {
        Coordinates { x, y }
    }

    pub fn clone_in_direction(&self, dir: Direction) {
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
        };
    }

    pub fn get_xy(&self) -> (usize, usize) {
        (self.x, self.y)
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
