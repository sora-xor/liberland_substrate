#!/bin/bash
set -e

test() {
    echo '⚡️ Running tests'
    cargo test --features runtime-benchmarks --no-fail-fast
}

build() {
    echo '⚡️ Running build'
    cargo b -r
}

# build func
if [ "$(type -t $1)" = "function" ]; then
    "$1"
else
    echo "Func '$1' is not exists in this workflow. Skipping."
fi