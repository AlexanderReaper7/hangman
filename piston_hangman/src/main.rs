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
use piston::input::mouse::MouseCursorEvent;
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::{EventLoop, ButtonEvent, ResizeEvent};

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Hangman", [WINDOW_DOTS, WINDOW_DOTS])
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
        e.render(|args| app.render(&args));
        e.update(|args| app.update(&args));
        e.resize(|args| app.resize(&args));
        e.button(|args| app.button(&args));
        MouseCursorEvent::mouse_cursor(&e, |args| app.update_mouse_cursor(args));
    }
}
