#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
image_name=telemetry-lmu-linux-nightly-bullseye

if docker info >/dev/null 2>&1; then
    docker_cli() { docker "$@"; }
else
    docker_cli() { sudo docker "$@"; }
fi

docker_cli build \
    --tag "$image_name" \
    --file "$repo_root/packaging/Dockerfile.linux" \
    "$repo_root"

docker_cli run --rm \
    --user "$(id -u):$(id -g)" \
    --env CARGO_HOME=/tmp/cargo \
    --volume "$repo_root:/workspace" \
    --workdir /workspace \
    "$image_name" \
    cargo build --release