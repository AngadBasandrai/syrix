#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texCoord;

uniform vec2 uPosition;
uniform vec2 uSize;

out vec2 TexCoord;

void main()
{
    gl_Position = vec4(position.x * uSize.x + uPosition.x, position.y * uSize.y + uPosition.y, position.z, 1.0);
    TexCoord = texCoord;
}