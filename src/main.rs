// main.rs
// Kaleb Burris
// 10-12-2023
// The code containing the application startup for `bevy-fluid-simulation`.

mod particle;
use particle::*;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<Gravity>()
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

    // PURPLE CIRCLES!!!
    for y in -50..50 {
        for x in -50..50 {
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    ..default()
                })
                .insert(ParticleBundle {
                    pos: Transform::from_xyz((x * 10) as f32, (y * 10) as f32, 0.0)
                        .with_scale(Vec3::splat(10.0)),
                    mass: Mass(1.0),
                    collider: CircleCollider::default(),
                    velocity: Velocity::new(0.0, -9.8),
                });
        }
    }
}
