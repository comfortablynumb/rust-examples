// Shadow map generation shader

struct Uniforms {
    light_view_proj: mat4x4<f32>,
    model: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> @builtin(position) vec4<f32> {
    let world_position = uniforms.model * vec4<f32>(in.position, 1.0);
    return uniforms.light_view_proj * world_position;
}

// No fragment shader needed - depth-only pass
