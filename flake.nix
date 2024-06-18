{
  description = "A simple rust flake using rust-overlay and craneLib";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    crane,
    flake-utils,
    nixpkgs,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };
        inherit (pkgs) lib;

        stableToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [
            "wasm32-unknown-unknown"
          ];
        };
        stableToolchainWithRustAnalyzer = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer"];
          # Extra targets if required
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain stableToolchain;
        src = craneLib.path ./.;
        commonArgs = {
          inherit src;
          buildInputs = with pkgs;
            []
            ++ pkgs.lib.optionals pkgs.stdenv.isLinux [
              alsa-lib
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.Foundation
              pkgs.darwin.apple_sdk.frameworks.AppKit
              pkgs.darwin.apple_sdk.frameworks.Vision
              pkgs.darwin.apple_sdk.frameworks.AVFoundation
              pkgs.darwin.apple_sdk.frameworks.MetalKit
            ];

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in rec {
        checks = {
          tadventure-clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });
          tadventure-fmt = craneLib.cargoFmt {
            inherit src;
          };
          tadventure-nextest = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            });
        };
        packages = rec {
          tadventure-unwrapped = craneLib.buildPackage ({
              pname = "tadventure";
              version = "0.1.0";
              inherit src cargoArtifacts;
            }
            // commonArgs);
          tadventure =
            if pkgs.stdenv.isLinux
            then
              (pkgs.buildFHSEnv {
                name = "tadventure";
                targetPkgs = pkgs:
                  with pkgs; [
                    tadventure-unwrapped
                    xorg.libX11
                    xorg.libXi
                    libGL
                    egl-wayland
                  ];
                multiPkgs = pkgs:
                  with pkgs; [
                    alsa-lib
                  ];
                runScript = ''
                  ${tadventure-unwrapped}/bin/tadventure
                '';
              })
            else tadventure-unwrapped;
          tadventure-wasm = craneLib.buildPackage {
            inherit src;
            doCheck = false;
            cargoExtraArgs = "--target wasm32-unknown-unknown";
            installPhase = ''
              mkdir -p $out/dist
              cp target/wasm32-unknown-unknown/release/tadventure.wasm $out/dist/tadventure.wasm
              cp $src/assets/index.html $out/dist/index.html
            '';
            meta.mainProgram = null;
          };
          default = tadventure;
        };

        apps = {
          default = let
            wasm = pkgs.writeShellApplication {
              # Our shell script name is serve
              # so it is available at $out/bin/serve
              name = "tadventure";
              # Caddy is a web server with a convenient CLI interface
              runtimeInputs = [pkgs.caddy];
              text = ''
                # Serve the current directory on port 8090
                caddy file-server --listen :8000 --root ${packages.tadventure-wasm}/dist
              '';
            };
          in {
            type = "app";
            program = "${wasm}/bin/tadventure";
          };
        };

        devShells.default = (craneLib.overrideToolchain stableToolchainWithRustAnalyzer).devShell (commonArgs
          // {
            buildInputs = [];
            nativeBuildInputs = [];
            packages = with pkgs; [
              cargo-nextest
              cargo-criterion
              trunk
            ];
          }
          // lib.optionalAttrs (pkgs.stdenv.isLinux) {
            LD_LIBRARY_PATH = builtins.concatStringsSep ":" [
              "${pkgs.xorg.libX11}/lib"
              "${pkgs.xorg.libXi}/lib"
              "${pkgs.libGL}/lib"
              "${pkgs.wayland}/lib"
            ];
          });
      }
    );
}
