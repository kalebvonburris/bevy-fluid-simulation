// main.rs
// Kaleb Burris
// 10-12-2023
// TODO: Write file description.

use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Update, bevy::window::close_on_esc)
    .run();
}