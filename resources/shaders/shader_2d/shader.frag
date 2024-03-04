#version 330 core

in vec4 gl_FragCoord; 
out vec4 Color;

uniform sampler2D tex1;

in VS_OUTPUT {
    vec4 Color;
    vec2 TexCoord;
} IN;



void main()
{
    Color = texture(tex1, IN.TexCoord);// * IN.Color;
    Color =  IN.Color;

}

