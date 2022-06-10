use std::ffi::CStr;
use std::path::Path;

use cgmath::{ vec2, vec3, Matrix4, Deg, perspective};

use crate::lib::shader::Shader;
use crate::lib::sprite_renderer::SpriteRenderer;
use crate::lib::texture::Texture2D;

pub enum GameState {
    GameActive,
    GameMenu,
    GameWin
}

pub struct Game {
    pub state: GameState,
    pub keys: Vec<bool>,
    pub width: u32,
    pub height: u32,
}

static mut RENDERER: SpriteRenderer = SpriteRenderer {
    shader: Shader { ID: 0 },
    quad_vao: 0
};

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let game = Game {
            state: GameState::GameActive,
            keys: Vec::new(),
            width: width,
            height: height
        };

        game
    }

    pub unsafe fn init(&self) {
        let shader = Shader::new(
            "resources/shaders/sprite_vs.glsl",
            "resources/shaders/sprite_fs.glsl");

        let projection: Matrix4<f32> = perspective(Deg(45.0), self.width as f32 / self.height as f32 , 0.1, 100.0);

        shader.useProgram();
        let text = CStr::from_bytes_with_nul_unchecked(concat!("image", "\0").as_bytes());
        shader.setInt(text, 0);
        let text = CStr::from_bytes_with_nul_unchecked(concat!("projection", "\0").as_bytes());
        shader.setMat4(text, &projection);

        RENDERER = SpriteRenderer::new(shader);
    }

    // pub fn update(dt: f32) {

    // }

    pub unsafe fn render(&self) {
        let img = image::open(&Path::new("resources/textures/awesomeface.png")).expect("Failed to load texture");
        let data = img.clone().into_bytes();
        
        let mut texture = Texture2D::default();
        texture.generate(img.width(), img.height(), data);

        RENDERER.draw_sprite(&texture, vec2(0.0, 0.0), vec2(300.0, 400.0), 45.0, vec3(0.0, 1.0, 0.0))
    }
}