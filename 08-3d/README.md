# wgpu 3D Graphics Examples

Comprehensive examples demonstrating modern 3D graphics programming with wgpu (WebGPU API for Rust). These examples progress from basic rendering to advanced techniques.

## Overview

This collection provides a complete learning path for 3D graphics programming:

1. **Fundamentals**: Understanding the graphics pipeline
2. **3D Mathematics**: Transformations, matrices, projections
3. **Lighting**: Realistic surface illumination
4. **Interactivity**: Camera controls and user input
5. **Scene Management**: Multiple objects and materials
6. **GPU Compute**: Parallel computation on the GPU
7. **Advanced Techniques**: Shadow mapping and more

## Examples

### 01-triangle: Basic wgpu Setup
**Concepts**: Render pipeline, shaders (WGSL), surface configuration, render loop

Learn the foundational wgpu workflow:
- GPU initialization (instance, adapter, device, queue)
- Surface configuration for window rendering
- Shader compilation and pipeline creation
- Command encoding and submission
- Basic WGSL vertex and fragment shaders

```bash
cd 01-triangle && cargo run
```

**Output**: A colored triangle with RGB gradient

---

### 02-buffers-indices: Vertex and Index Buffers
**Concepts**: Vertex buffers, index buffers, interleaved attributes, buffer layouts

Efficient geometry rendering:
- Creating vertex buffers on the GPU
- Index buffers for vertex reuse
- Interleaved vertex attributes (position + color)
- Vertex buffer layout descriptors
- bytemuck for safe type conversion

```bash
cd 02-buffers-indices && cargo run
```

**Output**: A colored square rendered with vertex and index buffers

---

### 03-textures: Texture Loading and Sampling
**Concepts**: Texture creation, samplers, bind groups, UV coordinates

Loading and displaying textures:
- Loading images from files
- Creating GPU textures from image data
- Texture samplers (filtering, addressing)
- Bind groups for resource binding
- UV texture coordinates

```bash
cd 03-textures && cargo run
```

**Output**: A textured quad (or checkerboard if no texture file provided)

---

### 04-3d-cube: 3D Transformations
**Concepts**: Model-View-Projection matrices, uniform buffers, depth testing

Enter 3D space:
- Coordinate space transformations
- Model, View, and Projection matrices
- MVP matrix combination with cgmath
- Uniform buffers for per-frame data
- Depth testing for correct occlusion
- Animated rotation

```bash
cd 04-3d-cube && cargo run
```

**Output**: A spinning colored cube with proper depth

---

### 05-camera: Camera Controls
**Concepts**: Interactive camera, keyboard/mouse input, delta time, camera matrices

Make the scene interactive:
- First-person camera system
- WASD keyboard controls for movement
- Mouse look for camera rotation
- Frame-rate independent movement with delta time
- Proper camera matrix calculations

```bash
cd 05-camera && cargo run
```

**Output**: Interactive 3D scene with WASD + mouse camera controls

**Controls**:
- W/A/S/D: Move forward/left/back/right
- Space/Shift: Move up/down
- Mouse: Look around
- Escape: Exit

---

### 06-lighting: Phong Shading
**Concepts**: Surface normals, Phong/Blinn-Phong shading, lighting uniforms

Realistic lighting:
- Per-vertex normal vectors
- Phong lighting model (ambient + diffuse + specular)
- Light position and color uniforms
- Per-fragment lighting calculations
- Animated light source
- Material properties (shininess, color)

```bash
cd 06-lighting && cargo run
```

**Output**: A rotating cube with dynamic Phong lighting

---

### 07-multiple-objects: GPU Instancing
**Concepts**: Instance buffers, per-instance attributes, efficient batching

Render many objects efficiently:
- GPU instancing for multiple objects
- Instance buffers with per-instance data
- Single draw call for hundreds of objects
- Per-instance transformations (position, rotation, scale)
- Random object generation

```bash
cd 07-multiple-objects && cargo run
```

**Output**: 100 rotating cubes with different positions, sizes, and colors

---

### 08-compute-shader: Particle System
**Concepts**: Compute shaders, storage buffers, workgroups, GPU physics

GPU-accelerated computation:
- Compute shader pipelines
- Storage buffers (read/write)
- Workgroup dispatch and sizing
- Ping-pong buffer technique
- GPU particle physics simulation (10,000 particles)
- Compute-render integration

```bash
cd 08-compute-shader && cargo run
```

**Output**: 10,000 particles with GPU-computed physics and gravitational attraction

---

## Prerequisites

- Rust 1.70 or later
- GPU with Vulkan, Metal, DX12, or WebGPU support
- Window system (X11, Wayland, Windows, macOS)

## Building All Examples

