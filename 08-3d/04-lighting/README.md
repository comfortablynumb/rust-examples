# wgpu Lighting Example (Phong Shading)

Demonstrates the Phong lighting model with ambient, diffuse, and specular components. This example shows how to implement realistic lighting in 3D graphics.

## What This Example Demonstrates

### Core Concepts

1. **Phong Lighting Model**
   - Ambient lighting (constant base illumination)
   - Diffuse lighting (surface angle to light)
   - Specular lighting (shiny highlights)
   - Material properties

2. **Surface Normals**
   - Normal vectors for lighting calculations
   - Normal transformation matrix
   - Per-face vs per-vertex normals
   - Normal interpolation

3. **Lighting in World Space**
   - Transforming normals to world space
   - Light position in world coordinates
   - Camera/view position for specular

4. **Advanced Uniforms**
   - Multiple transformation matrices
   - Lighting parameters
   - Material properties

## The Phong Lighting Model

Developed by Bui Tuong Phong in 1975, this lighting model combines three components to approximate realistic lighting.

### Formula

```
Final Color = (Ambient + Diffuse + Specular) × Material Color
```

### 1. Ambient Component

```
Ambient = ambient_strength × light_color
```

- Simulates indirect/scattered light
- Provides a base level of illumination
- Prevents completely black areas
- **No geometric calculation** - constant everywhere
- Typical strength: 0.1 - 0.3

**Physical basis**: In reality, light bounces around and illuminates surfaces indirectly. Ambient is a cheap approximation of this.

### 2. Diffuse Component (Lambertian Reflection)

```
Diffuse = diffuse_strength × max(N · L, 0) × light_color
```

Where:
- **N**: Surface normal (unit vector perpendicular to surface)
- **L**: Light direction (unit vector from surface to light)
- **N · L**: Dot product (cosine of angle between vectors)

- Simulates scattered/matte reflection
- Surfaces perpendicular to light are brightest
- **Angle-dependent** but **view-independent**
- Typical strength: 0.5 - 1.0

