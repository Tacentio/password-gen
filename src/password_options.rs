use std::error::Error;
use std::fmt;
use std::str::FromStr;

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

#[derive(Debug)]
pub struct ParseCharSetError {}
impl fmt::Display for ParseCharSetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "String did not match ascii, asciiextended, xkcd, numbers or alphanumeric"
        )
    }
}

impl Error for ParseCharSetError {}

impl FromStr for CharSet {
    type Err = ParseCharSetError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ascii" => Ok(CharSet::Ascii),
            "asciiextended" => Ok(CharSet::AsciiExtended),
            "xkcd" => Ok(CharSet::Xkcd),
            "numbers" => Ok(CharSet::Numbers),
            "alphanumeric" => Ok(CharSet::Alphanumeric),
            _ => Err(ParseCharSetError {}),
        }
    }
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
