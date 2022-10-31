#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;
out vec4 ParticleColor;

uniform mat4 projection;
uniform vec2 offset;
uniform vec4 color;

void main()
{
    float scale = 10.0f;
    gl_Position = projection * vec4((aPos * scale) + offset, 0.0, 1.0);
    TexCoord = vec2(aTexCoord.x, aTexCoord.y);
    ParticleColor = color;
}