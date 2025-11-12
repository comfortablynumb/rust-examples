// Compute shader for particle simulation
// This runs on the GPU and updates particle positions/velocities

struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
    color: vec4<f32>,
}

struct SimParams {
    delta_time: f32,
    time: f32,
}

@group(0) @binding(0)
var<uniform> params: SimParams;

@group(0) @binding(1)
var<storage, read> particles_src: array<Particle>;

@group(0) @binding(2)
var<storage, read_write> particles_dst: array<Particle>;

// Simple hash function for pseudo-random numbers
fn hash(value: u32) -> u32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ (state >> 16u);
    state = state * 2654435769u;
    state = state ^ (state >> 16u);
    state = state * 2654435769u;
    return state;
}

fn random(seed: u32) -> f32 {
    return f32(hash(seed)) / 4294967295.0;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    // Bounds check
    if (index >= arrayLength(&particles_src)) {
        return;
    }

    var particle = particles_src[index];

    // Update position
    particle.position = particle.position + particle.velocity * params.delta_time;

    // Bounce off walls
    if (abs(particle.position.x) > 1.0) {
        particle.velocity.x = -particle.velocity.x;
        particle.position.x = sign(particle.position.x);
    }
    if (abs(particle.position.y) > 1.0) {
        particle.velocity.y = -particle.velocity.y;
        particle.position.y = sign(particle.position.y);
    }

    // Add some gravitational attraction to center
    let center = vec2<f32>(0.0, 0.0);
    let to_center = center - particle.position;
    let dist = length(to_center);
    if (dist > 0.01) {
        let gravity_strength = 0.1;
        let gravity = normalize(to_center) * gravity_strength * params.delta_time;
        particle.velocity = particle.velocity + gravity;
    }

    // Add slight damping
    particle.velocity = particle.velocity * 0.99;

    // Limit maximum speed
    let max_speed = 1.0;
    let speed = length(particle.velocity);
    if (speed > max_speed) {
        particle.velocity = normalize(particle.velocity) * max_speed;
    }

    // Pulse the color based on speed and time
    let speed_factor = speed / max_speed;
    let time_pulse = sin(params.time * 2.0 + f32(index) * 0.1) * 0.5 + 0.5;
    particle.color.a = mix(0.3, 1.0, speed_factor * time_pulse);

    // Write updated particle
    particles_dst[index] = particle;
}
