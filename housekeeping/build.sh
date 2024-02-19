#!/bin/bash
set -e

build(){
    echo '⚡️ Running build'
    cargo b -r
}

build
