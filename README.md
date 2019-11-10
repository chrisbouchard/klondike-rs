# klondike-rs

[![Crates.io][crate-badge]][crate]
[![CircleCI][ci-badge]][ci]

CLI Klondike Solitaire written in Rust

![Default State](https://i.imgur.com/QMd6Gbw.png)
![Gameplay](https://i.imgur.com/m6gs2F1.png)

[ci-badge]: https://circleci.com/gh/chrisbouchard/klondike-rs.svg?style=shield
[ci]: https://circleci.com/gh/chrisbouchard/klondike-rs
[crate-badge]: https://img.shields.io/crates/v/klondike-rs
[crate]: https://crates.io/crates/klondike-rs


## Installing from Crates.io

The simplest way to run `klondike-rs` is to install it from [crates.io][crate]
using Cargo.

```
$ cargo install klondike-rs
```

Cargo will have to download and compile all necessary dependencies, as well as
compile this project's source, so it may take a few moments.

Then, assuming Cargo is set up on your path, you can run it using

```
$ klondike-rs
```

If you don't have Rust (or Cargo, its build tool), you can get it using
[Rustup][rustup]. I will try to keep the project compiling on stable Rust, but
I reserve the right to require nightly if there's a really nice feature I want.
:)

[rustup]: https://rustup.rs/


### Compiling from Source

Technically installing from crates.io _is_ compiling from source, but if you
would like to check out the code and mess around with it, you can do so using

```
$ git clone https://github.com/chrisbouchard/klondike-rs.git
  . . .
$ cd klondike-rs
$ cargo run  
```

To find out more about Cargo, you can check out [The Cargo Book][cargo-book].

[cargo-book]: https://doc.rust-lang.org/cargo/index.html


## Controls

You can always type `h` or `?` to get the help screen!

_To be written&hellip;_


## Configuration

Configuration uses [TOML](toml), a popular mark-up language among Rust
projects. Below is a sample configuration file which sets all settings to their
default values, so you'd only need to include a key if you wanted to change its
value.

```toml
[display]

# Whether to use ANSI color escapes
# TODO: Currently ignored
color = true

# Whether to use Unicode box-drawing characters
# TODO: Currently ignored
unicode = true


[game]

# How many cards to draw (usually 3 or 1)
# TODO: No bounds checking, so don't break things
draw_from_stock_len = 3

# Whether it's permitted to move a card out of a foundation
take_from_foundation = true
```

Configuration files are picked up from several locations, depending on your OS.

* `$HOME/.klondike-rs.toml` &mdash; _Any OS_
    * E.g., `/home/chris/.klondkie-rs.toml`
* `$XDG_CONFIG_HOME/klondike-rs/config.toml` &mdash; _Linux only_
    * E.g., `/home/chris/.config/klondkie-rs/config.toml`
* `$HOME/Library/Preferences/net.upliftinglemma.klondike-rs/config.toml` &mdash; _MacOS only_
    * E.g., `/Users/Chris/Library/Preferences/net.upliftinglemma.klondike-rs/config.toml`
* `{FOLDERID_RoamingAppData}\upliftinglemma\klondike-rs\config\config.toml` &mdash; _Windows only_
    * E.g., `C:\Users\Chris\AppData\Roaming\upliftinglemma\klondike-rs\config\config.toml`

[toml]: https://github.com/toml-lang/toml


## TODO:

This project is still _very_ alpha, but it's at least runnable. Some high-level
things that still need to happen:

* Respect configuration regarding color and Unicode.
* Document all public structs and functions and re-enable `warn(missing_docs)`.
* Offer more cosmetic configuration, e.g., card backs.


## Contributing

_To be written&hellip;_

