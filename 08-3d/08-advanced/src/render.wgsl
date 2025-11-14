// Main rendering shader with shadow mapping

struct Uniforms {
    view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
    normal_matrix: mat4x4<f32>,
    light_view_proj: mat4x4<f32>,
    light_position: vec3<f32>,
    _padding1: f32,
    light_color: vec3<f32>,
    _padding2: f32,
    camera_position: vec3<f32>,
    _padding3: f32,
    object_color: vec3<f32>,
    _padding4: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var shadow_map: texture_depth_2d;

@group(0) @binding(2)
var shadow_sampler: sampler_comparison;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) shadow_position: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let world_position = uniforms.model * vec4<f32>(in.position, 1.0);
    out.world_position = world_position.xyz;
    out.clip_position = uniforms.view_proj * world_position;

    let world_normal = uniforms.normal_matrix * vec4<f32>(in.normal, 0.0);
    out.world_normal = normalize(world_normal.xyz);

    // Calculate position in shadow map space
    let shadow_pos = uniforms.light_view_proj * world_position;
    out.shadow_position = shadow_pos.xyz / shadow_pos.w;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.world_normal);
    let light_dir = normalize(uniforms.light_position - in.world_position);
    let view_dir = normalize(uniforms.camera_position - in.world_position);

    // Phong lighting
    let ambient = 0.2 * uniforms.light_color;

    let diffuse_factor = max(dot(normal, light_dir), 0.0);
    let diffuse = diffuse_factor * uniforms.light_color;

    let reflect_dir = reflect(-light_dir, normal);
    let spec_factor = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
    let specular = 0.5 * spec_factor * uniforms.light_color;

    // Shadow mapping
    // Convert from NDC [-1,1] to texture coordinates [0,1]
    let shadow_coord = vec3<f32>(
        in.shadow_position.x * 0.5 + 0.5,
        in.shadow_position.y * -0.5 + 0.5,  // Flip Y for texture coordinates
        in.shadow_position.z
    );

    // Sample shadow map with PCF (Percentage Closer Filtering)
    var shadow = 0.0;
    let texel_size = 1.0 / 2048.0;

    // 3x3 PCF kernel for soft shadows
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let offset = vec2<f32>(f32(x), f32(y)) * texel_size;
            let sample_coord = shadow_coord.xy + offset;
            shadow += textureSampleCompare(
                shadow_map,
                shadow_sampler,
                sample_coord,
                shadow_coord.z - 0.005  // Bias to reduce shadow acne
            );
        }
    }
    shadow /= 9.0;  // Average the samples

    // Apply shadow to diffuse and specular only (not ambient)
    let lighting = ambient + shadow * (diffuse + specular);
    let final_color = lighting * uniforms.object_color;

    return vec4<f32>(final_color, 1.0);
}
