#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

out VS_OUTPUT {
    out vec4 Color;
    out vec2 TexCoord;
} OUT;

void main()
{
    gl_Position = proj * view * model * vec4(Position.xyz, 1.0);
    OUT.Color = vec4(1.0);
    OUT.TexCoord = TexCoord;

}
