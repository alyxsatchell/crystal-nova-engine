struct PlacementUniform {
    location: vec3<f32>,
}
@group(0) @binding(0)
var<uniform> placement: PlacementUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    // rotation eventually maybe
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

// Vertex Shader - Shaping

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(placement.location + model.position, 1.0);
    return out;
}

// Fragment Shader - Coloring

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.clip_position;
}