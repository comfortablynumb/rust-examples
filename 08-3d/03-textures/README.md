# Textures in wgpu

This example demonstrates how to load images from files, create GPU textures, and sample them in shaders. We render a textured quad using UV coordinates and bind groups.

## What This Example Demonstrates

### Core Concepts

1. **Texture Loading**
   - Loading images from files using the `image` crate
   - Converting image formats to RGBA8
   - Creating GPU textures from image data
   - Writing image data to GPU memory

2. **Texture Coordinates (UVs)**
   - Adding UV coordinates to vertices
   - UV coordinate system (0,0 to 1,1)
   - Texture coordinate interpolation
   - Coordinate space conventions

3. **Samplers**
   - Texture filtering modes (Linear, Nearest)
   - Address modes (Clamp, Repeat, Mirror)
   - Minification vs. magnification
   - Mipmap filtering

4. **Bind Groups**
   - Grouping related resources
   - Bind group layouts
   - Binding resources to shaders
   - Set and binding indices

## Texture Coordinates Explained

### UV Space

UV coordinates map positions on 3D geometry to positions in 2D texture space:

```
(0,0) -------- (1,0)
  |              |
  |   Texture    |
  |              |
(0,1) -------- (1,1)
```

**Important:** wgpu (like Vulkan and Metal) uses (0,0) at the top-left, unlike OpenGL which uses bottom-left.

### Vertex Mapping

```rust
Vertex { position: [-0.8,  0.8, 0.0], tex_coords: [0.0, 0.0] }, // Top-left
Vertex { position: [ 0.8,  0.8, 0.0], tex_coords: [1.0, 0.0] }, // Top-right
Vertex { position: [ 0.8, -0.8, 0.0], tex_coords: [1.0, 1.0] }, // Bottom-right
Vertex { position: [-0.8, -0.8, 0.0], tex_coords: [0.0, 1.0] }, // Bottom-left
```

The GPU automatically interpolates UV coordinates across triangles, so every pixel gets appropriate texture coordinates for sampling.

## Texture Sampling

### Filtering Modes

**Magnification** (texture smaller than rendered size):
- `FilterMode::Nearest`: Pixelated, blocky appearance
- `FilterMode::Linear`: Smooth, blurred appearance

**Minification** (texture larger than rendered size):
- `FilterMode::Nearest`: Can cause aliasing/shimmering
- `FilterMode::Linear`: Smoother but can still shimmer
- Use mipmaps for best quality

### Address Modes

What happens when UVs go outside [0,1]:

- `ClampToEdge`: Use edge color (default, prevents seams)
- `Repeat`: Tile the texture
- `MirrorRepeat`: Tile with mirroring
- `ClampToBorder`: Use border color

### Texture Formats

Common formats:
- `Rgba8Unorm`: 8 bits per channel, linear space
- `Rgba8UnormSrgb`: 8 bits per channel, sRGB space (use this for color textures!)
- `R8Unorm`: Single channel (for masks, heightmaps)
- `Rgba16Float`: HDR textures
- `Bc1RgbaUnorm`: Compressed textures (DXT1)

**sRGB vs. Linear:**
- Use sRGB for color/albedo textures (matches how monitors work)
- Use linear for data textures (normals, roughness, etc.)

## Bind Groups and Layouts

### Bind Group Layout

Defines **what types** of resources the shader expects:

```rust
BindGroupLayoutDescriptor {
    entries: &[
        BindGroupLayoutEntry {
            binding: 0,  // @binding(0) in shader
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Texture { ... },
        },
        BindGroupLayoutEntry {
            binding: 1,  // @binding(1) in shader
            ty: BindingType::Sampler { ... },
        },
    ],
}
```

### Bind Group

Contains the **actual resources**:

```rust
BindGroupDescriptor {
    layout: &bind_group_layout,
    entries: &[
        BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(&texture_view),
        },
        BindGroupEntry {
            binding: 1,
            resource: BindingResource::Sampler(&sampler),
        },
    ],
}
```

### In the Shader

```wgsl
@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;

// Sample the texture
let color = textureSample(t_diffuse, s_diffuse, uv);
```

## Texture Loading Pipeline

### 1. Load Image from File

```rust
let img = image::load_from_memory(bytes)?;
let rgba = img.to_rgba8();
```

### 2. Create GPU Texture

