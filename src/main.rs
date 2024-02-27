use bevy::prelude::*;
use components::background::Board;
use components::game::SnakeDirection;

mod components;
mod systems;

use crate::systems::game::*;
use crate::systems::setup::*;

fn main() {
    App::new()
        .init_resource::<Board>()
        .init_resource::<SnakeDirection>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, initialize_grid))
        .add_systems(
            Startup,
            (spawn_background, spawn_snake, spawn_apple).after(initialize_grid),
        )
        .add_systems(Update, read_player_input)
        .run();
}
