extern crate glfw;
use self::glfw::{ Context };

use gl;

mod game;
use game::Game;
mod macros;
mod lib {
    pub mod camera;
    pub mod common;
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
    let mut _delta_time: f32; // time between current frame and last frame
    let mut last_frame: f32 = 0.0;

    let (mut glfw, mut window, _events) = Window::create(SCR_WIDTH, SCR_HEIGHT, "BreakOut");

    let breakout = Game::new(SCR_WIDTH, SCR_HEIGHT);
    unsafe {
        // configure global opengl state
        // -----------------------------
        gl::Enable(gl::DEPTH_TEST);

        breakout.init();
    }

    // render loop
    // -----------
    while !window.should_close() {
        // per-frame time logic
        // --------------------
        let current_frame = glfw.get_time() as f32;
        _delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // println!("{}", delta_time);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            breakout.render();
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}