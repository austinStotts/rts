struct Parameters {
    sigma1: f32,
    tau: f32,
    gfact: f32,
    epsilon: f32,
    num_gvf_iterations: i32,
    enable_xdog: u32,
}

@group(0) @binding(0) var inputTexture: texture_2d<f32>;
@group(0) @binding(1) var sampler0: sampler;

@group(1) @binding(0) var<uniform> params: Parameters;

@fragment
fn frag_main(@location(0) texcoord: vec2<f32>) -> @location(0) vec4<f32> {
   // ... Obtain texture color ...
    // var original_color = textureSample(inputTexture, sampler0, texcoord);
    var color = textureSample(inputTexture, sampler0, texcoord);
    var original_color = color;

    var sigma2 = params.sigma1 / 16.0;
    var radius1 = params.sigma1 * 3.0;
    var radius2 = sigma2 * 2.0;

    if (params.enable_xdog == 1u) {
        // Gradient calculation using Sobel operators
         var sobel_x = array<vec3<f32>, 3>(
             vec3<f32>(-1.0, 0.0, 1.0),
             vec3<f32>(-2.0, 0.0, 2.0),
             vec3<f32>(-1.0, 0.0, 1.0)
         ); 

         var sobel_y = array<vec3<f32>, 3>(
             vec3<f32>(-1.0, -2.0, -1.0),
             vec3<f32>( 0.0,  0.0,  0.0),
             vec3<f32>( 1.0,  2.0,  1.0)
         );

        var dx = 0.0;
        var dy = 0.0;

        for (var i: i32 = -1; i <= 1; i++) {
            for (var j: i32 = -1; j <= 1; j++) {
                var offset : vec2<f32> = vec2<f32>(f32(i), f32(j)) / vec2<f32>(textureDimensions(inputTexture));          
                var sampleColor = textureSample(inputTexture, sampler0, texcoord + offset);
                dx += sampleColor.r * sobel_x[i + 1][j + 1]; 
                dy += sampleColor.r * sobel_y[i + 1][j + 1];
            }
        }        

        color.r = dx-dy;
        color.g = dx;
        color.b = dy;

    } // ...
    


    return vec4<f32>(color.r, color.g, color.b, 1.0);
}



fn approximate_inversesqrt(x: f32) -> f32 {
   let normalized_x = clamp(x, 0.0001, 1.0); 
   let index = min(floor(normalized_x * 256.0), 255.0); 
   return inverse_sqrt_table[u32(index)];              
}





