# wgpu Advanced - Shadow Mapping

Demonstrates shadow mapping, one of the most important advanced graphics techniques. Objects cast realistic shadows based on light position.

## Features

- **Shadow Mapping**: Two-pass rendering for dynamic shadows
- **PCF (Percentage Closer Filtering)**: Soft shadow edges
- **Depth Bias**: Prevents shadow acne
- **Orbiting Light**: Dynamic light position creates moving shadows
- **Multiple Objects**: Ground plane and cubes with shadows

## How Shadow Mapping Works

### Two-Pass Rendering

**Pass 1: Shadow Map Generation**
1. Render scene from light's point of view
2. Store depth values in shadow map texture
3. No color output - depth only

**Pass 2: Normal Rendering**
1. Render scene from camera's point of view
2. For each fragment, transform to light space
3. Compare fragment depth with shadow map
4. If deeper (farther from light), in shadow

### Shadow Map

A depth texture rendered from the light's perspective:
- Size: 2048x2048 (higher = sharper shadows)
- Format: Depth32Float
- Orthographic projection for directional light

### PCF (Percentage Closer Filtering)

Instead of single depth sample, sample 3x3 area:
- Reduces aliasing
- Creates soft shadow edges
- More realistic appearance

### Shadow Acne

Problem: Self-shadowing artifacts from precision issues.

Solution: **Depth Bias**
- Constant bias: Shifts depth comparison
- Slope bias: Adjusts based on surface angle
- Small offset when sampling: `z - 0.005`

### Peter Panning

Problem: Bias too large causes shadows to "float".

Solution: Balance bias carefully, use slope-scaled bias.

## Key Concepts

### Light View-Projection Matrix

Transforms world space to light's clip space:
```rust
let light_view = Matrix4::look_at_rh(light_pos, target, up);
let light_proj = ortho(-10, 10, -10, 10, near, far);
let light_view_proj = light_proj * light_view;
```

### Shadow Coordinate Transformation

```wgsl
// In vertex shader
shadow_pos = light_view_proj * world_position;

// In fragment shader
shadow_coord.xy = shadow_pos.xy * 0.5 + 0.5;  // NDC to [0,1]
shadow_coord.z = shadow_pos.z;  // Depth for comparison
```

### Comparison Sampler

Special sampler that compares depth:
```rust
compare: Some(CompareFunction::LessEqual)
```

```wgsl
textureSampleCompare(shadow_map, sampler, coord, compare_depth)
// Returns 1.0 if visible, 0.0 if shadowed
```

## Building

```bash
cargo run
```

Watch the light orbit around the scene, casting dynamic shadows!

## Advanced Topics

### Cascaded Shadow Maps (CSM)
- Multiple shadow maps at different distances
- Better quality for large scenes
- Used in AAA games

### Exponential Shadow Maps (ESM)
- Alternative to standard shadow maps
- Softer shadows without PCF
- Different artifacts

### Shadow Volumes
- Completely different technique
- Geometry-based, not texture-based
- Perfect hard shadows

### Ray-Traced Shadows
- Modern technique with ray tracing
- Physically accurate
- Expensive but highest quality

## Performance Notes

- Shadow map size affects quality and performance
- PCF kernel size = soft shadow quality vs speed
- Consider lower resolution for mobile/web
- Multiple lights = multiple shadow maps

## Further Reading

- [Learn OpenGL - Shadow Mapping](https://learnopengl.com/Advanced-Lighting/Shadows/Shadow-Mapping)
- [GPU Gems - Shadow Map Anti-Aliasing](https://developer.nvidia.com/gpugems/gpugems/part-ii-lighting-and-shadows/chapter-11-shadow-map-antialiasing)
- [Cascaded Shadow Maps](https://docs.microsoft.com/en-us/windows/win32/dxtecharts/cascaded-shadow-maps)
