#import bevy_sprite::mesh2d_view_bindings::globals 
#import shadplay::shader_utils::common::{NEG_HALF_PI, shader_toy_default, rotate2D, TWO_PI}
#import bevy_render::view::View
#import bevy_pbr::forward_io::VertexOutput;

@group(0) @binding(0) var<uniform> view: View;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let top = vec3(0.447, 0.529, 0.835);
    let bottom = vec3(0.027,0.047,0.310);

    let gradient = mix(top, bottom, uv.y);

    return vec4f(gradient, 1.0);
}    
