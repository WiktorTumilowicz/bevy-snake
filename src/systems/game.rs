use bevy::prelude::*;

use crate::components::background::{Board, Direction};
use crate::components::game::*;

pub fn spawn_snake(mut commands: Commands, asset_server: Res<AssetServer>, board: Res<Board>) {
    let (snake_head_x, snake_head_y) = board.get_translation(SNAKE_START_HEAD);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(snake_head_x, snake_head_y, 1.0),
            texture: asset_server.load("head_right.png"),
            ..default()
        },
        SnakeSegment {},
        SNAKE_START_HEAD,
    ));
    let (snake_body_x, snake_body_y) = board.get_translation(SNAKE_START_BODY);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(snake_body_x, snake_body_y, 1.0),
            texture: asset_server.load("body_horizontal.png"),
            ..default()
        },
        SnakeSegment {},
        SNAKE_START_BODY,
    ));
    let (snake_tail_x, snake_tail_y) = board.get_translation(SNAKE_START_TAIL);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(snake_tail_x, snake_tail_y, 1.0),
            texture: asset_server.load("tail_left.png"),
            ..default()
        },
        SnakeSegment {},
        SNAKE_START_TAIL,
    ));
}

pub fn spawn_apple(mut commands: Commands, asset_server: Res<AssetServer>, board: Res<Board>) {
    let (apple_x, apple_y) = board.get_translation(APPLE_START);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(apple_x, apple_y, 1.0),
            texture: asset_server.load("apple.png"),
            ..default()
        },
        Apple {},
        APPLE_START,
    ));
}

pub fn read_player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut snake_direction: ResMut<SnakeDirection>,
) {
    // Process player input direction
    if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
        snake_direction.direction = Direction::UP;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight)
        || keyboard_input.just_pressed(KeyCode::KeyD)
    {
        snake_direction.direction = Direction::RIGHT;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyS)
    {
        snake_direction.direction = Direction::DOWN;
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) || keyboard_input.just_pressed(KeyCode::KeyA)
    {
        snake_direction.direction = Direction::LEFT;
    }
}
