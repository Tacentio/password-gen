use crate::dictionary;
use crate::{CharSet, PasswordOptions};
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

/// Describes a password generator. Wrapper around a ThreadRng so it's
/// cached to improve performance when generating random characters.
pub struct PasswordGenerator {
    rng: ThreadRng,
}

impl PasswordGenerator {
    /// Initialises a new PasswordGenerator, instantiating the
    /// ThreadRng.
    ///
    /// ## Example
    /// ```
    /// use password_gen::PasswordGenerator;
    /// let generator = PasswordGenerator::new();
    /// ```
    pub fn new() -> PasswordGenerator {
        PasswordGenerator { rng: thread_rng() }
    }
}

impl PasswordGenerator {
    /// Generates a random password that match the given requirements.
    ///
    /// ## Example
    /// ```
    /// use password_gen::{PasswordOptions, PasswordGenerator};
    /// use password_gen::password_options::CharSet;
    /// let mut generator = PasswordGenerator::new();
    /// let options = PasswordOptions::new(15, CharSet::Ascii, true, true);
    /// let password = generator.generate_password(&options);
    /// ```
    pub fn generate_password(&mut self, options: &PasswordOptions) -> String {
        let mut chars: Vec<char> = Vec::new();
        match options.character_set {
            CharSet::Ascii => {
                while chars.len() < options.length as usize {
                    let c = self.generate_ascii_char();
                    if validate_char(&c, options) {
                        chars.push(c);
                    }
                }
            }
            CharSet::AsciiExtended => {
                while chars.len() < options.length as usize {
                    let c = self.generate_ascii_extended_char();
                    if validate_char(&c, options) {
                        chars.push(c);
                    }
                }
            }
            CharSet::Unicode => {
                while chars.len() < options.length as usize {
                    let c = self.generate_unicode_char();
                    if validate_char(&c, options) {
                        chars.push(c);
                    }
                }
            }
            CharSet::Xkcd => {
                let dict = dictionary::get_dictionary();
                let separators = dictionary::get_separators();
                let separator = self.generate_random_word(&separators);
                let mut password = String::new();
                for i in 0..options.length as usize {
                    let word = self.generate_random_word(&dict);
                    if rand::random() {
                        password.push_str(&word.to_uppercase());
                    } else {
                        password.push_str(&word);
                    }
                    if i < (options.length as usize) - 1 {
                        password.push_str(&separator);
                    }
                }
                return password;
            }
        }
        return String::from_iter(chars);
    }

    /// Generates a random ASCII extended character excluding control
    /// characters.
    fn generate_ascii_extended_char(&mut self) -> char {
        let x: u8 = self.rng.gen();
        let c = char::from(x);
        if c.is_control() {
            return self.generate_ascii_extended_char();
        }
        return c;
    }

    /// Generates a random ASCII character excluding control
    /// characters
    fn generate_ascii_char(&mut self) -> char {
        let x: u8 = self.rng.gen_range(0..127);
        let c = char::from(x);
        if c.is_control() {
            return self.generate_ascii_char();
        }
        return c;
    }

    /// Generates a random unicode characters excluding control
    /// characters.
    fn generate_unicode_char(&mut self) -> char {
        let c: char = rand::random();
        if c.is_control() {
            return self.generate_unicode_char();
        }
        return c;
    }

    /// Given a vector of words, randomly selects a word from the list.
    fn generate_random_word(&mut self, dict: &Vec<&str>) -> String {
        //println!("{}", dict.choose(&mut self.rng).unwrap());
        let word: &str = *dict.choose(&mut self.rng).unwrap();
        return String::from(word);
    }
}

/// Validate a given character with the given PasswordOptions
pub fn validate_char(c: &char, options: &PasswordOptions) -> bool {
    if (!options.include_whitespace && c.is_whitespace())
        || (options.alphanumeric_only && !c.is_alphanumeric())
    {
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_ascii_char_generates_ascii() {
        let mut generator = PasswordGenerator::new();
        let c = generator.generate_ascii_char();
        assert!(c.is_ascii());
        assert!(!c.is_control());
        assert!((c as u8) < 128);
    }

    #[test]
    fn generate_ascii_extended_char_generates_ascii() {
        let mut generator = PasswordGenerator::new();
        let c = generator.generate_ascii_char();
        assert!(c.is_ascii());
        assert!(!c.is_control());
    }

    #[test]
    fn generate_password_returns_correct_length() {
        let mut generator = PasswordGenerator::new();
        let options = PasswordOptions::new(10, CharSet::Ascii, false, true);
        let options2 = PasswordOptions::new(11, CharSet::AsciiExtended, false, true);
        let options3 = PasswordOptions::new(12, CharSet::Unicode, false, true);
        let password = generator.generate_password(&options);
        let password2 = generator.generate_password(&options2);
        let password3 = generator.generate_password(&options3);
        assert_eq!(password.chars().count(), 10);
        assert_eq!(password2.chars().count(), 11);
        assert_eq!(password3.chars().count(), 12);
    }

    #[test]
    fn it_generates_an_xkcd_password() {
        let mut generator = PasswordGenerator::new();
        let options = PasswordOptions::new(3, CharSet::Xkcd, false, false);
        let password = generator.generate_password(&options);
        assert!(password.chars().count() > 3 as usize);
    }

    #[test]
    fn chars_validate_correctly() {
        let options = PasswordOptions::new(10, CharSet::Ascii, true, false);
        let options_2 = PasswordOptions::new(10, CharSet::Ascii, false, true);
        let space = ' ';
        let non_alpha = '@';
        let number = '1';
        let letter = 'a';
        assert_eq!(validate_char(&space, &options), false);
        assert_eq!(validate_char(&space, &options_2), true);
        assert_eq!(validate_char(&non_alpha, &options), false);
        assert_eq!(validate_char(&non_alpha, &options_2), true);
        assert_eq!(validate_char(&number, &options), true);
        assert_eq!(validate_char(&number, &options_2), true);
        assert_eq!(validate_char(&letter, &options), true);
        assert_eq!(validate_char(&letter, &options_2), true);
    }
}
