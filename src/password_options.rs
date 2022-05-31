use serde::{Deserialize, Serialize};
/// Describes the set of characters or words
/// that will be used to generate a password.
#[derive(Serialize, Deserialize)]
pub enum CharSet {
    Ascii,
    AsciiExtended,
    Unicode,
    Xkcd,
}

/// Describes the options that are passed into the
/// generate_password function.
#[derive(Serialize, Deserialize)]
pub struct PasswordOptions {
    /// How many characters in the password. Or, how many words in the password.
    pub length: u32,
    /// The set of characters or words to randmoly select from.
    pub character_set: CharSet,
    /// Should the password contain only alphanumeric characters.
    pub alphanumeric_only: bool,
    /// Toggle whitespace. Generally better to set to false.
    pub include_whitespace: bool,
}

impl PasswordOptions {
    /// Create a set of options for generating a password.
    pub fn new(
        length: u32,
        character_set: CharSet,
        alphanumeric_only: bool,
        include_whitespace: bool,
    ) -> PasswordOptions {
        PasswordOptions {
            length,
            character_set,
            alphanumeric_only,
            include_whitespace,
        }
    }
}
