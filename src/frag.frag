#version 330 core

uniform float width;
uniform float height;

out vec4 color;

#define MAX_ITER 100

// void main() {
//     // Calculate pixel pos in range [-1.0, 1.0]
//     vec2 pixel_pos = gl_FragCoord.xy / vec2(width, height) * 2.0 - 1.0;

//     // Calculate the complex number c based on pixel position
//     vec2 c = pixel_pos;

//     // Mandelbrot iteration
//     vec2 z = vec2(0.0, 0.0);
//     int iteration = 0;

//     while(length(z) < 2.0 && iteration < MAX_ITER) {
//         vec2 z_squared = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
//         z = z_squared + c;
//         iteration++;
//     }

//     // set color based on n iterations
//     if (iteration == MAX_ITER) {
//         color = vec4(0.0, 0.0, 0.0, 1.0); // black
//     } else {
//         float normalized_iteration = float(iteration) / float(MAX_ITER);
//         color = vec4(normalized_iteration, normalized_iteration, normalized_iteration, 1.0);
//     }

// }

void main() {
    vec2 pixel_pos = gl_FragCoord.xy / vec2(width, height) * 2.0 - 1.0;
    vec2 c = pixel_pos;

    vec2 z = vec2(0.0, 0.0);
    int iteration = 0;

    while (length(z) < 2.0 && iteration < MAX_ITER) {
        vec2 z_squared = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
        z = z_squared + c;
        iteration++;
    }

    if (iteration == MAX_ITER) {
        color = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        float normalized_iteration = float(iteration) / float(MAX_ITER);

        float r = sin(normalized_iteration * 3.14159);
        float g = sin(normalized_iteration * 3.14159 + 2.0944);
        float b = sin(normalized_iteration * 3.14159 + 4.18879);

        color = vec4(r, g, b, 1.0);
    }
}
