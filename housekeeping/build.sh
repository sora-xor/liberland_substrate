#!/bin/bash
set -e

build(){
    echo '⚡️ Running build'
    cargo b -r
}

if [ "$1" == "build" ]; then
    build
else
    exit 1
fi