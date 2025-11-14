# wgpu Textures Example

Demonstrates texture loading, sampling, and rendering in wgpu. This example builds on the triangle example by adding vertex buffers, index buffers, and texture mapping.

## What This Example Demonstrates

### Core Concepts

1. **Texture Creation**
   - Loading textures from image files
   - Creating textures programmatically
   - Texture formats and dimensions
   - Writing data to GPU textures

2. **Texture Sampling**
   - Sampler configuration (filtering, wrapping)
   - Texture coordinates (UVs)
   - Address modes (Clamp, Repeat, Mirror)
   - Filter modes (Linear, Nearest)

3. **Vertex Buffers**
   - Defining vertex structures with attributes
   - Creating immutable vertex buffers
   - Vertex buffer layouts
   - Using bytemuck for safe casting

4. **Index Buffers**
   - Indexed drawing for efficiency
   - Creating index buffers
   - DrawIndexed vs Draw

5. **Bind Groups**
   - Binding textures and samplers to shaders
   - Bind group layouts
   - Shader resource binding

## Texture Pipeline

```
Image File → Decode → RGBA8 → GPU Texture → Texture View → Bind Group → Shader
```

### Detailed Flow:

1. **Image Loading**
   - Use the `image` crate to decode PNG, JPEG, etc.
   - Convert to RGBA8 format

2. **Texture Creation**
   - Define texture dimensions and format
   - Allocate GPU memory
   - Set usage flags (TEXTURE_BINDING | COPY_DST)

3. **Data Upload**
   - Use `queue.write_texture` to transfer data
   - Specify data layout (bytes per row, rows per image)

4. **View Creation**
   - Create a texture view (how to interpret the texture)
   - Views can provide different interpretations of the same data

5. **Sampler Creation**
   - Configure filtering (how to interpolate between pixels)
   - Configure address mode (what happens outside [0,1])

6. **Binding**
   - Create bind group layout (describes expected resources)
   - Create bind group (actual resource bindings)
   - Set bind group in render pass

## Texture Concepts

### Texture Coordinates (UVs)

Texture coordinates map vertices to positions in the texture:
- U: Horizontal axis (0 = left, 1 = right)
- V: Vertical axis (0 = top, 1 = bottom)

```
(0,0) -------- (1,0)
  |              |
  |   Texture    |
  |              |
(0,1) -------- (1,1)
```

### Filtering Modes

**Nearest (Point Sampling)**
- Fast but pixelated
- Good for pixel art
- Takes the color of the closest texel

**Linear (Bilinear Filtering)**
- Smooth but can be blurry
- Good for realistic graphics
- Interpolates between nearest 4 texels

### Address Modes

**ClampToEdge**
- Coordinates outside [0,1] use the edge color
- Prevents wrapping artifacts

**Repeat**
- Coordinates wrap around
- Good for tiling textures

**MirrorRepeat**
- Coordinates wrap with mirroring
- Creates seamless patterns

## Mipmapping

Mipmaps are pre-calculated, lower-resolution versions of a texture. Benefits:
- Reduces aliasing when textures are far away
- Improves performance (better cache coherency)
- Smoother appearance at distance

Creating mipmaps (not in this example, but important to know):
```rust
mip_level_count: texture.size.max_mips(),  // Instead of 1
```

## Vertex and Index Buffers

### Why Use Vertex Buffers?

Instead of hardcoding vertices in shaders:
- More flexible (can update vertex data)
- More efficient (shared vertices with indexing)
- Standard approach for complex geometry

### Why Use Index Buffers?

For a quad (4 vertices, 2 triangles):
- Without indexing: 6 vertices (2 triangles × 3 vertices)
- With indexing: 4 vertices + 6 indices

Saves memory and improves GPU cache performance.

## Bind Groups and Layouts

### Bind Group Layout
Describes what types of resources the shader expects:
```rust
- Binding 0: Texture2D
- Binding 1: Sampler
```

### Bind Group
The actual resources bound to those bindings:
```rust
- Binding 0: my_texture.view
- Binding 1: my_texture.sampler
```

Think of it as:
- Layout = Interface/Contract
- Bind Group = Implementation

## Code Structure

- `main.rs`: State management, texture creation, rendering
- `shader.wgsl`: Vertex and fragment shaders with texture sampling

## Building and Running

```bash
cargo run
```

You should see a checkerboard pattern on a quad.

## Texture Formats

Common formats:
- **Rgba8Unorm**: 8 bits per channel, linear color space
- **Rgba8UnormSrgb**: 8 bits per channel, sRGB color space
- **Bgra8UnormSrgb**: Like RGBA but blue and red swapped (common on Windows)
- **Rgba16Float**: 16-bit floating point per channel (HDR)

Always prefer sRGB formats for color textures to ensure correct color display.

## Performance Considerations

### Texture Size
- Power-of-2 dimensions (256, 512, 1024) are often optimal
- Modern GPUs support non-power-of-2, but POT may be faster

### Texture Compression
Not shown here, but real applications should use compressed formats:
- BC1-BC7 (DirectX)
- ASTC (Mobile)
- Reduces memory usage and bandwidth

### Mipmaps
- Essential for textures viewed at varying distances
- Reduces aliasing and improves performance
- Slight memory overhead (33% more)

## Common Issues

**Texture appears black?**
- Check texture format matches shader expectations
- Verify bind group is set before draw call
- Ensure texture view and sampler are correctly bound

**Texture is stretched or distorted?**
- Check texture coordinates
- Verify aspect ratio matches quad geometry

**Texture coordinates inverted?**
- Different APIs use different UV origins
- wgpu uses top-left as (0,0) for texture coordinates

## Next Steps

The next example (03-3d-cube) extends this by:
- Adding 3D transformations
- Using uniform buffers for matrices
- Implementing depth testing
- Creating actual 3D geometry

## Further Reading

- [wgpu Texture Documentation](https://docs.rs/wgpu/latest/wgpu/struct.Texture.html)
- [WGSL Texture Functions](https://www.w3.org/TR/WGSL/#texture-builtin-functions)
- [Learn wgpu Textures](https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures/)
