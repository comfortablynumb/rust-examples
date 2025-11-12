// Render shader for displaying particles as quads

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    @location(0) particle_pos: vec2<f32>,
    @location(1) particle_color: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;

    // Create a quad for each particle (billboard)
    // vertex_index 0-3 creates a quad: (0,0), (1,0), (0,1), (1,1)
    let quad_pos = vec2<f32>(
        f32(vertex_index & 1u),
        f32((vertex_index >> 1u) & 1u),
    );

    // Center and scale the quad
    let particle_size = 0.01;
    let centered_quad = (quad_pos - 0.5) * particle_size * 2.0;

    // Position the quad at the particle position
    let final_pos = particle_pos + centered_quad;

    out.position = vec4<f32>(final_pos, 0.0, 1.0);
    out.color = particle_color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
