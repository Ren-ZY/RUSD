#!/bin/bash

echo "Building RUSTSODA and install RUSTSODA by Cargo"
#cargo clean
#for debug version
#cargo install --debug --path "$(dirname "$0")" --force --features backtraces
#for release version
cargo install --path "$(dirname "$0")" --force
