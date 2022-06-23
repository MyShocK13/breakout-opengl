use std::cmp;
use std::sync::Mutex;
use std::ffi::CStr;

use glfw::{Key, Action};

use cgmath::{vec2, Vector2, vec3, Matrix4, ortho, dot};
use cgmath::prelude::*;

use crate::ball::Ball;
use crate::game_level::GameLevel;
use crate::game_object::GameObject;
use crate::lib::shader::Shader;
use crate::lib::sprite_renderer::SpriteRenderer;
use crate::particle::ParticleGenerator;
use crate::resource_manager::ResourceManager;

// Represents the current state of the game
#[derive(PartialEq)]
pub enum GameState {
    GameActive,
    // GameMenu,
    // GameWin
}

// Represents the four possible (collision) directions
#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    None
}

impl Direction {
    fn from_i8(value: i8) -> Direction {
        match value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => Direction::None
        }
    }
}

type Collision = (bool, Direction, Vector2<f32>);

static mut RENDERER: SpriteRenderer = SpriteRenderer {
    shader: Shader { id: 0 },
    quad_vao: 0
};

static mut PARTICLE_GENERATOR: ParticleGenerator = ParticleGenerator::new_empty();

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
    // particle_generator: ParticleGenerator,
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
        let sprite_shader = RESOURCES.lock().unwrap().load_shader(
            "resources/shaders/sprite_vs.glsl",
            "resources/shaders/sprite_fs.glsl",
            "sprite"
        );
        let particle_shader = RESOURCES.lock().unwrap().load_shader(
            "resources/shaders/particle_vs.glsl",
            "resources/shaders/particle_fs.glsl",
            "particle"
        );

        // configure shaders
        let projection: Matrix4<f32> = ortho(0.0, self.width as f32, self.height as f32, 0.0, -1.0, 1.0);
        sprite_shader.use_program();
        // let text = CStr::from_bytes_with_nul_unchecked(concat!("image", "\0").as_bytes());
        // sprite_shader.setInt(text, 0);
        let text = CStr::from_bytes_with_nul_unchecked(concat!("projection", "\0").as_bytes());
        sprite_shader.set_mat4(text, &projection);
        particle_shader.use_program();
        particle_shader.set_mat4(text, &projection);

        // load textures
        RESOURCES.lock().unwrap().load_texture("resources/textures/background.jpg", false, "background");
        let ball_texture = RESOURCES.lock().unwrap().load_texture("resources/textures/awesomeface.png", true, "face");
        RESOURCES.lock().unwrap().load_texture("resources/textures/block.png", false, "block");
        RESOURCES.lock().unwrap().load_texture("resources/textures/block_solid.png", false, "block_solid");
        let player_texture = RESOURCES.lock().unwrap().load_texture("resources/textures/paddle.png", true, "paddle");
        let particle_texture = RESOURCES.lock().unwrap().load_texture("resources/textures/particle.png", true, "particle");

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

        // set render-specific controls
        RENDERER = SpriteRenderer::new(sprite_shader);
        PARTICLE_GENERATOR = ParticleGenerator::new(particle_shader, particle_texture, 500);

        // Player initialization
        let player_pos = vec2(
            self.width as f32 / 2.0 - PLAYER_SIZE.x / 2.0,
            self.height as f32 - PLAYER_SIZE.y
        );
        self.player = GameObject::new(player_pos, PLAYER_SIZE, vec2(0.0, 0.0), vec3(1.0, 1.0 ,1.0), player_texture);

        // Ball initialization
        let ball_pos = player_pos + vec2(
            PLAYER_SIZE.x / 2.0 - BALL_RADIUS,
            -BALL_RADIUS * 2.0
        );
        self.ball = Ball::new(ball_pos, BALL_RADIUS, INITIAL_BALL_VELOCITY, ball_texture);
    }

    pub fn update(&mut self, dt: f32) {
        // update objects
        self.ball.move_ball(dt, self.width);
        // check for collisions
        self.do_collisions();
        // update particles
        unsafe {
            PARTICLE_GENERATOR.update(
                dt,
                &self.ball.game_object, 
                2,
                vec2(self.ball.radius / 2.0, self.ball.radius / 2.0)
            );
        }
        // check loss condition
        if self.ball.game_object.position.y >= self.height as f32 {
            self.reset_level();
            self.reset_player();
        }
    }

    pub unsafe fn render(&self) {
        if self.state == GameState::GameActive {
            // Draw background
            let background_tex = RESOURCES.lock().unwrap().get_texture("background");
            RENDERER.draw_sprite(&background_tex, vec2(0.0, 0.0), vec2(self.width as f32, self.height as f32), 0.0, vec3(1.0, 1.0, 1.0));
            // Draw level
            self.levels[self.actual_level].draw(&RENDERER);
            // draw player
            self.player.draw(&RENDERER);
            // draw particles	
            PARTICLE_GENERATOR.draw();
            // draw ball
            self.ball.draw(&RENDERER);
        }
    }

    pub fn process_input(&mut self, window: &glfw::Window, dt: f32) {
        if self.state == GameState::GameActive {
            let velocity = PLAYER_VELOCITY * dt;
            // move paddle
            if window.get_key(Key::A) == Action::Press {
                if self.player.position.x >= 0.0 {
                    self.player.position.x -= velocity;
                    if self.ball.stuck {
                        self.ball.game_object.position.x -= velocity;
                    }
                }
            }
            if window.get_key(Key::D) == Action::Press {
                if self.player.position.x <= self.width as f32 - self.player.size.x {
                    self.player.position.x += velocity;
                    if self.ball.stuck {
                        self.ball.game_object.position.x += velocity;
                    }
                }
            }
            if window.get_key(Key::Space) == Action::Press {
                self.ball.stuck = false;
            }
        }
    }

    pub fn reset_level(&mut self) {
        match self.actual_level {
            0 => {
                let mut one = GameLevel::new();
                one.load(RESOURCES.lock().unwrap(), "resources/levels/one.lvl", self.width, self.height / 2 );
                self.levels[0] = one;
            },
            1 => {
                let mut two = GameLevel::new();
                two.load(RESOURCES.lock().unwrap(), "resources/levels/two.lvl", self.width, self.height / 2 );
                self.levels[1] = two;
            },
            2 => {
                let mut three = GameLevel::new();
                three.load(RESOURCES.lock().unwrap(), "resources/levels/three.lvl", self.width, self.height / 2 );
                self.levels[2] = three;
            },
            3 => {
                let mut four = GameLevel::new();
                four.load(RESOURCES.lock().unwrap(), "resources/levels/four.lvl", self.width, self.height / 2 );
                self.levels[3] = four;
            },
            _ => ()
        }
    }

    pub fn reset_player(&mut self) {
        // reset player/ball stats
        let player_pos = vec2(
            self.width as f32 / 2.0 - PLAYER_SIZE.x / 2.0,
            self.height as f32 - PLAYER_SIZE.y
        );
        let ball_pos = player_pos + vec2(
            PLAYER_SIZE.x / 2.0 - BALL_RADIUS,
            -BALL_RADIUS * 2.0
        );
        
        self.player.size = PLAYER_SIZE;
        self.player.position = player_pos;

        self.ball.reset(ball_pos, INITIAL_BALL_VELOCITY);
    }

    pub fn do_collisions(&mut self) {
        for brick in &mut self.levels[self.actual_level].bricks {
            if !brick.destroyed {
                let collision: Collision = check_circle_collision(&self.ball, &brick);

                if collision.0 { // if collision is true
                    // destroy block if not solid
                    if !brick.is_solid {
                        brick.destroyed = true;
                    }
                    // collision resolution
                    let dir = collision.1;
                    let diff_vector = collision.2;
                    let mut penetration = 0.0;

                    match dir {
                        Direction::Left | Direction::Right => {
                            self.ball.game_object.velocity.x = -self.ball.game_object.velocity.x; // reverse horizontal velocity
                            // relocate
                            penetration = self.ball.radius - diff_vector.x.abs();
                        },
                        Direction::Up | Direction::Down => {
                            self.ball.game_object.velocity.y = -self.ball.game_object.velocity.y; // reverse vertical velocity
                            // relocate
                            penetration = self.ball.radius - diff_vector.y.abs();
                        },
                        _ => ()
                    }

                    match dir {
                        Direction::Left => self.ball.game_object.position.x += penetration, // move ball to right
                        Direction::Right => self.ball.game_object.position.x -= penetration, // move ball to left
                        Direction::Up => self.ball.game_object.position.y += penetration, // move ball back up
                        Direction::Down => self.ball.game_object.position.y -= penetration, // move ball back down
                        _ => ()
                    }
                }
            }
        }

        // check collisions for player pad (unless stuck)
        let pad_collision: Collision = check_circle_collision(&self.ball, &self.player);
        if !self.ball.stuck && pad_collision.0 {
            // check where it hit the board, and change velocity based on where it hit the board
            let center_board: f32 = self.player.position.x + self.player.size.x / 2.0;
            let distance: f32 = (self.ball.game_object.position.x + self.ball.radius) - center_board;
            let percentage: f32 = distance / (self.player.size.x / 2.0);
            // then move accordingly
            let strength = 2.0;
            let old_velocity = self.ball.game_object.velocity;
            self.ball.game_object.velocity.x = INITIAL_BALL_VELOCITY.x * percentage * strength; 
            //self.ball.game_object.velocity.y = -self.ball.game_object.velocity.y;
            self.ball.game_object.velocity = self.ball.game_object.velocity.normalize() * length(old_velocity); // keep speed consistent over both axes (multiply by length of old velocity, so total strength is not changed)
            // fix sticky paddle
            self.ball.game_object.velocity.y = -1.0 * self.ball.game_object.velocity.y.abs();
        }
    }
}

