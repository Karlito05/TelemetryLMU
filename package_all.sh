#! /usr/bin/env bash

cargo packager -c packaging/Windows.toml
cargo packager -c packaging/Pacman.toml
cargo packager -c packaging/Debian.toml
