// particle.rs
// Kaleb Burris
// 10-12-2023
// The necessary components to simulate fluid dynamics using particles.

use bevy::prelude::Component;

/// A 2 coordinate point representing
/// where a particle is in space.
struct Coordinate {
    x: f64,
    y: f64,
}

/// A 2 coordinate vector representing
/// x and y magnitudes.
struct Vec2(f64, f64);

impl Vec2 {
    /// Returns the manitude of the vector.
    fn mag(&self) -> f64 {
        ((self.0 * self.0) + ( self.1 * self.1)).sqrt()
    }

    /// Returns the normalized unit vector.
    fn normalize(&self) -> Vec2 {
        let magnitude = self.mag();
        Vec2(self.0 / magnitude, self.1 / magnitude)
    }
}

/// A particle of fluid.
#[derive(Component)]
pub struct Particle {
    coordinate: Coordinate,
    velocity: Vec2,
}

impl Particle { /* TODO! */}

pub fn particle_system() {
    todo!()
}