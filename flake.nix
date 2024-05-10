{
  description = "Direnv Configuration.";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    };
    flake-utils-plus = {
      url = "github:gytis-ivaskevicius/flake-utils-plus/v1.3.1";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
    };
  };

  outputs = { nixpkgs, flake-utils-plus, rust-overlay, ... }:
    let
      pkgsForSystem = system: import nixpkgs {
        overlays = [ (import rust-overlay) ];
        inherit system;
      };
    in flake-utils-plus.lib.eachDefaultSystem (system:
      let
        pkgs = pkgsForSystem system;
        rust-tools = (pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-analyzer" "rust-src" "rustfmt-preview" "clippy-preview" ];
        }));
      in {
        devShells.default = pkgs.mkShell {
          RUST_SRC_PATH = "${rust-tools}/lib/rustlib/src/rust/library";
          nativeBuildInputs = [
            pkgs.eza
            pkgs.fd
            pkgs.openssl
            pkgs.pkg-config
            pkgs.rust-analyzer
            rust-tools
          ];
          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };
      }
    );
}

