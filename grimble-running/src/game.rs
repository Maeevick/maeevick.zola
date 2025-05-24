use bevy::{input::touch::TouchPhase, prelude::*};
use rand::Rng;
use std::time::Duration;

pub const WINDOW_WIDTH: f32 = 600.0;
pub const WINDOW_HEIGHT: f32 = 200.0;
pub const BACKGROUND_COLOR: Color = Color::linear_rgb(1.0, 1.0, 1.0);

pub const GROUND_HEIGHT: f32 = 20.0;
pub const GROUND_THICKNESS: f32 = 5.0;
pub const OBSTACLE_SIZE: f32 = 20.0;

pub const GAME_SPEED: f32 = 200.0;
pub const GRAVITY: f32 = 980.0;

pub const GRIMBLE_SIZE: f32 = 50.0;
pub const JUMP_FORCE: f32 = 400.0;

#[derive(Resource)]
pub struct GameState {
    pub starting: bool,
    pub running: bool,
    pub score: u32,
}

#[derive(Component)]
pub struct Grimble {
    pub velocity: Vec2,
    pub on_ground: bool,
}

#[derive(Component)]
pub struct Obstacle;

#[derive(Resource)]
pub struct ObstacleTimer(pub Timer);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct SpeedText;

#[derive(Component)]
pub struct StartText;

#[derive(Component)]
pub struct GameOverText;

pub fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = BACKGROUND_COLOR;
    commands.spawn(Camera2d::default());
    commands.spawn((
        Text2d::new("Score: 0"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::BLACK.into()),
        Transform::from_xyz(
            -WINDOW_WIDTH / 2.0 + 100.0,
            WINDOW_HEIGHT / 2.0 - 35.0,
            10.0,
        ),
        ScoreText,
    ));
    commands.spawn((
        Text2d::new("Speed: 200"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::linear_rgb(0.1, 0.1, 1.0)),
        Transform::from_xyz(WINDOW_WIDTH / 2.0 - 120.0, WINDOW_HEIGHT / 2.0 - 35.0, 10.0),
        SpeedText,
    ));

    commands.spawn((
        Text2d::new("GRIMBLE RUNNING\nPress SPACE to start or clic/touch"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::linear_rgb(0.1, 1.0, 0.1)),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(0.0, 0.0, 10.0),
        StartText,
    ));

    commands.spawn((
        Text2d::new("GAME OVER!\n(restart by pressing R or clic/touch)"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::linear_rgb(1.0, 0.1, 0.1)),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(0.0, 0.0, 10.0),
        Visibility::Hidden,
        GameOverText,
    ));

    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.1, 0.1, 0.1),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, GROUND_THICKNESS)),
            ..default()
        },
        Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + GROUND_HEIGHT, 0.0),
    ));
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2 {
                x: GRIMBLE_SIZE,
                y: GRIMBLE_SIZE,
            }),
            ..default()
        },
        Transform::from_xyz(
            (-WINDOW_WIDTH) / 2.0 + GRIMBLE_SIZE,
            (-WINDOW_HEIGHT + GRIMBLE_SIZE + GROUND_THICKNESS) / 2.0 + GROUND_HEIGHT,
            1.0,
        ),
        Grimble {
            velocity: Vec2::ZERO,
            on_ground: true,
        },
    ));

    println!("Hello, Grimble! Are you reading for running?");
}

pub fn apply_gravity(mut query: Query<&mut Grimble>, time: Res<Time>) {
    for mut grimble in query.iter_mut() {
        grimble.velocity.y -= GRAVITY * time.delta_secs();
    }
}

pub fn move_grimble(mut query: Query<(&mut Transform, &mut Grimble)>, time: Res<Time>) {
    for (mut transform, mut grimble) in query.iter_mut() {
        transform.translation.y += grimble.velocity.y * time.delta_secs();

        let floor_y = (-WINDOW_HEIGHT + GRIMBLE_SIZE + GROUND_THICKNESS) / 2.0 + GROUND_HEIGHT;
        if transform.translation.y <= floor_y {
            transform.translation.y = floor_y;
            grimble.velocity.y = 0.0;
            grimble.on_ground = true;
        }
    }
}

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut touch_events: EventReader<TouchInput>,
    mut query: Query<&mut Grimble>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    obstacles: Query<Entity, With<Obstacle>>,
) {
    let is_click_or_touch = mouse_input.just_pressed(MouseButton::Left)
        || touch_events
            .read()
            .any(|touch| matches!(touch.phase, TouchPhase::Started));

    if (game_state.starting || !game_state.running)
        && (keyboard_input.pressed(KeyCode::KeyR) || is_click_or_touch)
    {
        game_state.running = true;
        game_state.score = 0;

        for entity in obstacles.iter() {
            commands.entity(entity).despawn();
        }
        println!("Here, we go again!");
    }

    if game_state.starting && (keyboard_input.pressed(KeyCode::Space) || is_click_or_touch) {
        game_state.starting = false;
        game_state.running = true;

        println!("Let's Go, Grimble!");

        return;
    }

    for mut grimble in query.iter_mut() {
        if grimble.on_ground && (keyboard_input.just_pressed(KeyCode::Space) || is_click_or_touch) {
            grimble.velocity.y = JUMP_FORCE;
            grimble.on_ground = false;

            println!("Grimble is almost flying!");
        }
    }
}

