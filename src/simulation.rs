//! Spatial hashing and physics systems for the particle simulation.

use bevy::prelude::*;

use crate::components::{Particle, SimConfig, Velocity};

#[derive(Clone, Copy)]
struct ParticleSnapshot {
    entity: Entity,
    pos: Vec2,
    vel: Vec2,
}

#[derive(Resource, Default)]
pub struct SpatialGrid {
    cells: Vec<Vec<ParticleSnapshot>>,
    dim_x: usize,
    dim_y: usize,
    cell_size: f32,
    origin: Vec2,
}

impl SpatialGrid {
    fn cell_index(&self, pos: Vec2) -> usize {
        let local = pos - self.origin;
        let cx = (local.x / self.cell_size).floor() as isize;
        let cy = (local.y / self.cell_size).floor() as isize;
        let cx = cx.clamp(0, self.dim_x as isize - 1) as usize;
        let cy = cy.clamp(0, self.dim_y as isize - 1) as usize;
        cy * self.dim_x + cx
    }

    fn neighbors(&self, pos: Vec2) -> impl Iterator<Item = &ParticleSnapshot> {
        let local = pos - self.origin;
        let cx = (local.x / self.cell_size).floor() as isize;
        let cy = (local.y / self.cell_size).floor() as isize;

        // Offsets for the 3×3 neighborhood.
        const OFFSETS: [(isize, isize); 9] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let dim_x = self.dim_x as isize;
        let dim_y = self.dim_y as isize;

        OFFSETS.iter().flat_map(move |&(dx, dy)| {
            let nx = cx + dx;
            let ny = cy + dy;
            if nx >= 0 && nx < dim_x && ny >= 0 && ny < dim_y {
                let idx = ny as usize * self.dim_x + nx as usize;
                self.cells[idx].as_slice()
            } else {
                &[]
            }
        })
    }
}

pub fn build_spatial_grid(
    config: Res<SimConfig>,
    window: Single<&Window>,
    query: Query<(Entity, &Transform, &Velocity), With<Particle>>,
    mut grid: ResMut<SpatialGrid>,
) {
    let (w, h) = (window.width(), window.height());
    if w * h == 0.0 {
        return;
    }

    let cell_size = config.smoothing_radius;
    let dim_x = (w / cell_size).ceil() as usize;
    let dim_y = (h / cell_size).ceil() as usize;
    let total = dim_x * dim_y;

    grid.cell_size = cell_size;
    grid.dim_x = dim_x;
    grid.dim_y = dim_y;
    grid.origin = Vec2::new(-w / 2.0, -h / 2.0);

    // Adjust grid capacity and clear all cells for the new frame.
    grid.cells.truncate(total);
    grid.cells.resize_with(total, Vec::new);
    grid.cells.iter_mut().for_each(|c| c.clear());

    for (entity, transform, velocity) in &query {
        let pos = transform.translation.xy();
        let idx = grid.cell_index(pos);
        grid.cells[idx].push(ParticleSnapshot {
            entity,
            pos,
            vel: velocity.0,
        });
    }
}

pub fn apply_forces(
    config: Res<SimConfig>,
    time: Res<Time>,
    grid: Res<SpatialGrid>,
    mut query: Query<(Entity, &Transform, &mut Velocity), With<Particle>>,
) {
    let dt = config.max_dt.min(time.delta_secs());
    let sr = config.smoothing_radius;
    let sr_sq = sr * sr;
    let radius = config.particle_radius;
    // Original sim collides when distance < one particle radius.
    let collision_threshold_sq = radius * radius;

    query
        .par_iter_mut()
        .for_each(|(entity, transform, mut velocity)| {
            let pos = transform.translation.xy();
            let mut force_accum = Vec2::ZERO;
            let mut vel_adjust = Vec2::ZERO;

            for other in grid.neighbors(pos) {
                if other.entity == entity {
                    continue;
                }

                let diff = pos - other.pos;
                let dist_sq = diff.length_squared();

                // --- Repulsive force (within smoothing radius) ---
                if dist_sq < sr_sq && dist_sq > f32::EPSILON {
                    let dist = dist_sq.sqrt();
                    let dir = diff / dist;
                    // Quadratic falloff: (dist - radius)^2
                    let strength = (dist - sr) * (dist - sr);
                    force_accum += dir * strength;
                } else if dist_sq <= f32::EPSILON {
                    // Overlapping — nudge in a random direction.
                    let angle = rand::random::<f32>() * std::f32::consts::TAU;
                    force_accum += Vec2::new(angle.cos(), angle.sin()) * sr_sq;
                }

                // --- Elastic collision ---
                if dist_sq < collision_threshold_sq && dist_sq > f32::EPSILON {
                    let diff_velocity = velocity.0 - other.vel;
                    // Only resolve if particles are approaching.
                    if diff.dot(-diff_velocity) > 0.0 {
                        // Equal-mass elastic: v' = v - dot(dv, dp) / |dp|^2 * dp
                        let impulse = diff_velocity.dot(diff) / dist_sq;
                        vel_adjust -= impulse * diff;
                    }
                }
            }

            velocity.0 += force_accum * dt;
            velocity.0 += vel_adjust;
        });
}

pub fn integrate(
    config: Res<SimConfig>,
    time: Res<Time>,
    window: Single<&Window>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Particle>>,
) {
    let dt = config.max_dt.min(time.delta_secs());
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;
    let r = config.particle_radius;
    let max_speed = config.max_speed;
    let damping = config.damping;

    query
        .par_iter_mut()
        .for_each(|(mut transform, mut velocity)| {
            // Clamp speed.
            let speed_sq = velocity.0.length_squared();
            if speed_sq > max_speed * max_speed {
                velocity.0 *= max_speed / speed_sq.sqrt();
            }

            // Integrate position.
            transform.translation.x += velocity.0.x * dt;
            transform.translation.y += velocity.0.y * dt;

            // Boundary collisions (reflect + damp).
            let min_x = -half_w + r;
            let max_x = half_w - r;
            let min_y = -half_h + r;
            let max_y = half_h - r;

            if transform.translation.x > max_x {
                transform.translation.x = max_x;
                velocity.0.x *= -damping;
            } else if transform.translation.x < min_x {
                transform.translation.x = min_x;
                velocity.0.x *= -damping;
            }

            if transform.translation.y > max_y {
                transform.translation.y = max_y;
                velocity.0.y *= -damping;
            } else if transform.translation.y < min_y {
                transform.translation.y = min_y;
                velocity.0.y *= -damping;
            }
        });
}
