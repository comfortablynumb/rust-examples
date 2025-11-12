// Uniforms containing matrices and lighting parameters
struct Uniforms {
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    normal_matrix: mat4x4<f32>,
    light_position: vec3<f32>,
    _padding1: f32,
    light_color: vec3<f32>,
    _padding2: f32,
    camera_position: vec3<f32>,
    _padding3: f32,
    ambient_strength: f32,
    diffuse_strength: f32,
    specular_strength: f32,
    shininess: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) color: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform position to world space
    let world_pos = uniforms.model * vec4<f32>(model.position, 1.0);
    out.world_position = world_pos.xyz;

    // Transform to clip space
    out.clip_position = uniforms.projection * uniforms.view * world_pos;

    // Transform normal to world space using normal matrix
    // Normal matrix ensures normals are transformed correctly with non-uniform scaling
    let world_normal = uniforms.normal_matrix * vec4<f32>(model.normal, 0.0);
    out.world_normal = normalize(world_normal.xyz);

    out.color = model.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Normalize interpolated normal (interpolation can denormalize it)
    let normal = normalize(in.world_normal);

    // Calculate lighting vectors
    let light_dir = normalize(uniforms.light_position - in.world_position);
    let view_dir = normalize(uniforms.camera_position - in.world_position);

    // --- Phong Lighting Model ---

    // 1. Ambient Component
    // Simulates indirect/ambient light - constant base illumination
    let ambient = uniforms.ambient_strength * uniforms.light_color;

    // 2. Diffuse Component (Lambertian Reflection)
    // Simulates scattered light based on surface angle to light
    // max(dot, 0.0) clamps to prevent negative values
    let diffuse_factor = max(dot(normal, light_dir), 0.0);
    let diffuse = uniforms.diffuse_strength * diffuse_factor * uniforms.light_color;

    // 3. Specular Component (Phong Reflection)
    // Simulates shiny highlights based on view angle
    // Reflects light direction around the normal
    let reflect_dir = reflect(-light_dir, normal);
    let spec_factor = pow(max(dot(view_dir, reflect_dir), 0.0), uniforms.shininess);
    let specular = uniforms.specular_strength * spec_factor * uniforms.light_color;

    // Combine all components with material color
    let lighting = ambient + diffuse + specular;
    let final_color = lighting * in.color;

    return vec4<f32>(final_color, 1.0);
}

// Alternative: Blinn-Phong Specular (more efficient and often looks better)
// Replace the specular calculation with:
//
// let halfway_dir = normalize(light_dir + view_dir);
// let spec_factor = pow(max(dot(normal, halfway_dir), 0.0), uniforms.shininess);
//
// Blinn-Phong uses the halfway vector instead of reflection,
// which is computationally cheaper and handles grazing angles better.
