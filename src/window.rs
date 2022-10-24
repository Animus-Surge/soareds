extern crate sdl2;
extern crate gl33;

use std::*;
use std::time::Duration;
use std::u32;
use sdl2::libc::labs;
use sdl2::video::Window;
use sdl2::video::GLProfile;

use gl33::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

pub fn create_display(title: &'static str, width: u32, height: u32) -> DisplayScreen {
    return DisplayScreen {
        title,
        fullscreen: false,
        width,
        height
    }
}

pub struct DisplayScreen {
    title: &'static str,
    fullscreen: bool,
    width: u32,
    height: u32,
}

impl DisplayScreen {
    fn set_title(&mut self, new_title: &'static str) {
        self.title = new_title;
    }

    fn get_title(&mut self) -> &str {
        return self.title;
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }

    fn fullscreen(&mut self) -> bool {
        return self.fullscreen;
    }

    pub unsafe fn init(&mut self) {
        let sdl_content = sdl2::init().unwrap();
        let video_sys = sdl_content.video().unwrap();

        //OpenGL stuff
        let gl_attr = video_sys.gl_attr();

        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_flags().debug().set();
        gl_attr.set_context_version(3, 3);

        gl_attr.set_multisample_buffers(1);
        gl_attr.set_multisample_samples(4);

        let window = video_sys.window(self.title, self.width, self.height).position_centered().opengl().build().unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        let mut event_sys = sdl_content.event_pump().unwrap();

        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
            for event in event_sys.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            canvas.present();
            thread::sleep(Duration::new(0, 1_000_000u32 / 60));
        }

    }
}

