default:
    just --list

run *args:
    cargo run -- {{args}}

run-release *args:
    cargo run --release -- {{args}}

alias r := run
alias rr := run-release

