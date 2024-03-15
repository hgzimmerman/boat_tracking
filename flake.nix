{
  description = "Minimal rust example";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        inputs = [ 
          rust 
          pkgs.wasm-bindgen-cli 
          pkgs.rust-analyzer 
          pkgs.sqlite 
          pkgs.diesel-cli 
          pkgs.openssl 
          pkgs.pkg-config
          pkgs.dioxus-cli
          ];
      in
      {
        defaultPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "boat_tracking";
          version = "1.0.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = inputs;

          buildPhase = ''
            cargo build --release 

            echo 'Creating out dir...'
            mkdir -p $out/src;

            # Optional, of course
            # echo 'Copying package.json...'
            # cp ./package.json $out/;

            # echo 'Generating node module...'
            # wasm-bindgen \
            #   --target nodejs \
            #   --out-dir $out/src \
            #   target/wasm32-unknown-unknown/release/gcd.wasm;
          '';
          installPhase = "echo 'Skipping installPhase'";
        };


        devShell = pkgs.mkShell { 
          packages = inputs;
          # packages = self;
          PKG_CONFIG_PATH = "{pkgs.openssl.dev}/lib/pkgconfig";
          DATABASE_URL = "db.sql";
        };
      }
    );
}
