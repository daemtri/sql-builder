set windows-shell := ["cmd.exe", "/c"]

alias c := check
alias b := build
alias t := test

alias ss:= stable
alias sb:= beta
alias sn:= nightly

alias up:= update
alias ur:= update-rust

clear:
    cls

check: clear
    cargo check

build: clear
    cargo build

test: clear
    cargo test --all

stable: clear
    rustup default stable

beta: clear
    rustup default beta

nightly: clear
    rustup default nightly

update: clear
    cargo update

update-rust: clear
    rustup update
