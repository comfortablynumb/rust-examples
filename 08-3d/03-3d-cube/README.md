# wgpu 3D Cube Example

Demonstrates 3D transformations, uniform buffers, and depth testing by rendering a rotating cube.

## What This Example Demonstrates

### Core Concepts

1. **3D Transformations**
   - Model matrix (object transformations)
   - View matrix (camera positioning)
   - Projection matrix (perspective)
   - MVP matrix combination

2. **Uniform Buffers**
   - Creating uniform buffers
   - Updating uniforms per frame
   - Binding uniforms to shaders
   - Uniform buffer layout requirements

3. **Depth Testing**
   - Depth buffer creation
   - Depth comparison functions
   - Proper rendering order

4. **3D Geometry**
   - Defining cube vertices and indices
   - Back-face culling
   - Indexed drawing

## Coordinate Space Transformations

3D graphics involves transforming vertices through multiple coordinate spaces:

```
Local/Model Space → World Space → View/Camera Space → Clip Space → Screen Space
```

### Transformation Pipeline

1. **Model Space (Local Space)**
   - Object's own coordinate system
   - Defined by the artist/programmer
   - Example: Cube centered at origin

2. **World Space**
   - **Model Matrix** transforms from model to world space
   - Positions objects in the scene
   - Rotations, translations, scaling applied here

3. **View Space (Camera Space)**
   - **View Matrix** transforms from world to view space
   - Positions the world relative to the camera
   - Camera at origin, looking down -Z axis

4. **Clip Space**
   - **Projection Matrix** transforms from view to clip space
   - Applies perspective or orthographic projection
   - Coordinates in range [-1, 1] for X and Y, [0, 1] for Z in wgpu

5. **Screen Space**
   - GPU automatically converts clip space to screen coordinates
   - Viewport transformation applied

## The MVP Matrix

MVP stands for Model-View-Projection. These three matrices are typically combined:

```rust
MVP = Projection × View × Model
```

### Why Combine?

- **Performance**: One matrix multiplication per vertex instead of three
- **Simplicity**: Pass one uniform instead of three
- **Precision**: Reduces floating-point accumulation errors

### Matrix Order Matters!

In WGSL/GPU:
```wgsl
clip_position = mvp * vertex_position
```

Matrix multiplication is **not commutative**: A×B ≠ B×A

The transformation is applied right-to-left:
1. Model matrix (rotate/scale/translate object)
2. View matrix (position camera)
3. Projection matrix (apply perspective)

## The Model Matrix

Transforms objects from model space to world space.

```rust
// Rotation around X and Y axes
let model = Matrix4::from_angle_x(Rad(rotation))
    * Matrix4::from_angle_y(Rad(rotation * 0.7));
```

Common operations:
- **Translation**: Move object in world
- **Rotation**: Rotate object
- **Scaling**: Resize object

Multiple transformations combine via multiplication.

## The View Matrix

Positions the camera in the world.

```rust
let view = Matrix4::look_at_rh(
    Point3::new(0.0, 2.0, 5.0),  // Eye position
    Point3::new(0.0, 0.0, 0.0),  // Look-at point
    Vector3::unit_y(),            // Up direction
);
```

- **look_at_rh**: Right-handed coordinate system
- Defines camera position and orientation
- Creates a coordinate system with:
  - Forward: from eye to look-at point
  - Up: specified up vector (typically +Y)
  - Right: cross product of forward and up

## The Projection Matrix

Defines the viewing frustum and applies perspective.

```rust
let projection = perspective(
    Deg(45.0),      // Field of view (FOV)
    aspect_ratio,   // Width / Height
    0.1,            // Near plane
    100.0,          // Far plane
);
```

### Perspective Projection

- Creates depth perception (distant objects appear smaller)
- Defines a frustum (truncated pyramid)
- **FOV**: Controls "zoom" (smaller FOV = more zoomed in)
- **Near/Far planes**: Define visible depth range

### Orthographic Projection

Not used in this example, but common for 2D or technical drawings:
```rust
let projection = ortho(left, right, bottom, top, near, far);
```

No perspective distortion - parallel lines stay parallel.

## Depth Testing

Without depth testing, triangles are drawn in order - later draws overwrite earlier ones, even if they're behind.

