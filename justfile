# use PowerShell instead of sh:
set windows-shell := ["powershell.exe", "-c"]

alias r := run
alias c := create
alias b := bench
alias tt := test
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
    cargo +nightly build

run DAY="": clear
    cargo +nightly run -- run {{DAY}}

run-release DAY="": clear
    cargo +nightly run --release -- run {{DAY}}

run-test DAY="": clear 
    cargo +nightly run -- run --test {{DAY}}

create DAY="": clear
    cargo +nightly run -- create {{DAY}}

bench DAY="": clear
    cargo +nightly run -- bench {{DAY}}

bench-release DAY="": clear
    cargo +nightly run --release -- bench {{DAY}}

bench-all: clear
    cargo +nightly run --release -- bench-all

test DAY="": clear
    cargo +nightly test -- --show-output {{DAY}}
    # cargo +nightly test -- --show-output --nocapture {{DAY}}