# Vertex Buffers and Index Buffers

This example demonstrates how to use vertex buffers and index buffers to efficiently render geometry in wgpu. We render a colored square (quad) using two triangles with shared vertices.

## What This Example Demonstrates

### Core Concepts

1. **Vertex Buffers**
   - Storing vertex data on the GPU
   - Interleaved vertex attributes (position + color)
   - Vertex buffer layout description
   - Using bytemuck for safe type conversion

2. **Index Buffers**
   - Efficient vertex reuse
   - Indexed drawing vs. non-indexed drawing
   - Triangle topology with indices
   - Memory savings with complex geometry

3. **Vertex Attributes**
   - Multiple attributes per vertex
   - Attribute locations and shader bindings
   - Interleaved vs. separate buffer layouts
   - Stride and offset calculations

4. **Buffer Management**
   - Creating GPU buffers with initial data
   - Buffer usage flags (VERTEX, INDEX)
   - Binding buffers to the render pipeline
   - Buffer slices and ranges

## Vertex Buffers vs. Hardcoded Vertices

### Previous Example (Hardcoded)

```rust
// Vertices defined in shader code
let x = f32(1 - i32(in_vertex_index)) * 0.5;
let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
```

**Limitations:**
- Cannot easily modify geometry
- Limited to simple, procedural shapes
- No per-vertex attributes (color, texcoords, normals)
- Not suitable for loading models from files

### This Example (Vertex Buffers)

```rust
// Vertices defined in Rust code
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    // ... more vertices
];
```

**Advantages:**
- Flexible geometry definition
- Multiple attributes per vertex
- Can load from files or generate procedurally
- Standard approach for real applications

## Index Buffers Explained

### Without Index Buffer

To draw a square using two triangles without indices:

```
Vertex 1: top-left
Vertex 2: top-right
Vertex 3: bottom-right
Vertex 4: top-left      (duplicate!)
Vertex 5: bottom-right  (duplicate!)
Vertex 6: bottom-left
```

**Total: 6 vertices** (with 2 duplicates)

### With Index Buffer

```
Vertices:
0: top-left
1: top-right
2: bottom-right
3: bottom-left

Indices: [0, 1, 2, 0, 2, 3]
```

**Total: 4 unique vertices + 6 indices**

### Memory Savings

For a square:
- Without indices: 6 vertices × 24 bytes = 144 bytes
- With indices: 4 vertices × 24 bytes + 6 indices × 2 bytes = 108 bytes
- Savings: **25%**

For complex models with thousands of triangles, savings can be **50-70%** or more, since vertices are typically shared by 6+ triangles on average.

## Interleaved Vertex Layout

Our `Vertex` struct uses an **interleaved layout**:

```
Memory layout: [pos0, color0, pos1, color1, pos2, color2, ...]
```

### Alternative: Separate Buffers

```
Buffer 0: [pos0, pos1, pos2, ...]
Buffer 1: [color0, color1, color2, ...]
```

### Interleaved vs. Separate

**Interleaved (used here):**
- Better cache coherency (related data together)
- Simpler to manage (one buffer)
- Standard for most applications
- Better for vertex shaders that use all attributes

**Separate:**
- Can update one attribute without affecting others
- Better when some attributes update frequently
- More complex to set up
- Can save bandwidth if shader doesn't use all attributes

## Vertex Attribute Format

Our vertex layout uses `Float32x3` format:

```rust
wgpu::VertexAttribute {
    offset: 0,
    shader_location: 0,
    format: wgpu::VertexFormat::Float32x3, // 3× f32
}
```

### Common Formats

- `Float32x2`: 2D positions, UV coordinates
- `Float32x3`: 3D positions, normals, colors
- `Float32x4`: Homogeneous positions, RGBA colors
- `Uint32x4`: Bone indices
- `Float16x2`: Half-precision UVs (saves memory)
- `Sint16x2Norm`: Normalized integer positions

## Buffer Creation

### Using create_buffer_init

```rust
let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("Vertex Buffer"),
    contents: bytemuck::cast_slice(VERTICES),
    usage: wgpu::BufferUsages::VERTEX,
});
```