**Physical basis**: Rough surfaces scatter light in all directions. The amount of light received depends on the surface angle to the light source (Lambert's cosine law).

### 3. Specular Component (Phong Reflection)

```
Specular = specular_strength × (max(R · V, 0))^shininess × light_color
```

Where:
- **R**: Reflection direction (light reflected around normal)
- **V**: View direction (from surface to camera)
- **shininess**: How focused the highlight is

- Simulates shiny/glossy highlights
- Only visible at specific viewing angles
- **View-dependent** and **angle-dependent**
- Typical strength: 0.2 - 1.0
- Typical shininess: 2 (very rough) to 256 (mirror-like)

**Physical basis**: Smooth surfaces reflect light in a preferred direction. The tighter the reflection, the shinier the material appears.

## Phong vs Blinn-Phong

### Phong (used in this example)
```wgsl
reflect_dir = reflect(-light_dir, normal)
spec_factor = pow(max(dot(view_dir, reflect_dir), 0.0), shininess)
```

### Blinn-Phong (alternative, often better)
```wgsl
halfway_dir = normalize(light_dir + view_dir)
spec_factor = pow(max(dot(normal, halfway_dir), 0.0), shininess)
```

**Blinn-Phong advantages:**
- More efficient (no reflect() call)
- Better behavior at grazing angles
- More physically accurate
- Used in many modern engines

**When to use each:**
- Phong: Educational, legacy compatibility
- Blinn-Phong: Production code

## Surface Normals

A **normal** is a vector perpendicular to a surface. Essential for lighting calculations.

### Why Normals Matter

Without normals:
- All surfaces lit equally
- No sense of shape or geometry
- Flat, unrealistic appearance

With normals:
- Light interacts with surface angle
- Curved surfaces show gradients
- Realistic shading

### Per-Face vs Per-Vertex Normals

**Per-Face (Flat Shading)**
- One normal per triangle
- All vertices of a face share the same normal
- Creates faceted/geometric look
- Used in this example for clarity

**Per-Vertex (Smooth Shading)**
- Different normal at each vertex
- Normals averaged from adjacent faces
- GPU interpolates across triangles
- Creates smooth curved appearance

### Calculating Face Normals

For a triangle with vertices A, B, C:
```
edge1 = B - A
edge2 = C - A
normal = normalize(edge1 × edge2)
```

The cross product gives a perpendicular vector.

## The Normal Matrix

When transforming objects, normals need special treatment.

### Why Not Use the Model Matrix?

Non-uniform scaling breaks normals:
```
If we scale X by 2 and Y by 1:
- Vertex positions scale correctly
- Normals get stretched incorrectly
```

### The Solution: Normal Matrix

```
Normal Matrix = transpose(inverse(Model Matrix))
```

This ensures normals remain perpendicular to surfaces after transformation.

**Optimization**: For uniform scaling and rotation only, the model matrix works fine. But for robustness, always use the normal matrix.

## Lighting Calculations: Vertex vs Fragment

### Vertex Lighting (Gouraud Shading)
- Calculate lighting at vertices
- Interpolate colors across triangle
- **Pros**: Fast
- **Cons**: Low quality, misses details

### Fragment Lighting (Phong Shading)
- Calculate lighting per pixel
- Interpolate normals across triangle
- **Pros**: High quality, accurate highlights
- **Cons**: More expensive

This example uses **Phong Shading** (per-fragment lighting) for best quality.

## Lighting in Different Coordinate Spaces

You can do lighting calculations in various spaces:

### World Space (this example)
```
Transform normals to world space
Light position in world space
Camera position in world space
```
**Pros**: Intuitive, easy to debug
**Cons**: More transformation work in shader

### View Space
```
Transform everything to view space
Light and camera relative to view
```
**Pros**: Camera always at origin
**Cons**: Multiple lights need transformation

### Tangent Space
```
Used for normal mapping
Lighting per-pixel with texture normals
```
**Pros**: Enables detail via textures
**Cons**: Complex setup

## Material Properties

### Ambient Strength
- How much ambient light affects the surface
- Higher = brighter base color
- Typical: 0.1 (subtle) to 0.3 (bright)

### Diffuse Strength
- How much the surface scatters light
- Higher = more matte appearance
- Typical: 0.5 to 1.0

### Specular Strength
- How reflective/shiny the surface is
- Higher = brighter highlights
- Typical: 0.2 (matte) to 1.0 (very shiny)

### Shininess
- How focused the specular highlight is
- Lower = rough (broad highlight)
- Higher = smooth (tight highlight)
- Range: 2 to 256
- Common values:
  - 2-10: Rough surfaces (cloth, wood)
  - 32: Plastic
  - 128: Polished metal
  - 256: Mirror

## Limitations of Phong

Phong lighting is a simplification:

**Not physically accurate:**
- Doesn't conserve energy
- Ambient is a hack
- No indirect lighting

**Limited realism:**
- No shadows (covered in advanced example)
- No inter-reflections
- No subsurface scattering

**Modern alternatives:**
- Blinn-Phong (simple improvement)
- Cook-Torrance (physically based)
- PBR (Physically Based Rendering) (covered in advanced example)

Despite limitations, Phong is:
- Fast and simple
- Good for learning
- Sufficient for many use cases

## Building and Running

```bash
cargo run
```

You should see a cube with realistic lighting and shiny highlights.

## Code Structure

- `main.rs`: Uniforms, normal matrix, lighting parameters
- `shader.wgsl`: Phong lighting calculations (ambient + diffuse + specular)

## Common Issues

**Normals inverted?**
- Check face winding order
- Verify normal matrix calculation
- Ensure normals are normalized

**Black faces?**
- `max(dot, 0.0)` clamps negative values
- Normals pointing away from light are dark (correct)

**No specular highlights?**
- Check camera position
- Increase specular strength
- Adjust shininess
- Verify view direction calculation

**Lighting looks wrong after rotation?**
- Use normal matrix, not model matrix
- Ensure normals are in world space

## Exercises to Try

1. **Adjust Parameters**
   - Change ambient/diffuse/specular strengths
   - Try different shininess values
   - Move the light position

2. **Implement Blinn-Phong**
   - Replace reflection vector with halfway vector
   - Compare visual results

3. **Add Multiple Lights**
   - Extend uniforms to support multiple lights
   - Sum contributions from each light

4. **Add Attenuation**
   - Make light fade with distance
   - Formula: `1.0 / (constant + linear*d + quadratic*d²)`

5. **Experiment with Colors**
   - Colored lights (red, green, blue)
   - Different material colors

## Next Steps

The next example (05-camera) extends this by:
- Implementing a controllable camera system
- Keyboard and mouse input for camera movement
- First-person or orbital camera controls
- Smooth camera movement

## Further Reading

- [Phong Reflection Model (Wikipedia)](https://en.wikipedia.org/wiki/Phong_reflection_model)
- [Blinn-Phong Shading](https://en.wikipedia.org/wiki/Blinn%E2%80%93Phong_reflection_model)
- [Learn OpenGL - Basic Lighting](https://learnopengl.com/Lighting/Basic-Lighting)
- [The Phong Lighting Model](http://www.lighthouse3d.com/tutorials/glsl-tutorial/the-phong-lighting-model/)
