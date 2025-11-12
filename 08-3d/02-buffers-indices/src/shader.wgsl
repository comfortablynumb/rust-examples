// Vertex shader with buffer inputs

// VertexInput defines the structure of data coming from the vertex buffer
struct VertexInput {
    // @location(0) matches shader_location: 0 in Vertex::desc()
    // This is the position attribute (3 floats from the vertex buffer)
    @location(0) position: vec3<f32>,

    // @location(1) matches shader_location: 1 in Vertex::desc()
    // This is the color attribute (3 floats from the vertex buffer)
    @location(1) color: vec3<f32>,
};

// VertexOutput defines what data is passed to the fragment shader
struct VertexOutput {
    // @builtin(position) is required - this is where the vertex ends up on screen
    @builtin(position) clip_position: vec4<f32>,

    // @location(0) is a custom attribute that will be interpolated
    @location(0) color: vec3<f32>,
};

// Vertex shader entry point
// Unlike the triangle example, we now read from vertex buffers instead of hardcoding
@vertex
fn vs_main(
    model: VertexInput, // Data from the vertex buffer
) -> VertexOutput {
    var out: VertexOutput;

    // Convert 3D position to 4D clip-space position
    // We add w=1.0 for proper homogeneous coordinates
    // This allows for perspective division later
    out.clip_position = vec4<f32>(model.position, 1.0);

    // Pass the color to the fragment shader
    // The GPU will automatically interpolate this value across the triangle
    out.color = model.color;

    return out;
}

// Fragment shader entry point
// This runs once per pixel covered by our triangles
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Return the interpolated color with full opacity
    // Notice how the color smoothly transitions between vertices
    return vec4<f32>(in.color, 1.0);
}
