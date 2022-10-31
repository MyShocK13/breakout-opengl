use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use gl;
use self::gl::types::*;
use cgmath::{Matrix, Matrix4, Vector2, Vector3, Vector4};
use cgmath::prelude::*;

#[derive(Copy, Clone)]
pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Shader {
        let mut shader = Shader { id: 0 };

        // 1. retrieve the vertex/fragment source code from filesystem
        let mut v_shader_file = File::open(vertex_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));
        let mut f_shader_file = File::open(fragment_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));
        let mut vertex_code = String::new();
        let mut fragment_code = String::new();
        v_shader_file
            .read_to_string(&mut vertex_code)
            .expect("Failed to read vertex shader");
        f_shader_file
            .read_to_string(&mut fragment_code)
            .expect("Failed to read fragment shader");
            
        let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
        let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.check_compile_errors(vertex, "VERTEX");
            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.check_compile_errors(fragment, "FRAGMENT");
            // shader Program
            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);
            shader.check_compile_errors(id, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.id = id;
        }

        shader
    }

    /// activate the shader
    /// ------------------------------------------------------------------------
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id)
    }

    /// utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn upload_uniform_int(&self, name: &str, value: i32) {
        let name_format = format!("{}{}", name, "\0");
        let text = CStr::from_bytes_with_nul_unchecked(name_format.as_bytes());
        gl::Uniform1i(gl::GetUniformLocation(self.id, text.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn upload_uniform_float(&self, name: &str, value: f32) {
        let name_format = format!("{}{}", name, "\0");
        let location = gl::GetUniformLocation(self.id, CStr::from_bytes_with_nul_unchecked(name_format.as_bytes()).as_ptr());
        gl::Uniform1f(location, value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn upload_uniform_int_array(&self, name: &str, values: &[i32], count: i32) {
        let name_format = format!("{}{}", name, "\0");
        let location = gl::GetUniformLocation(self.id, CStr::from_bytes_with_nul_unchecked(name_format.as_bytes()).as_ptr());
        gl::Uniform1iv(location, count, values.as_ptr());
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn upload_uniform_float_array(&self, name: &str, values: &[f32], count: i32) {
        let name_format = format!("{}{}", name, "\0");
        let location = gl::GetUniformLocation(self.id, CStr::from_bytes_with_nul_unchecked(name_format.as_bytes()).as_ptr());
        gl::Uniform1fv(location, count, values.as_ptr());
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn upload_uniform_float_vec2(&self, name: &str, value: &Vector2<f32>, count: i32) {
        let name_format = format!("{}{}", name, "\0");
        let text = CStr::from_bytes_with_nul_unchecked(name_format.as_bytes());
        gl::Uniform2fv(gl::GetUniformLocation(self.id, text.as_ptr()), count, value.as_ptr());
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vector3(&self, name: &CStr, value: &Vector3<f32>) {
        gl::Uniform3fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_ptr());
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_vector4(&self, name: &CStr, value: &Vector4<f32>) {
        gl::Uniform4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, value.as_ptr());
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_mat4(&self, name: &CStr, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.id, name.as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }

    /// utility function for checking shader compilation/linking errors.
    /// ------------------------------------------------------------------------
    unsafe fn check_compile_errors(&self, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(1024);
        info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&info_log).unwrap());
            }

        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(shader, 1024, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&info_log).unwrap());
            }
        }
    }
}