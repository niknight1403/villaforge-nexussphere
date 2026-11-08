#!/bin/bash

# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set stable Rust version
rustup default stable

# Add cargo to PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Verify installation
cargo --version