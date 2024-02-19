#!/bin/bash
set -e

build(){
    echo '⚡️ Running build'
    cargo b -r
}

test(){
    echo '⚡️ Running tests'
    cargo test -r
}

test
build
