{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem
    (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit overlays;
        inherit system;
      };
      rust-stable = pkgs.rust-bin.stable.latest.default.override {
        extensions = ["rust-src"];
      };
    in {
      devShells.default = with pkgs; pkgs.mkShell {
        buildInputs = [
          # Rust
          rust-stable
          pkgs.sea-orm-cli
          # Node
          pkgs.nodejs-18_x
          pkgs.nodePackages.npm
          # Other
          pkgs.postgresql
          pkgs.docker
        ] ++ (with pkgs.darwin.apple_sdk; lib.optionals stdenv.isDarwin [
            # macOS SDKs
            frameworks.SystemConfiguration
        ]) ++ lib.optional stdenv.isLinux pkgs.openssl;

        DATABASE_URL = "postgres://othello-server:password@0.0.0.0:5432/othello-server";
        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      };
    });
}
