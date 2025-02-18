// It takes a 2D position (`a_position`) as input and passes it 
// through directly to the pipeline, with the z-coordinate set to 0 and w-coordinate set to 1.
pub const VS_SOURCE: &str = r#"
    attribute vec2 a_position;
    attribute vec4 a_color;

    varying vec4 v_color;

    void main() {
        gl_Position = vec4(a_position, 0.0, 1.0);
        v_color = a_color;
    }
"#;

pub const FS_SOURCE: &str = r#"
   precision mediump float;

    varying vec4 v_color;

    void main() {
        gl_FragColor = v_color;
    }
"#;