pub fn move_obstacles(
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
    time: Res<Time>,
) {
    if game_state.starting || !game_state.running {
        return;
    }
    let current_speed = GAME_SPEED + game_state.score as f32;
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x -= current_speed * time.delta_secs();

        if transform.translation.x < -WINDOW_WIDTH / 2.0 - OBSTACLE_SIZE {
            commands.entity(entity).despawn();
            game_state.score += 1;
            println!("Obstacle deleted ! Score: {}", game_state.score);
        }
    }
}

pub fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ObstacleTimer>,
    game_state: Res<GameState>,
) {
    if game_state.starting || !game_state.running {
        return;
    }

    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();
        let random_width = rng.random_range(0.5..=1.5) * OBSTACLE_SIZE;
        let random_height = rng.random_range(0.5..=2.0) * OBSTACLE_SIZE;

        commands.spawn((
            Sprite {
                color: Color::linear_rgb(0.3, 0.3, 0.3),
                custom_size: Some(Vec2::new(random_width, random_height)),
                ..default()
            },
            Transform::from_xyz(
                (WINDOW_WIDTH - random_width) / 2.0,
                (-WINDOW_HEIGHT + random_height + GROUND_THICKNESS) / 2.0 + GROUND_HEIGHT,
                0.0,
            ),
            Obstacle,
        ));

        let random_duration = rng.random_range(0.75..=2.0);
        timer
            .0
            .set_duration(Duration::from_secs_f32(random_duration));
        println!(
            "New obstacle spotted ! Width: {} x Height: {}, Next in {} s",
            random_width, random_height, random_duration
        );
    }
}

pub fn check_collisions(
    grimble_query: Query<&Transform, With<Grimble>>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.starting || !game_state.running {
        return;
    }

    let grimble_transform = match grimble_query.single() {
        Ok(transform) => transform,
        Err(_) => {
            println!("Oho Grimble has disappeared from this world! This is an issue");
            return;
        }
    };
    let grimble_min_x = grimble_transform.translation.x - GRIMBLE_SIZE / 2.0;
    let grimble_max_x = grimble_transform.translation.x + GRIMBLE_SIZE / 2.0;
    let grimble_min_y = grimble_transform.translation.y - GRIMBLE_SIZE / 2.0;
    let grimble_max_y = grimble_transform.translation.y + GRIMBLE_SIZE / 2.0;

    for obstacle_transform in obstacle_query.iter() {
        let obstacle_min_x = obstacle_transform.translation.x - OBSTACLE_SIZE / 2.0;
        let obstacle_max_x = obstacle_transform.translation.x + OBSTACLE_SIZE / 2.0;
        let obstacle_min_y =
            obstacle_transform.translation.y - obstacle_transform.scale.y * OBSTACLE_SIZE / 2.0;
        let obstacle_max_y =
            obstacle_transform.translation.y + obstacle_transform.scale.y * OBSTACLE_SIZE / 2.0;

        if grimble_max_x > obstacle_min_x
            && grimble_min_x < obstacle_max_x
            && grimble_max_y > obstacle_min_y
            && grimble_min_y < obstacle_max_y
        {
            game_state.running = false;
            println!("You loose! Game Over! You score is: {}", game_state.score);
        }
    }
}

pub fn update_score(
    game_state: ResMut<GameState>,
    mut score_query: Query<&mut Text2d, With<ScoreText>>,
) {
    if game_state.starting || !game_state.running {
        return;
    }

    if let Ok(mut text) = score_query.single_mut() {
        text.0 = format!("Score: {}", game_state.score);
    }
}

pub fn update_speed(
    game_state: ResMut<GameState>,
    mut speed_query: Query<&mut Text2d, With<SpeedText>>,
) {
    if game_state.starting || !game_state.running {
        return;
    }

    if let Ok(mut text) = speed_query.single_mut() {
        text.0 = format!("Speed: {}", GAME_SPEED as u32 + game_state.score);
    }
}

pub fn toggle_welcome(
    game_state: ResMut<GameState>,
    mut game_over_query: Query<&mut Visibility, With<StartText>>,
) {
    if let Ok(mut visibility) = game_over_query.single_mut() {
        *visibility = if game_state.starting {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub fn toggle_game_over(
    game_state: ResMut<GameState>,
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
) {
    if let Ok(mut visibility) = game_over_query.single_mut() {
        *visibility = if game_state.starting || game_state.running {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}
