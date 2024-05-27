{
  pkgs,
  lib,
  stdenv,
  sea-orm-cli,
  nodejs-18_x,
  nodePackages,
  postgresql,
  docker,
}: let
  rust = pkgs.rust-bin.stable.latest.default.override {
    extensions = ["rust-src"];
  };
in
  pkgs.mkShell {
    buildInputs =
      [
        # Rust
        rust
        sea-orm-cli
        # Node
        nodejs-18_x
        nodePackages.npm
        # Other
        postgresql
        docker
      ]
      ++ (with pkgs.darwin.apple_sdk;
        lib.optionals stdenv.isDarwin [
          # macOS SDKs
          frameworks.SystemConfiguration
        ])
      ++ lib.optional stdenv.isLinux pkgs.openssl;

    DATABASE_URL = "postgres://olly:password@0.0.0.0:5432/olly";
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  }
