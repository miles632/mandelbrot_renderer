#version 330 core

in vec4 gl_FragCord; // Fragments pixel position
out vec4 frag_color; // Output color of the fragment

#define MAX_ITER 150

int get_iterations() {
	// normalizes down to 1 and then scales by 4 to a final range of -2.0 to 2.0
	float real = (gl_FragCord.x / 1080.0 - 0.5) * 4.0;
	float imag = (gl_FragCord.y / 1080.0 - 0.7) * 4.0;

	int iterations = 0;
	float const_real = real;
	float const_imag = imag;

	// escape sequence
	while(iterations < MAX_ITER) {
		float tmp_real = real;
		real = (real * real - imag * imag) + const_real;
		imag = (2.0 * tmp_real * imag) + const_imag;
			
		float dist = real * real + imag * imag;
			
		if (dist > 4.0)
		break;

		++iterations;
	}
	return iterations;
}

vec4 return_color() {
	int iter = get_iterations();
	if (iter == MAX_ITER) {
		gl_FragDepth = 0.0f;
		return vec4(0.0, iterations, 0.0f, 1.0f);
	}
	float iterations = float(iter) / MAX_ITER;
	return vec4(0.0f, iterations, 0.0f, 1.0f);
}

void main() {
	frag_color = return_color();
}


