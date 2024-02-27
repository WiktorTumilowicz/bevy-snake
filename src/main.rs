use std::time::Duration;

use bevy::prelude::*;
use components::game::AppleEaten;

mod components;
mod systems;

use crate::components::background::Board;
use crate::components::game::{Snake, SnakeDirection};
use crate::systems::game::*;
use crate::systems::setup::*;

const TIMESTEP: u64 = 500;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<RedrawSnakeEvent>()
        .init_state::<AppState>()
        .init_resource::<Board>()
        .init_resource::<SnakeDirection>()
        .init_resource::<Snake>()
        .init_resource::<AppleEaten>()
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(
            TIMESTEP,
        )))
        .add_systems(Startup, (spawn_camera, initialize_grid))
        .add_systems(
            Startup,
            (spawn_background, spawn_snake, spawn_apple).after(initialize_grid),
        )
        .add_systems(
            Update,
            (
                read_player_input,
                handle_redraw_snake,
                handle_snake_on_apple,
                check_snake_overlap,
            )
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(FixedUpdate, move_snake.run_if(in_state(AppState::Game)))
        .add_systems(OnEnter(AppState::GameOver), game_over)
        .run();
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    #[default]
    Game,
    GameOver,
}
