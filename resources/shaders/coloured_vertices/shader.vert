#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

out VS_OUTPUT {
    out vec3 Color;
} OUT;

void main()
{
    gl_Position = vec4(Position.x,Position.y,Position.z, 1.0);
    OUT.Color = Color;
}
