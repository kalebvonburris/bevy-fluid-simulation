//! Bevy particle simulation — repulsive force model with elastic collisions.
//!
//! All particles share a single mesh and material handle so Bevy's automatic
//! batching collapses rendering into ~1 instanced draw call regardless of
//! particle count.

#![windows_subsystem = "windows"]

mod components;
mod simulation;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use components::*;
use simulation::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .init_resource::<SimConfig>()
        .init_resource::<SpatialGrid>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (build_spatial_grid, apply_forces, integrate).chain(),
        )
        .run();
}

/// Spawns the camera and an NxN grid of particles.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<SimConfig>,
) {
    commands.spawn(Camera2d);

    // Single shared mesh + material for all particles → 1 instanced draw call.
    let mesh = meshes.add(Circle::new(1.0));
    let material = materials.add(ColorMaterial::from(Color::linear_rgba(0.0, 0.0, 1.0, 1.0)));

    let n = config.grid_spawn_size;
    let diameter = config.particle_radius * config.spawn_spacing;
    let half_extent = n as f32 * diameter / 2.0;

    let particle_radius = config.particle_radius;

    // Batch-spawn all particles for minimal ECS overhead.
    let particles: Vec<_> = (0..n)
        .flat_map(|y| {
            let mes = mesh.clone();
            let mat = material.clone();
            (0..n).map(move |x| {
                let px = x as f32 * diameter - half_extent;
                let py = half_extent - y as f32 * diameter;
                (
                    Mesh2d(mes.clone()),
                    MeshMaterial2d(mat.clone()),
                    Transform::from_xyz(px, py, 0.0).with_scale(Vec3::splat(particle_radius)),
                    Particle,
                    Velocity::default(),
                )
            })
        })
        .collect();

    commands.spawn_batch(particles);
}
