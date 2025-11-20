pub const VERTEX_SHADER_SOURCE: &str = r#"#version 330 core
layout (location = 0) in vec3 aPos;
void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"#version 330 core
out vec4 FragColor;
void main()
{
    FragColor = vec4(0.8f, 0.3f, 0.02f, 1.0f);
}"#;
