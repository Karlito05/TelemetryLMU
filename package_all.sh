#!/usr/bin/env bash
set -euo pipefail

rm -rf packaging/.cargo-packager

cargo packager -c packaging/Windows.toml
cargo packager -c packaging/Pacman.toml
cargo packager -c packaging/Debian.toml
