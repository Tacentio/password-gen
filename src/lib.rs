/*!
Secure password generation

## Example
```
use password_gen::{PasswordOptions, PasswordGenerator};
use password_gen::password_options::CharSet;
let mut generator = PasswordGenerator::new();
let options = PasswordOptions::new(15, CharSet::Ascii, true, true);
let password = generator.generate_password(&options);
```
*/

pub use crate::password_generator::PasswordGenerator;
pub use crate::password_options::{CharSet, PasswordOptions};

pub mod dictionary;
pub mod password_generator;
pub mod password_options;