```rust
let texture = device.create_texture(&TextureDescriptor {
    size: Extent3d { width, height, depth_or_array_layers: 1 },
    format: TextureFormat::Rgba8UnormSrgb,
    usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
    ...
});
```

### 3. Upload Data

```rust
queue.write_texture(
    ImageCopyTexture { texture, mip_level: 0, ... },
    &rgba,
    ImageDataLayout { bytes_per_row: Some(4 * width), ... },
    size,
);
```

### 4. Create View and Sampler

```rust
let view = texture.create_view(&TextureViewDescriptor::default());
let sampler = device.create_sampler(&SamplerDescriptor { ... });
```

### 5. Bind to Shader

```rust
render_pass.set_bind_group(0, &bind_group, &[]);
```

## Mipmaps

Mipmaps are pre-computed smaller versions of textures that improve:
- Visual quality (reduces aliasing/shimmering)
- Performance (fewer texture cache misses)

### Creating Mipmaps

```rust
let mip_level_count = (width.max(height) as f32).log2().floor() as u32 + 1;

let texture = device.create_texture(&TextureDescriptor {
    mip_level_count,
    ...
});

// Generate mipmaps (requires compute shader or external library)
```

This example doesn't use mipmaps for simplicity, but production code should.

## Texture Coordinates in Detail

### Why Float Coordinates?

UVs are floats because:
1. Interpolation across triangles produces fractional values
2. Filtering requires sampling between pixels
3. Flexibility for tiling, scrolling, effects

### UV Wrapping Example

```rust
// UV coordinates > 1.0 with Repeat mode
tex_coords: [2.0, 2.0] // Texture repeats 2x2 times
```

### Common UV Tricks

**Texture scrolling:**
```wgsl
let scrolled_uv = uv + vec2<f32>(time * 0.1, 0.0);
let color = textureSample(texture, sampler, scrolled_uv);
```

**Texture rotation:**
```wgsl
let center = vec2<f32>(0.5, 0.5);
let rotated_uv = rotate_2d(uv - center, angle) + center;
```

**Texture scaling:**
```wgsl
let scaled_uv = (uv - 0.5) * scale + 0.5;
```

## Performance Considerations

### Texture Size
- Power-of-two sizes (256, 512, 1024) often perform better
- Some hardware requires POT for mipmaps
- Larger textures use more memory and bandwidth

### Compression
- Use compressed formats (BC1-7, ASTC) for shipped games
- 4:1 to 16:1 compression ratios
- Slight quality loss but huge memory savings
- Must be pre-compressed (not done at runtime)

### Texture Arrays
- Pack similar textures into arrays
- Reduces bind group changes
- Better for batching

### Atlases
- Combine many small textures into one large texture
- Reduce draw calls and state changes
- Careful with filtering at edges (add padding)

## Building and Running

```bash
cargo run
```

### Adding Your Own Texture

1. Place an image file at `assets/texture.png`
2. The example will load it automatically
3. If the file doesn't exist, a checkerboard pattern is used

Supported formats: PNG, JPEG, GIF, BMP, TGA, and more (via `image` crate).

## Code Structure

- `main.rs`: Application with texture loading and bind group management
- `shader.wgsl`: Shaders with texture sampling
- `Texture` struct: Encapsulates texture, view, and sampler
- `assets/`: Directory for texture files (optional)

## What's Next?

The next example (04-3d-cube) builds on this by:
- Adding depth testing
- Using 3D transformations
- Implementing perspective projection
- Animating with matrix math
- Texturing 3D objects

## Common Issues

**Texture appears flipped?**
- Check UV coordinates (Y direction)
- wgpu uses (0,0) at top-left
- Flip during loading or adjust UVs

**Texture is distorted?**
- Verify aspect ratio matches quad
- Check bytes_per_row calculation
- Ensure image format matches texture format

**Texture is black?**
- Verify texture is bound before drawing
- Check bind group layout matches shader
- Ensure texture data was uploaded

**Seams between tiles?**
- Use `ClampToEdge` instead of `Repeat`
- Add padding to texture atlas entries
- Check for floating-point precision issues

## Further Reading

- [wgpu Texture Tutorial](https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures/)
- [GPU Gems: Texture Filtering](https://developer.nvidia.com/gpugems/gpugems2/part-iii-high-quality-rendering/chapter-20-fast-third-order-texture-filtering)
- [Texture Compression Formats](https://www.reedbeta.com/blog/understanding-bcn-texture-compression-formats/)
