// 1. Sample texture color
var color = textureSample(inputTexture, sampler0, texcoord);

// 2. Get texture dimensions
var ts = vec2<f32>(textureDimensions(inputTexture));

// 3. Define a larger Bayer matrix for luminance dithering
var bayer_matrix = array<vec4<f32>, 16>(
  vec4<f32>(0.0078, 0.1406, 0.2734, 0.4062),
  vec4<f32>(0.4390, 0.5718, 0.7046, 0.8374),
  vec4<f32>(0.8702, 0.0039, 0.1367, 0.2795),
  vec4<f32>(0.4023, 0.5351, 0.6679, 0.8007),
  vec4<f32>(0.8335, 0.9663, 0.0991, 0.2319),
  vec4<f32>(0.3667, 0.4995, 0.6323, 0.7651),
  vec4<f32>(0.7019, 0.8347, 0.9675, 0.1003),
  vec4<f32>(0.3347, 0.4675, 0.6003, 0.7331),
  vec4<f32>(0.6691, 0.8019, 0.9347, 0.0675),
  vec4<f32>(0.3019, 0.4347, 0.5675, 0.7003),
  vec4<f32>(0.6367, 0.7695, 0.9023, 0.0351),
  vec4<f32>(0.2695, 0.4023, 0.5351, 0.6679),
  vec4<f32>(0.6043, 0.7371, 0.8699, 0.0027),
  vec4<f32>(0.2371, 0.3700, 0.5028, 0.6356),
  vec4<f32>(0.5718, 0.7046, 0.8374, 0.9702),
  vec4<f32>(0.2046, 0.3374, 0.4702, 0.6030)
);

// 4. Calculate texture coordinates for Bayer matrix indexing
var coord = floor(texcoord * ts); // Use floor for integer coordinates

// 5. Tile the Bayer matrix based on texture coordinates (adjust factors for larger textures)
var tx = int(mod(f32(coord.x), 4.0 * f32(ts.x)));
var ty = int(mod(f32(coord.y), 4.0 * f32(ts.y)));

// 6. Get Bayer matrix index within the repeated pattern
var indexx = tx % 16; // Use 16 for the new matrix size
var indexy = ty % 16;

// 7. Look up dither value from the Bayer matrix
var dither = bayer_matrix[indexy][indexx];

// 8. Calculate luminance of the color
var luminance = dot(color.rgb, vec3(0.2126, 0.7152, 0.0722));

// 9. Define dither threshold based on luminance (adjust factor for desired effect)
var ditherThreshold = 0.5 + luminance * 0.2;

// 10. Apply dithering to red
