use zstd::stream::read::Decoder as ZstdDecoder;
use std::io::Read;
use lazy_static::lazy_static;

pub const ENGLISH_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
lazy_static! {
    pub static ref ENGLISH_WORD_LIST: Vec<String> = {
        const SOURCE: &[u8] = include_bytes!("../../assets/english_word_list.txt.zst");
        let mut decoder = ZstdDecoder::new(SOURCE).unwrap();
        let mut out = String::new();
        decoder.read_to_string(&mut out).unwrap();
        out.lines().map(|s| s.to_string()).collect::<Vec<String>>()
    };
}
pub fn is_word_in_dictionary(word: &str) -> bool {
    ENGLISH_WORD_LIST.binary_search(&word.to_uppercase()).is_ok()
}

#[derive(Debug, Clone, Copy)]
pub enum HangmanDrawingElements {
    Base = 1,
    VerticalBeam = 2,
    HorizontalBeam = 3,
    Rope = 4,
    Head = 5,
    Torso = 6,
    LeftArm = 7,
    RightArm = 8,
    LeftLeg = 9,
    RightLeg = 10,
    // extras for easier than default below
    SupportBeam = 11,
    LeftEye = 12,
    RightEye = 13,
    Mouth = 14,
}
use HangmanDrawingElements::*;
/// the drawing element order and the different difficulty versions
#[derive(Debug, Clone)]
pub struct DifficultyLevel(pub Vec<HangmanDrawingElements>, &'static str);
impl DifficultyLevel {
    pub fn get_easiest() -> DifficultyLevel {
        DifficultyLevel(
            vec![
                Base,
                VerticalBeam,
                HorizontalBeam,
                SupportBeam,
                Rope,
                Head,
                Torso,
                LeftArm,
                RightArm,
                LeftLeg,
                RightLeg,
                LeftEye,
                RightEye,
                Mouth,
            ],
            "Easiest",
        )
    }
    pub fn get_normal() -> DifficultyLevel {
        DifficultyLevel(
            vec![
                Base,
                VerticalBeam,
                HorizontalBeam,
                SupportBeam,
                Rope,
                Head,
                Torso,
                LeftArm,
                RightArm,
                LeftLeg,
                RightLeg,
                LeftEye,
                RightEye,
                Mouth,
            ],
            "Normal",
        )
    }
    pub fn get_hard() -> DifficultyLevel {
        DifficultyLevel(
            vec![
                Base,
                VerticalBeam,
                HorizontalBeam,
                Rope,
                Head,
                Torso,
                LeftArm,
                RightArm,
                LeftLeg,
                RightLeg,
            ],
            "Hard",
        )
    }
    pub fn get_hardest() -> DifficultyLevel {
        DifficultyLevel(
            vec![
                VerticalBeam,
                HorizontalBeam,
                Rope,
                Head,
                Torso,
                LeftArm,
                RightArm,
                LeftLeg,
                RightLeg,
            ],
            "Hardest",
        )
    }
}

/// (the character, has it been guessed?)
pub struct GuessableChar(pub char, pub bool);
impl GuessableChar {
    pub fn new_set(character_set: &str) -> Vec<GuessableChar> {
        character_set
            .chars()
            .map(|c| GuessableChar(c, false))
            .collect()
    }
}
pub struct Game {
    /// the word to be guessed
    pub word: String,
    /// the current state for the word, has _ for each missing letter
    pub in_progress_word: String,
    // list of valid characters and whether they have already been guessed
    pub guessable_characters: Vec<GuessableChar>,
    /// number of guesses made
    pub guess_count: i32,
    /// the difficulty level, number and order of drawing elements on the hangman
    pub difficulty: DifficultyLevel,
}
impl Game {
    /// create a new game from input word
    pub fn new(mut word: String, difficulty: DifficultyLevel) -> Result<Game, ()> {
        if word.len() < 1 {
            return Err(());
        }
        word = word.to_uppercase();
        for c in word.chars() {
            if !ENGLISH_ALPHABET.contains(c) {
                return Err(());
            }
        }
        if !is_word_in_dictionary(&word) {
            return Err(());
        }
        Ok(Game {
            guessable_characters: GuessableChar::new_set(ENGLISH_ALPHABET),
            guess_count: 0,
            in_progress_word: "_".repeat((&word).len()),
            word,
            difficulty,
        })
    }
    pub fn from_game(game: &Game, word: String) -> Result<Game, ()> {
        Game::new(word, game.difficulty.clone())
    }
    /// checks if the character is in the word and fill it in, returns true if it was in the word
    pub fn guess(&mut self, char: char) -> Option<bool> {
        // find the character in the list of guessable characters
        let guessed = self.guessable_characters
        .iter()
        .find(|gc| gc.0 == char)
        .map(|gc| gc).unwrap();
        // if the character has already been guessed, return None
        if guessed.1 {
            return None;
        }
        let mut out = false;
        // check if char is in the word
        if self.word.contains(char) {
            // replace all instances of the char in the in_progress_word
            for (i, c) in self.word.chars().enumerate() {
                if c == char {
                    self.in_progress_word
                        .replace_range(i..i + 1, char.to_string().as_str());
                    out = true;
                }
            }
        }
        // update the guessable character
        self.guessable_characters
            .iter_mut()
            .find(|gc| gc.0 == char)
            .map(|gc| gc.1 = true);
        // increment the guess count if the character was not in the word
        if !out {
            self.guess_count += 1;
        }
        Some(out)
    }
    /// returns Some if game is over, true on win and false on loss, None if game is not over
    pub fn get_game_state(&self) -> Option<bool> {
        if self.guess_count >= self.difficulty.0.len() as i32 {
            return Some(false);
        } else if self.in_progress_word == self.word {
            return Some(true);
        }
        None
    }
}
impl Default for Game {
    fn default() -> Self {
        Game {
            guessable_characters: GuessableChar::new_set(ENGLISH_ALPHABET),
            guess_count: 0,
            in_progress_word: "".to_string(),
            word: "".to_string(),
            difficulty: DifficultyLevel::get_easiest(),
        }
    }
}
