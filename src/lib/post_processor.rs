use std::mem;
use std::os::raw::c_void;
use std::ptr;

use gl;
use self::gl::types::*;

use crate::lib::shader::Shader;
use crate::lib::texture::Texture2D; 

pub struct PostProcessor {
    // state
    pub post_processing_shader: Shader,
    pub texture: Texture2D,
    pub width: i32,
    pub height: i32,
    // options
    pub confuse: bool,
    pub shake: bool,
    pub chaos: bool,
    // render state
    msfbo: u32, // MSFBO = Multisampled FBO
    fbo: u32, // FBO is regular, used for blitting MS color-buffer to texture
    rbo: u32, // RBO is used for multisampled color buffer
    vao: u32,
}

impl PostProcessor {
    pub const fn new_empty() -> Self {
        let post_processor = PostProcessor {
            post_processing_shader: Shader { id: 0 },
            texture: Texture2D::new_empty(),
            width: 0,
            height: 0,
            confuse: false,
            shake: false,
            chaos: false,
            msfbo: 0,
            fbo: 0,
            rbo: 0,
            vao: 0,
        };

        post_processor
    }

    pub unsafe fn new(shader: Shader, width: i32, height: i32) -> Self {
        let mut post_processor = PostProcessor {
            post_processing_shader: shader,
            texture: Texture2D::new(),
            width: width,
            height: height,
            confuse: false,
            shake: false,
            chaos: false,
            msfbo: 0,
            fbo: 0,
            rbo: 0,
            vao: 0,
        };

        // initialize renderbuffer/framebuffer object
        gl::GenFramebuffers(1, &mut post_processor.msfbo);
        gl::GenFramebuffers(1, &mut post_processor.fbo);
        gl::GenRenderbuffers(1, &mut post_processor.rbo);

        // initialize renderbuffer storage with a multisampled color buffer (don't need a depth/stencil buffer)
        gl::BindFramebuffer(gl::FRAMEBUFFER, post_processor.msfbo);
        gl::BindRenderbuffer(gl::RENDERBUFFER, post_processor.rbo);
        gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, 4, gl::RGB, width, height); // allocate storage for render buffer object
        gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::RENDERBUFFER, post_processor.rbo); // attach MS render buffer object to framebuffer
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::POSTPROCESSOR: Failed to initialize MSFBO");
        }
            
        // also initialize the FBO/texture to blit multisampled color-buffer to; used for shader operations (for postprocessing effects)
        gl::BindFramebuffer(gl::FRAMEBUFFER, post_processor.fbo);
        post_processor.texture.generate_raw(width as u32, height as u32);
        gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, post_processor.texture.id, 0); // attach texture to framebuffer as its color attachment
        if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
            println!("ERROR::POSTPROCESSOR: Failed to initialize FBO");
        }

        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        // initialize render data and uniforms
        post_processor.init_render_data();
        post_processor.post_processing_shader.use_program();
        post_processor.post_processing_shader.upload_uniform_int("scene", 0);
        let offset: f32 = 1.0 / 300.0;
        // let offsets: [[f32; 2]; 9] = [
        //     [ -offset,  offset  ],  // top-left
        //     [  0.0,     offset  ],  // top-center
        //     [  offset,  offset  ],  // top-right
        //     [ -offset,  0.0     ],  // center-left
        //     [  0.0,     0.0     ],  // center-center
        //     [  offset,  0.0     ],  // center - right
        //     [ -offset, -offset  ],  // bottom-left
        //     [  0.0,    -offset  ],  // bottom-center
        //     [  offset, -offset  ]   // bottom-right    
        // ];
        let offsets: [f32; 18] = [
             -offset,  offset  ,  // top-left
              0.0,     offset  ,  // top-center
              offset,  offset  ,  // top-right
             -offset,  0.0     ,  // center-left
              0.0,     0.0     ,  // center-center
              offset,  0.0     ,  // center - right
             -offset, -offset  ,  // bottom-left
              0.0,    -offset  ,  // bottom-center
              offset, -offset     // bottom-right    
        ];
        post_processor.post_processing_shader.upload_uniform_float_array("offsets", &offsets, 18);
        
        let edge_kernel: [i32; 9] = [
            -1, -1, -1,
            -1,  8, -1,
            -1, -1, -1
        ];
        post_processor.post_processing_shader.upload_uniform_int_array("edge_kernel", &edge_kernel, 9);
        
        let blur_kernel: [f32; 9] = [
            1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0,
            2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0,
            1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0
        ];
        post_processor.post_processing_shader.upload_uniform_float_array("blur_kernel", &blur_kernel, 9);

        post_processor
    }

    pub unsafe fn begin_render(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.msfbo);
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    pub unsafe fn end_render(&self) {
        // now resolve multisampled color-buffer into intermediate FBO to store to texture
        gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.msfbo);
        gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.fbo);
        gl::BlitFramebuffer(0, 0, self.width, self.height, 0, 0, self.width, self.height, gl::COLOR_BUFFER_BIT, gl::NEAREST);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0); // binds both READ and WRITE framebuffer to default framebuffer
    }

    pub unsafe fn render(&self, time: f32) {
        // // set uniforms/options
        self.post_processing_shader.use_program();
        self.post_processing_shader.upload_uniform_float("time", time);
        self.post_processing_shader.upload_uniform_int("confuse", self.confuse as i32);
        self.post_processing_shader.upload_uniform_int("chaos", self.chaos as i32);
        self.post_processing_shader.upload_uniform_int("shake", self.shake as i32);

        // render textured quad
        gl::ActiveTexture(gl::TEXTURE0);
        self.texture.bind();
        gl::BindVertexArray(self.vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        gl::BindVertexArray(0);
    }

    unsafe fn init_render_data(&mut self) {
        // configure VAO/VBO
        let vertices: [f32; 24] = [
            // pos        // tex
            -1.0, -1.0, 0.0, 0.0,
             1.0,  1.0, 1.0, 1.0,
            -1.0,  1.0, 0.0, 1.0,

            -1.0, -1.0, 0.0, 0.0,
             1.0, -1.0, 1.0, 0.0,
             1.0,  1.0, 1.0, 1.0
        ];

        let mut vbo = 0;

        gl::GenVertexArrays(1, &mut self.vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW);
            
        gl::BindVertexArray(self.vao);
        let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, stride, ptr::null());

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
}