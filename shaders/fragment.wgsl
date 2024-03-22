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
    if (params.shader_index == 0u) { 

        // *******************************************
        //                  invert



        let color = textureSample(inputTexture, sampler0, texcoord);
        let red = 1.0 - color.r;
        let green = 1.0 - color.g;
        let blue = 1.0 - color.b;
        return vec4<f32>(red, green, blue, color.a);
    } else if (params.shader_index == 1u) { 
        
        // *******************************************
        //                gaussian blur


        var kernelSize: i32 = i32(ceil(params.sigma1 * 3) * 2.0 + 1.0);

        var textureSize: vec2<f32> = vec2<f32>(textureDimensions(inputTexture));

        var result: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        var sum: f32 = 0.0;

        // Horizontal pass
        for (var offsetX : i32 = -kernelSize / 2; offsetX <= kernelSize / 2; offsetX++) {
            var samplePos: vec2<f32> = texcoord + vec2<f32>(f32(offsetX) / textureSize.x, 0.0);
            var weight: f32 = exp(-(f32(offsetX) * f32(offsetX)) / (2.0 * params.sigma1 * params.sigma1)) / (sqrt(2.0 * 3.14159) * params.sigma1);
            result += textureSample(inputTexture, sampler0, samplePos) * weight;
            sum += weight;
        }
        result /= sum;

        // Vertical pass
        sum = 0.0;
        for (var offsetY : i32 = -kernelSize / 2; offsetY <= kernelSize / 2; offsetY++) {
            var samplePos: vec2<f32> = texcoord + vec2<f32>(0.0, f32(offsetY) / textureSize.y);
            var weight: f32 = exp(-(f32(offsetY) * f32(offsetY)) / (2.0 * params.sigma1 * params.sigma1)) / (sqrt(2.0 * 3.14159) * params.sigma1);
            result += textureSample(inputTexture, sampler0, samplePos) * weight;
            sum += weight;
        }
        result /= sum;

        return result;
    } else if (params.shader_index == 2u) { 
        
        // *******************************************
        //               quantization


        let texelCoord = vec2<i32>(texcoord * vec2<f32>(textureDimensions(inputTexture)));
        let srcPixel: vec4<f32> = textureLoad(inputTexture, texelCoord, 0);

        // Quantize the pixel to a predefined set of colors
        let palette: array<vec3<f32>, 8> = array<vec3<f32>, 8>(
            vec3<f32>(0.0, 0.0, 0.0), // Black
            vec3<f32>(1.0, 1.0, 1.0), // White
            vec3<f32>(1.0, 0.0, 0.0), // Red
            vec3<f32>(0.0, 1.0, 0.0), // Green
            vec3<f32>(0.0, 0.0, 1.0), // Blue
            vec3<f32>(1.0, 1.0, 0.0), // Yellow
            vec3<f32>(1.0, 0.0, 1.0), // Magenta
            vec3<f32>(0.0, 1.0, 1.0)  // Cyan
        );

        // Find the closest color in the palette
        let black_distance = length(srcPixel.rgb - palette[0]);
        let white_distance = length(srcPixel.rgb - palette[1]);
        let red_distance = length(srcPixel.rgb - palette[2]);
        let green_distance = length(srcPixel.rgb - palette[3]);
        let blue_distance = length(srcPixel.rgb - palette[4]);
        let yellow_distance = length(srcPixel.rgb - palette[5]);
        let magenta_distance = length(srcPixel.rgb - palette[6]);
        let cyan_distance = length(srcPixel.rgb - palette[7]);

        var minDistance: f32 = black_distance;
        var closestColor: vec3<f32> = palette[0];

        if (white_distance < minDistance) {
            minDistance = white_distance;
            closestColor = palette[1];
        }

        if (red_distance < minDistance) {
            minDistance = red_distance;
            closestColor = palette[2];
        }

        if (green_distance < minDistance) {
            minDistance = green_distance;
            closestColor = palette[3];
        }

        if (blue_distance < minDistance) {
            minDistance = blue_distance;
            closestColor = palette[4];
        }

        if (yellow_distance < minDistance) {
            minDistance = yellow_distance;
            closestColor = palette[5];
        }

        if (magenta_distance < minDistance) {
            minDistance = magenta_distance;
            closestColor = palette[6];
        }

        if (cyan_distance < minDistance) {
            minDistance = cyan_distance;
            closestColor = palette[7];
        }

        return vec4<f32>(closestColor, 1.0);
    } else if (params.shader_index == 3u) { 
        
        // *******************************************
        //           sobel edge detection
        
        
        let texelCoord = vec2<i32>(texcoord * vec2<f32>(textureDimensions(inputTexture)));

        let srcPixel: vec4<f32> = textureLoad(inputTexture, texelCoord, 0);

        // Compute the luminance of the pixel
        let luminance = dot(srcPixel.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));

        // Sobel edge detection
        var gradientX: f32 = 0.0;
        var gradientY: f32 = 0.0;

        // Horizontal Sobel filter
        gradientX += textureLoad(inputTexture, texelCoord + vec2(-1, -1), 0).r * -1.0;
        gradientX += textureLoad(inputTexture, texelCoord + vec2(-1, 0), 0).r * -2.0;
        gradientX += textureLoad(inputTexture, texelCoord + vec2(-1, 1), 0).r * -1.0;
        gradientX += textureLoad(inputTexture, texelCoord + vec2(1, -1), 0).r * 1.0;
        gradientX += textureLoad(inputTexture, texelCoord + vec2(1, 0), 0).r * 2.0;
        gradientX += textureLoad(inputTexture, texelCoord + vec2(1, 1), 0).r * 1.0;

        // Vertical Sobel filter
        gradientY += textureLoad(inputTexture, texelCoord + vec2(-1, -1), 0).r * -1.0;
        gradientY += textureLoad(inputTexture, texelCoord + vec2(0, -1), 0).r * -2.0;
        gradientY += textureLoad(inputTexture, texelCoord + vec2(1, -1), 0).r * -1.0;
        gradientY += textureLoad(inputTexture, texelCoord + vec2(-1, 1), 0).r * 1.0;
        gradientY += textureLoad(inputTexture, texelCoord + vec2(0, 1), 0).r * 2.0;
        gradientY += textureLoad(inputTexture, texelCoord + vec2(1, 1), 0).r * 1.0;

        // Compute the magnitude of the gradient
        let magnitude = sqrt(gradientX * gradientX + gradientY * gradientY);

        // Output the result
        return vec4(magnitude, magnitude, magnitude, 1.0);
    } else if (params.shader_index == 4u) { 
        
        // *******************************************
        //         difference of gaussians



        var sigma2 = params.sigma1 / 16.0;
        var radius1 = params.sigma1 * 3.0;
        var radius2 = sigma2 * 2.0;

        var textureSize: vec2<f32> = vec2<f32>(textureDimensions(inputTexture));

        // ----- Gaussian Blur 1 -----
        var blurredImage1: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0); 
        var sum1: f32 = 0.0;

        var kernelSize1: i32 = i32(ceil(radius1) * 2.0 + 1.0); 
        for (var offsetX : i32 = -kernelSize1 / 2; offsetX <= kernelSize1 / 2; offsetX++) {
            var samplePos: vec2<f32> = texcoord + vec2<f32>(f32(offsetX) / textureSize.x, 0.0);
            var weight: f32 = exp(-(f32(offsetX) * f32(offsetX)) / (2.0 * params.sigma1 * params.sigma1)) / (sqrt(2.0 * 3.14159) * params.sigma1);
            blurredImage1 += textureSample(inputTexture, sampler0, samplePos) * weight;
            sum1 += weight;
        }
        blurredImage1 /= sum1;

        // ----- Gaussian Blur 2 -----
        var blurredImage2: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0); 
        var sum2: f32 = 0.0;

        var kernelSize2: i32 = i32(ceil(radius2) * 2.0 + 1.0); 
        for (var offsetX : i32 = -kernelSize2 / 2; offsetX <= kernelSize2 / 2; offsetX++) {
            var samplePos: vec2<f32> = texcoord + vec2<f32>(f32(offsetX) / textureSize.x, 0.0);
            var weight: f32 = exp(-(f32(offsetX) * f32(offsetX)) / (2.0 * sigma2 * sigma2)) / (sqrt(2.0 * 3.14159) * sigma2);
            blurredImage2 += textureSample(inputTexture, sampler0, samplePos) * weight;
            sum2 += weight;
        }
        blurredImage2 /= sum2;

        var difference = blurredImage1.r - blurredImage2.r; 

        if (abs(difference) >= params.tau) {
            difference = 1.0 - exp(-difference / params.tau);
        } else {
            difference = 0.0;
        }



        return vec4<f32>(difference, difference, difference, 1.0);
    } else if (params.shader_index == 5u) {


        // *******************************************
        //            flow based XDoG


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

            // Variables to store GVF (initialized to 0.0)
            var u: f32 = 0.0;
            var v: f32 = 0.0;

            // Perform GVF iterations (replace 'num_gvf_iterations' with your desired count)
            for (var i = 0; i < params.num_gvf_iterations; i++) {
                // Update equations (using dx and dy for gradients)
                let u_temp = dx + params.gfact * u;
                let v_temp = dy + params.gfact * v;
                
                // Normalize (sqrt function might require additional implementation)
                // let norm_factor = approximate_inversesqrt(u_temp * u_temp + v_temp * v_temp + params.epsilon * params.epsilon);
                let norm_factor = sqrt(u_temp * u_temp + v_temp * v_temp + params.epsilon);
                u = u_temp / norm_factor;
                v = v_temp / norm_factor;
            }

            // Gaussian Blur Implementation
            var kernelSize1 : i32 = i32(ceil(radius1) * 2.0 + 1.0); 
            var kernelSize2 : i32 = i32(ceil(radius2) * 2.0 + 1.0);
            var blurredImage1 = color; 
            var blurredImage2 = color; 
            var total_weight: f32 = 0.0;

            // Blur 1 Horizontal Pass
            for (var offsetX : i32 = -kernelSize1 / 2; offsetX <= kernelSize1 / 2; offsetX++) {
                let samplePos: vec2<f32> = texcoord + vec2<f32>(f32(offsetX) / f32(textureDimensions(inputTexture).x), 0.0);
                let weight: f32 = exp(-(f32(offsetX) * f32(offsetX)) / (2.0 * params.sigma1 * params.sigma1)) / (sqrt(2.0 * 3.14159) * params.sigma1);
                blurredImage1 += textureSample(inputTexture, sampler0, samplePos) * weight;
                total_weight += weight; // Keep track of total weight
            }


            // Blur 1 Vertical Pass
            for (var offsetY : i32 = -kernelSize1 / 2; offsetY <= kernelSize1 / 2; offsetY++) {
                let samplePos: vec2<f32> = texcoord + vec2<f32>(0.0, f32(offsetY) / f32(textureDimensions(inputTexture).y));
                let weight: f32 = exp(-(f32(offsetY) * f32(offsetY)) / (2.0 * params.sigma1 * params.sigma1)) / (sqrt(2.0 * 3.14159) * params.sigma1);
                blurredImage1 += textureSample(inputTexture, sampler0, samplePos) * weight;
                total_weight += weight; // Keep track of total weight
            }


            // Blur 2 Horizontal Pass
            for (var offsetX : i32 = -kernelSize2 / 2; offsetX <= kernelSize2 / 2; offsetX++) {
                let samplePos: vec2<f32> = texcoord + vec2<f32>(f32(offsetX) / f32(textureDimensions(inputTexture).x), 0.0);
                let weight: f32 = exp(-(f32(offsetX) * f32(offsetX)) / (2.0 * sigma2 * sigma2)) / (sqrt(2.0 * 3.14159) * sigma2);
                blurredImage2 += textureSample(inputTexture, sampler0, samplePos) * weight;
                total_weight += weight; // Keep track of total weight
            }


            // Blur 2 Vertical Pass
            for (var offsetY : i32 = -kernelSize2 / 2; offsetY <= kernelSize2 / 2; offsetY++) {
                let samplePos: vec2<f32> = texcoord + vec2<f32>(0.0, f32(offsetY) / f32(textureDimensions(inputTexture).y));
                let weight: f32 = exp(-(f32(offsetY) * f32(offsetY)) / (2.0 * sigma2 * sigma2)) / (sqrt(2.0 * 3.14159) * sigma2);
                blurredImage2 += textureSample(inputTexture, sampler0, samplePos) * weight;
                total_weight += weight; // Keep track of total weight
            }

            blurredImage1 /= total_weight;
            blurredImage2 /= total_weight;

            var xdog_difference = blurredImage2.r - blurredImage1.r; // Assumes only using red channel


            // Optional Thresholding  
            if (abs(xdog_difference) >= params.tau) {
                xdog_difference = 1.0 - exp(-xdog_difference / params.tau);
            } else {
                xdog_difference = 0.0;
            }

            

            color.r = xdog_difference;
            color.g = color.r;
            color.b = color.r;

            // color.r = abs(dx); // Visualize gradient magnitude X
            // color.g = abs(dy); // Visualize gradient magnitude Y
            // color.b = abs(xdog_difference); // Visualize the XDoG difference
        } // ...
        


        return vec4<f32>(color.r, color.g, color.b, 1.0);
    } else if (params.shader_index == 6u) {


        // *******************************************
        //       edge directions

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
    } else if (params.shader_index == 7u) {


        // *******************************************
        //          bayer dithering


        var ditherMatrix = array<vec4<f32>, 4>(
            vec4<f32>( 1.0/16.0,  9.0/16.0,  3.0/16.0, 11.0/16.0),
            vec4<f32>(13.0/16.0,  5.0/16.0, 15.0/16.0,  7.0/16.0),
            vec4<f32>( 4.0/16.0, 12.0/16.0,  2.0/16.0, 10.0/16.0),
            vec4<f32>(16.0/16.0,  8.0/16.0, 14.0/16.0,  6.0/16.0) 
        );

        let td = textureDimensions(inputTexture);
        var color = textureSample(inputTexture, sampler0, texcoord);

        var y0 = texcoord.y * f32(td.y);
        var y1 = y0 % f32(td.y);
        var y2 = y1 % 4.0;
        var y = y2 / 4.0;

        var x0 = texcoord.x * f32(td.x);
        var x1 = x0 % f32(td.x);
        var x2 = x1 % 4.0;
        var x = x2 / 4.0;

        // var x1 = texcoord.x / f32(td.x);
        // var x2 = x1 * 4.0;
        // var x = i32(floor(x2));

        // var coords = texcoord / vec2<f32>(td);
        // var matrix_coords = coords * 4.0;
        // var fcoords = vec2<i32>(floor(matrix_coords));

        var threshold = ditherMatrix[i32(y2)][i32(x2)];
        var out = color * threshold;

        var luminance = (color.rgb * vec3<f32>(0.2126, 0.7152, 0.0722));

        var ditherFactor = clamp(vec3<f32>(1.0) - luminance, vec3<f32>(0.0), vec3<f32>(1.0));
        var quantcolor = clamp(((color - threshold) * vec4<f32>(ditherFactor) * vec4<f32>(params.tau)), vec4<f32>(0.0), vec4<f32>(1.0));

        return vec4<f32>(quantcolor.r,quantcolor.g,quantcolor.b,1.0);
    
    
    }
    else {
        return textureSample(inputTexture, sampler0, texcoord);
    }
}