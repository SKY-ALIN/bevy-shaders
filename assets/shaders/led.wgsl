#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return color;
}
