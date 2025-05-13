use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Grimble Running".into(),
                resolution: (800.0, 400.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    println!("Hello, Grimble! Are you reading for running?");
}
