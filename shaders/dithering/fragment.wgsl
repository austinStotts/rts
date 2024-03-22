@group(0) @binding(0) var inputTexture: texture_2d<f32>;
@group(0) @binding(1) var sampler0: sampler;

@group(1) @binding(0) var<uniform> params: Parameters;

struct Parameters {
    sigma1: f32,
    tau: f32,
    gfact: f32,
    epsilon: f32,
    num_gvf_iterations: i32,
    enable_xdog: u32,
    shader_index: u32,
}

@fragment
fn frag_main(@location(0) texcoord: vec2<f32>) -> @location(0) vec4<f32> {
    var ditherMatrix = array<vec4<f32>, 4>(
        vec4<f32>( 1.0/16.0,  9.0/16.0,  3.0/16.0, 11.0/16.0),
        vec4<f32>(13.0/16.0,  5.0/16.0, 15.0/16.0,  7.0/16.0),
        vec4<f32>( 4.0/16.0, 12.0/16.0,  2.0/16.0, 10.0/16.0),
        vec4<f32>(16.0/16.0,  8.0/16.0, 14.0/16.0,  6.0/16.0) 
    );

    let td = textureDimensions(inputTexture);
    var color = textureSample(inputTexture, sampler0, texcoord);

    
    var matrix_index = vec2<i32>(vec2<i32>(texcoord.xy / 4.0));
    var threshold = ditherMatrix[matrix_index.y][matrix_index.x];


    return vec4<f32>(threshold,0.0,0.0, 1.0);
}