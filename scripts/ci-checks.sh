#!/usr/bin/env bash
# Presidium Messenger — CI Checks (local)
# Run this script to reproduce the CI pipeline locally.

set -euo pipefail

echo "🔍 Running Presidium CI checks locally..."
echo ""

FAILED=0

# Step 1: Format check
echo "📝 Checking formatting..."
if cargo fmt --all -- --check; then
    echo "✅ Format check passed."
else
    echo "❌ Format check failed. Run: cargo fmt --all"
    FAILED=1
fi
echo ""

# Step 2: Clippy
echo "🔎 Running Clippy..."
if cargo clippy --workspace --all-targets -- -D warnings; then
    echo "✅ Clippy passed."
else
    echo "❌ Clippy failed."
    FAILED=1
fi
echo ""

# Step 3: Build
echo "🏗️  Building workspace..."
if cargo build --workspace; then
    echo "✅ Build passed."
else
    echo "❌ Build failed."
    FAILED=1
fi
echo ""

# Step 4: Tests
echo "🧪 Running tests..."
if cargo test --workspace; then
    echo "✅ Tests passed."
else
    echo "❌ Tests failed."
    FAILED=1
fi
echo ""

# Step 5: Documentation
echo "📚 Generating documentation..."
if cargo doc --workspace --no-deps; then
    echo "✅ Documentation generated."
else
    echo "❌ Documentation generation failed."
    FAILED=1
fi
echo ""

# Step 6: Audit (if cargo-audit is installed)
if command -v cargo-audit &>/dev/null; then
    echo "🔒 Running security audit..."
    if cargo audit; then
        echo "✅ Security audit passed."
    else
        echo "❌ Security audit found issues."
        FAILED=1
    fi
else
    echo "⚠️  cargo-audit not installed, skipping security audit."
fi
echo ""

# Step 7: Deny (if cargo-deny is installed)
if command -v cargo-deny &>/dev/null; then
    echo "📋 Checking dependencies..."
    if cargo deny check; then
        echo "✅ Dependency check passed."
    else
        echo "❌ Dependency check found issues."
        FAILED=1
    fi
else
    echo "⚠️  cargo-deny not installed, skipping dependency check."
fi
echo ""

if [ $FAILED -eq 0 ]; then
    echo "🎉 All CI checks passed!"
else
    echo "💥 Some CI checks failed. Please fix the issues above."
    exit 1
fi
