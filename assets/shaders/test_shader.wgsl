// A struct to hold the position and velocity data of a givent particle
struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
    radius: f32,
}

struct SimulationWorld {
    delta_time: f32,
}

@group(0) @binding(0)
var<storage, read_write> particles: array<Particle>;

// We defind the mass of a particle as its radius squared
fn mass(particle: Particle) -> f32 {
    return particle.radius * particle.radius;
}

fn init() {

}

@compute @workgroup_size(16, 1, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>,@builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let particle = particles[invocation_id.x];
    for (var i: u32 = 0; i < particles.arrayLength(); i++) {
        if i == invocation_id.x { continue; } // Don't apply physics to an object that's itself
        
    }
}