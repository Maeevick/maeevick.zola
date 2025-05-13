use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 200.0;
const BACKGROUND_COLOR: Color = Color::linear_rgb(1.0, 1.0, 1.0);

const GROUND_HEIGHT: f32 = 20.0;
const GROUND_THICKNESS: f32 = 5.0;
const OBSTACLE_SIZE: f32 = 20.0;

const GAME_SPEED: f32 = 200.0;
const GRAVITY: f32 = 980.0;

const GRIMBLE_SIZE: f32 = 50.0;
const JUMP_FORCE: f32 = 400.0;

#[derive(Component)]
struct Grimble {
    velocity: Vec2,
    on_ground: bool,
}

#[derive(Component)]
struct Obstacle;

#[derive(Resource)]
struct ObstacleTimer(Timer);

fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = BACKGROUND_COLOR;
    commands.spawn(Camera2d::default());
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

fn apply_gravity(mut query: Query<&mut Grimble>, time: Res<Time>) {
    for mut grimble in query.iter_mut() {
        grimble.velocity.y -= GRAVITY * time.delta_secs();
    }
}

fn move_grimble(mut query: Query<(&mut Transform, &mut Grimble)>, time: Res<Time>) {
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

fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Grimble>) {
    for mut grimble in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) && grimble.on_ground {
            grimble.velocity.y = JUMP_FORCE;
            grimble.on_ground = false;

            println!("Grimble is almost flying!");
        }
    }
}

fn move_obstacles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
    time: Res<Time>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();

        if transform.translation.x < -WINDOW_WIDTH / 2.0 - OBSTACLE_SIZE {
            commands.entity(entity).despawn();
            println!("Obstacle deleted !");
        }
    }
}

fn spawn_obstacles(mut commands: Commands, time: Res<Time>, mut timer: ResMut<ObstacleTimer>) {
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Grimble Running".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ObstacleTimer(Timer::new(
            Duration::from_secs(1),
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                apply_gravity,
                move_grimble,
                handle_input,
                spawn_obstacles,
                move_obstacles,
            ),
        )
        .run();
}
