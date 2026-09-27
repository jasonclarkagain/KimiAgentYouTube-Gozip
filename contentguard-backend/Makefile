# Makefile for ContentGuard AI

.PHONY: all build build-release build-debug test clean install docker run-server

# Default target
all: build-release

# Build release binary
build-release:
	@echo "Building release binary..."
	@./scripts/build.sh

# Build debug binary
build-debug:
	@echo "Building debug binary..."
	@./scripts/build.sh --debug

# Build with GPU support
build-gpu:
	@echo "Building with GPU support..."
	@./scripts/build.sh --gpu

# Cross-compile for all targets
cross-compile:
	@echo "Cross-compiling for all targets..."
	@./scripts/cross-compile.sh

# Run tests
test:
	@echo "Running tests..."
	@cargo test --all-features

# Run integration tests
test-integration:
	@echo "Running integration tests..."
	@cargo test --test integration_tests --all-features

# Run benchmarks
bench:
	@echo "Running benchmarks..."
	@cargo bench

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	@cargo clean
	@rm -rf dist/

# Install locally
install: build-release
	@echo "Installing to /usr/local/bin..."
	@sudo cp target/release/contentguard /usr/local/bin/
	@echo "Installation complete!"

# Uninstall
uninstall:
	@echo "Uninstalling..."
	@sudo rm -f /usr/local/bin/contentguard
	@echo "Uninstall complete!"

# Build Docker image
docker:
	@echo "Building Docker image..."
	@docker build -t contentguard:latest .

# Run Docker container
docker-run:
	@echo "Running Docker container..."
	@docker run -p 8080:8080 -v $(PWD)/data:/app/data contentguard:latest

# Run API server locally
run-server: build-release
	@echo "Starting API server..."
	@./target/release/contentguard server

# Run CLI analysis
analyze: build-release
	@echo "Running analysis..."
	@./target/release/contentguard analyze --input $(INPUT) --output report.json

# Format code
fmt:
	@echo "Formatting code..."
	@cargo fmt

# Run clippy linter
lint:
	@echo "Running clippy..."
	@cargo clippy --all-features -- -D warnings

# Check code
check:
	@echo "Running cargo check..."
	@cargo check --all-features

# Generate documentation
docs:
	@echo "Generating documentation..."
	@cargo doc --all-features --no-deps --open

# Update dependencies
update:
	@echo "Updating dependencies..."
	@cargo update

# Security audit
audit:
	@echo "Running security audit..."
	@cargo audit

# Release build with all checks
release: fmt lint test build-release
	@echo "Release build complete!"
	@ls -lh target/release/contentguard

# Help
help:
	@echo "ContentGuard AI - Makefile targets"
	@echo ""
	@echo "Build targets:"
	@echo "  make build-release    Build optimized release binary"
	@echo "  make build-debug      Build debug binary"
	@echo "  make build-gpu        Build with GPU acceleration"
	@echo "  make cross-compile    Cross-compile for all targets"
	@echo ""
	@echo "Test targets:"
	@echo "  make test             Run all tests"
	@echo "  make test-integration Run integration tests"
	@echo "  make bench            Run benchmarks"
	@echo ""
	@echo "Development targets:"
	@echo "  make fmt              Format code"
	@echo "  make lint             Run clippy linter"
	@echo "  make check            Run cargo check"
	@echo "  make docs             Generate documentation"
	@echo ""
	@echo "Deployment targets:"
	@echo "  make install          Install to /usr/local/bin"
	@echo "  make uninstall        Remove from /usr/local/bin"
	@echo "  make docker           Build Docker image"
	@echo "  make docker-run       Run Docker container"
	@echo ""
	@echo "Utility targets:"
	@echo "  make clean            Clean build artifacts"
	@echo "  make update           Update dependencies"
	@echo "  make audit            Run security audit"
	@echo "  make release          Full release build with checks"
