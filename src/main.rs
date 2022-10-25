extern crate sdl2;
extern crate nolog;

use nolog::*;

use std::ffi::CString;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

mod objects;
mod rendersys;
mod util;

fn main() {
    logon!();

    info!("===== BOOT =====");
    info!("S.O.A.R.E.D.S. Hub Device Software");
    info!("Version 0.1.0-dev");

    //Window vars
    let width = 800;
    let height = 600;
    let title = "Test Window";

    let sdl_content = sdl2::init().unwrap();
    let video_sys = sdl_content.video().unwrap();

    //OpenGL stuff
    let gl_attr = video_sys.gl_attr();

    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_flags().debug().set();
    gl_attr.set_context_version(3, 3);

    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    //Create window stuff and opengl context
    let window = video_sys.window(title, width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut event_sys = sdl_content.event_pump().unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_sys.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, width.try_into().unwrap(), height.try_into().unwrap());
        gl::ClearColor(0.01, 0.01, 0.01, 1.0);
    }

    //Create the objects for the shader program
    let vertex = objects::shader::Shader::from_vertex_source(
        &CString::new(include_str!("test_vertex.glsl")).unwrap()
    ).unwrap();
    let fragment = objects::shader::Shader::from_fragment_source(
        &CString::new(include_str!("test_fragment.glsl")).unwrap()
    ).unwrap();

    let program = objects::program::Program::from_shaders(
        &[vertex, fragment]
    ).unwrap();
    program.use_program();

    //Create our test triangle
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0,
    ];

    let mut vbo: gl::types::GLuint = 0;
    let mut vao: gl::types::GLuint = 0;

    unsafe {
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
            std::ptr::null()
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }



    'running: loop {

        for event in event_sys.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        program.use_program();
        unsafe {
            gl::BindVertexArray(vao);

            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                3
            );
        }

        window.gl_swap_window();
    }
}