target-dir := env("CARGO_TARGET_DIR", "target")
clippy-dir := target-dir + "/clippy"
clippy-build := clippy-dir + "/build"
clippy-target := clippy-dir + "/target"

dev-args := "-F bevy/dynamic_linking -F bevy/bevy_dev_tools"

default:
    @just --choose

# Move to own build/target dir to not block compiles
check:
    CARGO_TARGET_DIR={{ clippy-dir }} \
    CARGO_BUILD_BUILD_DIR={{ clippy-build }} \
    CARGO_BUILD_TARGET_DIR={{ clippy-target }} \
    cargo clippy {{ dev-args }} --message-format=json-diagnostic-rendered-ansi

build:
    cargo build {{ dev-args }}

run:
    cargo run {{ dev-args }}

build-release:
    cargo build --release

run-release:
    cargo run --release

clean:
    cargo clean

build-clean: clean build
