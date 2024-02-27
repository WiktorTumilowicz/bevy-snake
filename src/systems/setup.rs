use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

use crate::components::background::{Board, Coordinates, Tile};

pub fn initialize_grid(window_query: Query<&Window, With<PrimaryWindow>>, mut commands: Commands) {
    let window = window_query.get_single().unwrap();
    let min_window_size = f32::min(window.width(), window.height());

    commands.insert_resource(Board::new(min_window_size));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Res<Board>,
) {
    let mat1 = materials.add(Color::rgb(172. / 255., 206. / 255., 94. / 255.));
    let mat2 = materials.add(Color::rgb(114. / 255., 183. / 255., 106. / 255.));

    let tile = Mesh2dHandle(meshes.add(Rectangle::new(board.gap_size(), board.gap_size())));

    board
        .translation_grid
        .iter()
        .enumerate()
        .for_each(|(y, row)| {
            row.iter()
                .enumerate()
                .for_each(|(x, &(translation_x, translation_y))| {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: tile.clone(),
                            material: match (x + y) % 2 {
                                0 => mat1.clone(),
                                1 => mat2.clone(),
                                _ => unreachable!(),
                            },
                            transform: Transform::from_xyz(
                                translation_x,
                                translation_y,
                                0.0, // Z translation, assuming 2D
                            ),
                            ..Default::default()
                        },
                        Tile {},
                        Coordinates::new(x, y),
                    ));
                });
        });
}
