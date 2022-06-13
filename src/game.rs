use std::sync::Mutex;
use std::ffi::CStr;

use glfw::{ Key, Action };

use cgmath::{ vec2, Vector2, vec3, Matrix4, ortho};

use crate::ball::Ball;
use crate::game_level::GameLevel;
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
// Initial velocity of the Ball
const INITIAL_BALL_VELOCITY: Vector2<f32> = vec2(100.0, -350.0);
// Radius of the ball object
const BALL_RADIUS: f32 = 12.5;

pub struct Game {
    pub state: GameState,
    pub width: u32,
    pub height: u32,
    pub player: GameObject,
    pub ball: Ball,
    pub levels: Vec<GameLevel>,
    pub actual_level: usize,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let game = Game {
            state: GameState::GameActive,
            width: width,
            height: height,
            player: GameObject::new_empty(),
            ball: Ball::new_empty(),
            levels: Vec::new(),
            actual_level: 0
        };

        game
    }

    pub unsafe fn init(&mut self) {
        // load shaders
        let shader = RESOURCES.lock().unwrap().load_shader(
            "resources/shaders/vertexShader.glsl",
            "resources/shaders/fragmentShader.glsl",
            "main"
        );

        // load textures
        RESOURCES.lock().unwrap().load_texture("resources/textures/background.jpg", false, "background");
        let ball_texture = RESOURCES.lock().unwrap().load_texture("resources/textures/awesomeface.png", true, "face");
        RESOURCES.lock().unwrap().load_texture("resources/textures/block.png", false, "block");
        RESOURCES.lock().unwrap().load_texture("resources/textures/block_solid.png", false, "block_solid");
        let player_texture = RESOURCES.lock().unwrap().load_texture("resources/textures/paddle.png", true, "paddle");

        // load levels
        let mut one = GameLevel::new();
        let mut two = GameLevel::new();
        let mut three = GameLevel::new();
        let mut four = GameLevel::new();
        one.load(RESOURCES.lock().unwrap(), "resources/levels/one.lvl", self.width, self.height / 2 );
        two.load(RESOURCES.lock().unwrap(), "resources/levels/two.lvl", self.width, self.height / 2 );
        three.load(RESOURCES.lock().unwrap(), "resources/levels/three.lvl", self.width, self.height / 2 );
        four.load(RESOURCES.lock().unwrap(), "resources/levels/four.lvl", self.width, self.height / 2 );
        self.levels.push(one);
        self.levels.push(two);
        self.levels.push(three);
        self.levels.push(four);

        let projection: Matrix4<f32> = ortho(0.0, self.width as f32, self.height as f32, 0.0, -1.0, 1.0);

        shader.use_program();
        // let text = CStr::from_bytes_with_nul_unchecked(concat!("image", "\0").as_bytes());
        // shader.setInt(text, 0);
        let text = CStr::from_bytes_with_nul_unchecked(concat!("projection", "\0").as_bytes());
        shader.set_mat4(text, &projection);

        RENDERER = SpriteRenderer::new(shader);

        // Player initialization
        //----------------------
        let player_pos = vec2(
            self.width as f32 / 2.0 - PLAYER_SIZE.x / 2.0,
            self.height as f32 - PLAYER_SIZE.y
        );

        self.player = GameObject::new(player_pos, PLAYER_SIZE, vec3(1.0, 1.0 ,1.0), player_texture);

        // Ball init
        //----------
        let ball_pos = player_pos + vec2(
            PLAYER_SIZE.x / 2.0 - BALL_RADIUS,
            -BALL_RADIUS * 2.0
        );
        self.ball = Ball::new(ball_pos, BALL_RADIUS, INITIAL_BALL_VELOCITY, ball_texture);
    }

    // pub fn update(dt: f32) {

    // }

    pub unsafe fn render(&self) {
        if self.state == GameState::GameActive {
            // Draw background
            let background_tex = RESOURCES.lock().unwrap().get_texture("background");
            RENDERER.draw_sprite(&background_tex, vec2(0.0, 0.0), vec2(self.width as f32, self.height as f32), 0.0, vec3(1.0, 1.0, 1.0));
            // Draw level
            self.levels[self.actual_level].draw(&RENDERER);
        }
        self.player.draw(&RENDERER);
        self.ball.draw(&RENDERER);
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