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
pub const SMOOTHING_RADIUS: f32 = 50.0;

// Max 60fps for simulation step
// TODO: Max this adapt to display refresh rate
const DELTA_TIME_MAX: f32 = 1.0 / 60.0;

// use std::f32::consts::PI;

use std::sync::RwLock;

/* -- Imports -- */
// Bevy imports
use bevy::{prelude::*, time::Time, window::Window};

/* -- Structs and impls -- */
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

#[derive(Component, Default, Debug)]
pub struct Particle {
    pub pos: Transform,
    pub collider: CircleCollider,
    pub velocity: Velocity,
}

#[derive(Default, Debug, Resource)]
pub struct ChunkMapDoubleBuffer {
    read_chunk_map: ChunkMap,
    write_chunk_map: ChunkMap,
}

impl ChunkMapDoubleBuffer {
    fn swap(&mut self) {
        std::mem::swap(&mut self.read_chunk_map, &mut self.write_chunk_map);
    }
}

#[derive(Default, Debug)]
pub struct ChunkMap {
    pub chunks: Vec<RwLock<Vec<(Entity, Transform, Velocity)>>>,
    pub dim_x: usize,
    pub dim_y: usize,
}

impl ChunkMap {
    /// Returns the x, y coordinates of a particle in a ChunkMap.
    pub fn get_chunk_coordinates(
        &self,
        particle: &Particle,
        win_dimensions: (f32, f32),
    ) -> (usize, usize) {
        let chunk_x = (((particle.pos.translation.x + (win_dimensions.0 / 2.0))
            / (SMOOTHING_RADIUS * 2.0)) as usize)
            .clamp(0, self.dim_x - 1);
        let chunk_y = (((-particle.pos.translation.y + (win_dimensions.1 / 2.0))
            / (SMOOTHING_RADIUS * 2.0)) as usize)
            .clamp(0, self.dim_y - 1);
        (chunk_x, chunk_y)
    }
}

/* -- Helper functions */

/// Detects if a particle is outside of the window, reverses it velocity, and moves it back into the window.
fn border_collision(particle: &mut Particle, window: &Window) {
    let win_width = window.width();
    let win_height = window.height();

    let half_radius = particle.collider.radius / 2.0;

    // Particle is to the right edge of the window
    if particle.pos.translation.x > win_width / 2.0 - half_radius {
        particle.pos.translation.x = win_width / 2.0 - half_radius;
        particle.velocity.vec[0] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }

    // Particle is to the left edge of the window
    if particle.pos.translation.x < -1.0 * win_width / 2.0 + half_radius {
        particle.pos.translation.x = -1.0 * win_width / 2.0 + half_radius;
        particle.velocity.vec[0] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }

    // Particle is above the window
    if particle.pos.translation.y > win_height / 2.0 - half_radius {
        particle.pos.translation.y = win_height / 2.0 - half_radius;
        particle.velocity.vec[1] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }

    // Particle is below the window.
    if particle.pos.translation.y < -1.0 * win_height / 2.0 + half_radius {
        particle.pos.translation.y = -1.0 * win_height / 2.0 + half_radius;
        particle.velocity.vec[1] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }
}

