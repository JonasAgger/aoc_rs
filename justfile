# use PowerShell instead of sh:
set windows-shell := ["powershell.exe", "-c"]

alias r := run
alias c := create
alias b := bench
alias rr := run-release
alias br := bench-release
alias rt := run-test
alias f := format

default:
  @just --list

clear:
    clear

format: 
    cargo fmt

build: clear
    cargo build

run DAY="": clear
    cargo run -- run -y 2018 {{DAY}}

run-release DAY="": clear
    cargo run --release -- run -y 2018 {{DAY}}

run-test DAY="": clear 
    cargo run -- run -y 2018 --test {{DAY}}

create DAY="": clear
    cargo run -- create -y 2018 {{DAY}}

bench DAY="": clear
    cargo run -- bench -y 2018 {{DAY}}

bench-release DAY="": clear
    cargo run --release -- bench -y 2018 {{DAY}}