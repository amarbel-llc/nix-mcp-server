# Build the server
build:
    nix build .#default

# Run the server directly
run:
    nix run .#default

# Development build with cargo
dev:
    cargo build

# Watch for changes and rebuild
watch:
    cargo watch -x build

# Run tests
test:
    cargo test

# Format code
fmt:
    cargo fmt

# Check code
check:
    cargo check
    cargo clippy

# Clean build artifacts
clean:
    cargo clean
