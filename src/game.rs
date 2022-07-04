const ENGLISH_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
enum HangmanDrawingElements {
    Base = 1,
    VerticalBeam = 2,
    Horizontalbeam = 3,
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
/// the drawing element order and the different difficulty versions
pub struct DifficultyLevel(Vec<HangmanDrawingElements>, &'static str);
impl DifficultyLevel {
    pub fn get_easiest() -> DifficultyLevel {
        DifficultyLevel(
            vec![
                HangmanDrawingElements::Base,
                HangmanDrawingElements::VerticalBeam,
                HangmanDrawingElements::Horizontalbeam,
                HangmanDrawingElements::SupportBeam,
                HangmanDrawingElements::Rope,
                HangmanDrawingElements::Head,
                HangmanDrawingElements::Torso,
                HangmanDrawingElements::LeftArm,
                HangmanDrawingElements::RightArm,
                HangmanDrawingElements::LeftLeg,
                HangmanDrawingElements::RightLeg,
                HangmanDrawingElements::LeftEye,
                HangmanDrawingElements::RightEye,
                HangmanDrawingElements::Mouth,
            ],
            "Easiest",
        )
    }
    pub fn get_normal() -> DifficultyLevel {
        DifficultyLevel(
            vec![
                HangmanDrawingElements::Base,
                HangmanDrawingElements::VerticalBeam,
                HangmanDrawingElements::Horizontalbeam,
                HangmanDrawingElements::SupportBeam,
                HangmanDrawingElements::Rope,
                HangmanDrawingElements::Head,
                HangmanDrawingElements::Torso,
                HangmanDrawingElements::LeftArm,
                HangmanDrawingElements::RightArm,
                HangmanDrawingElements::LeftLeg,
                HangmanDrawingElements::RightLeg,
                HangmanDrawingElements::LeftEye,
                HangmanDrawingElements::RightEye,
                HangmanDrawingElements::Mouth,
            ],
            "Normal",
        )
    }
    pub fn get_hard() -> DifficultyLevel {
        DifficultyLevel(
            vec![
                HangmanDrawingElements::Base,
                HangmanDrawingElements::VerticalBeam,
                HangmanDrawingElements::Horizontalbeam,
                HangmanDrawingElements::Rope,
                HangmanDrawingElements::Head,
                HangmanDrawingElements::Torso,
                HangmanDrawingElements::LeftArm,
                HangmanDrawingElements::RightArm,
                HangmanDrawingElements::LeftLeg,
                HangmanDrawingElements::RightLeg,
            ],
            "Hard",
        )
    }
    pub fn get_hardest() -> DifficultyLevel {
        DifficultyLevel(
            vec![
                HangmanDrawingElements::VerticalBeam,
                HangmanDrawingElements::Horizontalbeam,
                HangmanDrawingElements::Rope,
                HangmanDrawingElements::Head,
                HangmanDrawingElements::Torso,
                HangmanDrawingElements::LeftArm,
                HangmanDrawingElements::RightArm,
                HangmanDrawingElements::LeftLeg,
                HangmanDrawingElements::RightLeg,
            ],
            "Hardest",
        )
    }
}

/// (the character, has it been guessed?)
pub struct GuessableChar(char, bool);
impl GuessableChar {
    pub fn new(character_set: &str) -> Vec<GuessableChar> {
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
    pub fn new(mut word: String) -> Result<Game, ()> {
        if word.len() < 1 {
            return Err(());
        }
        word = word.to_lowercase();
        for c in word.chars() {
            if !ENGLISH_ALPHABET.contains(c) {
                return Err(());
            }
        }
        // TODO: check if word is in the dictionary
        Ok(Game {
            guessable_characters: GuessableChar::new(ENGLISH_ALPHABET),
            guess_count: 0,
            in_progress_word: "_".repeat((&word).len()),
            word,
            difficulty: DifficultyLevel::get_normal(),
        })
    }
    /// checks if the character is in the word and fill it in, returns true if it was in the word
    pub fn guess(&mut self, char: GuessableChar) -> bool {
        let mut out = false;
        // check if char is in the word
        if self.word.contains(char.0) {
            // replace all instances of the char in the in_progress_word
            for (i, c) in self.word.chars().enumerate() {
                if c == char.0 {
                    self.in_progress_word
                        .replace_range(i..i + 1, char.0.to_string().as_str());
                    out = true;
                }
            }
        }
        self.guess_count += 1;
        out
    }
    /// like guess but does not guess if the character has already been guessed
    pub fn safe_guess(&mut self, char: GuessableChar) -> Option<bool> {
        if !char.1 {
            return None;
        }
        Some(self.guess(char))
    }
    /// returns Some if game is over, true on win and false on loss, None if game is not over
    pub fn get_game_state(&self) -> Option<bool> {
        if self.guess_count >= self.difficulty.0.len() as i32 {
            return Some(true);
        } else if self.in_progress_word == self.word {
            return Some(false);
        }
        None
    }
}
