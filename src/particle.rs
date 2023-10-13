// particle.rs
// Kaleb Burris
// 10-12-2023
// The necessary components to simulate fluid dynamics using particles.


use bevy::{
    prelude::{Component, Res, Vec2},
    time::Time,
};

/// A 2 coordinate point representing
/// where a particle is in space.
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}
/// A particle of fluid.
#[derive(Component)]
pub struct Particle {
    coordinate: Coordinate,
    velocity: Vec2,
}

impl Particle {
    fn update_position(&mut self, time_step: f64) {
        self.coordinate.x += self.velocity.x as f64 * time_step;
        self.coordinate.y += self.velocity.y as f64 * time_step;
    }
}

pub fn particle_system(time: Res<Time>) {
    //println!("Time: {:?}", time);
}
