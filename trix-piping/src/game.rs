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

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct WelcomeUI;

#[derive(Component)]
pub struct PlayingUI;

#[derive(Component)]
pub struct ScoresUI;

fn hello_world() {
    println!("Hello, Trix!");
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_welcome_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            WelcomeUI,
        ))
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

fn cleanup_welcome_ui(mut commands: Commands, welcome_ui: Query<Entity, With<WelcomeUI>>) {
    for entity in welcome_ui.iter() {
        commands.entity(entity).despawn();
    }
}

fn handle_menu_button_interactions(
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

fn setup_playing_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            PlayingUI,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(20.0),
                        right: Val::Px(20.0),
                        width: Val::Px(100.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgb(0.2, 0.2, 0.2)),
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(8.0)),
                    ExitButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("EXIT"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn cleanup_playing_ui(mut commands: Commands, playing_ui: Query<Entity, With<PlayingUI>>) {
    for entity in playing_ui.iter() {
        commands.entity(entity).despawn();
    }
}

fn setup_scores_ui(mut commands: Commands) {
    let high_scores = [
        ("AAA", 40_000),
        ("BBB", 30_000),
        ("CCC", 20_000),
        ("DDD", 10_000),
        ("EEE", 5_000),
        ("FFF", 2_500),
        ("GGG", 1_000),
        ("HHH", 500),
        ("III", 250),
        ("JJJ", 100),
    ];

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ScoresUI,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Px(20.0),
                        right: Val::Px(20.0),
                        width: Val::Px(100.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::linear_rgb(0.2, 0.2, 0.2)),
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(8.0)),
                    ExitButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("EXIT"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0),
                    ..default()
                })
                .with_children(|scores_container| {
                    for (name, score) in high_scores.iter() {
                        scores_container.spawn((
                            Text::new(format!("{} - {:>6} pts", name, score)),
                            TextFont {
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    }
                });
        });
}

fn cleanup_scores_ui(mut commands: Commands, scores_ui: Query<Entity, With<ScoresUI>>) {
    for entity in scores_ui.iter() {
        commands.entity(entity).despawn();
    }
}

fn handle_exit_button_interactions(
    mut next_state: ResMut<NextState<GameState>>,
    exit_button_query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
) {
    for interaction in &exit_button_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Welcome);
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
    .add_systems(Startup, (hello_world, setup_camera))
    .add_systems(OnEnter(GameState::Welcome), setup_welcome_ui)
    .add_systems(OnEnter(GameState::Playing), setup_playing_ui)
    .add_systems(OnEnter(GameState::Scores), setup_scores_ui)
    .add_systems(OnExit(GameState::Welcome), cleanup_welcome_ui)
    .add_systems(OnExit(GameState::Playing), cleanup_playing_ui)
    .add_systems(OnExit(GameState::Scores), cleanup_scores_ui)
    .add_systems(
        Update,
        handle_menu_button_interactions.run_if(in_state(GameState::Welcome)),
    )
    .add_systems(
        Update,
        handle_exit_button_interactions.run_if(in_state(GameState::Playing)),
    )
    .add_systems(
        Update,
        handle_exit_button_interactions.run_if(in_state(GameState::Scores)),
    );
    app
}