### Depth Buffer

- Additional texture storing depth per pixel
- Same dimensions as color buffer
- Typically 32-bit float (Depth32Float)

### Depth Test Process

For each fragment:
1. Calculate depth (Z coordinate in clip space)
2. Compare with current depth buffer value
3. If test passes (closer):
   - Write new color to color buffer
   - Write new depth to depth buffer
4. If test fails (farther):
   - Discard fragment

### Depth Compare Functions

- **Less**: Fragment closer than current (most common)
- **LessEqual**: Fragment closer or equal
- **Greater**: Fragment farther (reverse depth)
- **Always**: Always pass (no depth testing)

### Why Depth Testing?

Without it:
- Drawing order matters (painter's algorithm)
- Complex scenes require sorting
- Difficult with intersecting objects

With it:
- Draw in any order
- GPU handles visibility
- Correct occlusion automatically

## Uniform Buffers

Uniforms are read-only data passed from CPU to GPU.

### Creating a Uniform Buffer

```rust
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mvp: [[f32; 4]; 4],
}
```

Key points:
- **#[repr(C)]**: Ensures consistent memory layout
- **bytemuck traits**: Allows safe byte casting
- Layout must match shader exactly

### Updating Uniforms

```rust
queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
```

- Updated every frame for animation
- CPU calculates new matrices
- GPU receives updated data

### Binding to Shaders

```rust
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;
```

- Uniforms are bound via bind groups
- Accessed in vertex shader (typically)
- Read-only from shader perspective

## Back-Face Culling

Optimization: don't render triangles facing away from camera.

```rust
cull_mode: Some(Face::Back),
front_face: FrontFace::Ccw,
```

- **Front face**: Defined by winding order (CCW or CW)
- **Cull mode**: Which faces to discard (Front, Back, or None)
- Reduces fragment processing by ~50% for closed objects

## Building and Running

```bash
cargo run
```

You should see a colorful rotating cube.

## Controls

- **Escape**: Close window
- Window resizing is supported (updates projection matrix)

## Code Structure

- `main.rs`: Rendering, matrices, uniforms, depth testing
- `shader.wgsl`: Vertex transformation with MVP matrix

## Common Issues

**Cube appears flat or distorted?**
- Check aspect ratio in projection matrix
- Verify matrix multiplication order
- Ensure matrices are column-major (cgmath default)

**Depth fighting / Z-fighting?**
- Adjust near/far plane ratio
- Use Depth32Float for better precision
- Avoid very small near planes

**Cube inside-out?**
- Check winding order of triangles
- Verify cull mode settings
- Ensure matrices are right-handed

**Nothing visible?**
- Verify camera is not inside the cube
- Check near/far plane distances
- Ensure depth testing is enabled

## Performance Considerations

### Uniform Buffer Updates

- Updated every frame (expensive)
- Use multiple buffers for different objects
- Consider instancing for many objects

### Depth Buffer Precision

- Use Depth32Float for best precision
- Use reverse depth (far=0, near=1) for even better precision
- Keep near plane as far as possible

## Matrix Mathematics

### Column-Major vs Row-Major

cgmath uses column-major matrices (same as WGSL):
- Vectors are columns
- Matrix-vector multiplication: M × v
- Matches GPU expectations

### Right-Handed vs Left-Handed

This example uses right-handed coordinates:
- +X: Right
- +Y: Up
- +Z: Out of screen (towards viewer)

wgpu/WGSL clip space:
- +X: Right
- +Y: Up
- +Z: Into screen (away from viewer)

The projection matrix handles this conversion.

## Next Steps

The next example (04-lighting) extends this by:
- Adding normal vectors to vertices
- Implementing Phong lighting model
- Using ambient, diffuse, and specular lighting
- Per-fragment lighting calculations

## Further Reading

- [WebGPU Matrix Math](https://webgpufundamentals.org/webgpu/lessons/webgpu-matrix-math.html)
- [Learn OpenGL - Coordinate Systems](https://learnopengl.com/Getting-started/Coordinate-Systems)
- [cgmath Documentation](https://docs.rs/cgmath/)
- [Understanding MVP Matrices](https://jsantell.com/model-view-projection/)
