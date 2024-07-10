struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(in_vertex_index % 100u) - 50.0;
    let y = f32(in_vertex_index / 100u) - 50.0;
    let grid_pos = vec2<f32>(x, y);
    
    out.clip_position = camera.view_proj * vec4<f32>(grid_pos, 0.0, 1.0);
    out.color = vec3<f32>(0.5, 0.5, 0.5);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}