#version 300 es
layout(location = 0) in vec2 a_position;
uniform float u_aspect_ratio;
uniform float u_zoom;
uniform vec2 u_offset;

void main() {
    // Apply zoom to the base position
    vec2 zoomedPosition = a_position * u_zoom;
    
    // Apply panning offset
    vec2 offsetPosition = zoomedPosition + u_offset;
    
    // Create infinite grid effect
    offsetPosition = fract(offsetPosition + 0.5) - 0.5;
    
    // Apply aspect ratio correction while maintaining grid spacing
    vec2 adjusted = offsetPosition * vec2(
        min(1.0, 1.0 / u_aspect_ratio),
        min(1.0, u_aspect_ratio)
    );
    
    gl_Position = vec4(adjusted, 0.0, 1.0);
    gl_PointSize = 2.0;
}
