use std::sync::Mutex;
use std::ffi::CStr;

use glfw::{ Key, Action };

use cgmath::{ vec2, Vector2, vec3, Matrix4, ortho};

use crate::game_object::GameObject;
use crate::lib::shader::Shader;
use crate::lib::sprite_renderer::SpriteRenderer;
use crate::resource_manager::ResourceManager;

#[derive(PartialEq)]
pub enum GameState {
    GameActive,
    // GameMenu,
    // GameWin
}

static mut RENDERER: SpriteRenderer = SpriteRenderer {
    shader: Shader { id: 0 },
    quad_vao: 0
};

lazy_static! {
    static ref RESOURCES: Mutex<ResourceManager<'static>> = Mutex::new(ResourceManager::new());
    // static ref PLAYER: Mutex<GameObject> = Mutex::new(GameObject::new_empty());
}

// Initial size of the player paddle
const PLAYER_SIZE: Vector2<f32> = vec2(100.0, 20.0);
// Initial velocity of the player paddle
const PLAYER_VELOCITY: f32 = 500.0;

pub struct Game {
    pub state: GameState,
    pub keys: Vec<bool>,
    pub width: u32,
    pub height: u32,
    pub player: GameObject,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let game = Game {
            state: GameState::GameActive,
            keys: Vec::new(),
            width: width,
            height: height,
            player: GameObject::new_empty(),
        };

        game
    }

    pub unsafe fn init(&mut self) {
        let shader = RESOURCES.lock().unwrap().load_shader(
            "resources/shaders/vertexShader.glsl",
            "resources/shaders/fragmentShader.glsl",
            "main"
        );

        RESOURCES.lock().unwrap().load_texture("resources/textures/background.jpg", false, "background");
        RESOURCES.lock().unwrap().load_texture("resources/textures/awesomeface.png", true, "face");
        RESOURCES.lock().unwrap().load_texture("resources/textures/block.png", false, "block");
        RESOURCES.lock().unwrap().load_texture("resources/textures/block_solid.png", false, "block_solid");
        let player_texture = RESOURCES.lock().unwrap().load_texture("resources/textures/paddle.png", true, "paddle");

        let projection: Matrix4<f32> = ortho(0.0, self.width as f32, 0.0, self.height as f32, -1.0, 1.0);

        shader.use_program();
        // let text = CStr::from_bytes_with_nul_unchecked(concat!("image", "\0").as_bytes());
        // shader.setInt(text, 0);
        let text = CStr::from_bytes_with_nul_unchecked(concat!("projection", "\0").as_bytes());
        shader.set_mat4(text, &projection);

        RENDERER = SpriteRenderer::new(shader);

        let player_pos = vec2(
            self.width as f32 / 2.0 - PLAYER_SIZE.x / 2.0,
            0.0
        );

        self.player = GameObject::new(player_pos, PLAYER_SIZE, vec2(0.0, 0.0), vec3(1.0, 1.0 ,1.0), 0.0, player_texture);
    }

    // pub fn update(dt: f32) {

    // }

    pub unsafe fn render(&self) {
        if self.state == GameState::GameActive {
            let background_tex = RESOURCES.lock().unwrap().get_texture("background");
            RENDERER.draw_sprite(&background_tex, vec2(0.0, 0.0), vec2(self.width as f32, self.height as f32), 0.0, vec3(1.0, 1.0, 1.0));
        }
        self.player.draw(&RENDERER);
    }

    pub fn process_input(&mut self, window: &glfw::Window, dt: f32) {
        if self.state == GameState::GameActive {
            let velocity = PLAYER_VELOCITY * dt;
            // move paddle
            if window.get_key(Key::A) == Action::Press {
                if self.player.position.x >= 0.0 {
                    self.player.position.x -= velocity;
                }
            }
            if window.get_key(Key::D) == Action::Press {
                if self.player.position.x <= self.width as f32 - self.player.size.x {
                    self.player.position.x += velocity;
                }
            }
        }
    }
}