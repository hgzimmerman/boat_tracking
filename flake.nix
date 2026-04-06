{
  description = "Boat tracking system for GGRC";

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
          pkgs.rust-analyzer
          pkgs.diesel-cli
          pkgs.openssl
          pkgs.pkg-config
          ];

        tauriDeps = with pkgs; [
          glib
          gtk3
          webkitgtk_4_1
          libsoup_3
          cairo
          gdk-pixbuf
          pango
          harfbuzz
          zlib
        ];

          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.glib.dev}/lib/pkgconfig:${pkgs.gtk3.dev}/lib/pkgconfig:${pkgs.webkitgtk_4_1.dev}/lib/pkgconfig:${pkgs.libsoup_3.dev}/lib/pkgconfig:${pkgs.cairo.dev}/lib/pkgconfig:${pkgs.gdk-pixbuf.dev}/lib/pkgconfig:${pkgs.pango.dev}/lib/pkgconfig:${pkgs.harfbuzz.dev}/lib/pkgconfig";
      in
      {
        defaultPackage = pkgs.rustPlatform.buildRustPackage {
          pname = "boat_tracking";
          version = "1.0.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          buildInputs = tauriDeps ++ pkgs.lib.optionals pkgs.hostPlatform.isWindows [
              pkgs.windows.mingw_w64_pthreads
          ];
          inherit PKG_CONFIG_PATH;

          nativeBuildInputs = inputs;

          buildPhase = ''
            echo 'Building server'
            cargo build --release
          '';
          installPhase = "echo 'Skipping installPhase'";
        };

        devShell = pkgs.mkShell {
          packages = inputs ++ tauriDeps ++ [ pkgs.tailwindcss ];
          nativeBuildInputs = with pkgs; [
            cargo-watch
            (writeShellScriptBin "watch-tailwind" ''
              tailwindcss -i ./input.css -o ./public/tailwind.css --watch
            '')
            (writeShellScriptBin "run-server" ''
              cargo run
            '')
            (writeShellScriptBin "watch-server" ''
              cargo watch -x "run" --clear -d 2.5 -w ./src
            '')
          ];
          inherit PKG_CONFIG_PATH;
          DATABASE_URL = "db.sql";
        };
      }
    );
}
