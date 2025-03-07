#import bevy_pbr::forward_io::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
// #import "shaders/custom_material_import.wgsl"::COLOR_MULTIPLIER

#import bevy_pbr::{
    mesh_view_bindings::globals
}


@group(2) @binding(0) var<uniform> t: f32;
@group(2) @binding(1) var noise_texture: texture_2d<f32>;
@group(2) @binding(2) var color_texture: texture_2d<f32>;
@group(2) @binding(3) var material_color_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var uv: vec2f = (mesh.uv * 2.0) - 1.0;
    //let x = sin(pi*u);
    ////let y = cos(pi*u);
    
    let s = globals.time/10;
    //let z = sin(pi*v);
    //let w = cos(pi*v);
    
    let uu = vec2<f32>(1.0);
    let vv = vec2<f32>(1.0);
    let noise1 = textureSample(noise_texture, material_color_sampler, (mesh.uv+0.1*s*uu)%1)*0.3;
    //let noise2 = textureSample(noise_texture, material_color_sampler, (mesh.uv+0.2*s*vv)%1)*0.3;

    return textureSample(color_texture, material_color_sampler, (mesh.uv+s*uu/2+noise1.xy)%1);
}
