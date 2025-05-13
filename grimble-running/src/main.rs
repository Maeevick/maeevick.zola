use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 200.0;
const BACKGROUND_COLOR: Color = Color::linear_rgb(1.0, 1.0, 1.0);

const GROUND_HEIGHT: f32 = 20.0;
const GROUND_THICKNESS: f32 = 5.0;

const GRAVITY: f32 = 980.0;

const GRIMBLE_SIZE: f32 = 50.0;
const JUMP_FORCE: f32 = 400.0;

#[derive(Component)]
struct Grimble {
    velocity: Vec2,
    on_ground: bool,
}

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
            (-WINDOW_WIDTH + GRIMBLE_SIZE) / 2.0,
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

        let floor_y = -WINDOW_HEIGHT / 2.0 + GROUND_HEIGHT + GRIMBLE_SIZE / 2.0;
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
            println!("Grimble saute!");
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Grimble is in the Air".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_gravity, move_grimble, handle_input))
        .run();
}
