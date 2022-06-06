use crate::dictionary::Dictionary;
use crate::{CharSet, PasswordOptions};
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Describes a password generator. Wrapper around a ThreadRng so it's
/// cached to improve performance when generating random characters.
/// Dictionary also loaded when instantiated so it doesn't have to be called
/// multiple times.
pub struct PasswordGenerator {
    rng: ThreadRng,
    dictionary: Dictionary,
}

impl PasswordGenerator {
    /// Initialises a new PasswordGenerator, instantiating the
    /// ThreadRng and dictionary
    ///
    /// ## Example
    /// ```
    /// use password_gen::PasswordGenerator;
    /// let generator = PasswordGenerator::new();
    /// ```
    pub fn new() -> PasswordGenerator {
        PasswordGenerator {
            rng: thread_rng(),
            dictionary: Dictionary::new(),
        }
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
    /// let options = PasswordOptions::new(15, CharSet::Ascii);
    /// let password = generator.generate_password(&options);
    /// ```
    pub fn generate_password(&mut self, options: &PasswordOptions) -> String {
        let mut password = String::new();
        match &options.character_set {
            CharSet::Xkcd => {
                let separator = *select_random_value(self.dictionary.separators(), &mut self.rng);
                for i in 0..options.min_length as usize {
                    let word = *select_random_value(&self.dictionary.words(), &mut self.rng);
                    if rand::random() {
                        password.push_str(&word.to_uppercase());
                    } else {
                        password.push_str(&word);
                    }
                    if i < (options.min_length as usize) - 1 {
                        password.push_str(&separator);
                    }
                }
            }
            _ => {
                for _i in 0..options.min_length {
                    password.push_str(*select_random_value(
                        self.dictionary.list_from_charset(&options.character_set),
                        &mut self.rng,
                    ));
                }
            }
        }
        return password;
    }
}

/// Selects a random value from &Vec<T> and returns &T.
///
/// ## Example
/// ```
/// use rand::{thread_rng, Rng};
/// use password_gen::password_generator::select_random_value;
///
/// let t: Vec<u32> = vec![1,2,3,4,5];
/// let mut rng = thread_rng();
/// let a:&u32 = select_random_value(&t, &mut rng);
/// ```
pub fn select_random_value<'a, T>(c: &'a [T], rng: &mut ThreadRng) -> &'a T {
    let val = c.choose(rng).unwrap();
    return val;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CharSet;

    #[test]
    fn correct_length() {
        let mut generator = PasswordGenerator::new();
        let options = PasswordOptions::new(12, CharSet::Ascii);
        let options2 = PasswordOptions::new(13, CharSet::AsciiExtended);
        let options3 = PasswordOptions::new(14, CharSet::Numbers);
        let options4 = PasswordOptions::new(15, CharSet::Alphanumeric);
        let password = generator.generate_password(&options);
        let password2 = generator.generate_password(&options2);
        let password3 = generator.generate_password(&options3);
        let password4 = generator.generate_password(&options4);
        assert_eq!(password.chars().count(), 12);
        assert_eq!(password2.chars().count(), 13);
        assert_eq!(password3.chars().count(), 14);
        assert_eq!(password4.chars().count(), 15);
    }

    #[test]
    fn it_generates_an_xkcd_password() {
        let mut generator = PasswordGenerator::new();
        let options = PasswordOptions::new(3, CharSet::Xkcd);
        let password = generator.generate_password(&options);
        assert!(password.chars().count() > 3 as usize);
    }

    #[test]
    fn it_generates_pin() {
        let options = PasswordOptions::new(4, CharSet::Numbers);
        let mut generator = PasswordGenerator::new();
        let password = generator.generate_password(&options);
        for c in password.chars() {
            assert!(c.is_numeric())
        }
    }

    #[test]
    fn it_generates_alphanumeric() {
        let options = PasswordOptions::new(10, CharSet::Alphanumeric);
        let mut generator = PasswordGenerator::new();
        let password = generator.generate_password(&options);
        for c in password.chars() {
            assert!(c.is_alphanumeric())
        }
    }
}
