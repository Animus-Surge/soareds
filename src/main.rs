extern crate sdl2;
extern crate nolog;

use nolog::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::video::GLProfile;

mod objects;
mod rendersys;
mod util;

fn main() {

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
        window.gl_swap_window();
    }
}