struct Uniforms {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    light_position: vec3<f32>,
    _padding1: f32,
    light_color: vec3<f32>,
    _padding2: f32,
    camera_position: vec3<f32>,
    _padding3: f32,
};

struct Material {
    color: vec3<f32>,
    _padding1: f32,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
};

struct InstanceData {
    model: mat4x4<f32>,
    normal_matrix: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(1) @binding(0)
var<uniform> material: Material;

var<push_constant> instance: InstanceData;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let world_pos = instance.model * vec4<f32>(model.position, 1.0);
    out.world_position = world_pos.xyz;
    out.clip_position = uniforms.projection * uniforms.view * world_pos;

    let world_normal = instance.normal_matrix * vec4<f32>(model.normal, 0.0);
    out.world_normal = normalize(world_normal.xyz);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.world_normal);
    let light_dir = normalize(uniforms.light_position - in.world_position);
    let view_dir = normalize(uniforms.camera_position - in.world_position);

    // Phong lighting with material properties
    let ambient = material.ambient * uniforms.light_color;

    let diffuse_factor = max(dot(normal, light_dir), 0.0);
    let diffuse = material.diffuse * diffuse_factor * uniforms.light_color;

    let reflect_dir = reflect(-light_dir, normal);
    let spec_factor = pow(max(dot(view_dir, reflect_dir), 0.0), material.shininess);
    let specular = material.specular * spec_factor * uniforms.light_color;

    let lighting = ambient + diffuse + specular;
    let final_color = lighting * material.color;

    return vec4<f32>(final_color, 1.0);
}
