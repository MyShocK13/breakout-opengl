#version 330 core

in vec2 TexCoord;
in vec4 ParticleColor;
out vec4 FragColor;

uniform sampler2D sprite;

void main()
{
    FragColor = (texture(sprite, TexCoord) * ParticleColor);
}  