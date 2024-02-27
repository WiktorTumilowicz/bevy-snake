use rand::seq::IteratorRandom;
use std::collections::HashSet;

use bevy::prelude::*;

use crate::components::background::{get_snake_image, Board, Coordinates, Direction, BOARDSIZE};
use crate::components::game::*;
use crate::AppState;

#[derive(Event)]
pub struct RedrawSnakeEvent {}

pub fn spawn_snake(mut commands: Commands, asset_server: Res<AssetServer>, board: Res<Board>) {
    let (snake_head_translation_x, snake_head_translation_y) =
        board.get_translation(SNAKE_START_HEAD);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(snake_head_translation_x, snake_head_translation_y, 1.0),
            texture: asset_server.load("head_right.png"),
            ..default()
        },
        SnakeSegment {},
    ));
    let (snake_body_translation_x, snake_body_translation_y) =
        board.get_translation(SNAKE_START_BODY);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(snake_body_translation_x, snake_body_translation_y, 1.0),
            texture: asset_server.load("body_horizontal.png"),
            ..default()
        },
        SnakeSegment {},
    ));
    let (snake_tail_translation_x, snake_tail_translation_y) =
        board.get_translation(SNAKE_START_TAIL);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(snake_tail_translation_x, snake_tail_translation_y, 1.0),
            texture: asset_server.load("tail_left.png"),
            ..default()
        },
        SnakeSegment {},
    ));
}

pub fn spawn_apple(mut commands: Commands, asset_server: Res<AssetServer>, board: Res<Board>) {
    let (apple_translation_x, apple_translation_y) = board.get_translation(APPLE_START);
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(apple_translation_x, apple_translation_y, 1.0),
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

pub fn move_snake(
    mut snake_direction: ResMut<SnakeDirection>,
    mut snake: ResMut<Snake>,
    mut redraw_snake_event_writer: EventWriter<RedrawSnakeEvent>,
    mut apple_eaten_event: ResMut<AppleEaten>,
) {
    // Ensure player can't make the snake go back on itself
    if snake_direction.prev_direction == snake_direction.direction.to_oposite() {
        snake_direction.direction = snake_direction.prev_direction;
    }

    let new_head = snake.array[0].clone_in_direction(&snake_direction.direction);
    snake.array.insert(0, new_head);

    if apple_eaten_event.value == true {
        apple_eaten_event.value = false;
    } else {
        // If an apple was eaten we don't move the tail
        snake.array.pop().unwrap();
    }

    redraw_snake_event_writer.send(RedrawSnakeEvent {});

    snake_direction.prev_direction = snake_direction.direction;
}

pub fn handle_redraw_snake(
    mut redraw_snake_event_reader: EventReader<RedrawSnakeEvent>,
    snake: Res<Snake>,
    query: Query<Entity, With<SnakeSegment>>,
    mut commands: Commands,
    board: Res<Board>,
    asset_server: Res<AssetServer>,
) {
    //Check if snake needs to be re-drawn
    if redraw_snake_event_reader.is_empty() {
        return;
    } else {
        redraw_snake_event_reader.clear();
    }

    //Redraw the snake
    for i in 0..snake.array.len() {
        let (snake_segment_translation_x, snake_segment_translation_y) =
            board.get_translation(snake.array[i]);

        let direction_in = if i == 0 {
            None
        } else {
            snake.array[i].is_adjacent(snake.array[i - 1])
        };
        let direction_out = if i == snake.array.len() - 1 {
            None
        } else {
            snake.array[i].is_adjacent(snake.array[i + 1])
        };

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    snake_segment_translation_x,
                    snake_segment_translation_y,
                    1.0,
                ),
                texture: asset_server.load(get_snake_image(direction_in, direction_out)),
                ..default()
            },
            SnakeSegment {},
        ));
    }

    //Delete the previous snake
    for entity in query.into_iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_snake_on_apple(
    query: Query<(Entity, &Coordinates), With<Apple>>,
    snake: Res<Snake>,
    board: Res<Board>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut apple_eaten_event: ResMut<AppleEaten>,
) {
    if let Ok((entity, coordinates)) = query.get_single() {
        if snake.array[0] == *coordinates {
            // Remove current apple
            commands.entity(entity).despawn();
            apple_eaten_event.value = true;

            // Compute all possible locations for new apple using hashset
            let mut options = HashSet::with_capacity(BOARDSIZE as usize * BOARDSIZE as usize * 2);
            for x in 0..BOARDSIZE {
                for y in 0..BOARDSIZE {
                    options.insert(Coordinates::new(x, y));
                }
            }
            // Exclude locations where snake exists
            for coordinates in &snake.array {
                options.remove(coordinates);
            }

            // Pick new apple location
            let mut rng = rand::thread_rng();
            let spawn_location = options.iter().choose(&mut rng).expect("You won!");

            // Spawn new apple
            let (apple_translation_x, apple_translation_y) = board.get_translation(*spawn_location);
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(apple_translation_x, apple_translation_y, 1.0),
                    texture: asset_server.load("apple.png"),
                    ..default()
                },
                Apple {},
                *spawn_location,
            ));
        }
    }
}

pub fn check_snake_overlap(mut next_app_state: ResMut<NextState<AppState>>, snake: Res<Snake>) {
    if snake.has_duplicates() {
        next_app_state.set(AppState::GameOver);
    }
}

pub fn game_over(mut commands: Commands) {
    commands.spawn(Text2dBundle {
        transform: Transform::from_xyz(0., 0., 10.),
        text: Text::from_section(
            "Game Over!",
            TextStyle {
                font_size: 22.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        ..default()
    });
}
