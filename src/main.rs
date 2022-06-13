extern crate glfw;
use self::glfw::{ Context };

#[macro_use]
extern crate lazy_static;

use gl;

mod ball;
mod game;
use game::Game;
mod game_level;
mod game_object;
mod resource_manager;
mod lib {
    pub mod shader;
    pub mod sprite_renderer;
    pub mod texture;
    pub mod window;
}
use lib::window::Window;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    // Delta time variables
    // -------------------
    let mut delta_time: f32; // time between current frame and last frame
    let mut last_frame: f32 = 0.0;

    // Window
    // ------
    let (mut glfw, mut window, _events) = Window::create(SCR_WIDTH, SCR_HEIGHT, "BreakOut");

    // Game initialization
    // -------------------
    let mut breakout = Game::new(SCR_WIDTH, SCR_HEIGHT);
    unsafe {
        breakout.init();
    }

    // render loop
    // -----------
    while !window.should_close() {
        // per-frame time logic
        // --------------------
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // println!("{}", delta_time);

        // input
        // -----
        breakout.process_input(&window, delta_time);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            breakout.render();
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}