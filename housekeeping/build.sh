#!/bin/bash

set -e
rustcVersion=nightly-2023-03-21
rustup override set $rustcVersion
cargo b -r
