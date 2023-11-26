// particle.rs
// Kaleb Burris
// 10-12-2023
// The necessary components to simulate fluid dynamics using particles.

// The amount of velocity lost on a collision.
const PARTICLE_DAMPENING_FACTOR: f32 = 0.65;

// Smoothing radius for smoothing kernel.
// Defines how far from a point we consider for particle interactions.
const SMOOTHING_RADIUS: f32 = 50.0;

// Max 60fps for simulation step
const DELTA_TIME_MAX: f32 = 1.0 / 60.0;

// use std::f32::consts::PI;

/* -- Imports -- */
// Bevy imports
use bevy::{prelude::*, sprite::ColorMaterial, time::Time, window::Window};

// Rayon for parallelism
// use rayon::prelude::*;

// rand for random number generation
// not actually needed as an import
// use rand::prelude::*;

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
    pub vec: Vec3,
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            vec: Vec3::new(x, y, z),
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            vec: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Resource)]
struct DensityMap {}

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

    // Particle is to the right edge of the window
    if pos.translation.x > win_width / 2.0 - 5.0 {
        pos.translation.x = win_width / 2.0 - 5.0;
        velocity.vec[0] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }

    // Particle is to the left edge of the window
    if pos.translation.x < -1.0 * win_width / 2.0 + 5.0 {
        pos.translation.x = -1.0 * win_width / 2.0 + 5.0;
        velocity.vec[0] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }

    // Particle is above the window
    if pos.translation.y > win_height / 2.0 - 5.0 {
        pos.translation.y = win_height / 2.0 - 5.0;
        velocity.vec[1] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }

    // Particle is below the window.
    if pos.translation.y < -1.0 * win_height / 2.0 + 5.0 {
        pos.translation.y = -1.0 * win_height / 2.0 + 5.0;
        velocity.vec[1] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }
}

/// Clamps an f32 to be within [0, 1) for any value >= 0
fn clamp_positive(x: f32) -> f32 {
    1.0 - (-x).exp()
}

pub fn color_particle(
    query: Query<(Entity, &Velocity)>,
    mut color_assets: ResMut<Assets<ColorMaterial>>,
    color_handles: Query<&mut Handle<ColorMaterial>>,
) {
    for (entity, velocity) in query.iter() {
        if let Ok(material_handle) = color_handles.get(entity) {
            // Normalize the velocity vector. Larger values approach 1, smaller values approach 0
            let absolute_velocity_normalized = clamp_positive(velocity.vec.length() / 150.0);
            // Grab the ColorMaterial
            let material = color_assets.get_mut(material_handle).unwrap();
            // Apply the normalized velocity to the material color
            material.color = Color::rgba(
                absolute_velocity_normalized,
                absolute_velocity_normalized,
                1.0,
                1.0,
            );
        }
    }
}

