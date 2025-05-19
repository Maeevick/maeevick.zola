use std::time::Duration;

use bevy::prelude::*;
use wasm_bindgen::prelude::*;

mod game;
use game::*;

#[wasm_bindgen]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

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
        .insert_resource(GameState {
            starting: true,
            running: false,
            score: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                apply_gravity,
                move_grimble,
                handle_input,
                spawn_obstacles,
                move_obstacles,
                check_collisions,
                toggle_welcome,
                toggle_game_over,
                update_score,
                update_speed,
            ),
        )
        .run();
}
