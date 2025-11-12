# wgpu Compute Shader Example - Particle System

Demonstrates compute shaders for GPU-accelerated particle physics simulation. 10,000 particles updated entirely on the GPU.

## Features

- **Compute Shaders**: Physics simulation runs on GPU
- **Storage Buffers**: Read/write particle data
- **Ping-Pong Buffers**: Double buffering for compute
- **Particle System**: Gravity, lifetime, respawning
- **Billboard Rendering**: Particles face camera

## Key Concepts

### Compute Shaders
Unlike vertex/fragment shaders, compute shaders are general-purpose GPU programs. They can:
- Read and write arbitrary buffers
- Perform parallel computations
- Don't render directly

### Workgroups
Compute work is divided into workgroups:
- `@workgroup_size(256)`: 256 threads per group
- `dispatch_workgroups(N, 1, 1)`: Launch N workgroups
- Total threads = workgroup_size × workgroups

### Storage Buffers
Storage buffers allow random read/write access:
- `@storage, read`: Read-only
- `@storage, read_write`: Can modify data
- Larger than uniform buffers

### Ping-Pong Buffering
Two buffers alternate as input/output:
- Frame 1: Read A, Write B
- Frame 2: Read B, Write A
- Prevents read/write conflicts

## Building

```bash
cargo run
```

Watch 10,000 particles fountain from the origin with gravity!

## Performance

All physics runs on GPU - extremely fast for thousands of particles.
