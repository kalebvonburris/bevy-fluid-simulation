// particle.rs
// Kaleb Burris
// 12-3-2023
// The necessary components to simulate fluid dynamics using particles.

/* -- Constants -- */

// The amount of velocity lost on a collision.
const PARTICLE_DAMPENING_FACTOR: f32 = 0.85;

// The maximum velocity of a particle.
pub const VELOCITY_MAX: f32 = 300.0;

// Smoothing radius for smoothing kernel.
// Defines how far from a point we consider for particle interactions.
// Also defines how large chunks are.
pub const SMOOTHING_RADIUS: f32 = 70.0;

// Max 60fps for simulation step
// TODO: Make this adapt to display refresh rate
const DELTA_TIME_MAX: f32 = 1.0 / 60.0;

use std::sync::RwLock;

/* -- Imports -- */
// Bevy imports
use bevy::{math::Vec3A, prelude::*, time::Time, window::Window};

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
    pub read_chunk_map: ChunkMap,
    pub write_chunk_map: ChunkMap,
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
        position: &(f32, f32),
        win_dimensions: &(f32, f32),
    ) -> (usize, usize) {
        let chunk_x = (((position.0 + (win_dimensions.0 / 2.0)) / (SMOOTHING_RADIUS * 2.0))
            as usize)
            .clamp(0, self.dim_x - 1);
        let chunk_y = (((-position.1 + (win_dimensions.1 / 2.0)) / (SMOOTHING_RADIUS * 2.0))
            as usize)
            .clamp(0, self.dim_y - 1);
        (chunk_x, chunk_y)
    }

    pub fn resize(&mut self, new_dim_x: usize, new_dim_y: usize) {
        // If so, create a new chunk map
        self.dim_x = new_dim_x.max(1);
        self.dim_y = new_dim_y.max(1);
        self.chunks = (0..(new_dim_x * new_dim_y).max(1))
            .map(|_| RwLock::new(Vec::new()))
            .collect();
    }

    /// Runs a .clear() operation on each chunk in the ChunkMap.
    pub fn clear_chunks(&mut self) {
        self.chunks.iter_mut().for_each(|chunk| {
            if let Ok(mut chunk_lock) = chunk.write() {
                chunk_lock.clear();
            }
        });
    }

    /// Distributes
    pub fn distribute_particles(
        &mut self,
        query: &mut Query<(Entity, &mut Particle, &mut Transform)>,
        win_dimensions: &(f32, f32),
    ) {
        // Write the current data about the particles to the chunk map
        query.iter().for_each(|(id, particle, _)| {
            // Grab the coordinates of the particle
            let chunk_coord = self.get_chunk_coordinates(
                &(particle.pos.translation.x, particle.pos.translation.y),
                win_dimensions,
            );
            // Calculate the index of the chunk
            let index = chunk_coord.0 + (chunk_coord.1 * self.dim_x);
            // Grab the chunk and write the particle to it
            if let Some(chunk) = self.chunks.get(index) {
                let mut chunk_lock = chunk.write().unwrap(); // handle locking
                chunk_lock.push((id, particle.pos, particle.velocity.clone()));
            }
        });
    }
}

/* -- Helper functions -- */

/// Detects if a particle is outside of the window, reverses it velocity, and moves it back into the window.
fn border_collision(particle: &mut Particle, win_dimensions: &(f32, f32)) {
    let win_width = win_dimensions.0;
    let win_height = win_dimensions.1;

    let radius = particle.collider.radius;

    // Particle is to the right edge of the window
    if particle.pos.translation.x > win_width / 2.0 - radius {
        particle.pos.translation.x = win_width / 2.0 - radius;
        particle.velocity.vec[0] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }

    // Particle is to the left edge of the window
    if particle.pos.translation.x < -1.0 * win_width / 2.0 + radius {
        particle.pos.translation.x = -1.0 * win_width / 2.0 + radius;
        particle.velocity.vec[0] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }

    // Particle is above the window
    if particle.pos.translation.y > win_height / 2.0 - radius {
        particle.pos.translation.y = win_height / 2.0 - radius;
        particle.velocity.vec[1] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }

    // Particle is below the window.
    if particle.pos.translation.y < -1.0 * win_height / 2.0 + radius {
        particle.pos.translation.y = -1.0 * win_height / 2.0 + radius;
        particle.velocity.vec[1] *= -1.0 * PARTICLE_DAMPENING_FACTOR;
    }
}

