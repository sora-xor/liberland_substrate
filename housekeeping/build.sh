#!/bin/bash
set -e

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