# Delve

Delve is a minecraft clone written entirely in Rust.  Its purpose is to allow me
to experiment with game engine and game design code without worrying too much
about graphics.

## Prerequisites

As well as Rust & Cargo (head over [here](http://rustup.rs) to install it), I
use a couple of tools to help with building and documentation.  These tools will
need to be installed:

| Tool | How to install | Description |
|-|-|
| Just | `cargo install just` | Task runner to help build and read docs |
| mdbook-mermaid | `cargo install mdbook-mermaid` | Allows cool diagrams inside the docs |

## Documentation

You can read the documentation by typing the command `just read`.  This will
build the code documentation and the Mdbook and open up tabs in your default
browser.  The Mdbook will provide much information on how the game works and
even the dev-diary.

## Building and running

You build and run the game in the normal way:

```
$ cargo run --release
```

Don't you love Rust tooling?
