use std::error::Error;
use std::time;
use graphics::*;

use super::colors::*;
use super::App;
use super::AppState;

/// draws a fps counter in the top left corner
pub fn fps_counter(app: &mut App, c: &Context) -> Result<(), String> {
    text(
        WHITE,
        28,
        (1.0 / app.previous_frame_instant.elapsed().as_secs_f64())
            .ceil()
            .to_string()
            .as_str(),
        &mut app.glyph_cache,
        c.transform.trans(40.0, 40.0),
        &mut app.gl,
    )
}

/// draws the hangman character
pub fn hangman(
    app: &mut App,
    transform: &types::Matrix2d,
    args: &piston::RenderArgs,
) -> Result<(), Box<dyn Error>> {
    {
        let color = match app.state {
            AppState::GameOver(state) => {
                if state {
                    VICTORY_GREEN
                } else {
                    LOSS_RED
                }
            }
            _ => WHITE,
        };
        let hangman_anchor = transform
            .trans(-50.0 * app.scale, -args.window_size[1] * 0.1)
            .scale(app.scale, app.scale);
        let rotation_factor = f64::sin(
            std::f64::consts::PI
                * 0.3
                * (time::SystemTime::now().duration_since(time::UNIX_EPOCH))?.as_secs_f64(),
        );
        let rope_anchor = transform
            .trans(-50.0 * app.scale, -args.window_size[1] * 0.1)
            .trans(97.0 * app.scale, -150.0 * app.scale)
            .rot_deg(20.0 * rotation_factor)
            .scale(app.scale, app.scale);
        let draw_count = if app.state == AppState::Selecting || app.game.guess_count >= app.game.difficulty.0.len() as i32 {
            app.game.difficulty.0.len()
        } else {
            app.game.guess_count as usize
        };
        for i in 0..draw_count {
            let elem = &app.game.difficulty.0[i];
            use super::HangmanDrawingElements::*;
            match elem {
                Base => line(
                    color,
                    4.0,
                    [-20.0, 0.0, 20.0, 0.0],
                    hangman_anchor,
                    &mut app.gl,
                ),
                VerticalBeam => line(
                    color,
                    3.0,
                    [0.0, 0.0, 0.0, -150.0],
                    hangman_anchor,
                    &mut app.gl,
                ),
                HorizontalBeam => line(
                    color,
                    3.0,
                    [-3.0, -150.0, 100.0, -150.0],
                    hangman_anchor,
                    &mut app.gl,
                ),
                Rope => line(color, 3.0, [0.0, 0.0, 0.0, 20.0], rope_anchor, &mut app.gl),
                Head => circle_arc(
                    color,
                    2.0,
                    0.0,
                    std::f64::consts::TAU,
                    [-15.0, 20.0, 30.0, 30.0],
                    rope_anchor,
                    &mut app.gl,
                ),
                Torso => line(color, 3.0, [0.0, 50.0, 0.0, 90.0], rope_anchor, &mut app.gl),
                LeftArm => line(
                    color,
                    3.0,
                    [0.0, 50.0, -20.0, 80.0],
                    rope_anchor,
                    &mut app.gl,
                ),
                RightArm => line(
                    color,
                    3.0,
                    [0.0, 50.0, 20.0, 80.0],
                    rope_anchor,
                    &mut app.gl,
                ),
                LeftLeg => line(
                    color,
                    3.0,
                    [0.0, 90.0, -20.0, 120.0],
                    rope_anchor,
                    &mut app.gl,
                ),
                RightLeg => line(
                    color,
                    3.0,
                    [0.0, 90.0, 20.0, 120.0],
                    rope_anchor,
                    &mut app.gl,
                ),
                SupportBeam => line(
                    color,
                    3.0,
                    [0.0, -100.0, 50.0, -150.0],
                    hangman_anchor,
                    &mut app.gl,
                ),
                LeftEye => ellipse(color, [-5.0, 32.0, 2.5, 2.5], rope_anchor, &mut app.gl),
                RightEye => ellipse(color, [5.0 - 2.0, 32.0, 2.5, 2.5], rope_anchor, &mut app.gl),
                Mouth => line(
                    color,
                    1.0,
                    [-5.0, 39.0, 5.0, 39.0],
                    rope_anchor,
                    &mut app.gl,
                ),
            }
        }
        Ok(())
    }
}

/// draws the guessable characters
pub fn guessable_characters(
    app: &mut App,
    c: &Context,
    font_size: u32,
) -> Result<(), String> {
    let scaled_font_size: u32 = (font_size as f64 * app.scale) as u32;
    for i in 0..app.game.guessable_characters.len() {
        let color = match app.state {
            AppState::GameOver(state) => {
                if state {
                    if app.game.guessable_characters[i].1 {
                        VICTORY_GREEN_DARK
                    } else {
                        VICTORY_GREEN
                    }
                } else {
                    if app.game.guessable_characters[i].1 {
                        LOSS_RED_DARK
                    } else {
                        LOSS_RED
                    }
                }
            }
            AppState::Selecting => WHITE,
            AppState::Guessing => {
                if app.game.guessable_characters[i].1 {
                    GREY
                } else {
                    WHITE
                }
            }
        };
        text(
            color,
            (scaled_font_size as f64 * 0.8) as u32,
            app.game.guessable_characters[i].0.to_string().as_str(),
            &mut app.glyph_cache,
            c.transform
                .trans(app.guessable_char_pos[i][0], app.guessable_char_pos[i][1]),
            &mut app.gl,
        )?;
    }
    Ok(())
}

/// draws the in progress word
pub fn in_progress_word(
    app: &mut App,
    transform: &types::Matrix2d,
    font_size: u32,
) -> Result<(), String> {
    let scaled_font_size: u32 = (font_size as f64 * app.scale) as u32;
    let text_color = if let AppState::GameOver(state) = app.state {
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
        &app.game.in_progress_word,
        &mut app.glyph_cache,
        transform.trans(
            (scaled_font_size as usize * &app.game.in_progress_word.chars().count())
                as f64
                / -2.5,
            0.0,
        ),
        &mut app.gl,
    )
}

/// draws the instructions for how to select a word and difficulty
pub fn instructions() {
    todo!()
}