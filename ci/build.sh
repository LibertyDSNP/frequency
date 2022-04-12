#!/usr/bin/env bash

set -eux

SRT_TOOL_VERSION="${SRT_TOOL_VERSION:-1.57.0}"
PACKAGE="${PACKAGE:-mrc-runtime}" #Need to replicate job for all runtimes

# Enable warnings about unused extern crates
export RUSTFLAGS=" -W unused-extern-crates"

./scripts/init.sh install-toolchain

rustc --version
rustup --version
cargo --version

case $TARGET in
	build-node)
		cargo build --release "$@"
		;;

  build-runtime)
    export RUSTC_VERSION=$SRT_TOOL_VERSION
    echo "Building runtime with rustc version $RUSTC_VERSION"
    docker run --rm -e PACKAGE=$PACKAGE -v $PWD:/build -v /tmp/cargo:/cargo-home paritytech/srtool:$RUSTC_VERSION build
    ;;

  tests)
    cargo test --workspace
    ;;

  lint)
    cargo fmt -- --check
esac
