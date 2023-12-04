// main.rs
// Kaleb Burris
// 10-12-2023
// The code containing the application startup for `bevy-fluid-simulation`.

//#![windows_subsystem = "windows"]

mod particle;

use particle::*;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    // Removes the cmd window when running
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .init_resource::<Gravity>()
        .init_resource::<ChunkMapDoubleBuffer>()
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, simulate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create camera for 2D environment.
    commands.spawn(Camera2dBundle::default());

    let particle_diameter = 10.0;

    let x_max = 80.0;
    let y_max = 100.0;

    // Generate particles
    for y in 0..(x_max as u32) {
        for x in 0..(y_max as u32) {
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    ..default()
                })
                .insert(Particle {
                    pos: Transform::from_xyz(
                        (x as f32 - (x_max / 2.0)) * particle_diameter,
                        ((y_max / 2.0) - y as f32) * particle_diameter,
                        0.0,
                    )
                    .with_scale(Vec3::splat(particle_diameter)),
                    collider: CircleCollider::new(particle_diameter / 2.0),
                    velocity: Velocity::new(0.0, 0.0, 0.0),
                })
                .insert(
                    Transform::from_xyz(
                        (x as f32 - (x_max / 2.0)) * particle_diameter,
                        ((y_max / 2.0) - y as f32) * particle_diameter,
                        0.0,
                    )
                    .with_scale(Vec3::splat(particle_diameter)),
                );
        }
    }
}
