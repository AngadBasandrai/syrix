use std::{ffi::CString, fs};

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let vertex_source = fs::read_to_string(vertex_path)?;
        let fragment_source = fs::read_to_string(fragment_path)?;
        let vertex_source = CString::new(vertex_source)?;
        let fragment_source = CString::new(fragment_source)?;
        let vertex_shader = unsafe {
            let shader = gl::CreateShader(gl::VERTEX_SHADER);

            gl::ShaderSource(shader, 1, &vertex_source.as_ptr(), std::ptr::null());

            gl::CompileShader(shader);

            shader
        };
        let fragment_shader = unsafe {
            let shader = gl::CreateShader(gl::FRAGMENT_SHADER);

            gl::ShaderSource(shader, 1, &fragment_source.as_ptr(), std::ptr::null());

            gl::CompileShader(shader);

            shader
        };
        let program = unsafe {
            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::LinkProgram(program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            program
        };
        Ok(Self { id: program })
    }
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}
