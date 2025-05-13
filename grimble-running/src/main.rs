use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 200.0;
const GROUND_HEIGHT: f32 = 20.0;
const GROUND_THICKNESS: f32 = 5.0;
const GRIMBLE_SIZE: f32 = 50.0;
const BACKGROUND_COLOR: Color = Color::linear_rgb(1.0, 1.0, 1.0);

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
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
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
            (-WINDOW_WIDTH + GRIMBLE_SIZE) / 2.0,
            (-WINDOW_HEIGHT + GRIMBLE_SIZE + GROUND_THICKNESS) / 2.0 + GROUND_HEIGHT,
            1.0,
        ),
    ));

    println!("Hello, Grimble! Are you reading for running?");
}
