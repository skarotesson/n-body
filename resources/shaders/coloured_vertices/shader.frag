#version 330 core

out vec4 Color;

in VS_OUTPUT {
    vec3 Color;
} IN;



void main()
{
    Color = vec4(IN.Color, 1.0f);
}
