{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils.url = "github:numtide/flake-utils";
  };
  outputs = {
    self,
    nixpkgs,
    utils,
    rust-overlay,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        bi = with pkgs; [
          libxkbcommon
          udev
          alsa-lib
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          vulkan-tools
          vulkan-headers
          vulkan-loader
          vulkan-validation-layers
          pkgs.llvmPackages.bintools
          clang
          llvm
          llvmPackages.libclang
          rust-bin.stable.latest.default
          rust-analyzer
        ];
      in {
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            # Everything below is a hard bevy dep
          ];
          buildInputs = bi;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath bi;
        };
      }
    );
}
