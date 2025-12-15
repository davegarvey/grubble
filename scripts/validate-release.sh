#!/bin/bash
# Workflow validation script
# Run this before pushing to ensure everything is configured correctly

set -e

echo "🔍 Validating Grubble Release Workflow Configuration"
echo "=================================================="
echo ""

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

check_pass() {
    echo -e "${GREEN}✓${NC} $1"
}

check_fail() {
    echo -e "${RED}✗${NC} $1"
    exit 1
}

check_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    check_fail "Not in project root (Cargo.toml not found)"
fi
check_pass "Found Cargo.toml"

# Check if release workflow exists
if [ ! -f ".github/workflows/release.yml" ]; then
    check_fail "Release workflow not found"
fi
check_pass "Found .github/workflows/release.yml"

# Check if action.yml exists
if [ ! -f "action.yml" ]; then
    check_fail "action.yml not found"
fi
check_pass "Found action.yml"

# Validate Cargo.toml has required metadata
echo ""
echo "📦 Validating Cargo.toml metadata..."

if ! grep -q "homepage" Cargo.toml; then
    check_warn "Missing homepage field in Cargo.toml"
else
    check_pass "Homepage field present"
fi

if ! grep -q "documentation" Cargo.toml; then
    check_warn "Missing documentation field in Cargo.toml"
else
    check_pass "Documentation field present"
fi

if ! grep -q "package.metadata.binstall" Cargo.toml; then
    check_fail "Missing binstall metadata in Cargo.toml"
fi
check_pass "Binstall metadata present"

# Check if project builds
echo ""
echo "🔨 Building project..."
if cargo build --release > /dev/null 2>&1; then
    check_pass "Project builds successfully"
else
    check_fail "Project build failed"
fi

# Check if tests pass
echo ""
echo "🧪 Running tests..."
if cargo test > /dev/null 2>&1; then
    check_pass "All tests pass"
else
    check_fail "Tests failed"
fi

# Check if binary works
echo ""
echo "🚀 Testing binary..."
if ./target/release/bump --help > /dev/null 2>&1; then
    check_pass "Binary executes successfully"
else
    check_fail "Binary execution failed"
fi

# Validate workflow YAML syntax
echo ""
echo "📝 Validating workflow YAML..."

# Check for common workflow issues
if grep -q "uses: actions/create-release@v1" .github/workflows/release.yml; then
    check_pass "Uses create-release action"
fi

if grep -q "uses: actions/upload-release-asset@v1" .github/workflows/release.yml; then
    check_pass "Uses upload-release-asset action"
fi

if grep -q "cross build" .github/workflows/release.yml; then
    check_pass "Uses cross for compilation"
fi

# Check all targets are present
echo ""
echo "🎯 Validating build targets..."

TARGETS=(
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-musl"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-msvc"
)

for target in "${TARGETS[@]}"; do
    if grep -q "$target" .github/workflows/release.yml; then
        check_pass "Target $target configured"
    else
        check_fail "Target $target missing from workflow"
    fi
done

# Check action.yml structure
echo ""
echo "🎬 Validating action.yml..."

if grep -q "name:" action.yml && grep -q "description:" action.yml; then
    check_pass "Action metadata present"
else
    check_fail "Action metadata incomplete"
fi

if grep -q "inputs:" action.yml; then
    check_pass "Action inputs defined"
else
    check_fail "Action inputs missing"
fi

if grep -q "outputs:" action.yml; then
    check_pass "Action outputs defined"
else
    check_warn "Action outputs missing (optional)"
fi

# Check for required inputs
INPUTS=("push" "tag" "release-notes" "raw" "preset")
for input in "${INPUTS[@]}"; do
    if grep -q "$input:" action.yml; then
        check_pass "Input '$input' defined"
    else
        check_warn "Input '$input' not found"
    fi
done

# Check README has installation instructions
echo ""
echo "📚 Validating README..."

if grep -q "## Installation" README.md; then
    check_pass "Installation section present"
else
    check_fail "Installation section missing"
fi

if grep -q "GitHub Action" README.md; then
    check_pass "GitHub Action usage documented"
else
    check_warn "GitHub Action usage not documented"
fi

if grep -q "cargo install" README.md; then
    check_pass "Cargo install documented"
else
    check_warn "Cargo install not documented"
fi

# Check version consistency
echo ""
echo "🔢 Checking version consistency..."

CARGO_VERSION=$(grep '^version' Cargo.toml | head -1 | cut -d'"' -f2)
echo "Cargo.toml version: $CARGO_VERSION"

if [ -f "CHANGELOG.md" ]; then
    if grep -q "$CARGO_VERSION" CHANGELOG.md; then
        check_pass "Version documented in CHANGELOG"
    else
        check_warn "Version not in CHANGELOG (may need update)"
    fi
fi

# Final summary
echo ""
echo "=================================================="
echo -e "${GREEN}✅ Validation complete!${NC}"
echo ""
echo "Next steps:"
echo "1. Commit all changes"
echo "2. Create and push a tag: git tag v$CARGO_VERSION && git push origin v$CARGO_VERSION"
echo "3. Monitor the release workflow: https://github.com/davegarvey/grubble/actions"
echo "4. Verify binaries are uploaded to the release"
echo "5. Test the GitHub Action in a separate repository"
echo ""
