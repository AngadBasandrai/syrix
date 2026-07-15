#version 330 core

in vec2 TexCoord;

uniform sampler2D uTexture;
uniform vec3 uColor;

out vec4 FragColor;

void main()
{
    float alpha = texture(uTexture, TexCoord).r;

    FragColor = vec4(uColor, alpha);
}