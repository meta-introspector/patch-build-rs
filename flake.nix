# flake.nix
{
  description = "Fetch rustc source by version programmatically using Nixpkgs and rust-overlay";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      pkgs = import nixpkgs {
        overlays = [ rust-overlay.overlays.default ];
      };
    in {
      # Expose a specific rust-src derivation
      packages.x86_64-linux.rustc-src-1_70_0 = pkgs.rust-bin.stable."1.70.0".rust-src;

      # A development shell to easily get the path
      devShells.x86_64-linux.default = pkgs.mkShell {
        packages = [ pkgs.rust-bin.stable."1.70.0".rust-src ];
        shellHook = ''
          export RUSTC_SOURCE_PATH_1_70_0="$(nix path-info --store $(nix build --no-link --print-build-logs .#rustc-src-1_70_0))"
          echo "RUSTC_SOURCE_PATH_1_70_0: $RUSTC_SOURCE_PATH_1_70_0"
        '';
      };
    };
}
