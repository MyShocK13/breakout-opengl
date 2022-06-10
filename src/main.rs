use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

extern crate glfw;
use self::glfw::{ Context, Key, Action };

use gl;
use self::gl::types::*;

use image;
use image::GenericImage;

use cgmath::{ Matrix4, Vector3, vec3,  Deg, perspective, Point3, Rad };
use cgmath::prelude::*;

mod game;
use game::Game;
mod macros;
use macros::*;
mod lib {
    pub mod camera;
    pub mod common;
    pub mod shader;
    pub mod sprite_renderer;
    pub mod texture;
    pub mod window;
}
use lib::camera::Camera;
use lib::common::{ process_events, processInput };
use lib::shader::Shader;
use lib::texture::Texture2D;
use lib::window::Window;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    // let mut camera = Camera {
    //     Position: Point3::new(0.0, 0.0, 3.0),
    //     ..Camera::default()
    // };

    // let mut firstMouse = true;
    // let mut lastX: f32 = SCR_WIDTH as f32 / 2.0;
    // let mut lastY: f32 = SCR_HEIGHT as f32 / 2.0;
    
    

    // Delta time variables
    // -------------------
    let mut delta_time: f32; // time between current frame and last frame
    let mut last_frame: f32 = 0.0;

    let (mut glfw, mut window, events) = Window::create(SCR_WIDTH, SCR_HEIGHT, "BreakOut");

    let breakout = Game::new(SCR_WIDTH, SCR_HEIGHT);
    unsafe {
        breakout.init();
    }
    
    // let (ourShader, VBO, VAO, texture1, cubePositions) = unsafe {
    //     // configure global opengl state
    //     // -----------------------------
    //     gl::Enable(gl::DEPTH_TEST);

    //     // build and compile our shader program
    //     // ------------------------------------
    //     let ourShader = Shader::new(
    //         "resources/shaders/vertexShader.glsl",
    //         "resources/shaders/fragmentShader.glsl");

    //     // set up vertex data (and buffer(s)) and configure vertex attributes
    //     // ------------------------------------------------------------------
    //     let vertices: [f32; 180] = [
    //          -0.5, -0.5, -0.5,  0.0, 0.0,
    //           0.5, -0.5, -0.5,  1.0, 0.0,
    //           0.5,  0.5, -0.5,  1.0, 1.0,
    //           0.5,  0.5, -0.5,  1.0, 1.0,
    //          -0.5,  0.5, -0.5,  0.0, 1.0,
    //          -0.5, -0.5, -0.5,  0.0, 0.0,

    //          -0.5, -0.5,  0.5,  0.0, 0.0,
    //           0.5, -0.5,  0.5,  1.0, 0.0,
    //           0.5,  0.5,  0.5,  1.0, 1.0,
    //           0.5,  0.5,  0.5,  1.0, 1.0,
    //          -0.5,  0.5,  0.5,  0.0, 1.0,
    //          -0.5, -0.5,  0.5,  0.0, 0.0,

    //          -0.5,  0.5,  0.5,  1.0, 0.0,
    //          -0.5,  0.5, -0.5,  1.0, 1.0,
    //          -0.5, -0.5, -0.5,  0.0, 1.0,
    //          -0.5, -0.5, -0.5,  0.0, 1.0,
    //          -0.5, -0.5,  0.5,  0.0, 0.0,
    //          -0.5,  0.5,  0.5,  1.0, 0.0,

    //           0.5,  0.5,  0.5,  1.0, 0.0,
    //           0.5,  0.5, -0.5,  1.0, 1.0,
    //           0.5, -0.5, -0.5,  0.0, 1.0,
    //           0.5, -0.5, -0.5,  0.0, 1.0,
    //           0.5, -0.5,  0.5,  0.0, 0.0,
    //           0.5,  0.5,  0.5,  1.0, 0.0,

    //          -0.5, -0.5, -0.5,  0.0, 1.0,
    //           0.5, -0.5, -0.5,  1.0, 1.0,
    //           0.5, -0.5,  0.5,  1.0, 0.0,
    //           0.5, -0.5,  0.5,  1.0, 0.0,
    //          -0.5, -0.5,  0.5,  0.0, 0.0,
    //          -0.5, -0.5, -0.5,  0.0, 1.0,

    //          -0.5,  0.5, -0.5,  0.0, 1.0,
    //           0.5,  0.5, -0.5,  1.0, 1.0,
    //           0.5,  0.5,  0.5,  1.0, 0.0,
    //           0.5,  0.5,  0.5,  1.0, 0.0,
    //          -0.5,  0.5,  0.5,  0.0, 0.0,
    //          -0.5,  0.5, -0.5,  0.0, 1.0
    //     ];
    //     // world space positions of our cubes
    //     let cubePositions: [Vector3<f32>; 10] = [vec3(0.0, 0.0, 0.0),
    //                                              vec3(2.0, 5.0, -15.0),
    //                                              vec3(-1.5, -2.2, -2.5),
    //                                              vec3(-3.8, -2.0, -12.3),
    //                                              vec3(2.4, -0.4, -3.5),
    //                                              vec3(-1.7, 3.0, -7.5),
    //                                              vec3(1.3, -2.0, -2.5),
    //                                              vec3(1.5, 2.0, -2.5),
    //                                              vec3(1.5, 0.2, -1.5),
    //                                              vec3(-1.3, 1.0, -1.5)];
    //     let (mut VBO, mut VAO) = (0, 0);
    //     gl::GenVertexArrays(1, &mut VAO);
    //     gl::GenBuffers(1, &mut VBO);

    //     gl::BindVertexArray(VAO);

    //     gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    //     gl::BufferData(gl::ARRAY_BUFFER,
    //                    (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
    //                    &vertices[0] as *const f32 as *const c_void,
    //                    gl::STATIC_DRAW);

    //     let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
    //     // position attribute
    //     gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    //     gl::EnableVertexAttribArray(0);
    //     // texture coord attribute
    //     gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    //     gl::EnableVertexAttribArray(1);

    //     // texture 1
    //     // ---------
    //     // load image, create texture and generate mipmaps
    //     let img = image::open(&Path::new("resources/textures/container.jpg")).expect("Failed to load texture");
    //     let data = img.clone().into_bytes();
        
    //     let mut texture1 = Texture2D::default();
    //     texture1.generate(img.width(), img.height(), data);

    //     // // texture 2
    //     // // ---------
    //     // gl::GenTextures(1, &mut texture2);
    //     // gl::BindTexture(gl::TEXTURE_2D, texture2);
    //     // // set the texture wrapping parameters
    //     // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
    //     // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    //     // // set texture filtering parameters
    //     // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    //     // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
    //     // // load image, create texture and generate mipmaps
    //     // let img = image::open(&Path::new("resources/textures/awesomeface.png")).expect("Failed to load texture");
    //     // let img = img.flipv(); // flip loaded texture on the y-axis.
    //     // let data = img.clone().into_bytes();
    //     // // note that the awesomeface.png has transparency and thus an alpha channel, so make sure to tell OpenGL the data type is of GL_RGBA
    //     // gl::TexImage2D(gl::TEXTURE_2D,
    //     //                0,
    //     //                gl::RGB as i32,
    //     //                img.width() as i32,
    //     //                img.height() as i32,
    //     //                0,
    //     //                gl::RGBA,
    //     //                gl::UNSIGNED_BYTE,
    //     //                &data[0] as *const u8 as *const c_void);
    //     // gl::GenerateMipmap(gl::TEXTURE_2D);

    //     // tell opengl for each sampler to which texture unit it belongs to (only has to be done once)
    //     // -------------------------------------------------------------------------------------------
    //     ourShader.useProgram();
    //     let text = CStr::from_bytes_with_nul_unchecked(concat!("texture1", "\0").as_bytes());
    //     ourShader.setInt(text, 0);
    //     // ourShader.setInt(c_str!("texture2"), 1);

    //     // (ourShader, VBO, VAO, texture1, texture2, cubePositions)
    //     (ourShader, VBO, VAO, texture1, cubePositions)
    // };

    // render loop
    // -----------
    while !window.should_close() {
        // per-frame time logic
        // --------------------
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        // println!("{}", delta_time);

        // events
        // -----
        // process_events(&mut window, &events);
        // process_events(&events, &mut firstMouse, &mut lastX, &mut lastY, &mut camera);

        // input
        // -----
        // processInput(&mut window, delta_time, &mut camera);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            breakout.render();

        //     // bind textures on corresponding texture units
        //     gl::ActiveTexture(gl::TEXTURE0);
        //     texture1.bind();
        //     // gl::ActiveTexture(gl::TEXTURE1);
        //     // gl::BindTexture(gl::TEXTURE_2D, texture2);

        //     // activate shader
        //     ourShader.useProgram();

        //     let view = camera.GetViewMatrix();
        //     let text = CStr::from_bytes_with_nul_unchecked(concat!("view", "\0").as_bytes());
        //     ourShader.setMat4(text, &view);
            
        //     let projection: Matrix4<f32> = perspective(Deg(camera.Zoom), SCR_WIDTH as f32 / SCR_HEIGHT as f32 , 0.1, 100.0);
        //     let text = CStr::from_bytes_with_nul_unchecked(concat!("projection", "\0").as_bytes());
        //     ourShader.setMat4(text, &projection);

        //     // render boxes
        //     gl::BindVertexArray(VAO);
        //     for (i, position) in cubePositions.iter().enumerate() {
        //         // calculate the model matrix for each object and pass it to shader before drawing
        //         let mut model: Matrix4<f32> = Matrix4::from_translation(*position);
        //         let angle = 20.0 * i as f32;
        //         model = model * Matrix4::from_axis_angle(vec3(1.0, 0.3, 0.5).normalize(), Deg(angle));
        //         let text = CStr::from_bytes_with_nul_unchecked(concat!("model", "\0").as_bytes());
        //         ourShader.setMat4(text, &model);

        //         gl::DrawArrays(gl::TRIANGLES, 0, 36);
        //     }
        }






        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }

    // optional: de-allocate all resources once they've outlived their purpose:
    // ------------------------------------------------------------------------
    // unsafe {
    //     gl::DeleteVertexArrays(1, &VAO);
    //     gl::DeleteBuffers(1, &VBO);
    // }
}

// fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
//     for (_, event) in glfw::flush_messages(events) {
//         match event {
//             glfw::WindowEvent::FramebufferSize(width, height) => {
//                 // make sure the viewport matches the new window dimensions; note that width and
//                 // height will be significantly larger than specified on retina displays.
//                 unsafe { gl::Viewport(0, 0, width, height) }
//             }
//             glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
//             _ => {}
//         }
//     }
// }