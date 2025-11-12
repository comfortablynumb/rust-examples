// Vertex shader

// VertexOutput is the data passed from vertex shader to fragment shader
struct VertexOutput {
    // @builtin(position) is a special built-in that tells the GPU where the vertex is
    // This is in clip space: x, y, z in [-1, 1] and w is for perspective division
    @builtin(position) clip_position: vec4<f32>,

    // @location(0) is a custom output that we'll use for color
    // The fragment shader will receive interpolated values
    @location(0) color: vec3<f32>,
};

// Vertex shader main function
// @vertex marks this as the vertex shader entry point
// @builtin(vertex_index) gives us the index of the current vertex (0, 1, or 2 for a triangle)
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    // Hardcoded triangle vertices in clip space
    // Clip space: center is (0,0), right is +X, up is +Y
    // The Z coordinate is used for depth testing (not used in this example)
    // The W coordinate is for perspective division (we use 1.0 for orthographic)

    // Define positions for the three vertices of our triangle
    // Triangle points: top (0, 0.5), bottom-left (-0.5, -0.5), bottom-right (0.5, -0.5)
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;

    // Alternative: explicit position array
    // var positions = array<vec2<f32>, 3>(
    //     vec2<f32>(0.0, 0.5),    // Top
    //     vec2<f32>(-0.5, -0.5),  // Bottom left
    //     vec2<f32>(0.5, -0.5),   // Bottom right
    // );
    // out.clip_position = vec4<f32>(positions[in_vertex_index], 0.0, 1.0);

    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);

    // Assign different colors to each vertex
    // The fragment shader will interpolate these colors across the triangle
    if (in_vertex_index == 0u) {
        out.color = vec3<f32>(1.0, 0.0, 0.0); // Red
    } else if (in_vertex_index == 1u) {
        out.color = vec3<f32>(0.0, 1.0, 0.0); // Green
    } else {
        out.color = vec3<f32>(0.0, 0.0, 1.0); // Blue
    }

    return out;
}

// Fragment shader

// @fragment marks this as the fragment shader entry point
// The fragment shader runs once for each pixel covered by the triangle
// It receives interpolated values from the vertex shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Return the interpolated color with full opacity (alpha = 1.0)
    // The color values are automatically interpolated across the triangle
    // by the GPU's rasterizer
    return vec4<f32>(in.color, 1.0);
}
