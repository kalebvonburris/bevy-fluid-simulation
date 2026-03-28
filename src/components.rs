use bevy::prelude::*;

#[derive(Resource)]
pub struct SimConfig {
    pub damping: f32,
    pub max_speed: f32,
    pub smoothing_radius: f32,
    pub max_dt: f32,
    pub particle_radius: f32,
    pub grid_spawn_size: u32,
    pub spawn_spacing: f32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            damping: 0.85,
            max_speed: 500.0,
            smoothing_radius: 30.0,
            max_dt: 1.0 / 60.0,
            particle_radius: 3.25,
            grid_spawn_size: 100,
            spawn_spacing: 2.0,
        }
    }
}

#[derive(Component, Default)]
pub struct Particle;

#[derive(Component, Default, Clone)]
pub struct Velocity(pub Vec2);
