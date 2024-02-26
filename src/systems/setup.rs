use bevy::{
    prelude::*,
    window::PrimaryWindow,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::components::background::Tile;
use crate::components::background::BOARDSIZE;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let min_window_size = f32::min(window.width(), window.height());

    let mat1 = materials.add(Color::rgb(172./255., 206./255., 94./255.));
    let mat2 = materials.add(Color::rgb(114./255., 183./255., 106./255.));

    let tile = Mesh2dHandle(meshes.add(Rectangle::new(
        min_window_size / BOARDSIZE as f32,
        min_window_size / BOARDSIZE as f32,
    )));

    for y in 0..BOARDSIZE {
        for x in 0..BOARDSIZE {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: tile.clone(),
                    material: match (x + y) % 2 {
                        0 => mat1.clone(),
                        1 => mat2.clone(),
                        _ => unreachable!(),
                    },
                    transform: Transform::from_xyz(
                        // Distribute shapes from -min_window_size/2 to +min_window_size/2.
                        -min_window_size as f32 / 2. + (x as f32 + 0.5) / (BOARDSIZE) as f32 * min_window_size as f32, 
                        -min_window_size as f32 / 2. + (y as f32 + 0.5) / (BOARDSIZE) as f32 * min_window_size as f32, 
                        0.0,
                    ),
                    ..default()
                },
                Tile { x: x, y: y },
            ));
        }
    }
}
