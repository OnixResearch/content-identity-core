{
  description = "Product-neutral Lattice-compatible BLAKE3 framing";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      eachSystem = function: nixpkgs.lib.genAttrs systems (system: function system);
    in
    {
      checks = eachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
        in
        {
          tests = rustPlatform.buildRustPackage {
            pname = "content-identity-core-tests";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [ "--all-targets" ];
            cargoTestFlags = [ "--all-targets" ];
            installPhase = ''
              touch "$out"
            '';
          };

          no-std = rustPlatform.buildRustPackage {
            pname = "content-identity-core-no-std";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            cargoBuildFlags = [
              "--lib"
              "--target=wasm32-unknown-unknown"
            ];
            doCheck = false;
            installPhase = ''
              touch "$out"
            '';
          };

          format =
            pkgs.runCommand "content-identity-core-format"
              {
                nativeBuildInputs = [ toolchain ];
                src = self;
              }
              ''
                cd "$src"
                cargo fmt --all -- --check
                touch "$out"
              '';
        }
      );

      devShells = eachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          default = pkgs.mkShell { packages = [ toolchain ]; };
        }
      );
    };
}
