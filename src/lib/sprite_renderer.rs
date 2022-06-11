use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use gl;
use self::gl::types::*;
use cgmath::{ vec3, Vector2, Vector3, Matrix4, Rad };
use cgmath::prelude::*;

use crate::lib::shader::Shader;
use crate::lib::texture::Texture2D; 

pub struct SpriteRenderer {
    pub shader: Shader,
    pub quad_vao: u32,
}

impl SpriteRenderer {
    pub fn new(shader: Shader) -> Self {
        let mut sprite_renderer = SpriteRenderer {
            shader: shader,
            quad_vao: 0,
        };

        sprite_renderer.init_render_data();

        sprite_renderer
    }

    pub fn draw_sprite(&self, texture: &Texture2D, position: Vector2<f32>, size: Vector2<f32>, rotate: f32, color: Vector3<f32>) {
        unsafe {
            self.shader.useProgram();

            let mut model: Matrix4<f32> = Matrix4::identity();
            model = model * Matrix4::<f32>::from_translation(vec3(position.x, position.y, 0.0));
            model = model * Matrix4::<f32>::from_angle_z(Rad(rotate));
            model = model * Matrix4::<f32>::from_nonuniform_scale(size.x, size.y, 1.0);

            let text = CStr::from_bytes_with_nul_unchecked(concat!("model", "\0").as_bytes());
            self.shader.setMat4(text, &model);
            let text = CStr::from_bytes_with_nul_unchecked(concat!("spriteColor", "\0").as_bytes());
            self.shader.setVector3(text, &color);
        
            gl::ActiveTexture(gl::TEXTURE0);
            texture.bind();

            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

    fn init_render_data(&mut self) {
        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        let vertices: [f32; 24] = [
            // pos    // tex
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 
        
            0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 1.0, 0.0
        ];

        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut self.quad_vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(self.quad_vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

            let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;
            // position attribute
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);
            // color attribute
            // gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            // gl::EnableVertexAttribArray(1);
            // texture coord attribute
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (2 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);
        }
    }
}