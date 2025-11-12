// Camera uniforms
struct CameraUniforms {
    view_proj: mat4x4<f32>,
    view_pos: vec3<f32>,
};

// Light uniforms for Phong shading
struct LightUniforms {
    position: vec3<f32>,
    color: vec3<f32>,
    ambient_strength: f32,
    specular_strength: f32,
    shininess: f32,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

@group(0) @binding(1)
var<uniform> light: LightUniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform position to world space (we're using model matrix in view_proj)
    let world_pos = vec4<f32>(model.position, 1.0);
    out.world_position = world_pos.xyz;

    // Transform to clip space
    out.clip_position = camera.view_proj * world_pos;

    // Pass normal to fragment shader
    // Note: For proper lighting with non-uniform scaling, should use normal matrix
    out.world_normal = model.normal;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Object color (you can make this a uniform or texture)
    let object_color = vec3<f32>(0.8, 0.2, 0.3);

    // Normalize the interpolated normal
    let normal = normalize(in.world_normal);

    // Calculate light direction
    let light_dir = normalize(light.position - in.world_position);

    // Ambient component
    // Provides base illumination so objects aren't completely black
    let ambient = light.ambient_strength * light.color;

    // Diffuse component (Lambertian reflection)
    // Brightness depends on angle between normal and light direction
    let diff = max(dot(normal, light_dir), 0.0);
    let diffuse = diff * light.color;

    // Specular component (Blinn-Phong)
    // Creates shiny highlights
    let view_dir = normalize(camera.view_pos - in.world_position);
    let halfway_dir = normalize(light_dir + view_dir);
    let spec = pow(max(dot(normal, halfway_dir), 0.0), light.shininess);
    let specular = light.specular_strength * spec * light.color;

    // Combine all lighting components
    let result = (ambient + diffuse + specular) * object_color;

    return vec4<f32>(result, 1.0);
}
