use std::cmp;
use std::sync::Mutex;
use std::ffi::CStr;

use glfw::{Key, Action};

use cgmath::{vec2, Vector2, vec3, Matrix4, ortho, dot};
use cgmath::prelude::*;
use rand::prelude::*;
use cpal::{Data, Sample, SampleFormat};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::ball::Ball;
use crate::game_level::GameLevel;
use crate::game_object::GameObject;
use crate::lib::post_processor::PostProcessor;
use crate::lib::shader::Shader;
use crate::lib::sprite_renderer::SpriteRenderer;
use crate::particle::ParticleGenerator;
use crate::power_up::PowerUp;
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
static mut POST_PROCESSOR: PostProcessor = PostProcessor::new_empty();
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
    levels: Vec<GameLevel>,
    actual_level: usize,
    power_ups: Vec<PowerUp>,
    shake_time: f32,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Game {
            state: GameState::GameActive,
            width: width,
            height: height,
            player: GameObject::new_empty(),
            ball: Ball::new_empty(),
            levels: Vec::new(),
            actual_level: 0,
            power_ups: Vec::new(),
            shake_time: 0.0
        }
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
        let effects_shader = RESOURCES.lock().unwrap().load_shader(
            "resources/shaders/effects_vs.glsl",
            "resources/shaders/effects_fs.glsl",
            "effects"
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
        RESOURCES.lock().unwrap().load_texture("resources/textures/powerup_sticky.png", true, "powerup_sticky");
        RESOURCES.lock().unwrap().load_texture("resources/textures/powerup_speed.png", true, "powerup_speed");
        RESOURCES.lock().unwrap().load_texture("resources/textures/powerup_passthrough.png", true, "powerup_passthrough");
        RESOURCES.lock().unwrap().load_texture("resources/textures/powerup_increase.png", true, "powerup_increase");
        RESOURCES.lock().unwrap().load_texture("resources/textures/powerup_confuse.png", true, "powerup_confuse");
        RESOURCES.lock().unwrap().load_texture("resources/textures/powerup_chaos.png", true, "powerup_chaos");

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
        POST_PROCESSOR = PostProcessor::new(effects_shader, self.width as i32, self.height as i32);

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

        // Sound initialization
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device available");
        
        let mut supported_configs_range = device.supported_output_configs()
            .expect("error while querying configs");
        let supported_config = supported_configs_range.next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        
        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        // let stream = device.build_output_stream(&config, write_silence, err_fn).unwrap();

        let sample_format = supported_config.sample_format();
        let config = supported_config.into();
        let stream = device.build_output_stream(&config, write_silence, err_fn).unwrap();
        // let stream = match sample_format {
        //     SampleFormat::F32 => device.build_output_stream(&config, write_silence::<f32>, err_fn),
        //     SampleFormat::I16 => device.build_output_stream(&config, write_silence::<i16>, err_fn),
        //     SampleFormat::U16 => device.build_output_stream(&config, write_silence::<u16>, err_fn),
        // }.unwrap();
        stream.play().unwrap();
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
        // update PowerUps
        self.update_power_ups(dt);
        // update effects
        if self.shake_time > 0.0 {
            self.shake_time -= dt;
            if self.shake_time <= 0.0 {
                unsafe {
                    POST_PROCESSOR.shake = false;
                }
            }
        }
        // check loss condition
        if self.ball.game_object.position.y >= self.height as f32 {
            self.reset_level();
            self.reset_player();
        }
    }

    pub unsafe fn render(&self, time: f32) {
        if self.state == GameState::GameActive {
            // begin rendering to postprocessing framebuffer
            POST_PROCESSOR.begin_render();

            // Draw background
            let background_tex = RESOURCES.lock().unwrap().get_texture("background");
            RENDERER.draw_sprite(&background_tex, vec2(0.0, 0.0), vec2(self.width as f32, self.height as f32), 0.0, vec3(1.0, 1.0, 1.0));
            // Draw level
            self.levels[self.actual_level].draw(&RENDERER);
            // draw player
            self.player.draw(&RENDERER);
            // draw powerups
            for power_up in &self.power_ups {
                if !power_up.game_object.destroyed {
                    power_up.game_object.draw(&RENDERER);
                }
            }
            // draw particles	
            PARTICLE_GENERATOR.draw();
            // draw ball
            self.ball.draw(&RENDERER);

            // end rendering to postprocessing framebuffer
            POST_PROCESSOR.end_render();
            // render postprocessing quad
            POST_PROCESSOR.render(time);
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

        // also disable all active powerups
        self.player.color = vec3(1.0, 1.0, 1.0);
        unsafe {
            POST_PROCESSOR.confuse = false;
            POST_PROCESSOR.chaos = false;
        }

        self.power_ups.clear();
    }

    pub fn do_collisions(&mut self) {
        for brick in &mut self.levels[self.actual_level].bricks {
            if !brick.destroyed {
                let collision: Collision = check_circle_collision(&self.ball, &brick);

                if collision.0 { // if collision is true
                    // destroy block if not solid
                    if !brick.is_solid {
                        brick.destroyed = true;
                        if let Some(power_up) = spawn_power_ups(brick.position) {
                            self.power_ups.push(power_up);
                        }
                    } else { // if block is solid, enable shake effect
                        self.shake_time = 0.05;
                        unsafe {
                            POST_PROCESSOR.shake = true;
                        }
                    }
                    // collision resolution
                    let dir = collision.1;
                    let diff_vector = collision.2;
                    let mut penetration = 0.0;

                    if !(self.ball.passthrough && !brick.is_solid) {
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
        }

        // also check collisions on PowerUps and if so, activate them
        for power_up in &mut self.power_ups {
            if !power_up.game_object.destroyed {
                // first check if powerup passed bottom edge, if so: keep as inactive and destroy
                if power_up.game_object.position.y >= self.height as f32 {
                    power_up.game_object.destroyed = true;
                }

                if check_square_collision(&self.player, &power_up.game_object) {
                    // collided with player, now activate powerup
                    let pw_type = power_up.pw_type.as_str();
                    match pw_type {
                        "sticky" => {
                            self.ball.sticky = true;
                            self.player.color = vec3(1.0, 0.5, 1.0);
                        }
                        "speed" => {
                            self.ball.game_object.velocity *= 1.2;
                        }
                        "passthrough" => {
                            self.ball.passthrough = true;
                            self.ball.game_object.color = vec3(1.0, 0.5, 0.5);
                        }
                        "increase" => {
                            self.player.size.x += 50.0;
                        }
                        "confuse" => unsafe {
                            if !POST_PROCESSOR.chaos {
                                POST_PROCESSOR.confuse = true;
                            }
                        }
                        "chaos" => unsafe {
                            if !POST_PROCESSOR.confuse {
                                POST_PROCESSOR.chaos = true;
                            }
                        }
                        &_ => {}
                    }
                    power_up.game_object.destroyed = true;
                    power_up.activated = true;
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

            // if Sticky powerup is activated, also stick ball to paddle once new velocity vectors were calculated
            self.ball.stuck = self.ball.sticky;
        }
    }

    fn update_power_ups(&mut self, dt: f32) {
        let mut power_up_list = self.power_ups.clone();

        for power_up in &mut self.power_ups {
            power_up.game_object.position += power_up.game_object.velocity;

            if power_up.activated {
                power_up.duration -= dt;

                if power_up.duration <= 0.0 {
                    // remove powerup from list (will later be removed)
                    power_up.activated = false;

                    if let Some(pos) = power_up_list.iter().position(|pu| (pu.pw_type == power_up.pw_type) && pu.activated ) {
                        power_up_list[pos].activated = false;
                    }

                    // deactivate effects
                    if power_up.pw_type == "sticky" {
                        if !is_other_power_up_active(&power_up_list, "sticky".to_string()) {
                            self.ball.sticky = false;
                            self.player.color = vec3(1.0, 1.0, 1.0);
                        }
                    }
                    if power_up.pw_type == "speed" {
                        if !is_other_power_up_active(&power_up_list, "speed".to_string()) {
                            self.ball.game_object.velocity /= 1.2;
                        }
                    }
                    if power_up.pw_type == "passthrough" {
                        if !is_other_power_up_active(&power_up_list, "passthrough".to_string()) {
                            self.ball.passthrough = false;
                            self.ball.game_object.color = vec3(1.0, 1.0, 1.0);
                        }
                    }
                    if power_up.pw_type == "increase" {
                        if !is_other_power_up_active(&power_up_list, "increase".to_string()) {
                            self.player.size.x -= 50.0;
                        }
                    }
                    if power_up.pw_type == "confuse" {
                        if !is_other_power_up_active(&power_up_list, "confuse".to_string()) {
                            unsafe {
                                POST_PROCESSOR.confuse = false;
                            }
                        }
                    }
                    if power_up.pw_type == "chaos" {
                        if !is_other_power_up_active(&power_up_list, "chaos".to_string()) {
                            unsafe {
                                POST_PROCESSOR.chaos = false;
                            }
                        }
                    }
                }
            }
        }

        // self.power_ups.retain(|pu| pu.duration > 0.0);
        self.power_ups.retain(|pu| pu.duration > 0.0);
    }
}

fn spawn_power_ups(pos: Vector2<f32>) -> Option<PowerUp> {
    let resources = RESOURCES.lock().unwrap();

    if power_up_should_spawn(75) {
        Some(PowerUp::new(pos, vec3(1.0, 0.5, 1.0), resources.get_texture("powerup_sticky"), "sticky", 15.0, false))
    } else if power_up_should_spawn(75) {
        Some(PowerUp::new(pos, vec3(0.5, 0.5, 1.0), resources.get_texture("powerup_speed"), "speed", 15.0, false))
    } else if power_up_should_spawn(75) {
        Some(PowerUp::new(pos, vec3(0.5, 1.0, 0.5), resources.get_texture("powerup_passthrough"), "passthrough", 10.0, false))
    } else if power_up_should_spawn(75) {
        Some(PowerUp::new(pos, vec3(1.0, 0.6, 0.4), resources.get_texture("powerup_increase"), "increase", 15.0, false))
    } else if power_up_should_spawn(15) {
        Some(PowerUp::new(pos, vec3(1.0, 0.3, 0.3), resources.get_texture("powerup_confuse"), "confuse", 15.0, false))
    } else if power_up_should_spawn(15) {
        Some(PowerUp::new(pos, vec3(0.9, 0.25, 0.25), resources.get_texture("powerup_chaos"), "chaos", 15.0, false))
    } else {
        None
    }
}

fn power_up_should_spawn(chance: u32) -> bool {
    let mut rng = rand::thread_rng();
    let random: u32 = rng.gen::<u32>() % chance;
    random == 0
}

fn is_other_power_up_active(power_ups: &Vec<PowerUp>, pw_type: String) -> bool {
    // Check if another PowerUp of the same type is still active
    // in which case we don't disable its effect (yet)
    for power_up in power_ups {
        if power_up.activated && power_up.pw_type == pw_type {
            return true;
        }
    }

    return false;
}

// AABB - AABB collision
fn check_square_collision(one: &GameObject, two: &GameObject) -> bool {
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

// fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
//     for sample in data.iter_mut() {
//         *sample = Sample::from(&0.0);
//     }
// }

fn write_silence(data: &mut [f32], _: &cpal::OutputCallbackInfo) {
    let mut counter = 0;
    for sample in data.iter_mut() {
        let s = if (counter / 20) % 2 == 0 { &1.0 } else { &0.0 };
        counter = counter + 1;
        *sample = Sample::from(s);
    }
    println!("{:?}", data);
}