use std::collections::HashMap;

use crate::lib::shader::Shader;
use crate::lib::texture::Texture2D;

pub struct ResourceManager<'a> {
    pub shaders: HashMap<&'a str, Shader>,
    pub textures: HashMap<&'a str, &'a Texture2D>,
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

    pub fn get_shader(&self, name: &str) -> Shader {
        let shader = self.shaders.get(name).unwrap();

        *shader
    }
}