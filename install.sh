#!/bin/bash
set -e

REPO="${NOTION_CLI_REPO:-https://github.com/Sankalpcreat/Notion-CLI}"
INSTALL_DIR="${NOTION_CLI_INSTALL_DIR:-/usr/local/bin}"
BIN_NAME="notion"

echo "Installing notion-cli..."

if ! command -v cargo >/dev/null 2>&1; then
    echo "Rust required. Install from https://rustup.rs"
    exit 1
fi

tmp=$(mktemp -d)
trap "rm -rf $tmp" EXIT
git clone --depth 1 "$REPO" "$tmp"
(cd "$tmp" && cargo build --release)
sudo cp "$tmp/target/release/notion" "$INSTALL_DIR/$BIN_NAME"

echo ""
echo "Installed: $INSTALL_DIR/$BIN_NAME"
echo ""
echo "1. Set token: export NOTION_API_KEY=secret_xxx"
echo "2. Verify: $BIN_NAME user me"
echo "3. Agents/CI: set NOTION_API_KEY in environment"
