{
  description = "Genotype dev environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
      in
      {
        devShells.default =
          with pkgs;
          let
            rust = rust-bin.stable.latest.default.override {
              extensions = [
                "rust-src"
              ];
            };
          in
          mkShell {
            buildInputs = [
              # Tools
              direnv
              just
              entr
              # Rust
              rust
              # Cargo
              cargo-watch
              cargo-nextest
              cargo-release
              cargo-binstall
              # Rust Tools
              evcxr
              bacon
              # Node.js
              nodejs
              corepack
              # Python
              python3
            ];

            shellHook = ''
              # Make cargo install work
              export PATH="$PATH:$HOME/.cargo/bin"
            '';
          };

      }
    );
}
