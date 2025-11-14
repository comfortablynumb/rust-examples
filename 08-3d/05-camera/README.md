# wgpu Camera Controls Example

Demonstrates a first-person camera system with keyboard and mouse controls. This example shows how to implement interactive 3D camera movement.

## What This Example Demonstrates

### Core Concepts

1. **Camera System**
   - Position, rotation (yaw/pitch)
   - Forward, right, up vectors
   - View and projection matrix generation

2. **Input Handling**
   - Keyboard input for movement (WASD)
   - Mouse input for looking around
   - Delta time for frame-rate independent movement

3. **Camera Controller**
   - Separating camera state from controller logic
   - Movement speed and mouse sensitivity
   - Smooth movement with velocity

## Camera Basics

A camera in 3D graphics defines:
1. **Where** the camera is (position)
2. **Where** it's looking (direction)
3. **How** the 3D world projects to 2D screen (projection)

### Camera Components

**Position**: Point in 3D space
```rust
position: Point3<f32>
```

**Orientation**: Yaw (horizontal) and Pitch (vertical) angles
```rust
yaw: f32    // Rotation around Y axis
pitch: f32  // Rotation around X axis (up/down look)
```

**Projection**: Field of view, aspect ratio, clipping planes
```rust
fov, aspect, near, far
```

## Camera Coordinate System

The camera uses a **right-handed coordinate system**:

```
     +Y (Up)
     |
     |
     +------ +X (Right)
    /
   /
  +Z (Forward, into screen)
```

### Direction Vectors

**Forward Vector**
```rust
Vector3::new(
    yaw.cos() * pitch.cos(),
    pitch.sin(),
    yaw.sin() * pitch.cos(),
)
```
Points where the camera is looking.

**Right Vector**
```rust
forward.cross(up)
```
Points to the right of the camera.

**Up Vector**
```rust
right.cross(forward)
```
Points upward relative to the camera.

## View Matrix

The view matrix transforms from world space to view (camera) space.

```rust
Matrix4::look_at_rh(
    eye_position,
    eye_position + forward_direction,
    up_vector,
)
```

This matrix:
- Moves the world so the camera is at the origin
- Rotates the world so the camera looks down -Z
- Is the inverse of the camera's model matrix

## First-Person Camera

A first-person camera simulates a human's view:
- WASD keys move the camera
- Mouse moves the view direction
- Can look around freely

### Yaw (Horizontal Rotation)

```
Top View:

       0° (North, -Z)
       |
270° --+-- 90° (East, +X)
       |
     180° (South, +Z)
```

Yaw rotates around the Y (up) axis.

### Pitch (Vertical Rotation)

```
     +90° (Up)
       |
       |
   0° -+- (Forward)
       |
       |
     -90° (Down)
```

Pitch is typically clamped to prevent camera flipping.

## Movement System

### Frame-Rate Independence

Movement uses **delta time (dt)** to ensure consistent speed regardless of frame rate:

```rust
position += velocity * speed * dt
```

- **60 FPS**: dt ≈ 0.016s, moves small amount each frame
- **30 FPS**: dt ≈ 0.033s, moves larger amount each frame
- **Result**: Same distance per second

### Velocity Calculation

```rust
let mut velocity = Vector3::zero();

if forward { velocity += camera.forward() }
if backward { velocity -= camera.forward() }
if right { velocity += camera.right() }
if left { velocity -= camera.right() }

// Normalize to prevent faster diagonal movement
if velocity.magnitude() > 0.0 {
    velocity = velocity.normalize();
}
```

### Diagonal Movement Problem

Without normalization:
- Moving forward: speed = 1.0
- Moving forward + right: speed = √2 ≈ 1.414

Normalization ensures consistent speed in all directions.

## Mouse Controls

### Mouse Delta

Track change in mouse position:
```rust
dx = current_x - last_x
dy = current_y - last_y
```

### Applying to Camera

```rust
yaw += dx * sensitivity * dt
pitch -= dy * sensitivity * dt  // Negative: up mouse = look up
```

### Pitch Clamping

Prevent camera flipping:
```rust
pitch = pitch.clamp(-PI/2 + epsilon, PI/2 - epsilon)
```

Small epsilon (0.01) prevents gimbal lock at exact vertical.

## Controls

### Keyboard
- **W / Up**: Move forward
- **S / Down**: Move backward
- **A / Left**: Strafe left
- **D / Right**: Strafe right
- **Space**: Move up
- **Shift**: Move down
- **Escape**: Close window

### Mouse
- **Click and drag**: Look around

## Camera Types

### First-Person (this example)
- Camera is a point
- WASD movement
- Free look with mouse
- Used in FPS games

### Third-Person
- Camera orbits around a target
- Target can move independently
- Camera follows at a distance
- Used in adventure games

### Orbital
- Camera orbits around a fixed point
- Mouse rotates around target
- Useful for model viewers

### Free-Fly
- Unrestricted 6DOF movement
- Used in editors and viewers
- No gravity or constraints

## Building and Running

```bash
cargo run
```

Use WASD + mouse to fly around the cube!

## Code Structure

- `Camera`: Position, orientation, matrices
- `CameraController`: Input handling, movement logic
- `State`: Integrates camera with rendering

## Common Issues

**Camera flips upside down?**
- Clamp pitch to [-π/2, π/2]
- Use small epsilon to avoid exact vertical

**Movement too fast/slow?**
- Adjust `speed` parameter
- Ensure using delta time correctly

**Jittery mouse movement?**
- Reset mouse delta after applying
- Use consistent sensitivity value

**Diagonal movement faster?**
- Normalize velocity vector
- Ensures constant speed in all directions

## Enhancements to Try

1. **Smooth Movement**
   - Add acceleration and deceleration
   - Interpolate between positions

2. **Camera Collision**
   - Prevent moving through objects
   - Ray casting or sphere collision

3. **Camera Shake**
   - Add random offset for impact effects
   - Smooth falloff over time

4. **Zoom**
   - Adjust FOV with mouse wheel
   - Clamp to reasonable range (15° - 90°)

5. **Sprint/Crouch**
   - Multiple speed modes
   - Different speeds for different keys

6. **Head Bob**
   - Simulate walking motion
   - Sin wave on Y axis while moving

## Performance Notes

- Camera updates are CPU-side
- Very lightweight calculation
- Delta time calculation negligible
- Main cost is updating uniform buffer (minimal)

## Next Steps

The next example (06-scene) extends this by:
- Multiple objects with different materials
- Scene graph or entity management
- Object picking with mouse
- More complex scene organization

## Further Reading

- [Learn OpenGL - Camera](https://learnopengl.com/Getting-started/Camera)
- [Camera Systems in Unity](https://docs.unity3d.com/Manual/CamerasOverview.html)
- [Understanding Camera Matrices](https://jsantell.com/model-view-projection/)
