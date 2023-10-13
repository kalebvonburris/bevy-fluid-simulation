// particle.rs
// Kaleb Burris
// 10-12-2023
// The necessary components to simulate fluid dynamics using particles.

// The amount of velocity lost on a collision.
const PARTICLE_DAMPENING_FACTOR: f32 = 1.0;

// Max 60fps.
const DELTA_TIME_MAX: f32 = 1.0 / 60.0;

use bevy::{
    prelude::{Bundle, Component, Query, Res, Resource, Transform, Vec2},
    time::Time, window::Window,
};

use std::cmp::min;

#[derive(Debug, Resource)]
pub struct Gravity(Vec2);

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

fn border_collision(pos: &Transform, window: &Window) -> bool {
    let win_width = window.width();
    let win_height = window.height();
    if pos.translation.x > win_width / 2.0 || pos.translation.x < -1.0 * win_width / 2.0 { return true; }
    if pos.translation.y > win_height / 2.0 || pos.translation.y < -1.0 * win_height / 2.0 { return true; }
    false
}

/// Moves objects in the physics world
pub fn simulate(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut window: Query<&Window>,
    mut query: Query<(&mut Transform, &Mass, &mut Velocity)>,
) {
    let delta_seconds = DELTA_TIME_MAX;
    let gravity = gravity.into_inner();

    for (mut pos, mass, mut velocity) in query.iter_mut() {
        // Move by the velocity we've stored.
        pos.translation.x += velocity.vec[0] * delta_seconds;
        pos.translation.y += velocity.vec[1] * delta_seconds;
        // Apply physics!
        velocity.vec[0] += gravity.0[0] * delta_seconds;
        velocity.vec[1] += gravity.0[1] * delta_seconds;
        // Check for collision
        if border_collision(&pos, window.single_mut()) {
            velocity.vec[0] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
            velocity.vec[1] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
        }
    }
}
