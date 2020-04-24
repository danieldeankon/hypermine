#version 450

#include "common.h"

layout(location = 0) in vec2 texcoords;

layout(location = 0) out vec4 fog;

layout(input_attachment_index=0, set=0, binding=1) uniform subpassInput depth;

void main() {
    vec4 clip_pos = vec4(texcoords * 2.0 - 1.0, subpassLoad(depth).x, 1.0);
    vec4 scaled_view_pos = inverse_projection * clip_pos;
    // Cancel out perspective, obtaining klein ball position
    vec3 view_pos = scaled_view_pos.xyz / scaled_view_pos.w;
    float view_length = length(view_pos);
    // Convert to true hyperbolic distance, taking care to respect atanh's domain
    float dist = view_length >= 1.0 ? INFINITY : atanh(view_length);
    // piecewise constant/exponential dropoff fog
    float terrain_proportion;
    float threshold = 1.0;
    if (dist <= threshold)
    {
        terrain_proportion = 1.0;
    } 
    else 
    {
        terrain_proportion = exp(-(dist - threshold) * fog_density); 
    }
    // 
    fog = vec4(0.15, 0.25, 0.95, terrain_proportion);
}
