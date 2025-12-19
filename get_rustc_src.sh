#!/bin/bash
set -euo pipefail

# This script fetches the rustc source path for a given version using Nix
# and prints it to stdout.

RUSTC_VERSION="$1"
if [ -z "$RUSTC_VERSION" ]; then
    echo "Usage: $0 <rustc_version>"
    exit 1
fi

FLAKE_NIX_CONTENT=$(cat <<EOF
# flake.nix - auto-generated
{
  description = "Fetch rustc source by version programmatically using Nixpkgs and rust-overlay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, system # <--- ADDED 'system' as an argument again
             }:
    let
      # No 'system = builtins.currentSystem;' here. 'system' comes from outputs arguments.
      pkgs = import nixpkgs {
        inherit system; # Pass the received system to nixpkgs import
        overlays = [ rust-overlay.overlays.default ];
      };
      # Define the desired rustc version
      targetRustcVersion = "${RUSTC_VERSION}";
    in {
      # Use the received system argument
      packages.\${system}.default = pkgs.rust-bin.stable."\${targetRustcVersion}".rust-src;
    };
}
EOF
)

# Create a temporary directory for the flake
TMP_DIR=$(mktemp -d)
echo "$FLAKE_NIX_CONTENT" > "$TMP_DIR/flake.nix"

# Build the rust-src derivation and get its store path
# Simplified flake access to default package.
# We explicitly pass the system to nix build this time.
RUSTC_SOURCE_PATH=$(nix build --no-link --print-build-logs --json --system "$(nix eval --raw --expr 'builtins.currentSystem')" "$TMP_DIR" | jq -r '.[0].outputs.out')

# Clean up temporary directory
rm -rf "$TMP_DIR"

echo "$RUSTC_SOURCE_PATH"