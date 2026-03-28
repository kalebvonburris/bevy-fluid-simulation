{
  description = "Shady Casino — Bevy 0.18 casino roguelike";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };

        # Build-time tools (compilers, linkers, pkg-config).
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          clang
          mold
        ];

        # Bevy's runtime / link-time Linux dependencies.
        linuxBuildInputs = with pkgs; [
          # Audio
          alsa-lib

          # Binary compression
          upx

          # Windowing — X11
          libx11
          libxcursor
          libxi
          libxrandr

          # Windowing — Wayland
          libxkbcommon
          wayland

          # GPU / Vulkan
          vulkan-loader

          # udev for gamepad / input
          udev
        ];

        buildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux linuxBuildInputs;
      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;

          # Ensure the dynamic linker can find Vulkan, X11, Wayland, etc.
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;

          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
