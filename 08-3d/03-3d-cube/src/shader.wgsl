// Uniform buffer containing the MVP matrix
struct Uniforms {
    mvp: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    // Transform the vertex position from model space to clip space
    // uniforms.mvp is the combined Model-View-Projection matrix
    // This single matrix multiplication transforms through all coordinate spaces:
    // Model space → World space → View space → Clip space
    out.clip_position = uniforms.mvp * vec4<f32>(model.position, 1.0);

    // Pass the color through
    out.color = model.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
