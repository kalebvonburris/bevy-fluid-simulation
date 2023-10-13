// main.rs
// Kaleb Burris
// 10-12-2023
// The code containing the application startup for `bevy-fluid-simulation`.

mod particle;
use particle::*;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, setup)
    .add_systems(Update, particle_system)
    .add_systems(Update, bevy::window::close_on_esc)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create camera for 2D environment.
    commands.spawn(Camera2dBundle::default());
    // PURPLE SQUARE!!!
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
}