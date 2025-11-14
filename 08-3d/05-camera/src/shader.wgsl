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

    let world_pos = uniforms.model * vec4<f32>(model.position, 1.0);
    out.world_position = world_pos.xyz;
    out.clip_position = uniforms.projection * uniforms.view * world_pos;

    let world_normal = uniforms.normal_matrix * vec4<f32>(model.normal, 0.0);
    out.world_normal = normalize(world_normal.xyz);

    out.color = model.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.world_normal);
    let light_dir = normalize(uniforms.light_position - in.world_position);
    let view_dir = normalize(uniforms.camera_position - in.world_position);

    // Phong lighting
    let ambient = uniforms.ambient_strength * uniforms.light_color;

    let diffuse_factor = max(dot(normal, light_dir), 0.0);
    let diffuse = uniforms.diffuse_strength * diffuse_factor * uniforms.light_color;

    let reflect_dir = reflect(-light_dir, normal);
    let spec_factor = pow(max(dot(view_dir, reflect_dir), 0.0), uniforms.shininess);
    let specular = uniforms.specular_strength * spec_factor * uniforms.light_color;

    let lighting = ambient + diffuse + specular;
    let final_color = lighting * in.color;

    return vec4<f32>(final_color, 1.0);
}
