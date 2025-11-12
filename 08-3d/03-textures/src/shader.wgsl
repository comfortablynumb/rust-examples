// Texture and sampler bindings
// @group(0) corresponds to set_bind_group(0, ...)
// @binding(N) corresponds to the binding index in the bind group layout

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>; // The texture itself

@group(0) @binding(1)
var s_diffuse: sampler; // The sampler defines how to sample the texture

// Vertex input from buffer
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>, // UV coordinates
};

// Vertex output / Fragment input
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>, // Interpolated UV coordinates
};

// Vertex shader
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);

    // Pass texture coordinates to fragment shader
    // These will be automatically interpolated across the triangle
    out.tex_coords = model.tex_coords;

    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the texture at the interpolated UV coordinates
    // textureSample() uses the sampler to determine filtering, wrapping, etc.
    // The result is a vec4<f32> containing the RGBA color from the texture
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
