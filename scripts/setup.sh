#!/usr/bin/env bash
# Presidium Messenger — Development Environment Setup
# Run this script after cloning the repository.

set -euo pipefail

echo "🔧 Setting up Presidium Messenger development environment..."

# Check Rust installation
if ! command -v rustup &>/dev/null; then
    echo "❌ Rust is not installed. Please install via https://rustup.rs/"
    exit 1
fi

echo "✅ Rust toolchain: $(rustc --version)"

# Install required components
echo "📦 Installing Rust components..."
rustup component add rustfmt clippy rust-analyzer

# Install cargo tools
echo "📦 Installing cargo tools..."
cargo install cargo-audit cargo-deny 2>/dev/null || {
    echo "⚠️  Some cargo tools may already be installed."
}

# Set up git hooks (if using the repo's hook directory)
if [ -d ".git" ]; then
    echo "🪝 Setting up git hooks..."
    mkdir -p .git/hooks
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
set -e
echo "🔍 Running pre-commit checks..."
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
echo "✅ Pre-commit checks passed."
EOF
    chmod +x .git/hooks/pre-commit
    echo "✅ Pre-commit hook installed."
fi

# Build the workspace
echo "🏗️  Building workspace..."
cargo build --workspace

# Run tests
echo "🧪 Running tests..."
cargo test --workspace

echo ""
echo "✅ Setup complete! Presidium Messenger is ready for development."
echo ""
echo "Quick commands:"
echo "  cargo build --workspace     # Build all crates"
echo "  cargo test --workspace      # Run all tests"
echo "  cargo fmt --all -- --check  # Check formatting"
echo "  cargo clippy --workspace    # Run linter"
echo "  cargo doc --workspace --open # Generate docs"
