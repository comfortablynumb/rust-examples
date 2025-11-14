# Compute Shaders - Particle System

This example demonstrates using compute shaders for GPU-accelerated particle simulation. We simulate 10,000 particles entirely on the GPU using compute shaders and storage buffers.

## What This Example Demonstrates

1. **Compute Shaders**
   - GPU-accelerated computation
   - Parallel processing
   - Non-graphics GPU work
   - Workgroup dispatch

2. **Storage Buffers**
   - Read/write buffers in shaders
   - Large data arrays
   - Ping-pong buffering
   - Buffer barriers

3. **Particle Simulation**
   - Position updates
   - Velocity integration
   - Collision detection
   - Force application

4. **Compute-Render Integration**
   - Compute pass followed by render pass
   - Sharing buffers between pipelines
   - Synchronization

## Compute Shader Architecture

### Workgroups

Compute shaders execute in parallel workgroups:

```wgsl
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    // Process particle at index
}
```

**Workgroup size**: 64 threads per workgroup
**Dispatch**: `num_particles / 64` workgroups

### Storage Buffers

Unlike uniforms (read-only, small), storage buffers are:
- Large (can hold millions of elements)
- Read-write capable
- Used for bulk data processing

```wgsl
@group(0) @binding(1)
var<storage, read> particles_src: array<Particle>;

@group(0) @binding(2)
var<storage, read_write> particles_dst: array<Particle>;
```

### Ping-Pong Pattern

To avoid read-write hazards, we use two buffers:

```
Frame 0: Read from Buffer A, Write to Buffer B
Frame 1: Read from Buffer B, Write to Buffer A
Frame 2: Read from Buffer A, Write to Buffer B
...
```

This ensures we never read and write the same buffer simultaneously.

## Physics Simulation

### Position Integration

```wgsl
particle.position += particle.velocity * delta_time;
```

Basic Euler integration - simple but effective for particles.

### Boundary Collision

```wgsl
if (abs(particle.position.x) > 1.0) {
    particle.velocity.x = -particle.velocity.x;
    particle.position.x = sign(particle.position.x);
}
```

Bounce particles off screen edges with perfect reflection.

### Gravitational Attraction

```wgsl
let to_center = center - particle.position;
let gravity = normalize(to_center) * gravity_strength;
particle.velocity += gravity;
```

Pulls particles toward the center, creating interesting patterns.

## Performance

### Why Compute Shaders?

**CPU Simulation (10,000 particles):**
- ~10-30 FPS (serial processing)
- Cache misses
- Limited by single-thread performance

**GPU Simulation (10,000 particles):**
- 60+ FPS (parallel processing)
- Thousands of threads
- Optimized memory access

### Optimization Tips

1. **Workgroup Size**: 64-256 is typical, hardware-dependent
2. **Memory Access**: Coalesced reads/writes are faster
3. **Barriers**: Minimize workgroup synchronization
4. **Atomics**: Avoid if possible (slow)

## Compute Pass Flow

```
1. Update uniforms (delta time, etc.)
2. Begin compute pass
3. Set compute pipeline
4. Set bind group (buffers)
5. Dispatch workgroups
6. End compute pass
7. Begin render pass
8. Render particles
9. End render pass
```

## Use Cases for Compute Shaders

- **Particle Systems**: Smoke, fire, rain, explosions
- **Physics**: Cloth simulation, fluid dynamics
- **Image Processing**: Blur, convolution, filters
- **Ray Tracing**: Path tracing, ambient occlusion
- **AI/ML**: Neural networks, matrix operations
- **Procedural Generation**: Terrain, textures

## Building and Running

```bash
cargo run
```

You'll see 10,000 particles swirling around with gravitational attraction to the center.

## Common Issues

**Particles disappear?**
- Check buffer ping-pong logic
- Verify array bounds in shader
- Ensure workgroup dispatch covers all particles

**Slow performance?**
- Reduce particle count
- Optimize workgroup size
- Check for synchronization overhead

**Validation errors?**
- Storage buffers need STORAGE usage flag
- Compute and render passes need proper barriers
- Buffer sizes must match array lengths

## Advanced Topics

### Shared Memory

Workgroups can share memory:

```wgsl
var<workgroup> shared_data: array<f32, 64>;
```

Useful for reduction operations, prefix sums, etc.

### Barriers

Synchronize threads within a workgroup:

```wgsl
workgroupBarrier();
```

Ensures all threads reach this point before continuing.

### Atomic Operations

Thread-safe modifications:

```wgsl
atomicAdd(&counter, 1u);
```

Slower than regular operations but necessary for concurrent writes.

## Further Reading

- [WebGPU Compute Shader Spec](https://www.w3.org/TR/webgpu/#compute-pass)
- [GPU Gems: Parallel Algorithms](https://developer.nvidia.com/gpugems/gpugems3/part-vi-gpu-computing)
- [Compute Shader Best Practices](https://developer.nvidia.com/blog/cuda-pro-tip-write-flexible-kernels-grid-stride-loops/)
