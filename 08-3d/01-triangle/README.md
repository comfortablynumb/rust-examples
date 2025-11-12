# wgpu Triangle Example

A foundational example demonstrating the basic wgpu graphics pipeline by rendering a colored triangle.

## What This Example Demonstrates

### Core Concepts

1. **wgpu Initialization**
   - Creating a GPU instance
   - Requesting an adapter (physical GPU)
   - Creating a logical device and command queue
   - Setting up a window surface

2. **Render Pipeline**
   - Shader module creation
   - Pipeline layout definition
   - Vertex and fragment shader stages
   - Primitive assembly (triangle list)
   - Color target configuration

3. **WGSL Shaders**
   - Vertex shader with hardcoded positions
   - Fragment shader for pixel coloring
   - Vertex attributes and interpolation
   - Built-in variables (@builtin, @location)

4. **Render Loop**
   - Command encoder creation
   - Render pass setup
   - Draw call execution
   - Command submission and presentation

### Graphics Pipeline Flow

```
Vertex Shader → Primitive Assembly → Rasterization → Fragment Shader → Output
```

#### Detailed Flow:

1. **Vertex Shader (`vs_main`)**
   - Input: Vertex index (0, 1, 2)
   - Output: Clip-space position and color
   - Runs once per vertex (3 times for our triangle)

2. **Primitive Assembly**
   - Assembles vertices into triangles
   - Uses topology: TriangleList (every 3 vertices = 1 triangle)

3. **Rasterization**
   - Converts triangles to fragments (potential pixels)
   - Performs clipping and culling
   - Interpolates vertex attributes (color) across the triangle

4. **Fragment Shader (`fs_main`)**
   - Input: Interpolated vertex attributes
   - Output: Final pixel color
   - Runs once per pixel covered by the triangle

5. **Output Merger**
   - Writes colors to the framebuffer
   - Handles blending (we use REPLACE - no blending)

## Key wgpu Concepts

### Surface Configuration

The surface represents the window where we render. Configuration includes:

```rust
- format: Pixel format (BGRA8UnormSrgb, RGBA8UnormSrgb, etc.)
- usage: How the texture is used (RENDER_ATTACHMENT)
- present_mode: How frames are presented (FIFO, Immediate, Mailbox)
- width/height: Render resolution
```

### Command Encoding

wgpu uses command buffers to record GPU operations:

```rust
1. Create encoder
2. Begin render pass
3. Set pipeline
4. Issue draw calls
5. End render pass
6. Submit commands to queue
```

This batching approach minimizes CPU-GPU communication overhead.

### Coordinate Systems

- **Clip Space**: After vertex shader, coordinates are in clip space
  - X: -1 (left) to +1 (right)
  - Y: -1 (bottom) to +1 (top) in wgpu (unlike OpenGL)
  - Z: 0 (near) to 1 (far)
  - W: For perspective division

## Building and Running

```bash
cargo run
```

### Controls

- **Escape**: Close window
- Window resizing is supported

## Code Structure

- `main.rs`: Application setup, event loop, state management
- `shader.wgsl`: WGSL vertex and fragment shaders

## What's Next?

The next example (02-textures) builds on this foundation by:
- Loading image files
- Creating texture resources
- Using samplers for texture filtering
- Rendering textured quads

## Understanding the Triangle

The triangle is colored using vertex interpolation:
- Top vertex: Red (1.0, 0.0, 0.0)
- Bottom-left: Green (0.0, 1.0, 0.0)
- Bottom-right: Blue (0.0, 0.0, 1.0)

The GPU automatically interpolates these colors across the triangle surface, creating a smooth gradient. This is called **attribute interpolation** or **varying interpolation**.

## Common Issues

**Black screen?**
- Check that the shader compiled successfully
- Verify surface format compatibility
- Ensure viewport dimensions are valid

**Validation errors?**
- Enable the wgpu validation layer for detailed errors
- Check that bind group layouts match shader declarations

**Window resizing broken?**
- Surface must be reconfigured on resize
- Texture dimensions must be updated

## Performance Notes

This simple example should run at hundreds or thousands of FPS on modern hardware. The GPU is essentially idle - we're only drawing 3 vertices per frame with no complex operations.

## Further Reading

- [wgpu Documentation](https://docs.rs/wgpu/)
- [WebGPU Specification](https://www.w3.org/TR/webgpu/)
- [WGSL Specification](https://www.w3.org/TR/WGSL/)
- [Learn wgpu Tutorial](https://sotrh.github.io/learn-wgpu/)
