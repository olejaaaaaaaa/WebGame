




pub const VERTEX_SHADER_1: &str = r#"
    attribute vec3 pos;

    void main() {
        gl_Position = vec4(pos, 1.0);
    }

"#;

pub const FRAGMENT_SHADER_1: &str = r#"
    void main() {
        gl_FragColor = vec4(0.0, 0.0, 1.0, 1.0);
    }
"#;