#[cfg(feature = "cereal")]
extern crate serde;

#[cfg(feature = "cereal")]
use serde::{Deserialize, Serialize};
/// Describes the set of characters or words
/// that will be used to generate a password.
#[cfg_attr(feature = "cereal", derive(Serialize, Deserialize))]
pub enum CharSet {
    Ascii,
    AsciiExtended,
    Xkcd,
    Numbers,
    Alphanumeric,
}

/// Describes the options that are passed into the
/// generate_password function.
#[cfg_attr(feature = "cereal", derive(Serialize, Deserialize))]
pub struct PasswordOptions {
    /// How many characters in the password. Or, how many words in the password. 
    pub min_length: u32,
    /// The set of characters or words to randmoly select from.
    pub character_set: CharSet,
}

impl PasswordOptions {
    /// Create a set of options for generating a password.
    ///
    /// ## Example
    ///
    /// ```
    /// use password_gen::{PasswordOptions, PasswordGenerator};
    /// use password_gen::password_options::CharSet;
    /// let mut generator = PasswordGenerator::new();
    /// let options = PasswordOptions::new(15, CharSet::Ascii);
    /// let password = generator.generate_password(&options);
    /// ```
    ///
    pub fn new(min_length: u32, character_set: CharSet) -> PasswordOptions {
        PasswordOptions {
            min_length,
            character_set,
        }
    }
}
