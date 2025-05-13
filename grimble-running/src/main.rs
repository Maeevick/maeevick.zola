use bevy::prelude::*;

fn main() {
    App::new()
    .add_systems(Update, hello_system)
    .run();
}

fn hello_system() {
  println!("Hello, Grimble!");
}