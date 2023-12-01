// particle.rs
// Kaleb Burris
// 10-12-2023
// The necessary components to simulate fluid dynamics using particles.

// The amount of velocity lost on a collision.
const PARTICLE_DAMPENING_FACTOR: f32 = 0.85;

// The maximum velocity of a particle.
const VELOCITY_MAX: f32 = 250.0;

// Smoothing radius for smoothing kernel.
// Defines how far from a point we consider for particle interactions.
const SMOOTHING_RADIUS: f32 = 50.0;

// Max 60fps for simulation step
// TODO: Max this adapt to display refresh rate
const DELTA_TIME_MAX: f32 = 1.0 / 60.0;

// use std::f32::consts::PI;

use std::sync::RwLock;

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
#[derive(Component, Debug, Clone)]
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

#[derive(Component, Debug, Clone)]
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

#[derive(Default, Debug, Bundle)]
pub struct ParticleBundle {
    pub pos: Transform,
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
            let absolute_velocity_normalized = clamp_positive(velocity.vec.length() / VELOCITY_MAX);
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

fn get_chunk_coordinates(
    chunk_dims: (&usize, &usize),
    pos: &Vec3,
    win_dimensions: (&f32, &f32),
) -> (usize, usize) {
    let chunk_x = (((pos.x + (win_dimensions.0 / 2.0)) / (SMOOTHING_RADIUS * 2.0)) as usize)
        .clamp(0, *chunk_dims.0 - 1);
    let chunk_y = (((-pos.y + (win_dimensions.1 / 2.0)) / (SMOOTHING_RADIUS * 2.0)) as usize)
        .clamp(0, *chunk_dims.1 - 1);
    (chunk_x, chunk_y)
}

/// Simulates the movement of particles.
pub fn simulate(
    //gravity: Res<Gravity>,
    time: Res<Time>,
    window: Query<&Window>,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &CircleCollider)>,
) {
    // Create chunks
    // Extract the size of the window
    let w_dimensions = &window.get_single().unwrap().resolution;

    let win_width = w_dimensions.width();
    let win_height = w_dimensions.height();

    // Don't compute steps where computation is nonsensication (0 sized world)
    if win_width * win_height == 0.0 {
        return;
    }

    let chunks_dim_x = (win_width / (SMOOTHING_RADIUS * 2.0)) as usize;
    let chunks_dim_y = (win_height / (SMOOTHING_RADIUS * 2.0)) as usize;
    // Allocate a vec to store the chunks, with each chunk being wrapped in a Mutex
    let chunks: Vec<RwLock<Vec<(Entity, Transform, Velocity)>>> = (0..(chunks_dim_x * chunks_dim_y)
        .max(1))
        .map(|_| RwLock::new(Vec::new()))
        .collect();

    // Parallel iteration over the query
    query.par_iter().for_each(|(id, pos, velocity, _)| {
        let chunk_coord = get_chunk_coordinates(
            (&chunks_dim_x, &chunks_dim_y),
            &pos.translation,
            (&win_width, &win_height)
        );

        // Calculate the index of the chunk
        let index = chunk_coord.0 + (chunk_coord.1 * chunks_dim_x);
        // Grab the chunk and write the particle to it
        if let Some(chunk) = chunks.get(index) {
            let mut chunk_guard = chunk.write().unwrap(); // handle locking
            chunk_guard.push((id, *pos, velocity.clone()));
        }
    });

    // Grab the time since the last frame, using a const value for the min physics time
    let delta_seconds = DELTA_TIME_MAX.min(time.delta_seconds());

    query
        .par_iter_mut()
        .for_each(|(id, mut pos, mut velocity, collider)| {
            let chunks_in_range: Vec<(usize, usize)> = get_nearby_particles(
                &pos,
                (&chunks_dim_x, &chunks_dim_y),
                (&win_width, &win_height),
            );

            for chunk_coord in chunks_in_range {
                if let Some(chunk) = chunks.get(chunk_coord.0 + (chunk_coord.1 * chunks_dim_x)) {
                    // Perform collision detection
                    let chunk_lock = chunk.read().unwrap();
                    for (other_id, other_pos, other_velocity) in chunk_lock.iter() {
                        if id.index() == other_id.index() {
                            continue;
                        }
                        // Apply fluid dispersion force.
                        let force = calculate_force(&pos, other_pos);
                        velocity.vec[0] += force.x * delta_seconds;
                        velocity.vec[1] += force.y * delta_seconds;

                        let diff_pos = pos.translation - other_pos.translation;
                        let diff_velocity = velocity.vec - other_velocity.vec;

                        // .max() is used for the case of total overlap (distance is 0)
                        let distance = diff_pos.length().max(0.1);

                        // TODO: Fix collider being reused!
                        if distance < (collider.radius + collider.radius) / 2.0
                            && diff_pos.dot(-diff_velocity) > 0.0
                        {
                            let m1 = collider.radius.powf(2.0);
                            let m2 = collider.radius.powf(2.0);

                            let m = m1 + m2;

                            let d = distance.powf(2.0);

                            let u1 = velocity.vec
                                - ((m2 * 2.0 / m)
                                    * ((diff_velocity).dot(diff_pos) / d)
                                    * (diff_pos));

                            velocity.vec = u1;
                        }
                    }
                }
            }

            // Max velocity check
            if velocity.vec.length() > VELOCITY_MAX {
                let scalar = VELOCITY_MAX / velocity.vec.length();
                velocity.vec *= scalar;
            }

            // Apply gravity!
            //velocity.vec[0] += gravity.0[0] * delta_seconds;
            //velocity.vec[1] += gravity.0[1] * delta_seconds;

            // Move by the velocity we've stored.
            pos.translation.x += velocity.vec[0] * delta_seconds;
            pos.translation.y += velocity.vec[1] * delta_seconds;

            // Check for border collision
            border_collision(&mut pos, &mut velocity, window.single());
        });
}

/// Uses chunking and the position of a particle to return the particles nearby that
/// are relevant to the current one.
fn get_nearby_particles(
    pos: &Transform,
    chunk_dims: (&usize, &usize),
    window_dims: (&f32, &f32),
) -> Vec<(usize, usize)> {
    let mut nearby_chunks: Vec<(usize, usize)> = Vec::with_capacity(9);

    let chunks_to_check: [(i32, i32); 9] = [
        (-1, 1),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let chunk_coords_usize = get_chunk_coordinates(chunk_dims, &pos.translation, window_dims);
    let chunk_coords = (chunk_coords_usize.0 as i32, chunk_coords_usize.1 as i32);

    for coord in chunks_to_check {
        // Check if the coordinate is out of bounds (skip)
        if coord.0 + chunk_coords.0 < 0
            || coord.0 + chunk_coords.0 >= (*chunk_dims.0 as i32)
            || coord.1 + chunk_coords.1 < 0
            || coord.1 + chunk_coords.1 >= (*chunk_dims.1 as i32)
        {
            continue;
        }

        nearby_chunks.push((
            (coord.0 + chunk_coords.0) as usize,
            (coord.1 + chunk_coords.1) as usize,
        ));
    }

    nearby_chunks
}

//pub fn gpu_test(query: Query<(&mut Transform, &Mass, &mut Velocity, &CircleCollider)>) {}

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
