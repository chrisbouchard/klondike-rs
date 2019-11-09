# klondike-rs

[![Crates.io][crate-badge]][crate]

CLI Klondike Solitaire written in Rust

![Default State](https://i.imgur.com/QMd6Gbw.png)
![Gameplay](https://i.imgur.com/m6gs2F1.png)

[crate-badge]: https://img.shields.io/crates/v/klondike-rs
[crate]: https://crates.io/crates/klondike-rs


## Running

This project is built using standard `cargo` commands. To run it, check out the
repository and run

```
$ cargo run
```

Before it can run, `cargo` will have to download and compile all necessary
dependencies, as well as compile this project's source, so it may take a few
moments.

If you don't have Rust, you can get it using [Rustup][rustup]. I will try to
keep the project compiling on stable Rust, but I reserve the right to require
nightly if there's a really nice feature I want. :)

[rustup]: https://rustup.rs/


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

Configuration files are picked up from several locations, depending on your OS:

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

