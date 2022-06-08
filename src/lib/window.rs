use std::sync::mpsc::Receiver;

extern crate glfw;
use self::glfw::{ Context };

pub struct Window {}

impl Window {
    pub fn create(width: u32, height: u32, title: &str) -> (glfw::Glfw, glfw::Window, Receiver<(f64, glfw::WindowEvent)>) {
        // glfw: initialize and configure
        // ------------------------------
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

        // glfw window creation
        // --------------------
        let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_scroll_polling(true);

        // gl: load all OpenGL function pointers
        // ---------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        (glfw, window, events)
    }
}