/// Simulates the movement of particles.
pub fn simulate(
    //gravity: Res<Gravity>,
    window: Query<&Window>,
    mut query: Query<(&mut Transform, &mut Velocity, &CircleCollider)>,
) {
    // Grab the time since the last frame.
    let delta_seconds = DELTA_TIME_MAX;
    // Grab the defined gravity constant.
    //let gravity = gravity.into_inner();
    // Grab every combination between particles.
    let mut combinations = query.iter_combinations_mut();
    // Loop over every particle combination and apply a repelling force.
    while let Some(
        [(pos, mut velocity, collider), (other_pos, mut other_velocity, other_collider)],
    ) = combinations.fetch_next()
    {
        // Apply fluid dispersion force.
        let force = calculate_force(&pos, &other_pos);
        velocity.vec[0] += force.x * delta_seconds / 2.0;
        velocity.vec[1] += force.y * delta_seconds / 2.0;
        other_velocity.vec[0] -= force.x * delta_seconds / 2.0;
        other_velocity.vec[1] -= force.y * delta_seconds / 2.0;

        /*let diff_pos = pos.translation - other_pos.translation;
        let distance = diff_pos.length();
        if distance < (collider.radius + other_collider.radius) / 2.0 {
            let m1 = collider.radius.powf(2.0);
            let m2 = other_collider.radius.powf(2.0);

            let M = m1 + m2;

            let diff_velocity = velocity.vec - other_velocity.vec;

            let d = diff_pos.length().powf(2.0);

            let mut u1 =
                velocity.vec - (m2 * 2.0 / M) * ((diff_velocity).dot(diff_pos) / d * diff_pos);

            
            if !u1.x.is_normal() { u1.x = 0.0; }
            if !u1.y.is_normal() { u1.y = 0.0; }
            if !u1.z.is_normal() { u1.z = 0.0; }
            

            let mut u2 = 
                other_velocity.vec - (m1 * 2.0 / M) * (-(diff_velocity).dot(diff_pos) / d * -diff_pos);

            
            if !u2.x.is_normal() { u2.x = 0.0; }
            if !u2.y.is_normal() { u2.y = 0.0; }
            if !u2.z.is_normal() { u2.z = 0.0; }
            

            /*println!(
                "p:{:?} op{:?} v:{:?} ov:{:?} dp:{:?} m1:{:?} m2:{:?} dv:{:?} M:{:?} u1:{:?} u2:{:?} d:{d:?}",
                &pos.translation, &other_pos.translation, &velocity.vec, &other_velocity.vec, &diff_pos, &m1, &m2, &diff_velocity, &M, &u1, &u2
            );*/

            velocity.vec = u1;
            other_velocity.vec = u2;
        }*/
    }

    for (mut pos, mut velocity, _) in &mut query {
        // Reset velocity if it's nan
        if velocity.vec.is_nan() {
            println!("Found a NaN velocity");
            velocity.vec[0] = 0.0;
            velocity.vec[1] = 0.0;
            velocity.vec[2] = 0.0;
        }
        // Reset the position if it's nan
        if pos.translation.is_nan() {
            println!("Found a NaN position");
            pos.translation.x = 0.0;
            pos.translation.y = 0.0;
            pos.translation.z = 0.0;
        }

        // Apply gravity!
        //velocity.vec[0] += gravity.0[0] * delta_seconds;
        //velocity.vec[1] += gravity.0[1] * delta_seconds;

        // Move by the velocity we've stored.
        pos.translation.x += velocity.vec[0] * delta_seconds;
        pos.translation.y += velocity.vec[1] * delta_seconds;

        // Check for border collision
        border_collision(&mut pos, &mut velocity, window.single());
    }
}

pub fn gpu_test(query: Query<(&mut Transform, &Mass, &mut Velocity, &CircleCollider)>) {}

//pub fn calculate_density_map(positions: Query<&Transform>, density_map: ResMut<DensityMap>) {
//    let density = positions
//        .iter()
//        .map(|pos| {
//            smoothing_kernel(20.0, (pos.translation -   .translation).length());
//        })
//        .sum();
//}

//fn smoothing_kernel(radius: f32, distance: f32) -> f32 {
//    let vol = PI * radius.powf(8.0) / 4.0;
//    let val = 0.0_f32.max(radius * radius - distance * distance);
//    val * val * val / vol
//}

fn calculate_force(pos1: &Transform, pos2: &Transform) -> Vec2 {
    let distance = (pos2.translation - pos1.translation).length();

    // Ignore stacked cases and particles outside of the influence of a particle.
    if distance > SMOOTHING_RADIUS {
        return Vec2::new(0.0, 0.0);
    }

    let mut force = match (pos1.translation - pos2.translation).try_normalize() {
        Some(x) => x,
        None => Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0).normalize(),
    };

    // Vector pointing from pos1 to pos2.
    // We normalize and then apply a force function
    // based on the distance between the particles.
    force *= (distance - SMOOTHING_RADIUS).powf(2.0);
    // Reduce the force generated so we have
    // less chaotic particles.
    Vec2::new(force.x, force.y)
}
