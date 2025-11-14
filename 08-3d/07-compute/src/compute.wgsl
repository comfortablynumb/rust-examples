// Compute shader for particle physics simulation

struct Particle {
    position: vec3<f32>,
    _padding1: f32,
    velocity: vec3<f32>,
    _padding2: f32,
    color: vec4<f32>,
    life: f32,
    _padding3: vec3<f32>,
};

struct Uniforms {
    delta_time: f32,
    time: f32,
    _padding: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var<storage, read> particles_in: array<Particle>;

@group(0) @binding(2)
var<storage, read_write> particles_out: array<Particle>;

// Simplenoise/hash function for pseudo-random values
fn hash(n: f32) -> f32 {
    return fract(sin(n) * 43758.5453123);
}

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;

    // Bounds check
    if (index >= arrayLength(&particles_in)) {
        return;
    }

    var particle = particles_in[index];

    // Update velocity with gravity
    let gravity = vec3<f32>(0.0, -9.8, 0.0);
    particle.velocity += gravity * uniforms.delta_time;

    // Update position
    particle.position += particle.velocity * uniforms.delta_time;

    // Decrease life
    particle.life -= uniforms.delta_time;

    // Reset particle if dead
    if (particle.life <= 0.0) {
        // Use particle index and time for deterministic randomness
        let seed = f32(index) + uniforms.time;
        let theta = hash(seed) * 6.28318530718; // TAU
        let phi = hash(seed + 1.0) * 3.14159265359; // PI
        let speed = hash(seed + 2.0) * 2.0 + 1.0;

        particle.position = vec3<f32>(0.0, 0.0, 0.0);
        particle.velocity = vec3<f32>(
            cos(theta) * sin(phi) * speed,
            cos(phi) * speed,
            sin(theta) * sin(phi) * speed,
        );
        particle.life = hash(seed + 3.0) * 5.0 + 2.0;
        particle.color = vec4<f32>(
            hash(seed + 4.0),
            hash(seed + 5.0),
            hash(seed + 6.0),
            1.0,
        );
    }

    // Write updated particle
    particles_out[index] = particle;
}
