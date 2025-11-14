// Render shader for particle visualization

struct Uniforms {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) velocity: vec3<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Generate a quad billboard around each particle
    // Each particle is drawn as a 6-vertex quad (2 triangles)
    var quad_positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
    );

    let quad_pos = quad_positions[in.vertex_index];
    let particle_size = 0.05;

    // Calculate billboard position
    // In a proper billboard, we'd use the view matrix to face camera
    // For simplicity, we just offset in XY
    let billboard_pos = in.position + vec3<f32>(quad_pos * particle_size, 0.0);

    out.clip_position = uniforms.view_proj * vec4<f32>(billboard_pos, 1.0);
    out.color = in.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
