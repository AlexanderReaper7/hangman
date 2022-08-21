extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use crate::game::*;
use opengl_graphics::GlyphCache;
use opengl_graphics::{GlGraphics, OpenGL, TextureSettings};
use piston::input::{RenderArgs, UpdateArgs};
use piston::{Button, ButtonArgs, Key, ResizeArgs};
use std::cmp::min;
use std::time::Instant;

pub enum AppState {
    PreGame,
    Game,
    GameOver(bool),
}

/// logical size of the window to help with positioning elements on the screen
pub const WINDOW_DOTS: f64 = 1000.0;
const FONT_SIZE: u32 = 52;

pub struct App<'a> {
    /// OpenGL drawing backend.
    gl: GlGraphics,
    /// Time between frames.
    pub previous_frame_instant: Instant,
    /// Cache for the font.
    glyph_cache: GlyphCache<'a>,
    /// the game itself
    pub game: Game,
    /// the state of the app
    pub state: AppState,
    /// position of the mouse
    pub mouse_position: [f64; 2],
    /// the size of the window
    pub window_size: [f64; 2],
    /// the position of the guessable characters
    guessable_char_pos: Vec<[f64;2]>,
    /// the width of the guessable characters, height = 2 * width
    guessable_char_size: f64,
    scale: f64,
}
impl Default for App<'_> {
    fn default() -> App<'static> {
        let game = Game::new("QuickBrownFox".to_string()).unwrap();
        let mut app = App {
            gl: GlGraphics::new(OpenGL::V3_2),
            previous_frame_instant: Instant::now(),
            glyph_cache: GlyphCache::new("assets/courier-prime-code.regular.ttf", (), TextureSettings::new())
                .unwrap(),
            state: AppState::Game,
            mouse_position: [0.0, 0.0],
            window_size: [WINDOW_DOTS, WINDOW_DOTS],
            guessable_char_pos: vec![[0.0, 0.0]; game.guessable_characters.len()],
            game,
            guessable_char_size: FONT_SIZE as f64,
            scale: 1.0,
        };
        app.update_guessable_char_positions();
        app
    }
}
impl App<'_> {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const VICTORY_GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const LOSS_RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const VICTORY_GREEN_DARK: [f32; 4] = [0.0, 0.4, 0.0, 1.0];
        const LOSS_RED_DARK: [f32; 4] = [0.4, 0.0, 0.0, 1.0];
        const GREY: [f32; 4] = [0.4, 0.4, 0.4, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // --- SETUP ---
            // Clear the screen.
            clear(BLACK, gl);
            // center of window transform
            let center_anchor = c.transform.trans(args.window_size[0] / 2.0, args.window_size[1] / 2.0);
            // with scaling
            let scaled_center_anchor = center_anchor.scale(self.scale, self.scale);
            // standard font size
            let scaled_font_size: u32 = (FONT_SIZE as f64 * self.scale) as u32;
            // --- in progress word ---
            {
                let text_color = if let AppState::GameOver(state) = self.state {
                    if state {
                        VICTORY_GREEN
                    } else {
                        LOSS_RED
                    }
                } else {
                    WHITE
                };
                text(
                    text_color,
                    scaled_font_size,
                    &self.game.in_progress_word,
                    &mut self.glyph_cache,
                    center_anchor.trans(
                        (scaled_font_size as usize * &self.game.in_progress_word.chars().count()) as f64
                            / -2.5,
                        0.0,
                    ),
                    gl,
                )
                .unwrap();
            }
            line(WHITE, 1.0, [0.0, -20.0, 0.0, 20.0], scaled_center_anchor, gl);
            // --- Guessable characters ---
            {
                for i in 0..self.game.guessable_characters.len() {
                    let color = match self.state {
                        AppState::GameOver(state) => {
                            if state {
                                if self.game.guessable_characters[i].1 {
                                    VICTORY_GREEN_DARK
                                } else {
                                    VICTORY_GREEN
                                }
                            } else {
                                if self.game.guessable_characters[i].1 {
                                    LOSS_RED_DARK
                                } else {
                                    LOSS_RED
                                }
                            }
                        }
                        AppState::PreGame => WHITE,
                        AppState::Game => {
                            if self.game.guessable_characters[i].1 {
                                GREY
                            } else {
                                WHITE
                            }
                        }
                    };
                    text(
                        color,
                        (scaled_font_size as f64 * 0.8) as u32,
                        self.game.guessable_characters[i].0.to_string().as_str(),
                        &mut self.glyph_cache,
                        c.transform.trans(self.guessable_char_pos[i][0], self.guessable_char_pos[i][1]),
                        gl,
                    )
                    .unwrap();
                }
            }
            // fps counter
            text(
                WHITE,
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

    pub fn resize(&mut self, args: &ResizeArgs) {
        self.window_size = args.window_size;
        self.scale = (self.window_size[0] / WINDOW_DOTS).min(self.window_size[1] / WINDOW_DOTS);
        self.update_guessable_char_positions();
    }

    pub fn update(&mut self, _args: &UpdateArgs) {
        
    }

    /// updates the internal position of the mouse
    pub fn update_mouse_cursor(&mut self, pos: [f64; 2]) {
        self.mouse_position = pos;
    }

    pub fn button(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(key) => {
                if key >= Key::A && key <= Key::Z {
                    let alphabet_index = key as usize - Key::A as usize;
                    self.game
                        .safe_guess(self.game.guessable_characters[alphabet_index].0);
                    if let Some(end_state) = self.game.get_game_state() {
                        self.state = AppState::GameOver(end_state);
                    }
                }
            }
            Button::Mouse(button) => {
                if button == piston::MouseButton::Left {
                    // scale into virtual coords
                    // let scalar = self.
                    // let (x, y) = (
                    //     self.mouse_position[0] / self.gl. * 1000.0,
                    //     self.mouse_position[1] / args.window_size[1] * 1000.0,
                    // );
                }
            }
            _ => {}
        }
    }

    fn update_guessable_char_positions(&mut self) {
        // calculate total number of rows needed to display all the characters
        let scaled_font_size: u32 = (FONT_SIZE as f64 * self.scale) as u32;
        let row_spacing = scaled_font_size as f64 * 2.0;
        let col_spacing = scaled_font_size as f64 * 1.3;
        let max_row_size_scalar: f64 = 0.8;
        let chars_per_row = (self.window_size[0] * max_row_size_scalar / col_spacing).floor() as u32;
        let rows = (self.guessable_char_pos.len() as f64 / chars_per_row as f64).ceil() as u32;
        // --- per char ---
        for row in 0..rows {
            // number of chars in this row
            let cols = min(
                chars_per_row, // max
                self.guessable_char_pos.len() as u32 - row * chars_per_row, // leftovers
            );
            for col in 0..cols {
                // positioning
                self.guessable_char_pos[(row*chars_per_row+col) as usize] = [
                    (col as f64 * col_spacing) + ((chars_per_row - cols) as f64 * col_spacing / 2.0) + (self.window_size[0] * (1.0 - max_row_size_scalar) / 2.0),
                    self.window_size[1] / (2.0 * 0.8) + (row as f64 * row_spacing)
                ];
            }
        }
    }
}
