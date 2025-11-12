# 3D Cube with Transformations

This example demonstrates rendering a spinning 3D cube with proper depth testing, perspective projection, and matrix transformations using cgmath.

## What This Example Demonstrates

### Core Concepts

1. **3D Transformations**
   - Model matrix (object placement/rotation)
   - View matrix (camera positioning)
   - Projection matrix (perspective)
   - Matrix multiplication order

2. **Depth Testing**
   - Depth buffer creation
   - Depth comparison functions
   - Proper 3D occlusion
   - Z-fighting prevention

3. **Uniform Buffers**
   - Creating uniform buffers
   - Updating uniforms each frame
   - Binding uniforms to shaders
   - Buffer usage flags

4. **3D Geometry**
   - Cube vertex definition
   - Face winding order
   - Backface culling
   - Index buffer efficiency

## Coordinate Systems and Transformations

### The Graphics Pipeline Spaces

Vertices travel through several coordinate spaces:

```
Model Space → World Space → View Space → Clip Space → NDC → Screen Space
```

#### 1. Model Space (Local Space)
- Coordinates relative to the object's center
- Our cube: vertices from -1 to +1 on each axis
- Each object has its own model space

#### 2. World Space
- Global coordinate system for the scene
- **Model Matrix** transforms from model → world
- Includes translation, rotation, scale

#### 3. View Space (Camera Space)
- Coordinates relative to the camera
- **View Matrix** transforms from world → view
- Camera at origin, looking down -Z axis

#### 4. Clip Space
- After perspective division
- **Projection Matrix** transforms from view → clip
- X, Y, Z all in range based on frustum

