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

        stableToolchain = pkgs.rust-bin.stable.latest.default;
        stableToolchainWithRustAnalyzer = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer"];
          # Extra targets if required
          targets = [
            "wasm32-unknown-unknown"
            # "x86_64-unknown-linux-gnu"
            # "x86_64-unknown-linux-musl"
            # "x86_64-apple-darwin"
            # "aarch64-apple-darwin"
          ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain stableToolchain;
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        commonArgs = {
          inherit src;
          buildInputs = with pkgs;
            [
              # alsa-lib
              # wayland
              # xorg.libX11
              # xorg.libXi
              # libGL
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.Foundation
              pkgs.darwin.apple_sdk.frameworks.CoreServices
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
              pkgs.darwin.apple_sdk.frameworks.System
              pkgs.darwin.apple_sdk.frameworks.ImageIO
              pkgs.darwin.apple_sdk.frameworks.AppKit
              pkgs.darwin.apple_sdk.frameworks.Vision
              pkgs.darwin.apple_sdk.frameworks.CoreGraphics
              pkgs.darwin.apple_sdk.frameworks.Metal
              pkgs.darwin.apple_sdk.frameworks.AVFoundation
              pkgs.darwin.apple_sdk.frameworks.MetalKit
              # "-lsystem" "-framework" "Foundation" "-framework" "ImageIO" "-framework" "AppKit" "-framework" "Vision" "-framework" "CoreGraphics" "-framework" "Metal" "-framework" "AVFoundation" "-framework" "CoreMidi" "-framework" "MetalKit" "-lobjc" "-liconv" "-lSystem" "-lc" "-lm"
            ]; # Inputs required for the TARGET system

          nativeBuildInputs = with pkgs; [
            # often required for c/c++ libs
            pkg-config
          ]; # Intputs required for the HOST system
          # This is often requird for any ffi based packages that use bindgen
          # LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          # For using pkg-config that many libraries require
          # PKG_CONFIG_PATH = lib.makeSearchPath "lib/pkgconfig" (with pkgs;[ openssl.dev zlib.dev ]);
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in {
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
          default = tadventure;
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
