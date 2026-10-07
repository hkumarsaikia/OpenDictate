#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="$DIR/target/release/opendictate"

# Rebuild only if binary is missing or source files/configs are newer than the binary
if [ ! -f "$BINARY" ] || [ -n "$(find "$DIR/src" "$DIR/Cargo.toml" "$DIR/Cargo.lock" -newer "$BINARY" 2>/dev/null)" ]; then
    cargo build --release --quiet
fi

# Launch the OpenDictate GTK4/Libadwaita application
exec "$BINARY" "$@"
