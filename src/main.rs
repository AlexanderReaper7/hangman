#![windows_subsystem = "windows"]
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
mod app;
mod game;
use app::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::EventLoop;

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Hangman", [1920, 1080])
        .graphics_api(opengl)
        .samples(8)
        .exit_on_esc(true)
        .vsync(false)
        .build()
        .unwrap();
    
    // Create a new game and run it.
    let mut app = App::default();

    let mut events = Events::new(EventSettings::new().max_fps(120));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
