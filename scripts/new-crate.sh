#!/usr/bin/env bash

if [ ! -d "crates" ] || [ ! -d "scripts" ] || [ ! -f "Cargo.toml" ]; then
    echo "Error: Run from the repo root"
    exit 1
fi

if [ $# -eq 0 ]; then
    echo "Usage: $0 <crate_name>"
    exit 1
fi

CRATE_NAME="$1"
LICENSE_MODE="MIT"
LICENSE_FILE="LICENSE"

if [[ ! "$CRATE_NAME" =~ ^[a-z0-9_]+$ ]]; then
    echo "Error: Crate name must be lowercase and contain only alphanumeric characters and underscores"
    exit 1
fi

CRATE_PATH="crates/$CRATE_NAME"
if [ -e "$CRATE_PATH/Cargo.toml" ]; then
    echo "Error: $CRATE_PATH already exists"
    exit 1
fi
mkdir -p "$CRATE_PATH/src"

ln -sf "../../$LICENSE_FILE" "$CRATE_PATH/$LICENSE_FILE"

CARGO_TOML_TEMPLATE=$(cat << 'EOF'
[package]
name = "$CRATE_NAME"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true
license = "$LICENSE_MODE"

[lints]
workspace = true

[lib]
path = "src/$CRATE_NAME.rs"

[features]
default = []

[dependencies]
EOF
)

CARGO_TOML_CONTENT=$(echo "$CARGO_TOML_TEMPLATE" | sed \
    -e "s/\$CRATE_NAME/$CRATE_NAME/g" \
    -e "s/\$LICENSE_MODE/$LICENSE_MODE/g")

echo "$CARGO_TOML_CONTENT" > "$CRATE_PATH/Cargo.toml"

echo "//! # $CRATE_NAME" > "$CRATE_PATH/src/$CRATE_NAME.rs"

echo "Created new crate: $CRATE_NAME in $CRATE_PATH"
echo "License: $LICENSE_MODE (symlinked from $LICENSE_FILE)"
echo "Don't forget to add the new crate to the workspace!"
