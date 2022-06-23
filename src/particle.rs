use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::{ vec2, vec4, Vector2, Vector4 };
use gl;
use self::gl::types::*;
use rand::prelude::*;

use crate::game_object::GameObject;
use crate::lib::shader::Shader;
use crate::lib::texture::Texture2D;

// stores the index of the last particle used (for quick access to next dead particle)
static mut LAST_USED_PARTICLE: u32 = 0;

// Represents a single particle and its state
struct Particle {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub color: Vector4<f32>,
    pub life: f32,
}

impl Particle {
    pub fn new_empty() -> Self {
        let particle = Particle {
            position: vec2(0.0, 0.0),
            velocity: vec2(0.0, 0.0),
            color: vec4(1.0, 1.0, 1.0, 1.0),
            life: 0.0,
        };

        particle
    }
}

pub struct ParticleGenerator {
    particles: Vec<Particle>,
    amount: u32,
    shader: Shader,
    texture: Texture2D,
    vao: u32,
}

impl ParticleGenerator {
    pub const fn new_empty() -> Self {
        let particle_generator = ParticleGenerator {
            particles: Vec::new(),
            amount: 0,
            shader: Shader { id: 0 },
            texture: Texture2D::new_empty(),
            vao: 0,
        };

        particle_generator
    }

    pub fn new(shader: Shader, texture: Texture2D, amount: u32) -> Self {
        let mut particle_generator = ParticleGenerator {
            particles: Vec::new(),
            amount: amount,
            shader: shader,
            texture: texture,
            vao: 0,
        };

        particle_generator.init();

        particle_generator
    }

    // render all particles
    pub unsafe fn draw(&self) {
        // use additive blending to give it a 'glow' effect
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
        self.shader.use_program();
        for particle in &self.particles {
            if particle.life > 0.0 {
                let text = CStr::from_bytes_with_nul_unchecked(concat!("offset", "\0").as_bytes());
                self.shader.set_vector2(text, &particle.position);
                let text = CStr::from_bytes_with_nul_unchecked(concat!("color", "\0").as_bytes());
                self.shader.set_vector4(text, &particle.color);
                self.texture.bind();
                gl::BindVertexArray(self.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
                gl::BindVertexArray(0);
            }
        }
        // don't forget to reset to default blending mode
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    pub fn update(&mut self, dt: f32, object: &GameObject, new_particles: u32, offset: Vector2<f32>) {
        // add new particles 
        for _i in 0..new_particles {
            let unused_particle = unsafe { 
                self.first_unused_particle() 
            };

            self.respawn_particle(unused_particle as usize, object, offset);
        }
        // update all particles
        for i in 0..self.amount {
            let i = i as usize;
            self.particles[i].life -= dt; // reduce life
            if self.particles[i].life > 0.0 {	// particle is alive, thus update
                let new_velocity = self.particles[i].velocity * dt;
                self.particles[i].position -= new_velocity; 
                self.particles[i].color.w -= dt * 2.5;
            }
        }
    }

    // initializes buffer and vertex attributes
    fn init(&mut self) {
        // set up mesh and attribute properties
        let particle_quad: [f32; 24] = [
            0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0,

            0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 1.0,
            1.0, 0.0, 1.0, 0.0
        ];

        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(self.vao);

            // fill mesh buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                       (particle_quad.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &particle_quad[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

            let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;
            // position attribute
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);
            // texture coord attribute
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (2 * mem::size_of::<GLfloat>()) as *const c_void);
            gl::EnableVertexAttribArray(1);
        }

        // create self.amount default particle instances
        for _i in 0..self.amount {
            self.particles.push(Particle::new_empty());
        }
    }

    // returns the first Particle index that's currently unused e.g. life <= 0.0 or 0 if no particle is currently inactive
    unsafe fn first_unused_particle(&self) -> u32 {
        // first search from last used particle, this will usually return almost instantly
        for i in LAST_USED_PARTICLE..self.amount {
            if self.particles[i as usize].life <= 0.0 {
                LAST_USED_PARTICLE = i;
                return i
            }
        }

        // otherwise, do a linear search
        for i in 0..self.amount {
            if self.particles[i as usize].life <= 0.0 {
                LAST_USED_PARTICLE = i;
                return i
            }
        }

        // all particles are taken, override the first one (note that if it repeatedly hits this case, more particles should be reserved)
        LAST_USED_PARTICLE = 0;
        return 0    
    }

    fn respawn_particle(&mut self, index: usize, object: &GameObject, offset: Vector2<f32>) {
        let mut rng = rand::thread_rng();
        let random: f32 = ((rng.gen::<f32>() % 100.0) - 50.0) / 10.0;
        let rColor: f32 = 0.5 + ((rng.gen::<f32>() % 100.0) / 100.0);
        self.particles[index].position = vec2(object.position.x + random + offset.x, object.position.y + random + offset.y);
        self.particles[index].color = vec4(rColor, rColor, rColor, 1.0);
        self.particles[index].life = 1.0;
        self.particles[index].velocity = object.velocity * 0.1;
    }
}