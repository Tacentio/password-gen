use password_gen::{self, CharSet, PasswordGenerator, PasswordOptions};

fn main() {
    let mut generator = PasswordGenerator::new();
    let options = PasswordOptions::new(15, CharSet::Ascii, true, false);
    println!("{}", generator.generate_password(&options));
}
