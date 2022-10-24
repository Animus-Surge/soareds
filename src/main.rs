extern crate sdl2;

//*
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod window;


mod testmod;

fn main() {

    testmod::test();

    unsafe { window::create_display("Test", 800, 600).init(); }

}