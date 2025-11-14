# Multiple Objects with Instancing

This example demonstrates rendering many objects efficiently using GPU instancing. We render 100 cubes with different positions, scales, colors, and rotations using a single draw call.

## What This Example Demonstrates

1. **GPU Instancing**
   - Instance buffers
   - Per-instance attributes
   - VertexStepMode::Instance
   - Single draw call for many objects

2. **Instance Attributes**
   - Position per instance
   - Scale per instance
   - Color per instance
   - Rotation per instance

3. **Performance Optimization**
   - Batching similar objects
   - Reducing draw calls
   - GPU-side transformations
   - Efficient memory usage

4. **Random Generation**
   - Procedural object placement
   - Random properties
   - Scene population

## Instancing Explained

### Without Instancing

```rust
for object in objects {
    update_uniforms(object.transform);
    render_pass.draw(...);  // One draw call per object
}
```

**100 objects = 100 draw calls** (slow!)

### With Instancing

```rust
render_pass.set_vertex_buffer(1, instance_buffer);
render_pass.draw_indexed(..., 0..100);  // One draw call for all!
```

**100 objects = 1 draw call** (fast!)

## Instance Buffer Layout

```rust
#[repr(C)]
struct Instance {
    position: [f32; 3],
    scale: f32,
    color: [f32; 3],
    rotation: f32,
}
```

Each instance gets its own data, allowing unique transformations and materials.

## Building and Running

```bash
cargo run
```

You'll see 100 cubes rotating and arranged randomly in 3D space.

## Performance Benefits

- **Draw calls**: 100 objects in 1 draw call vs. 100 separate calls
- **CPU overhead**: Minimal CPU work per object
- **GPU efficiency**: GPU processes instances in parallel
- **Scalability**: Can render thousands of objects efficiently

## Use Cases

- Particle systems
- Vegetation/foliage rendering
- Crowds
- Repeated architectural elements
- Procedural content

## Further Reading

- [OpenGL Instancing](https://learnopengl.com/Advanced-OpenGL/Instancing)
- [GPU Performance Tips](https://developer.nvidia.com/blog/opengl-performance-tips-vertex-buffer-object/)
