.PHONY: help build run test clean fmt lint check doc watch release debug install uninstall build-release

# Default target
help:
	@echo "NutriGo Rust - Available Make Commands:"
	@echo ""
	@echo "Build & Run:"
	@echo "  make build          - Build the project in debug mode"
	@echo "  make build-release  - Build the project in release mode"
	@echo "  make run            - Build and run the project"
	@echo "  make release        - Build and run in release mode"
	@echo ""
	@echo "Testing & Quality:"
	@echo "  make test           - Run all tests"
	@echo "  make test-verbose   - Run tests with output"
	@echo "  make fmt            - Format code with rustfmt"
	@echo "  make fmt-check      - Check code formatting without modifying"
	@echo "  make lint           - Run clippy for linting"
	@echo "  make check          - Check code without building"
	@echo ""
	@echo "Documentation & Dependencies:"
	@echo "  make doc            - Generate documentation"
	@echo "  make doc-open       - Generate and open documentation"
	@echo "  make update         - Update dependencies"
	@echo "  make outdated       - Check for outdated dependencies"
	@echo ""
	@echo "Development:"
	@echo "  make watch          - Watch and rebuild on file changes"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make install        - Install the binary locally"
	@echo "  make uninstall      - Uninstall the binary"
	@echo "  make size           - Check binary size"
	@echo "  make deps           - Show dependency tree"
	@echo ""
	@echo "Database:"
	@echo "  make db-create      - Create database"
	@echo "  make db-drop        - Drop database"
	@echo "  make db-migrate     - Run migrations"
	@echo ""
	@echo "Debugging:"
	@echo "  make debug          - Run with debug logging (RUST_LOG=debug)"
	@echo "  make debug-verbose  - Run with trace logging (RUST_LOG=trace)"
	@echo "  make bench          - Run benchmarks"

# Build targets
build:
	@echo "Building project in debug mode..."
	cargo build

build-release:
	@echo "Building project in release mode..."
	cargo build --release

release: build-release
	@echo "Running release build..."
	./target/release/nutriGo-Rust

run: build
	@echo "Running debug build..."
	./target/debug/nutriGo-Rust

# Testing targets
test:
	@echo "Running tests..."
	cargo test

test-verbose:
	@echo "Running tests with output..."
	cargo test -- --nocapture

# Code quality targets
fmt:
	@echo "Formatting code..."
	cargo fmt

fmt-check:
	@echo "Checking code formatting..."
	cargo fmt -- --check

lint:
	@echo "Running clippy linter..."
	cargo clippy -- -D warnings

check:
	@echo "Checking code without building..."
	cargo check

# Documentation targets
doc:
	@echo "Generating documentation..."
	cargo doc --no-deps

doc-open:
	@echo "Generating and opening documentation..."
	cargo doc --no-deps --open

# Dependency targets
update:
	@echo "Updating dependencies..."
	cargo update

outdated:
	@echo "Checking for outdated dependencies..."
	cargo outdated

deps:
	@echo "Showing dependency tree..."
	cargo tree

# Development targets
watch:
	@echo "Watching for changes and rebuilding..."
	cargo watch -x check -x build

clean:
	@echo "Cleaning build artifacts..."
	cargo clean

install:
	@echo "Installing binary..."
	cargo install --path .

uninstall:
	@echo "Uninstalling binary..."
	cargo uninstall nutriGo-Rust

size:
	@echo "Checking binary size..."
	@ls -lh target/release/nutriGo-Rust 2>/dev/null || echo "Build with 'make build-release' first"

# Debugging targets
debug:
	@echo "Running with debug logging (RUST_LOG=debug)..."
	RUST_LOG=debug ./target/debug/nutriGo-Rust

debug-verbose:
	@echo "Running with trace logging (RUST_LOG=trace)..."
	RUST_LOG=trace ./target/debug/nutriGo-Rust

bench:
	@echo "Running benchmarks..."
	cargo bench

# Database targets (adjust as needed)
db-create:
	@echo "Creating database..."
	createdb nutri_go_db

db-drop:
	@echo "Dropping database..."
	dropdb nutri_go_db

db-migrate:
	@echo "Running database migrations..."
	sqlx migrate run

# Combined targets
all: fmt lint test build
	@echo "All checks passed!"

pre-commit: fmt lint check test
	@echo "Pre-commit checks completed!"

ci: fmt-check lint check test build-release
	@echo "CI checks completed!"