```bash
# Build all examples
for dir in 0*/; do
    cd "$dir" && cargo build --release && cd ..
done

# Run a specific example
cd 03-3d-cube && cargo run --release
```

## Project Structure

Each example is a complete Cargo project:

```
0X-example-name/
├── Cargo.toml          # Dependencies
├── README.md           # Detailed explanation
└── src/
    ├── main.rs         # Rust application code
    └── shader.wgsl     # WGSL shaders (or compute.wgsl, etc.)
```

## Learning Path

**Beginners**: Start with 01-triangle, work through in order

**Intermediate**: Skip to 03-3d-cube if you know basic graphics

**Advanced**: Jump to specific topics (compute, shadows)

## Key wgpu Concepts

### Graphics Pipeline

```
Vertex Input → Vertex Shader → Primitive Assembly → Rasterization
→ Fragment Shader → Depth/Stencil Test → Blending → Framebuffer
```

### Resource Binding

- **Bind Groups**: Collections of resources (textures, samplers, buffers)
- **Bind Group Layouts**: Describe expected resource types
- **Pipeline Layout**: Defines all bind groups used by a pipeline

### Shader Stages

- **Vertex Shader**: Processes vertices, outputs clip-space positions
- **Fragment Shader**: Colors pixels
- **Compute Shader**: General-purpose GPU computation

### Coordinate Spaces

1. **Model/Local Space**: Object's own coordinate system
2. **World Space**: Scene coordinate system
3. **View/Camera Space**: Relative to camera
4. **Clip Space**: After projection, normalized device coordinates
5. **Screen Space**: Final pixel coordinates

## Common Patterns

### Render Loop

```rust
loop {
    update();         // Update state (camera, objects)
    render();         // Record and submit GPU commands
    present();        // Display frame
}
```

### Uniform Buffer Updates

```rust
// CPU: Update uniforms
uniforms.mvp = (projection * view * model).into();

// Upload to GPU
queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
```

### Multi-Pass Rendering

```rust
// Pass 1: Shadow map
{
    let mut shadow_pass = encoder.begin_render_pass(...);
    // Render to shadow texture
}

// Pass 2: Main rendering
{
    let mut render_pass = encoder.begin_render_pass(...);
    // Render with shadows
}

queue.submit(encoder.finish());
```

## Performance Tips

1. **Minimize State Changes**: Group draws by pipeline, bind groups
2. **Use Instancing**: Render many objects in one draw call
3. **Optimize Buffers**: Reuse buffers, avoid unnecessary updates
4. **LOD (Level of Detail)**: Simpler meshes at distance
5. **Frustum Culling**: Don't render off-screen objects
6. **Occlusion Culling**: Don't render hidden objects

## Troubleshooting

**Black screen?**
- Check shader compilation errors
- Verify surface format compatibility
- Enable wgpu validation layer: `RUST_LOG=warn`

**Depth issues?**
- Ensure depth buffer is cleared
- Check depth compare function (usually `Less`)
- Verify near/far plane values

**Slow performance?**
- Profile with `cargo flamegraph`
- Check for CPU-GPU synchronization stalls
- Reduce draw calls via instancing

## Resources

### Official Documentation
- [wgpu Documentation](https://docs.rs/wgpu/)
- [wgpu GitHub](https://github.com/gfx-rs/wgpu)
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [WGSL Specification](https://www.w3.org/TR/WGSL/)

### Tutorials
- [Learn wgpu](https://sotrh.github.io/learn-wgpu/)
- [WebGPU Fundamentals](https://webgpufundamentals.org/)
- [Learn OpenGL](https://learnopengl.com/) (concepts apply to wgpu)

### Mathematics
- [Scratchapixel](https://www.scratchapixel.com/)
- [3D Math Primer](http://www.gameenginebook.com/)
- [cgmath Documentation](https://docs.rs/cgmath/)

### Community
- [wgpu Matrix Chat](https://matrix.to/#/#wgpu:matrix.org)
- [Rust GameDev Discord](https://discord.gg/yNtPTb2)

## Next Steps

After completing these examples, explore:

1. **Normal Mapping**: Add surface detail via textures
2. **PBR (Physically Based Rendering)**: Modern material system
3. **Deferred Rendering**: Handle many lights efficiently
4. **Screen-Space Effects**: SSAO, SSR, bloom, etc.
5. **Skeletal Animation**: Character animation
6. **Terrain Rendering**: Heightmaps, LOD, chunking
7. **GPU Culling**: Compute-based frustum culling

## License

MIT License - See LICENSE file in repository root

## Contributing

Found an issue or want to improve an example? Contributions welcome!

---

**Happy rendering!** These examples provide a solid foundation for 3D graphics programming. Experiment, modify, and build upon them to create your own projects.
