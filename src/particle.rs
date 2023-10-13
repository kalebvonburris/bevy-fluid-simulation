// particle.rs
// Kaleb Burris
// 10-12-2023
// The necessary components to simulate fluid dynamics using particles.

/// A 2 coordinate point representing
/// where a particle is in space.
struct Coordinate {
    x: f64,
    y: f64,
}

/// A 2 coordinate vector representing
/// x and y magnitudes.
struct Vec2(f64, f64);

/// A particle of fluid.
pub struct Particle {
    coordinate: Coordinate,
    velocity: Vec2,
}