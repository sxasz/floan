#!/bin/bash

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Add Solana CLI to PATH
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Set Solana config to mainnet
solana config set --url https://api.mainnet-beta.solana.com

# Create a new keypair
solana-keygen new --outfile ~/.config/solana/id.json

# Set the keypair as the default
solana config set --keypair ~/.config/solana/id.json

# Build the program
cargo build-bpf --manifest-path=Cargo.toml --bpf-out-dir=dist/program

# Deploy the program
PROGRAM_ID=$(solana program deploy dist/program/solana_flash_loan.so | grep 'Program Id:' | awk '{print $3}')

echo "Program deployed with ID: $PROGRAM_ID"
echo "Update your bot with this Program ID"
