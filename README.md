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

## Character Set

Supports different character sets each useful in their own right.

| CharSet   | Example                            | Use                                                                                           |
| --------- | ---------------------------------- | --------------------------------------------------------------------------------------------- |
| Unicode   | 򔩠 𥠔 𖔾񻲺񖠭󲂰 𭝔 򳾄󀪤򱉀򼎈󟸂򃼾񑾶񿾿󕁦             | - Passwords you never need to remember or type and the service can deal with any utf-8 string |
| Xkcd      | underrogue/orchesography/ARAMITESS | - Passwords you need to remember or type                                                      |
| Ascii     | bpM2vqmGRfzaSmm5                   | - Password for services that can't deal with special characters                               |
| Ascii Ext | ÝýIêø¹Ü¹fBDI2î1Ù                   | - Extra entroy compared to Ascii but might not be accepted by some services                   |
