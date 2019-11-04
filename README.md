# klondike-rs

CLI Klondike Solitaire written in Rust

![Default State](img/default.png?raw=true "Default State")
![Gameplay](img/gameplay.png?raw=true "Gameplay")


## Running

This project is built using standard `cargo` commands. To run it, check out the
repository and run

```
$ cargo run
```

Before it can run, `cargo` will have to download and compile all necessary
dependencies, as well as compile this project's source, so it may take a few
moments.


### Controls

You can always type `h` or `?` to get the help screen!

_To be written&hellip;_


### Configuration

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

