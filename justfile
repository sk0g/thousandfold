target-dir := env("CARGO_TARGET_DIR", "target")
clippy-dir := target-dir + "/clippy"
clippy-build := clippy-dir + "/build"
clippy-target := clippy-dir + "/target"

default:
    @just --choose

# Move to own build/target dir to not block compiles
check:
    CARGO_TARGET_DIR={{ clippy-dir }} \
    CARGO_BUILD_BUILD_DIR={{ clippy-build }} \
    CARGO_BUILD_TARGET_DIR={{ clippy-target }} \
    cargo clippy -F bevy/dynamic_linking --message-format=json-diagnostic-rendered-ansi

dev:
    cargo build -F bevy/dynamic_linking

run:
    cargo run -F bevy/dynamic_linking

release:
    cargo build --release

run-release:
    cargo run --release

clean:
    cargo clean

dev-clean: clean dev
