extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod rendering;
mod colors;
use colors::*;

use crate::game::*;
use opengl_graphics::GlyphCache;
use opengl_graphics::{GlGraphics, OpenGL, TextureSettings};
use piston::input::{RenderArgs, UpdateArgs};
use piston::{Button, ButtonArgs, Key, ResizeArgs};
use std::cmp::min;
use std::time::Instant;

#[derive(Debug, PartialEq)]
pub enum AppState {
    Selecting,
    Guessing,
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
    pub window_size: [f64; 2], // TODO: this is likely redundant due to GlGraphics having a window size in GLGraphics::current_viewport::window_size
    /// the position of the guessable characters
    guessable_char_pos: Vec<[f64; 2]>,
    scale: f64,
}
impl Default for App<'_> {
    fn default() -> App<'static> {
        let game = Game::default();
        let mut app = App {
            gl: GlGraphics::new(OpenGL::V3_2),
            previous_frame_instant: Instant::now(),
            glyph_cache: GlyphCache::from_bytes(
                include_bytes!("../../assets/courier-prime-code.regular.ttf"),
                (),
                TextureSettings::new(),
            )
            .unwrap(),
            state: AppState::Selecting,
            mouse_position: [0.0, 0.0],
            window_size: [WINDOW_DOTS, WINDOW_DOTS],
            guessable_char_pos: vec![[0.0, 0.0]; game.guessable_characters.len()],
            game,
            scale: 1.0,
        };
        app.update_guessable_char_positions();
        app
    }
}
impl App<'_> {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let c = self.gl.draw_begin(args.viewport());
            // Clear the screen.
            clear(BLACK, &mut self.gl);
            // center of window transform
            let center_anchor = c
                .transform
                .trans(args.window_size[0] / 2.0, args.window_size[1] / 2.0);
            if self.state == AppState::Selecting {
                //rendering::instructions(self, &center_anchor, FONT_SIZE).unwrap();
            }
            rendering::in_progress_word(self, &center_anchor, FONT_SIZE).unwrap();
            rendering::guessable_characters(self, &c, FONT_SIZE).unwrap();
            rendering::hangman(self, &center_anchor, args).unwrap();
            #[cfg(debug_assertions)]
            rendering::fps_counter(self, &c).unwrap();
        self.gl.draw_end();
        self.previous_frame_instant = Instant::now();
    }

    pub fn resize(&mut self, args: &ResizeArgs) {
        self.window_size = args.window_size;
        self.scale = (self.window_size[0] / WINDOW_DOTS).min(self.window_size[1] / WINDOW_DOTS);
        self.update_guessable_char_positions();
    }

    pub fn update(&mut self, _args: &UpdateArgs) {}

    /// updates the internal position of the mouse
    pub fn update_mouse_cursor(&mut self, pos: [f64; 2]) {
        self.mouse_position = pos;
    }

    pub fn button(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(key) => {
                if args.state != piston::ButtonState::Press {
                    return;
                }
                match self.state {
                    AppState::GameOver(_) => {
                        if key == Key::Space || key == Key::Return {
                            self.game.in_progress_word = "".to_string();
                            self.state = AppState::Selecting;
                        }
                    },
                    AppState::Selecting => {
                        if key >= Key::A && key <= Key::Z {
                            let alphabet_index = key as usize - Key::A as usize;
                            self.game.in_progress_word.push(self.game.guessable_characters[alphabet_index].0);
                        }
                        else if key == Key::Backspace {
                            self.game.in_progress_word.pop();
                        }
                        else if key == Key::Return {
                            self.game = match Game::from_game(&self.game, self.game.in_progress_word.clone()) {
                                Ok(g) => g,
                                Err(_) => return,
                            };
                            self.state = AppState::Guessing;
                        }
                    },
                    AppState::Guessing => {
                        if key >= Key::A && key <= Key::Z {
                            let alphabet_index = key as usize - Key::A as usize;
                            self.game
                                .guess(self.game.guessable_characters[alphabet_index].0);
                            if let Some(end_state) = self.game.get_game_state() {
                                self.state = AppState::GameOver(end_state);
                            }
                        }
                    }
                }
            }
            Button::Mouse(button) => {
                if button == piston::MouseButton::Left { // TODO: mouse click the letters
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
        let chars_per_row =
            (self.window_size[0] * max_row_size_scalar / col_spacing).floor() as u32;
        let rows = (self.guessable_char_pos.len() as f64 / chars_per_row as f64).ceil() as u32;
        // --- per char ---
        for row in 0..rows {
            // number of chars in this row
            let cols = min(
                chars_per_row,                                              // max
                self.guessable_char_pos.len() as u32 - row * chars_per_row, // leftovers
            );
            for col in 0..cols {
                // positioning
                self.guessable_char_pos[(row * chars_per_row + col) as usize] = [
                    (col as f64 * col_spacing)
                        + ((chars_per_row - cols) as f64 * col_spacing / 2.0)
                        + (self.window_size[0] * (1.0 - max_row_size_scalar) / 2.0),
                    self.window_size[1] / (2.0 * 0.8) + (row as f64 * row_spacing),
                ];
            }
        }
    }
}
