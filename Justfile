default: build

build:
    cargo build -p rlax-launcher

release:
    cargo build -p rlax-launcher --release
    strip target/release/rlax-launcher
    ls -lh target/release/rlax-launcher

run:
    cargo run -p rlax-launcher

check:
    cargo check -p rlax-launcher

test:
    cargo test -p rlax-security -- --nocapture

audit:
    cargo audit || echo "install: cargo install cargo-audit"

fmt:
    cargo fmt --all

clippy:
    cargo clippy -p rlax-launcher -- -W clippy::pedantic

clean:
    cargo clean

install:
    cargo build -p rlax-launcher --release
    sudo install -Dm755 target/release/rlax-launcher /usr/local/bin/rlax-launcher
    @echo "Installed to /usr/local/bin/rlax-launcher"
    @echo "Add to hyprland.conf: bind = SUPER, SPACE, exec, rlax-launcher"