/// Uses chunking and the position of a particle to return indices of neighboring chunks.
pub fn get_nearby_chunks(
    position: &(f32, f32),
    chunk_map: &ChunkMap,
    window_dims: &(f32, f32),
) -> Vec<usize> {
    let mut nearby_chunks: Vec<usize> = Vec::with_capacity(9); // Still preallocate space for 9 chunks.

    // Get the current chunk coordinates as usize, then convert them to i32 for arithmetic.
    let chunk_coords_usize = chunk_map.get_chunk_coordinates(position, window_dims);
    let (chunk_x, chunk_y) = (chunk_coords_usize.0 as i32, chunk_coords_usize.1 as i32);

    // Define the range of x and y positions to check around the current chunk.
    let x_range = (chunk_x - 1).max(0)..=(chunk_x + 1).min(chunk_map.dim_x as i32 - 1);
    let y_range = (chunk_y - 1).max(0)..=(chunk_y + 1).min(chunk_map.dim_y as i32 - 1);

    // Iterate over the 3x3 grid of chunks around the current chunk within valid bounds.
    for y in y_range {
        for x in x_range.clone() {
            nearby_chunks.push((x + y * chunk_map.dim_x as i32) as usize);
        }
    }

    nearby_chunks
}

/// Returns the force generated by the proximity of two particles.
pub fn calculate_force(pos1: Vec3, pos2: Vec3) -> Vec2 {
    let distance = (pos2 - pos1).length();

    // Ignore stacked cases and particles outside of the influence of a particle.
    if distance > SMOOTHING_RADIUS {
        return Vec2::ZERO;
    }

    let mut force = match (pos1 - pos2).try_normalize() {
        Some(x) => x,
        None => Vec3::new(rand::random::<f32>(), rand::random::<f32>(), 0.0).normalize(),
    };

    // Vector pointing from pos1 to pos2.
    // We normalize and then apply a force function
    // based on the distance between the particles.
    force *= (distance - SMOOTHING_RADIUS).powi(2);
    // Reduce the force generated so we have
    // less chaotic particles.
    Vec2::new(force.x, force.y)
}

/* -- Simulation -- */

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
    let win_resolution = &window.single().resolution;

    let win_width = win_resolution.width();
    let win_height = win_resolution.height();

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

    // Check if the chunk map has changed and resize it
    if chunk_map_write.dim_x != chunks_dim_x || chunk_map_write.dim_y != chunks_dim_y {
        chunk_map_write.resize(chunks_dim_x, chunks_dim_y);
    } else {
        // Clear the output chunk map
        chunk_map_write.clear_chunks();
    }

    chunk_map_write.distribute_particles(&mut query, &win_dimensions);

    // Grab the time since the last frame, using a const value for the min physics time
    let delta_seconds = DELTA_TIME_MAX.min(time.delta_seconds());

    query
        .par_iter_mut()
        .for_each(|(id, mut particle, mut render_pos)| {
            let chunks_in_range: Vec<usize> = get_nearby_chunks(
                &(particle.pos.translation.x, particle.pos.translation.y),
                chunk_map_read,
                &win_dimensions,
            );

            for chunk_index in chunks_in_range {
                if let Some(chunk) = chunk_map_read.chunks.get(chunk_index) {
                    // Perform collision detection
                    let chunk_lock = chunk.read().unwrap();
                    for (other_id, other_pos, other_velocity) in chunk_lock.iter() {
                        if id.index() == other_id.index() {
                            continue;
                        }
                        // Apply fluid dispersion force.
                        let force =
                            calculate_force(particle.pos.translation, other_pos.translation);
                        particle.velocity.vec[0] += force.x * delta_seconds;
                        particle.velocity.vec[1] += force.y * delta_seconds;

                        let diff_pos: Vec3A =
                            (particle.pos.translation - other_pos.translation).into();
                        let diff_velocity: Vec3A =
                            (particle.velocity.vec - other_velocity.vec).into();

                        // .max() is used for the case of total overlap (distance is 0)
                        let distance = diff_pos.length().max(0.1);

                        // TODO: Fix collider being reused!
                        if distance < (particle.collider.radius + particle.collider.radius) / 2.0
                            && diff_pos.dot(-diff_velocity) > 0.0
                        {
                            let m1 = particle.collider.radius.powi(2);
                            let m2 = particle.collider.radius.powi(2);

                            let m = m1 + m2;

                            let d = distance.powi(2);

                            let u1: Vec3A = Vec3A::from(particle.velocity.vec)
                                - ((m2 * 2.0 / m)
                                    * ((diff_velocity).dot(diff_pos) / d)
                                    * (diff_pos));

                            particle.velocity.vec = u1.into();
                        }
                    }
                }
            }

            // Max velocity check
            if particle.velocity.vec.length() > VELOCITY_MAX {
                let scalar = VELOCITY_MAX / particle.velocity.vec.length();
                particle.velocity.vec *= scalar;
            }

            // Move by the velocity we've stored.
            particle.pos.translation.x += particle.velocity.vec[0] * delta_seconds;
            particle.pos.translation.y += particle.velocity.vec[1] * delta_seconds;

            // Check for border collision
            border_collision(&mut particle, &win_dimensions);

            render_pos.translation = particle.pos.translation;
        });

    chunk_map_double_buffer.swap();
}
