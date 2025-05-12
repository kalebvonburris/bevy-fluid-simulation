// main.rs
// Kaleb Burris
// 12-3-2023
// The code containing the application startup for `bevy-fluid-simulation`.

#![windows_subsystem = "windows"]

mod particle;

use particle::*;

use bevy::prelude::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

fn main() {
    // Removes the cmd window when running
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin {
            ..Default::default()
        })
        .init_resource::<ChunkMapDoubleBuffer>()
        .add_systems(Startup, setup)
        .add_systems(Update, simulate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create camera for 2D environment.
    commands.spawn(Camera2d::default());

    let particle_diameter = 6.5;

    let x_max = 100.0;
    let y_max = 100.0;

    // Generate particles
    for y in 0..(x_max as u32) {
        for x in 0..(y_max as u32) {
            commands
                .spawn((
                    Mesh2d(meshes.add(Mesh::from(Circle::default())).into()),
                    MeshMaterial2d( materials.add(ColorMaterial::from(Color::LinearRgba(LinearRgba { red: 0., green: 0., blue: 1.0, alpha: 1.0 }))))
                ))
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
