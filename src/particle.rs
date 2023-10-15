// particle.rs
// Kaleb Burris
// 10-12-2023
// The necessary components to simulate fluid dynamics using particles.

// The amount of velocity lost on a collision.
const PARTICLE_DAMPENING_FACTOR: f32 = 0.9;

//
const SMOOTHING_RADIUS: f32 = 25.0;

// Max 60fps.
const DELTA_TIME_MAX: f32 = 1.0 / 60.0;

use bevy::{
    prelude::{Bundle, Component, Query, Res, Resource, Transform, Vec2, Vec3},
    time::Time,
    window::Window,
};

#[derive(Debug, Resource)]
pub struct Gravity(Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0., 0.))
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
}

/// Detects if a particle is outside of the window, reverses it velocity, and moves it back into the window.
fn border_collision(pos: &mut Transform, velocity: &mut Velocity, window: &Window) {
    let win_width = window.width();
    let win_height = window.height();
    let mut collision: bool = false;

    if pos.translation.x > win_width / 2.0 {
        pos.translation.x = win_width / 2.0;
        collision = true;
    }

    if pos.translation.x < -1.0 * win_width / 2.0 {
        pos.translation.x = -1.0 * win_width / 2.0;
        collision = true;
    }

    if pos.translation.y > win_height / 2.0 {
        pos.translation.y = win_height / 2.0;
        collision = true;
    }

    if pos.translation.y < -1.0 * win_height / 2.0 {
        pos.translation.y = -1.0 * win_height / 2.0;
        collision = true;
    }

    if collision {
        velocity.vec[0] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
        velocity.vec[1] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }
}

/// Moves objects in the physics world
pub fn simulate(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut window: Query<&Window>,
    mut query: Query<(&mut Transform, &Mass, &mut Velocity)>,
) {
    let delta_seconds = DELTA_TIME_MAX.max(time.delta_seconds());
    let gravity = gravity.into_inner();

    for (mut pos, _, mut velocity) in query.iter_mut() {
        // Move by the velocity we've stored.
        pos.translation.x += velocity.vec[0] * delta_seconds;
        pos.translation.y += velocity.vec[1] * delta_seconds;
        // Apply physics!
        velocity.vec[0] += gravity.0[0] * delta_seconds;
        velocity.vec[1] += gravity.0[1] * delta_seconds;
        // Check for collision
        border_collision(&mut pos, &mut velocity, window.single_mut());
    }

    let mut combinations = query.iter_combinations_mut();

    while let Some([(pos1, _mass, mut velocity), (other_pos, _, mut other_velocity)])  = combinations.fetch_next()  {
        // Apply fluid dispersion force.
        let force = calculate_force(&pos1, &other_pos);
        velocity.vec[0] += force.x * delta_seconds / 2.0;
        velocity.vec[1] += force.y * delta_seconds / 2.0;
        other_velocity.vec[0] += force.x * delta_seconds / 2.0;
        other_velocity.vec[1] += force.y * delta_seconds / 2.0;
    }
}

fn calculate_density(sample_point: Vec3, particles: Vec<&Transform>) -> f32 {
    let mut density: f32 = 0.0;
    let mut mass: f32 = 1.0;

    for particle in particles {
        let distance = particle.translation.distance(sample_point);

        let influence = smoothing_calculation(SMOOTHING_RADIUS, distance);
        
        density += mass * influence;
    }

    density
}

fn smoothing_calculation(radius: f32, distance: f32) -> f32 {
    let value: f32 = 0_f32.max(radius * radius - distance * distance);
    return value * value * value;
}

fn calculate_force(pos1: &Transform, pos2: &Transform) -> Vec2 {
    // Distance between the two particles.
    let distance = (pos2.translation - pos1.translation).length();
    // Ignore stacked cases and particles outside of the influence of a particle.
    if distance > SMOOTHING_RADIUS || distance == 0.0 {
        return Vec2::new(0.0, 0.0);
    }
    // Vector pointing from pos1 to pos2.
    let vec = 
    (pos1.translation - pos2.translation).normalize()
    * (distance - SMOOTHING_RADIUS);
    Vec2::new(vec.x, vec.y)
}
