# password-gen

A rust library for generating passwords.

## Features

- Random unicode passwords.
- Random [Xkcd-esque](https://xkcd.com/936/) passwords. Uses [this](https://github.com/dwyl/english-words/blob/master/words_alpha.txt) wordlist.
- Random ASCII (extended) passwords.

## Usage

Add this to your `Cargo.toml`

```toml
[dependencies]
password-gen = "<latest-version>
```

To get started, see [the api documentation](https://docs.rs/password-gen/latest/password_gen/)
