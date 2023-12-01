// main.rs
// Kaleb Burris
// 10-12-2023
// The code containing the application startup for `bevy-fluid-simulation`.

// TODO: Uncomment this! #![windows_subsystem = "window

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
        .init_resource::<ChunkMap>()
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, simulate)
        .add_systems(Update, color_particle)
        //.add_systems(Update, render_background)
        .run();
}

fn setup(
    mut commands: Commands,
    window: Query<&Window>,
    mut chunk_map: ResMut<ChunkMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create camera for 2D environment.
    commands.spawn(Camera2dBundle::default());

    let chunk_map = chunk_map.into_inner();

    let window = window.single();

    // Note: SMOOTHING_RADIUS is stored in particle.rs
    let chunks_dim_x = (window.width() / (SMOOTHING_RADIUS * 2.0)) as usize;
    let chunks_dim_y = (window.height() / (SMOOTHING_RADIUS * 2.0)) as usize;

    // Generate particles
    for y in -20..20 {
        for x in -30..30 {
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    ..default()
                })
                .insert(Particle {
                    pos: Transform::from_xyz((x * 10) as f32, (y * 10) as f32, 0.0)
                        .with_scale(Vec3::splat(10.0)),
                    collider: CircleCollider::new(10.0),
                    velocity: Velocity::new(0.0, 0.0, 0.0),
                });
        }
    }
}
