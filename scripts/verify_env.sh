#!/bin/bash
# NexusSphere Environment Verification Script

echo "Checking Rust toolchain..."
if ! command -v rustup &> /dev/null; then
    echo "rustup not found. Installing..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

echo "Verifying cargo version..."
cargo --version
if [ $? -eq 0 ]; then
    echo "Build environment verified successfully."
    exit 0
else
    echo "Build environment verification failed."
    exit 1
fi