This is a convenience method that:
1. Creates the buffer
2. Maps it to CPU memory
3. Copies data to it
4. Unmaps it

### Manual Approach

```rust
// Create empty buffer
let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
    size: std::mem::size_of_val(VERTICES) as u64,
    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    mapped_at_creation: false,
});

// Write data
queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(VERTICES));
```

This approach is useful for buffers that will be updated frequently.

## Indexed Drawing

### Draw Call Comparison

**Non-indexed:**
```rust
render_pass.draw(vertices, instances);
```

**Indexed (used here):**
```rust
render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
render_pass.draw_indexed(indices, base_vertex, instances);
```

### Index Format

- `Uint16`: Indices 0-65,535 (saves memory)
- `Uint32`: Indices 0-4,294,967,295 (more vertices)

Use `Uint16` unless you have more than 65k vertices in a single buffer.

## bytemuck for Safe Type Conversion

The `bytemuck` crate allows safe casting of Rust types to byte slices:

```rust
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
```

Requirements:
- `#[repr(C)]`: Guaranteed memory layout
- `Pod`: "Plain Old Data" - can be safely cast to bytes
- `Zeroable`: Can be safely initialized with zeros
- No padding issues (wgpu aligns attributes automatically)

## Building and Running

```bash
cargo run
```

You should see a colorful square with:
- Top-left: Red
- Top-right: Green
- Bottom-right: Blue
- Bottom-left: Yellow
- Smooth color interpolation across the surface

## Code Structure

- `main.rs`: Application setup with vertex/index buffer creation
- `shader.wgsl`: Shaders that read from vertex buffers
- `Vertex` struct: Defines vertex layout
- `Vertex::desc()`: Describes layout to wgpu

## What's Next?

The next example (03-textures) builds on this by:
- Loading images from files
- Creating texture resources
- Adding UV coordinates to vertices
- Sampling textures in the fragment shader
- Using bind groups for texture access

## Performance Considerations

### CPU Performance
- Buffer creation is relatively expensive (use create_buffer_init for static data)
- Minimize buffer updates per frame
- Batch geometry into fewer, larger buffers

### GPU Performance
- Indexed rendering is almost always faster
- Vertex reuse improves post-transform cache hit rate
- Smaller index format (Uint16) can be faster
- Interleaved attributes usually perform better

### Memory Usage
- Index buffers dramatically reduce memory usage
- Use smallest practical vertex format
- Share buffers between similar objects
- Consider vertex compression for large scenes

## Common Patterns

### Loading from File

```rust
let vertices = load_obj_file("model.obj")?;
let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("Model Vertices"),
    contents: bytemuck::cast_slice(&vertices),
    usage: wgpu::BufferUsages::VERTEX,
});
```

### Dynamic Geometry

```rust
// Create with COPY_DST for updates
let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
    size: max_vertices * std::mem::size_of::<Vertex>(),
    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    mapped_at_creation: false,
});

// Update each frame
queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&new_vertices));
```

### Multiple Vertex Buffers

```rust
// Buffer 0: positions
render_pass.set_vertex_buffer(0, position_buffer.slice(..));
// Buffer 1: colors
render_pass.set_vertex_buffer(1, color_buffer.slice(..));
```

## Troubleshooting

**Nothing renders?**
- Check vertex buffer is bound before draw call
- Verify vertex format matches shader input
- Ensure indices don't reference out-of-bounds vertices

**Wrong colors?**
- Check color attribute offset and format
- Verify interleaved layout matches Vertex struct
- Ensure bytemuck derives are present

**Crash or validation error?**
- Vertex struct must be `#[repr(C)]`
- Check buffer size matches data size
- Verify index format matches index data type

## Further Reading

- [wgpu Buffer Documentation](https://docs.rs/wgpu/latest/wgpu/struct.Buffer.html)
- [Vertex Buffer Best Practices](https://developer.nvidia.com/content/vertex-buffer-best-practices)
- [Index Buffer Optimization](https://developer.nvidia.com/blog/optimizing-gpu-rendering-using-index-buffers/)
