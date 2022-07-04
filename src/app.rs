extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;


use crate::game::*;
use opengl_graphics::GlyphCache;
use opengl_graphics::{GlGraphics, OpenGL, TextureSettings};
use piston::input::{RenderArgs, UpdateArgs};
use std::time::Instant;

pub struct App<'a> {
    /// OpenGL drawing backend.
    gl: GlGraphics,
    /// Rotation for the square.
    pub rotation: f64,
    /// Time between frames.
    pub previous_frame_instant: Instant, 
    /// Cache for the font.
    glyph_cache: GlyphCache<'a>,

    pub game: Game,
}
impl Default for App<'_> {
    fn default() -> App<'static> {
        App {
            gl: GlGraphics::new(OpenGL::V3_2),
            rotation: 0.0,
            previous_frame_instant: Instant::now(),
            glyph_cache: GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new())
                .unwrap(),
            game: Game::new("hello".to_string()).unwrap(),
        }
    }
}
impl App<'_> {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
            text(
                RED,
                28,
                (1.0 / self.previous_frame_instant.elapsed().as_secs_f64())
                    .ceil()
                    .to_string()
                    .as_str(),
                &mut self.glyph_cache,
                c.transform.trans(40.0, 40.0),
                gl,
            )
            .unwrap();
        });
        self.previous_frame_instant = Instant::now();
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}