// AABB - AABB collision
fn _check_square_collision(one: &GameObject, two: &GameObject) -> bool {
    // collision x-axis?
    let collision_x = one.position.x + one.size.x >= two.position.x && two.position.x + two.size.x >= one.position.x;
    // collision y-axis?
    let collision_y = one.position.y + one.size.y >= two.position.y && two.position.y + two.size.y >= one.position.y;
    // collision only if on both axes
    collision_x && collision_y
}

// AABB - circle collision
fn check_circle_collision(one: &Ball, two: &GameObject) -> Collision {
    // get center point circle first 
    let center = vec2(one.game_object.position.x + one.radius, one.game_object.position.y + one.radius);
    // calculate AABB info (center, half-extents)
    let aabb_half_extents = vec2(two.size.x / 2.0, two.size.y / 2.0);
    let aabb_center = vec2(
        two.position.x + aabb_half_extents.x, 
        two.position.y + aabb_half_extents.y
    );
    // get difference vector between both centers
    let difference = center - aabb_center;
    let clamped = vec2(
        clamp(difference.x as i32, -aabb_half_extents.x as i32, aabb_half_extents.x as i32),
        clamp(difference.y as i32, -aabb_half_extents.y as i32, aabb_half_extents.y as i32),    
    );
    // add clamped value to AABB_center and we get the value of box closest to circle
    let closest = aabb_center + clamped;
    // retrieve vector between center circle and closest point AABB and check if length <= radius
    let difference = closest - center;

    if length(difference) <= one.radius {
        (true, vector_direction(difference), difference)
    } else {
        (false, Direction::Up, vec2(0.0, 0.0))
    }
}

fn clamp(value: i32, min: i32, max: i32) -> f32 {
    cmp::max(min, cmp::min(max, value)) as f32
}

fn length(vector: Vector2<f32>) -> f32 {
    (vector.x.powi(2) + vector.y.powi(2)).sqrt()
}

fn vector_direction(target: Vector2<f32>) -> Direction {
    let compass = vec![
        vec2(0.0, 1.0), // up
        vec2(1.0, 0.0), // right
        vec2(0.0, -1.0), // down
        vec2(-1.0, 0.0), // left
    ];

    let mut max = 0.0;
    let mut best_match: i8 = -1;
    
    for (i, _dir) in compass.iter().enumerate() {
        let dot_product = dot(target.normalize(), compass[i]);
        if dot_product > max {
            max = dot_product;
            best_match = i as i8;
        }
    };

    Direction::from_i8(best_match)
}