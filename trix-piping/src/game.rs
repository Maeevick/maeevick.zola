use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, States, Default)]
pub enum GameState {
    #[default]
    Welcome,
    Playing,
    Scores,
}

#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct ScoresButton;

fn hello_world() {
    println!("Hello, Trix!");
}

fn setup_welcome_ui(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Welcome Challenger"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            top: Val::Px(80.0),
                            bottom: Val::Px(150.0),
                            left: Val::Px(0.0),
                            right: Val::Px(0.0),
                        },
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgb(0.2, 0.2, 0.2)),
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(10.0)),
                    StartButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("START"),
                        TextFont {
                            font_size: 80.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(30.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgb(0.15, 0.15, 0.15)),
                    BorderColor(Color::linear_rgb(0.7, 0.7, 0.7)),
                    BorderRadius::all(Val::Px(10.0)),
                    ScoresButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("highest scores"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::linear_rgb(0.8, 0.8, 0.8)),
                    ));
                });
        });
}

fn handle_button_interactions(
    mut next_state: ResMut<NextState<GameState>>,
    start_button_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    scores_button_query: Query<&Interaction, (Changed<Interaction>, With<ScoresButton>)>,
) {
    for interaction in &start_button_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
            println!(
                "Start Button Pressed: GameState is {:?}",
                GameState::Playing
            );
        }
    }

    for interaction in &scores_button_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Scores);
            println!("Score Button Pressed: GameState is {:?}", GameState::Scores);
        }
    }
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
    .init_state::<GameState>()
    .add_systems(Startup, (hello_world, setup_welcome_ui))
    .add_systems(
        Update,
        handle_button_interactions.run_if(in_state(GameState::Welcome)),
    );
    app
}
