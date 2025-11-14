struct Uniforms {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Vertex data (per-vertex)
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

// Instance data (per-instance)
struct InstanceInput {
    @location(2) instance_position: vec3<f32>,
    @location(3) instance_scale: f32,
    @location(4) instance_color: vec3<f32>,
    @location(5) instance_rotation: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

// Create a rotation matrix around Y axis
fn rotate_y(angle: f32) -> mat4x4<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return mat4x4<f32>(
        vec4<f32>(c, 0.0, s, 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(-s, 0.0, c, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0),
    );
}

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    // Create model matrix for this instance
    let rotation = rotate_y(instance.instance_rotation);
    let scale = instance.instance_scale;
    let translation = instance.instance_position;

    // Transform vertex position
    let scaled_pos = vertex.position * scale;
    let rotated_pos = (rotation * vec4<f32>(scaled_pos, 1.0)).xyz;
    let world_pos = rotated_pos + translation;

    // Apply view-projection
    out.clip_position = uniforms.view_proj * vec4<f32>(world_pos, 1.0);

    // Mix vertex color with instance color
    out.color = vertex.color * instance.instance_color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
