// main.rs
// Kaleb Burris
// 10-12-2023
// The code containing the application startup for `bevy-fluid-simulation`.

mod particle;
use particle::*;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::MaterialMesh2dBundle, render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .init_resource::<Gravity>()
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, simulate)
        .add_systems(Update, color_particle)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();
    // Create background image
    let size = Extent3d {
        height: window.height() as u32,
        width: window.width() as u32,
        depth_or_array_layers: 1,
    };


    let mat = 


    commands.spawn(
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Box::default())).into(),
            material: materials.add(Image::new_fill(size, TextureDimension::D2, &[0], TextureFormat)),
            ..default()
        }
    );

    // Create camera for 2D environment.
    commands.spawn(Camera2dBundle::default());

    // Generate particles
    for y in -10..10 {
        for x in -25..25 {
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
                    collider: CircleCollider::new(10.0),
                    velocity: Velocity::new(0.0, 0.0),
                    color: (),
                });
        }
    }
}