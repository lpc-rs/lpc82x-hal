#!/usr/bin/env bash
set -e

# Target triple of the host machine. Set to x86-64 Linux by default, as that's
# the correct value for the CI server.
#
# Unfotunately there doesn't seem to be a way to tell Cargo to just use the
# target of the host machine, and since we override the default target in
# `.cargo/config` for convenience, this variable is required.
HOST_TARGET=${HOST_TARGET:-x86_64-unknown-linux-gnu}

# Determine whether this build should run stable checks. This should be the case
# on CI when the stable channel is used, or locally regardless of Rust version
# used for the build.
[ "$TRAVIS_RUST_VERSION" = stable ] && \
    STABLE_CI_BUILD=true || STABLE_CI_BUILD=false
[ -z "$TRAVIS_RUST_VERSION" ] && \
    NO_CI_BUILD=true || NO_CI_BUILD=false
([ "$STABLE_CI_BUILD" = true ] || [ "$NO_CI_BUILD" = true ]) && \
    STABLE_CHECKS=true || STABLE_CHECKS=false

# Fail build, if there are any warnings.
export RUSTFLAGS="-D warnings"

# Check for formatting with the stable rustfmt
if [ "$STABLE_CHECKS" = true ]; then
    # Only install rustup on stable, since it's not needed otherwise (and sometimes unavailable)
    rustup component add rustfmt
    cargo fmt -- --check
fi

function build() {
    echo ""
    echo "### Building target $1"
    echo ""

    cargo test \
        --verbose \
        --features=$1,no-target-warning,trybuild \
        --target=$HOST_TARGET
    cargo build --verbose --features=$1-rt,no-target-warning --examples
}

build 82x
build 845
