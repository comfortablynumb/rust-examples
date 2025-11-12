# wgpu Scene with Multiple Objects

Demonstrates rendering multiple objects with different geometries, materials, and transformations. Shows scene management with push constants for efficient per-object data.

## Features

- **Multiple Mesh Types**: Cubes and spheres
- **Per-Object Materials**: Different colors and lighting properties
- **Push Constants**: Efficient per-object matrix updates
- **Scene Management**: Organized object hierarchy
- **Automatic Camera**: Orbiting camera for viewing

## Key Concepts

### Push Constants
Fast way to update small amounts of data per draw call without buffer updates.

### Material System
Each object has its own material properties (color, shininess, etc.)

### Scene Graph
Organized collection of objects with transforms and properties.

## Building

```bash
cargo run
```

The camera automatically rotates around the scene showing 4 objects with different materials.
