use password_gen::{PasswordGenerator, PasswordOptions, CharSet};

fn main() {
    let mut generator = PasswordGenerator::new();
    let options = PasswordOptions::new(10, CharSet::Ascii);
    let password = generator.generate_password(&options);
    println!("{password}");
}
