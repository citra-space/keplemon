#!/bin/bash
# Setup script to install git hooks for development

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
HOOKS_DIR="$PROJECT_ROOT/.git/hooks"

echo "Installing git hooks..."

# Create pre-commit hook
cat > "$HOOKS_DIR/pre-commit" << 'EOF'
#!/bin/bash
# Pre-commit hook to run cargo fmt before committing

# Run cargo fmt on all Rust files
echo "Running cargo fmt..."
cargo fmt

# Check if formatting made any changes
if ! git diff --exit-code --quiet; then
    echo "✓ Code formatted successfully"
    echo "  Formatted files have been staged automatically"
    # Stage the formatted files
    git add -u
fi

exit 0
EOF

# Make it executable
chmod +x "$HOOKS_DIR/pre-commit"

echo "✓ Pre-commit hook installed successfully"
echo ""
echo "The hook will automatically run 'cargo fmt' before each commit."
