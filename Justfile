default:
    just --list

run *args:
    cargo run -- {{args}}

run-release *args:
    cargo run --release -- {{args}}

read:
    cargo doc --open
    mdbook build --open docs

alias r := run
alias rr := run-release

