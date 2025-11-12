// Uniform buffer containing transformation matrices
struct Uniforms {
    view_proj: mat4x4<f32>, // Combined model-view-projection matrix
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Vertex input from buffer
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

// Vertex output / Fragment input
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

// Vertex shader
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    // Transform the vertex position from model space to clip space
    // The view_proj matrix combines:
    // 1. Model matrix: positions object in world
    // 2. View matrix: positions camera in world
    // 3. Projection matrix: applies perspective
    out.clip_position = uniforms.view_proj * vec4<f32>(model.position, 1.0);

    // Pass color through to fragment shader
    out.color = model.color;

    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simply return the interpolated color
    return vec4<f32>(in.color, 1.0);
}
