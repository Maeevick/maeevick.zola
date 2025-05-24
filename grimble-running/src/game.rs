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

const START_BUTTON_COLOR: Color = Color::linear_rgb(0.89, 0.13, 0.74);
const START_BUTTON_HOVER: Color = Color::linear_rgb(0.71, 0.09, 0.58);

const RESTART_BUTTON_COLOR: Color = Color::linear_rgb(0.0, 0.63, 0.87);
const RESTART_BUTTON_HOVER: Color = Color::linear_rgb(0.10, 0.76, 1.0);

const JUMP_PROTECTION_FRAMES: u8 = 3;

#[derive(Debug, Clone, PartialEq)]
pub enum GamePhase {
    Menu,
    Running,
    GameOver,
}

#[derive(Resource)]
pub struct GameState {
    pub phase: GamePhase,
    pub score: u32,
    pub prevent_jump_frames: u8,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            phase: GamePhase::Menu,
            score: 0,
            prevent_jump_frames: 0,
        }
    }

    pub fn is_menu(&self) -> bool {
        self.phase == GamePhase::Menu
    }

    pub fn is_running(&self) -> bool {
        self.phase == GamePhase::Running && self.prevent_jump_frames == 0
    }

    pub fn is_game_over(&self) -> bool {
        self.phase == GamePhase::GameOver
    }

    pub fn start_game(&mut self) {
        self.phase = GamePhase::Running;
        self.prevent_jump_frames = JUMP_PROTECTION_FRAMES;
    }

    pub fn game_over(&mut self) {
        self.phase = GamePhase::GameOver;
    }

    pub fn restart_game(&mut self) {
        self.phase = GamePhase::Running;
        self.score = 0;
        self.prevent_jump_frames = JUMP_PROTECTION_FRAMES;
    }

    pub fn tick_frame(&mut self) {
        if self.prevent_jump_frames > 0 {
            self.prevent_jump_frames -= 1;
        }
    }
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
pub struct StartButton;

#[derive(Component)]
pub struct RestartButton;

pub fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = BACKGROUND_COLOR;
    commands.spawn(Camera2d::default());
    commands.spawn((
        Text2d::new("Score: 0"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::linear_rgb(0.89, 0.13, 0.74)),
        Transform::from_xyz(-WINDOW_WIDTH / 2.0 + 60.0, WINDOW_HEIGHT / 2.0 - 15.0, 10.0),
        ScoreText,
    ));
    commands.spawn((
        Text2d::new("Speed: 200"),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::linear_rgb(0.0, 0.63, 0.87)),
        Transform::from_xyz(-WINDOW_WIDTH / 2.0 + 60.0, WINDOW_HEIGHT / 2.0 - 35.0, 10.0),
        SpeedText,
    ));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(280.0),
                        height: Val::Px(80.0),
                        border: UiRect::all(Val::Px(3.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        display: Display::Flex,
                        ..default()
                    },
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(START_BUTTON_COLOR),
                    StartButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("START RUNNING\nGambare Grimble!"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(280.0),
                        height: Val::Px(80.0),
                        border: UiRect::all(Val::Px(3.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        display: Display::None,
                        ..default()
                    },
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(RESTART_BUTTON_COLOR),
                    RestartButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("GAME OVER\nrestart\n(at your own risk)"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
        });

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

pub fn toggle_start_button(
    game_state: Res<GameState>,
    mut button_query: Query<&mut Node, With<StartButton>>,
) {
    for mut style in button_query.iter_mut() {
        style.display = if game_state.is_menu() {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn toggle_restart_button(
    game_state: Res<GameState>,
    mut button_query: Query<&mut Node, With<RestartButton>>,
) {
    for mut style in button_query.iter_mut() {
        style.display = if game_state.is_game_over() {
            Display::Flex
        } else {
            Display::None
        };
    }
}

pub fn handle_button_interactions(
    start_interactions: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<StartButton>),
    >,
    restart_interactions: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<RestartButton>),
    >,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    obstacles: Query<Entity, With<Obstacle>>,
) {
    for interaction in &start_interactions {
        if *interaction == Interaction::Pressed && game_state.is_menu() {
            game_state.start_game();
            println!("Let's Go, Grimble! (Button clicked)");
        }
    }

    for interaction in &restart_interactions {
        if *interaction == Interaction::Pressed && game_state.is_game_over() {
            for entity in obstacles.iter() {
                commands.entity(entity).despawn();
            }
            game_state.restart_game();
            println!("Here, we go again! (Button clicked)");
        }
    }
}

pub fn button_visual_feedback(
    mut start_buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<StartButton>),
    >,
    mut restart_buttons: Query<
        (&Interaction, &mut BackgroundColor),
        (With<Button>, With<RestartButton>, Without<StartButton>),
    >,
) {
    for (interaction, mut color) in &mut start_buttons {
        *color = match *interaction {
            Interaction::Pressed => START_BUTTON_COLOR.into(),
            Interaction::None => START_BUTTON_COLOR.into(),
            Interaction::Hovered => START_BUTTON_HOVER.into(),
        };
    }

    for (interaction, mut color) in &mut restart_buttons {
        *color = match *interaction {
            Interaction::Pressed => RESTART_BUTTON_COLOR.into(),
            Interaction::None => RESTART_BUTTON_COLOR.into(),
            Interaction::Hovered => RESTART_BUTTON_HOVER.into(),
        };
    }
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
    game_state: Res<GameState>,
) {
    if !game_state.is_running() {
        return;
    }
    let should_jump = keyboard_input.just_pressed(KeyCode::Space)
        || mouse_input.just_pressed(MouseButton::Left)
        || touch_events
            .read()
            .any(|touch| matches!(touch.phase, TouchPhase::Started));

    for mut grimble in query.iter_mut() {
        if grimble.on_ground && should_jump {
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
    if !game_state.is_running() {
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
    if !game_state.is_running() {
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
    if !game_state.is_running() {
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
            game_state.game_over();
            println!("You lose! Game Over! You score is: {}", game_state.score);
        }
    }
}

pub fn update_score(
    game_state: ResMut<GameState>,
    mut score_query: Query<&mut Text2d, With<ScoreText>>,
) {
    if !game_state.is_running() {
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
    if !game_state.is_running() {
        return;
    }

    if let Ok(mut text) = speed_query.single_mut() {
        text.0 = format!("Speed: {}", GAME_SPEED as u32 + game_state.score);
    }
}

pub fn tick_game_state(mut game_state: ResMut<GameState>) {
    game_state.tick_frame();
}
