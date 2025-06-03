use bevy::prelude::*;

fn hello_world() {
    println!("Hello, Trix!");
}

pub fn create_app(for_wasm: bool) -> App {
    let window = if for_wasm {
        Window {
            title: "Trix Piping".into(),
            resolution: (800.0, 600.0).into(),
            canvas: Some("#game-canvas".to_string()),
            ..default()
        }
    } else {
        Window {
            title: "Trix Piping".into(),
            resolution: (800.0, 600.0).into(),
            ..default()
        }
    };

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window),
        ..default()
    }))
    .add_systems(Startup, hello_world);
    app
}
