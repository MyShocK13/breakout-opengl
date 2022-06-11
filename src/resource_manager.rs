use std::collections::HashMap;
use std::path::Path;

use crate::lib::shader::Shader;
use crate::lib::texture::Texture2D;

pub struct ResourceManager<'a> {
    pub shaders: HashMap<&'a str, Shader>,
    pub textures: HashMap<&'a str, Texture2D>,
}

impl<'a> ResourceManager<'a> {
    pub fn new() -> ResourceManager<'a> {
        let resource_manager = ResourceManager {
            shaders: HashMap::new(),
            textures: HashMap::new(),
        };

        resource_manager
    }

    pub fn load_shader(&mut self, vertex_path: &str, fragment_path: &str, name: &'a str) -> Shader {
        // build and compile our shader program
        // ------------------------------------
        let shader = Shader::new(vertex_path, fragment_path);
        self.shaders.insert(name, shader);

        shader
    }

    // pub fn get_shader(&self, name: &str) -> Shader {
    //     let shader = self.shaders.get(name).unwrap();

    //     *shader
    // }

    pub fn load_texture(&mut self, path: &str, alpha: bool, name: &'a str) -> Texture2D {
        let img = image::open(&Path::new(path)).expect("Failed to load texture");
        let data = img.clone().into_bytes();
        
        let mut texture = Texture2D::default();

        if alpha {
            texture.internal_format = gl::RGBA;
            texture.image_format = gl::RGBA;
        }

        unsafe {
            texture.generate(img.width(), img.height(), data);
        }

        self.textures.insert(name, texture);

        texture
    }

    pub fn get_texture(&self, name: &str) -> Texture2D {
        let texture = self.textures.get(name).unwrap();

        *texture
    }
}