#### 5. Normalized Device Coordinates (NDC)
- After perspective division (clip.xyz / clip.w)
- X, Y, Z in range [-1, 1]
- wgpu uses Z: [0, 1] (unlike OpenGL's [-1, 1])

#### 6. Screen Space
- Final pixel coordinates
- GPU handles this conversion automatically

### Matrix Mathematics

#### Model Matrix

Positions and orients an object in the world:

```rust
// Rotation around Y axis
let rotation = Matrix4::from_angle_y(Rad(elapsed));

// Translation
let translation = Matrix4::from_translation(vec3(x, y, z));

// Scale
let scale = Matrix4::from_scale(2.0);

// Combined (applied right to left: scale, then rotate, then translate)
let model = translation * rotation * scale;
```

**Order matters!** `T * R * S` is different from `S * R * T`.

#### View Matrix

Positions the camera in the scene:

```rust
let view = Matrix4::look_at_rh(
    Point3::new(0.0, 2.0, 5.0),  // Eye: where the camera is
    Point3::new(0.0, 0.0, 0.0),  // Center: what it's looking at
    Vector3::unit_y(),            // Up: which direction is "up"
);
```

**Right-handed coordinate system** (rh):
- +X: right
- +Y: up
- +Z: towards viewer (out of screen)

#### Projection Matrix

Creates perspective (things farther away appear smaller):

```rust
let proj = cgmath::perspective(
    Deg(45.0),              // Field of view (angle)
    width / height,         // Aspect ratio
    0.1,                    // Near plane (closest visible)
    100.0,                  // Far plane (farthest visible)
);
```

**FOV** (Field of View):
- Smaller angle (e.g., 30°): telephoto lens, less distortion
- Larger angle (e.g., 90°): wide-angle lens, more distortion
- Typical: 45-60°

**Near/Far Planes**:
- Near: typically 0.1 to 1.0
- Far: depends on scene size
- Ratio affects depth buffer precision

### Combined Transformation

In the shader, we apply one combined matrix:

```rust
let mvp = proj * view * model;
```

This is more efficient than three separate matrices. The vertex shader does:

```wgsl
out.clip_position = uniforms.view_proj * vec4<f32>(position, 1.0);
```

## Depth Testing

### Why We Need It

Without depth testing, triangles are drawn in the order they're submitted. Later triangles overdraw earlier ones, even if they're behind them. This causes incorrect rendering.

With depth testing, the GPU tracks the depth (distance from camera) of each pixel and only draws fragments that are closer than what's already there.

### Depth Buffer

```rust
let depth_texture = device.create_texture(&TextureDescriptor {
    format: TextureFormat::Depth24Plus,  // 24-bit depth precision
    usage: TextureUsages::RENDER_ATTACHMENT,
    ...
});
```

**Common Formats:**
- `Depth24Plus`: 24-bit depth, widely supported
- `Depth32Float`: 32-bit floating point, more precision
- `Depth24PlusStencil8`: Depth + 8-bit stencil buffer

### Depth State

```rust
depth_stencil: Some(DepthStencilState {
    format: TextureFormat::Depth24Plus,
    depth_write_enabled: true,           // Write depth values
    depth_compare: CompareFunction::Less, // Pass if closer
    ...
}),
```

**Compare Functions:**
- `Less`: Pass if new depth < current depth (standard)
- `LessEqual`: For rendering equal depths
- `Greater`: Reverse depth (better precision in some cases)
- `Always`: Always pass (disables depth test)

### Depth Precision and Z-Fighting

**Z-Fighting** occurs when two surfaces are at nearly the same depth, causing flickering.

**Causes:**
- Near/far plane ratio too large
- Surfaces too close together
- Insufficient depth precision

**Solutions:**
- Keep near plane as far as possible
- Keep far plane as close as possible
- Use 32-bit depth instead of 24-bit
- Offset surfaces slightly (polygon offset)

**Reverse Depth:**
Using `Greater` comparison with [1, 0] range can improve precision for distant objects.

## Backface Culling

Only render front-facing triangles:

```rust
cull_mode: Some(Face::Back),
```

**Why?**
- 50% performance improvement (half the triangles)
- We never see inside of closed meshes
- Requires correct winding order

**Winding Order:**
- Counter-clockwise (CCW): front face (standard)
- Clockwise (CW): back face

Our cube indices are wound counter-clockwise when viewed from outside.

## Uniform Buffers

### Creating

```rust
let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
    contents: bytemuck::cast_slice(&[uniforms]),
    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
});
```

**Usage Flags:**
- `UNIFORM`: Can be bound as uniform
- `COPY_DST`: Can be updated with write_buffer

### Updating

```rust
queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
```

This updates the GPU buffer without recreating it. Efficient for per-frame updates.

### Binding

```rust
render_pass.set_bind_group(0, &uniform_bind_group, &[]);
```

The `0` corresponds to `@group(0)` in the shader.

## Animation

We use `Instant` to track elapsed time:

```rust
let elapsed = start_time.elapsed().as_secs_f32();
let rotation = Matrix4::from_angle_y(Rad(elapsed));
```

This creates smooth rotation independent of frame rate.

## Building and Running

```bash
cargo run
```

You should see a colorful cube spinning smoothly in 3D space.

## Code Structure

- `main.rs`: Application with 3D transformations and depth testing
- `shader.wgsl`: Vertex shader with matrix transformations
- `Vertex` struct: 3D positions with colors
- `Uniforms` struct: Transformation matrices

## Common Issues

**Cube appears flat?**
- Check depth testing is enabled
- Verify depth buffer is created and bound
- Ensure projection matrix is set up

**Cube inside-out?**
- Check winding order of indices
- Verify front_face is set to CCW
- Check model matrix transformations

**Cube wobbles or jitters?**
- Use elapsed time, not frame count
- Ensure time is in seconds (float)
- Check matrix calculations

**Nothing renders?**
- Verify near/far planes include the object
- Check camera position and look-at point
- Ensure matrices are combined correctly

**Z-fighting visible?**
- Increase near plane value
- Decrease far plane value
- Use Depth32Float instead of Depth24Plus

## Performance Considerations

### Matrix Updates
- Only update matrices when needed
- Consider separating view and model matrices
- Pre-compute static transformations

### Depth Buffer
- Match size with render target
- Recreate on window resize
- Clear to far plane (1.0)

### Vertex Count
- Use index buffers (we use 8 vertices instead of 36)
- Consider level of detail (LOD)
- Frustum culling for many objects

## What's Next?

The next example (05-camera) builds on this by:
- Implementing interactive camera controls
- WASD keyboard movement
- Mouse look with pitch/yaw
- Proper camera class structure
- Delta time for frame-independent movement

## Further Reading

- [LearnOpenGL - Coordinate Systems](https://learnopengl.com/Getting-started/Coordinate-Systems)
- [LearnOpenGL - Transformations](https://learnopengl.com/Getting-started/Transformations)
- [Depth Buffer Precision](https://developer.nvidia.com/content/depth-precision-visualized)
- [cgmath Documentation](https://docs.rs/cgmath/)
