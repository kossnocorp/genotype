{
  description = "Genotype dev environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              # System
              cacert
              openssl
              # Tools
              just
              entr
              # Rust
              rustc
              cargo
              cargo-watch
              cargo-nextest
              cargo-release
              rustfmt
              # Node.js
              nodejs
              corepack
              # Python
              python3
            ];

            env.LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.stdenv.cc.cc.lib
              pkgs.libz
            ];

            # Provide rust-src for rust-analyzer
            env.RUST_SRC_PATH = pkgs.rust.packages.stable.rustPlatform.rustLibSrc;

            shellHook = ''
              # Make cargo install work
              export PATH="$PATH:$HOME/.cargo/bin"
              # Tell TLS libraries where to find CA certificates
              export SSL_CERT_FILE="${cacert}/etc/ssl/certs/ca-bundle.crt"
            '';
          };
      }
    );
}
