default:
    @just --choose

check:
    cargo clippy -F bevy/dynamic_linking

dev:
    cargo build -F bevy/dynamic_linking

run:
    cargo run -F bevy/dynamic_linking

clean:
    cargo clean

dev-clean: clean dev
