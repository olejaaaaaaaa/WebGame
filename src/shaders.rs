




pub const VERTEX_SHADER_1: &str = r#"
    #version 140

    in vec3 pos;
    in float Time;

    void main() {
        gl_Position = vec4(pos, 1.0);
    }

"#;


pub const VERTEX_SHADER_2: &str = r#"
    #version 140

    in vec3 pos;
    uniform mat4 matrix;
    uniform mat4 view;
    uniform mat4 camera;
    out float Time_x;
    out float Time_y;
    out float Time_z;

    void main() {
        gl_Position = camera  * matrix * vec4(pos, 1.0);
        Time_x = pos[0];
        Time_y = pos[1];
        Time_z = pos[2];
    }

"#;

pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}





pub const FRAGMENT_SHADER_1: &str = r#"
    #version 140

    out vec4 color;
    in float Time_x;
    in float Time_y;
    in float Time_z;

    void main() {
        float z = 1/(1.0 - Time_z);
        color = vec4( cos(Time_x) * z, cos(Time_y) * z, Time_z * z, 1.0);
    }
"#;