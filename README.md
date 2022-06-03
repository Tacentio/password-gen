# password-gen

A rust library for generating passwords.

## Features

- Random [Xkcd-esque](https://xkcd.com/936/) passwords. Uses [this](https://github.com/dwyl/english-words/blob/master/words_alpha.txt) wordlist.
- Random ASCII (extended) passwords.
- Random PIN numbers.
- Random Alphanumeric passwords.

## Usage

Add this to your `Cargo.toml`

```toml
[dependencies]
password-gen = "<latest-version>
```

To get started, see [the api documentation](https://docs.rs/password-gen/latest/password_gen/)

## Character Set

Supports different character sets each useful in their own right.

| CharSet       | Example                              |
| ------------- | ------------------------------------ |
| Xkcd          | `underrogue/orchesography/ARAMITESS` |
| Ascii         | `yB"hF<%\H`                          |
| AsciiExtended | `ÝýIêø¹Ü¹fBDI2î1Ù`                   |
| Alphanumeric  | `bpM2vqmGRfzaSmm5`                   |
| Numbers       | `9837`                               |

## Feature Flags

- `cereal` - Enable `serde` `Serialization` and `Deserialization` support on `PasswordOptions`
