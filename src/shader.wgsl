struct PlacementUniform {
    location: vec4<f32>,
    color: vec4<f32>,
}
@group(0) @binding(0)
var<uniform> placement: PlacementUniform;

struct WindowSizeUniform {
    x: u32,
    y: u32,
}

@group(0) @binding(1)
var<uniform> window_size: WindowSizeUniform; // change to a matrix

struct VertexInput {
    @location(0) position: vec3<f32>,
    // rotation eventually maybe
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

// Vertex Shader - Shaping

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = placement.location + vec4<f32>(model.position, 1.0);
    out.color = placement.color;
    return out;
}

// Fragment Shader - Coloring

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}