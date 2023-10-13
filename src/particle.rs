// particle.rs
// Kaleb Burris
// 10-12-2023
// The necessary components to simulate fluid dynamics using particles.
const GRAVITY: f32 = -9.8;

use bevy::{
    prelude::{
        shape, Bundle, Component, Material, Mesh, Query, Res, Resource, Transform, Vec2, Vec3,
    },
    time::Time,
};

#[derive(Debug, Resource)]
pub struct Gravity(pub Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0., -9.81))
    }
}

#[derive(Component, Debug)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.) // Default to 1 kg
    }
}

#[derive(Component, Debug)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self { radius: 0.5 }
    }
}

#[derive(Component, Debug)]
pub struct Velocity {
    pub vec: Vec2,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            vec: Vec2::new(x, y),
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            vec: Vec2::new(0.0, 0.0),
        }
    }
}

#[derive(Default, Debug, Bundle)]
pub struct ParticleBundle {
    pub pos: Transform,
    pub mass: Mass,
    pub collider: CircleCollider,
    pub velocity: Velocity,
}

/// Moves objects in the physics world
pub fn simulate(time: Res<Time>, mut query: Query<(&mut Transform, &Mass, &mut Velocity)>) {
    for (mut pos, mass, mut velocity) in query.iter_mut() {
        pos.translation.y += velocity.vec[1] * time.delta_seconds();
        velocity.vec[1] += GRAVITY * time.delta_seconds();
    }
}
