# Advent of Code 2025 - Development Tasks
# https://github.com/casey/just

# Default recipe lists all available commands
default:
    @just --list

# Initialize environment (install rust nightly if needed)
init:
    @echo "Initializing development environment..."
    rustup toolchain install nightly
    rustup override set nightly
    @echo "✓ Environment ready"

# Run all days
run:
    cargo run --release

# Run specific day
run-day day:
    cargo run --release -- --day {{day}}

# Run all implementations for all days
run-all-impls:
    cargo run --release -- --all-impls

# Run all implementations for specific day
run-all-impls-day day:
    cargo run --release -- --day {{day}} --all-impls

# Run tests
test:
    cargo test

# Run tests for specific day
test-day day:
    cargo test --lib day{{day}}

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run benchmarks
bench:
    cargo +nightly bench

# Run benchmarks for specific day
bench-day day:
    cargo +nightly bench day{{day}}

# Generate documentation
doc:
    cargo doc --open --no-deps

# Check code without building
check:
    cargo check

# Format code
fmt:
    cargo fmt

# Check if code is formatted
fmt-check:
    cargo fmt -- --check

# Lint code
lint:
    cargo clippy -- -D warnings

# Lint and suggest fixes
lint-fix:
    cargo clippy --fix

# Create new day (requires day number)
new-day day:
    @echo "Creating day {{day}}..."
    @./scripts/new_day.sh {{day}}

# Clean build artifacts
clean:
    cargo clean

# Full CI check (fmt, clippy, test, build)
ci: fmt-check lint test
    cargo build --release
    @echo "✓ All CI checks passed"

# Watch and run tests on file changes (requires cargo-watch)
watch:
    cargo watch -x test

# Watch and run specific day on file changes (requires cargo-watch)
watch-day day:
    cargo watch -x "test --lib day{{day}}"

# Build in release mode
build:
    cargo build --release

# Build in debug mode
build-debug:
    cargo build

# Show project statistics (requires tokei)
stats:
    tokei

# Update dependencies
update:
    cargo update

# Check for outdated dependencies (requires cargo-outdated)
outdated:
    cargo outdated
