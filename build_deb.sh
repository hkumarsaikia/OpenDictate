#!/usr/bin/env bash
# OpenDictate Debian Build Entrypoint
# Delegates directly to the pure Rust release packaging pipeline.
set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$PROJECT_ROOT/build_rust_deb.sh" "$@"
