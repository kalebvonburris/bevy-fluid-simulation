// particle.rs
// Kaleb Burris
// 10-12-2023
// The necessary components to simulate fluid dynamics using particles.

// The amount of velocity lost on a collision.
const PARTICLE_DAMPENING_FACTOR: f32 = 0.85;

//
const SMOOTHING_RADIUS: f32 = 50.0;

// Max 60fps.
const DELTA_TIME_MAX: f32 = 1.0 / 60.0;

use bevy::{
    prelude::{Bundle, Component, Query, Res, Resource, Transform, Vec2},
    time::Time,
    window::Window,
};

#[derive(Debug, Resource)]
pub struct Gravity(Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0.0, -98.0))
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

impl CircleCollider {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
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
    pub color: (), // TODO: Apply color to particles based on absolute velocity.
}

/// Detects if a particle is outside of the window, reverses it velocity, and moves it back into the window.
fn border_collision(pos: &mut Transform, velocity: &mut Velocity, window: &Window) {
    let win_width = window.width();
    let win_height = window.height();
    let mut collision: bool = false;

    // Particle is to the right edge of the window
    if pos.translation.x > win_width / 2.0 {
        pos.translation.x = win_width / 2.0;
        velocity.vec[0] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
        collision = true;
    }

    // Particle is to the left edge of the window
    if pos.translation.x < -1.0 * win_width / 2.0 {
        pos.translation.x = -1.0 * win_width / 2.0;
        velocity.vec[0] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
        collision = true;
    }

    // Particle is above the window
    if pos.translation.y > win_height / 2.0 {
        pos.translation.y = win_height / 2.0;
        velocity.vec[1] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
        collision = true;
    }

    // Particle is below the window.
    if pos.translation.y < -1.0 * win_height / 2.0 {
        pos.translation.y = -1.0 * win_height / 2.0;
        velocity.vec[1] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
        collision = true;
    }

    // If the particle went out of bounds, we reverse the direction of velocity.
    if collision {
        // Placeholder
    }
}

/// Simulates the movement of particles.
pub fn simulate(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut window: Query<&Window>,
    mut query: Query<(&mut Transform, &Mass, &mut Velocity)>,
) {
    // Grab the time since the last frame.
    let delta_seconds = DELTA_TIME_MAX.max(time.delta_seconds());
    // Grab the defined gravity constant.
    let gravity = gravity.into_inner();
    // Grab every combination between particles.
    let mut combinations = query.iter_combinations_mut();
    // Loop over every particle combination and apply a repelling force.
    while let Some([(mut pos, _mass, mut velocity), (mut other_pos, _, mut other_velocity)]) =
        combinations.fetch_next()
    {
        let distance = (other_pos.translation - pos.translation).length();
        if distance < 10.0 {
            // Directional vector from pos to other_pos.
            let mut dir_vec = (other_pos.translation - pos.translation).normalize();
            // The distance for each particle to move.
            let distance_to_move = (distance / 2.0) - 10.0;
            // Move both particles.
            dir_vec *= distance_to_move;
            pos.translation -= dir_vec;
            other_pos.translation += dir_vec;
        }
        // Apply fluid dispersion force.
        let force = calculate_force(&pos, &other_pos);
        velocity.vec[0] += force.x * delta_seconds / 2.0;
        velocity.vec[1] += force.y * delta_seconds / 2.0;
        other_velocity.vec[0] -= force.x * delta_seconds / 2.0;
        other_velocity.vec[1] -= force.y * delta_seconds / 2.0;
    }

    for (mut pos, _, mut velocity) in query.iter_mut() {
        // Apply gravity!
        velocity.vec[0] += gravity.0[0] * delta_seconds;
        velocity.vec[1] += gravity.0[1] * delta_seconds;

        // Move by the velocity we've stored.
        pos.translation.x += velocity.vec[0] * delta_seconds;
        pos.translation.y += velocity.vec[1] * delta_seconds;

        // Check for collision
        border_collision(&mut pos, &mut velocity, window.single_mut());
    }
}

fn calculate_force(pos1: &Transform, pos2: &Transform) -> Vec2 {
    let distance = (pos2.translation - pos1.translation).length();

    // Ignore stacked cases and particles outside of the influence of a particle.
    if distance > SMOOTHING_RADIUS {
        return Vec2::new(0.0, 0.0);
    }

    // Vector pointing from pos1 to pos2.
    // We normalize and then apply a force function
    // based on the distance between the particles.
    let vec = (pos1.translation - pos2.translation).normalize() / 2.0
        * (distance - SMOOTHING_RADIUS).powf(2.0);
    // Reduce the force generated so we have
    // less chaotic particles.
    Vec2::new(vec.x, vec.y)
}
