# Lighting with Phong Shading

This example demonstrates implementing realistic lighting using the Phong/Blinn-Phong shading model with ambient, diffuse, and specular components.

## What This Example Demonstrates

1. **Phong Shading Model**
   - Ambient lighting (base illumination)
   - Diffuse lighting (matte surfaces)
   - Specular lighting (shiny highlights)
   - Blinn-Phong optimization

2. **Normal Vectors**
   - Per-vertex normals
   - Normal interpolation
   - Surface orientation for lighting

3. **Light Uniforms**
   - Light position
   - Light color
   - Lighting parameters
   - Animated light source

4. **Fragment Shader Lighting**
   - Per-pixel lighting calculations
   - Vector math for lighting
   - Color composition

## Phong Lighting Model

### Components

**Ambient**: Base lighting that affects everything equally
```wgsl
let ambient = ambient_strength * light_color;
```

**Diffuse**: Directional lighting based on surface angle
```wgsl
let diff = max(dot(normal, light_dir), 0.0);
let diffuse = diff * light_color;
```

**Specular**: Shiny highlights from reflected light
```wgsl
let spec = pow(max(dot(normal, halfway_dir), 0.0), shininess);
let specular = specular_strength * spec * light_color;
```

**Final Color**:
```wgsl
let result = (ambient + diffuse + specular) * object_color;
```

## Building and Running

```bash
cargo run
```

You'll see a rotating cube with realistic lighting and an animated light source.

## Further Reading

- [LearnOpenGL - Basic Lighting](https://learnopengl.com/Lighting/Basic-Lighting)
- [Blinn-Phong Shading](https://en.wikipedia.org/wiki/Blinn%E2%80%93Phong_reflection_model)
