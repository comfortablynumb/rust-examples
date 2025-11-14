// Texture and sampler bindings
// @group(0) refers to the bind group at index 0
// @binding(N) refers to the binding index within that group

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;  // The texture data

@group(0) @binding(1)
var s_diffuse: sampler;  // The sampler (filtering, wrapping, etc.)

// Vertex shader input
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

// Vertex shader output / Fragment shader input
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

// Vertex shader
@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the texture at the given coordinates
    // textureSample performs texture filtering based on the sampler settings
    // - If mag_filter is Linear, it interpolates between nearby texels
    // - If address_mode is Repeat, coordinates outside [0,1] wrap around
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
