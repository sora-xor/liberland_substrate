#!/bin/bash
set -e

build(){
    cargo b -r
}

test(){
    cargo test
}

build
test