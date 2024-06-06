{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config

          # added later
          taplo # Toml language server and formatter
          lldb # Rust debugger
          cargo-modules
          cargo-watch
        ];

        buildInputs = with pkgs; [
          openssl

          # added later
        ];
      in
      with pkgs;
      {
        devShells.default = mkShell { inherit buildInputs nativeBuildInputs; };

        shellHook = ''
          echo "Rust development environment"
        '';
      }
    );
}