/// Uses chunking and the position of a particle to return the particles nearby that
/// are relevant to the current one.
fn get_nearby_particles(
    particle: &Particle,
    chunk_map: &ChunkMap,
    window_dims: (f32, f32),
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

    let chunk_coords_usize = chunk_map.get_chunk_coordinates(particle, window_dims);
    let chunk_coords = (chunk_coords_usize.0 as i32, chunk_coords_usize.1 as i32);

    for coord in chunks_to_check {
        // Check if the coordinate is out of bounds (skip)
        if coord.0 + chunk_coords.0 < 0
            || coord.0 + chunk_coords.0 >= (chunk_map.dim_x as i32)
            || coord.1 + chunk_coords.1 < 0
            || coord.1 + chunk_coords.1 >= (chunk_map.dim_y as i32)
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

/// Returns the force generated by the proximity of two particles.
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


/// Simulates the movement of particles.
pub fn simulate(
    //gravity: Res<Gravity>,
    time: Res<Time>,
    mut chunk_map_double_buffer: ResMut<ChunkMapDoubleBuffer>,
    window: Query<&Window>,
    mut query: Query<(Entity, &mut Particle, &mut Transform)>,
) {
    // Create chunks
    // Extract the size of the window
    let w_dimensions = &window.single().resolution;

    let win_width = w_dimensions.width();
    let win_height = w_dimensions.height();

    let win_dimensions = (win_width, win_height);

    // Don't compute steps where computation is nonsensication (0 sized world)
    if win_width * win_height == 0.0 {
        return;
    }

    let mut_ref = chunk_map_double_buffer.as_mut();

    let chunk_map_read = &mut_ref.read_chunk_map;
    let chunk_map_write = &mut mut_ref.write_chunk_map;

    let chunks_dim_x = (win_width / (SMOOTHING_RADIUS * 2.0)) as usize;
    let chunks_dim_y = (win_height / (SMOOTHING_RADIUS * 2.0)) as usize;

    // Check if the chunk map has changed
    if chunk_map_write.dim_x != chunks_dim_x || chunk_map_write.dim_y != chunks_dim_y {
        // If so, create a new chunk map
        chunk_map_write.dim_x = chunks_dim_x;
        chunk_map_write.dim_y = chunks_dim_y;
        chunk_map_write.chunks = (0..(chunks_dim_x * chunks_dim_y).max(1))
            .map(|_| RwLock::new(Vec::new()))
            .collect();
    }

    // Clear the output chunk map
    chunk_map_write.chunks.iter_mut().for_each(|chunk| {
        if let Ok(mut chunk_lock) = chunk.write() {
            chunk_lock.clear();
        }
    });

    // Write the current data about the particles to the chunk map
    query.iter().for_each(|(id, particle, _)| {
        // Grab the coordinates of the particle
        let chunk_coord = chunk_map_read.get_chunk_coordinates(particle, win_dimensions);
        // Calculate the index of the chunk
        let index = chunk_coord.0 + (chunk_coord.1 * chunk_map_read.dim_x);
        // Grab the chunk and write the particle to it
        if let Some(chunk) = chunk_map_read.chunks.get(index) {
            let mut chunk_lock = chunk.write().unwrap(); // handle locking
            chunk_lock.push((id, particle.pos, particle.velocity.clone()));
        }
    });

    // Grab the time since the last frame, using a const value for the min physics time
    let delta_seconds = DELTA_TIME_MAX.min(time.delta_seconds());

    query
        .par_iter_mut()
        .for_each(|(id, mut particle, mut render_pos)| {
            let chunks_in_range: Vec<(usize, usize)> =
                get_nearby_particles(&particle, chunk_map_read, win_dimensions);

            for chunk_coord in chunks_in_range {
                if let Some(chunk) = chunk_map_read
                    .chunks
                    .get(chunk_coord.0 + (chunk_coord.1 * chunk_map_read.dim_x))
                {
                    // Perform collision detection
                    let chunk_lock = chunk.read().unwrap();
                    for (other_id, other_pos, other_velocity) in chunk_lock.iter() {
                        if id.index() == other_id.index() {
                            continue;
                        }
                        // Apply fluid dispersion force.
                        let force = calculate_force(&particle.pos, other_pos);
                        particle.velocity.vec[0] += force.x * delta_seconds;
                        particle.velocity.vec[1] += force.y * delta_seconds;

                        let diff_pos = particle.pos.translation - other_pos.translation;
                        let diff_velocity = particle.velocity.vec - other_velocity.vec;

                        // .max() is used for the case of total overlap (distance is 0)
                        let distance = diff_pos.length().max(0.1);

                        // TODO: Fix collider being reused!
                        if distance < (particle.collider.radius + particle.collider.radius) / 2.0
                            && diff_pos.dot(-diff_velocity) > 0.0
                        {
                            let m1 = particle.collider.radius.powf(2.0);
                            let m2 = particle.collider.radius.powf(2.0);

                            let m = m1 + m2;

                            let d = distance.powf(2.0);

                            let u1 = particle.velocity.vec
                                - ((m2 * 2.0 / m)
                                    * ((diff_velocity).dot(diff_pos) / d)
                                    * (diff_pos));

                            particle.velocity.vec = u1;
                        }
                    }
                }
            }

            // Max velocity check
            if particle.velocity.vec.length() > VELOCITY_MAX {
                let scalar = VELOCITY_MAX / particle.velocity.vec.length();
                particle.velocity.vec *= scalar;
            }

            // Apply gravity!
            //velocity.vec[0] += gravity.0[0] * delta_seconds;
            //velocity.vec[1] += gravity.0[1] * delta_seconds;

            // Move by the velocity we've stored.
            particle.pos.translation.x += particle.velocity.vec[0] * delta_seconds;
            particle.pos.translation.y += particle.velocity.vec[1] * delta_seconds;

            // Check for border collision
            border_collision(&mut particle, window.single());

            render_pos.translation = particle.pos.translation;
        });

    chunk_map_double_buffer.swap();
}