use bevy::prelude::*;

mod components;
mod resources;
mod systems;

use crate::systems::setup::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_background))
        .run();
}
