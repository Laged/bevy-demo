{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1"; # unstable Nixpkgs
    fenix = {
      url = "https://flakehub.com/f/nix-community/fenix/0.1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, ... }@inputs:

    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        inputs.nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [
                inputs.self.overlays.default
              ];
            };
          }
        );
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          with inputs.fenix.packages.${prev.stdenv.hostPlatform.system};
          combine (
            with stable;
            [
              clippy
              rustc
              cargo
              rustfmt
              rust-src
            ]
          );
      };

      packages = forEachSupportedSystem (
        { pkgs }:
        let
          isLinux = pkgs.stdenv.isLinux;
          isDarwin = pkgs.stdenv.isDarwin;

          linuxBuildInputs = with pkgs; [
            alsa-lib
            udev
            libxkbcommon
            wayland
            vulkan-loader
          ];

          darwinBuildInputs = with pkgs; [
            # Metal for graphics on macOS
            libiconv
          ];

          buildInputsBase = with pkgs; [
            openssl
          ];

          allBuildInputs = buildInputsBase ++ (if isLinux then linuxBuildInputs else if isDarwin then darwinBuildInputs else []);

          linuxLibraryPath = "${pkgs.lib.makeLibraryPath [
            pkgs.alsa-lib
            pkgs.udev
            pkgs.libxkbcommon
            pkgs.wayland
            pkgs.vulkan-loader
          ]}:$LD_LIBRARY_PATH";
        in
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "hell-game";
            version = "0.1.0";
            src = self;

            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = with pkgs; [
              pkg-config
              gcc
            ];

            buildInputs = allBuildInputs;

            env = if isLinux then {
              LD_LIBRARY_PATH = linuxLibraryPath;
            } else {
              # macOS doesn't need LD_LIBRARY_PATH
            };
          };
        }
      );

      apps = forEachSupportedSystem (
        { pkgs }:
        {
          default = {
            type = "app";
            program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.default}/bin/hell-game";
            meta = {
              description = "A Bevy 2D survival shooter";
              mainProgram = "hell-game";
            };
          };
        }
      );

      devShells = forEachSupportedSystem (
        { pkgs }:
        let
          isLinux = pkgs.stdenv.isLinux;
          isDarwin = pkgs.stdenv.isDarwin;

          linuxPackages = with pkgs; [
            alsa-lib
            udev
            libxkbcommon
            wayland
            vulkan-loader
          ];

          darwinPackages = with pkgs; [
            # macOS development tools
            libiconv
          ];

          basePackages = with pkgs; [
            rustToolchain
            gcc
            openssl
            pkg-config
            cargo-deny
            cargo-edit
            cargo-watch
            rust-analyzer
          ];

          allPackages = basePackages ++ (if isLinux then linuxPackages else if isDarwin then darwinPackages else []);

          linuxLibraryPath = "${pkgs.lib.makeLibraryPath [
            pkgs.alsa-lib
            pkgs.udev
            pkgs.libxkbcommon
            pkgs.wayland
            pkgs.vulkan-loader
          ]}:$LD_LIBRARY_PATH";
        in
        {
          default = pkgs.mkShell {
            packages = allPackages;

            env = {
              # Required by rust-analyzer
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            } // (if isLinux then {
              LD_LIBRARY_PATH = linuxLibraryPath;
            } else {
              # macOS doesn't need LD_LIBRARY_PATH
            });
          };
        }
      );
    };
}
