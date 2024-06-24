{ inputs, ... }:
{
  perSystem =
    {
      self',
      pkgs,
      system,
      ...
    }:
    let
      fnx = inputs.fenix.packages.${system};
      generalPkgs = [
        pkgs.pkg-config
        pkgs.udev
        pkgs.alsaLib
        pkgs.vulkan-loader
        pkgs.wayland
        pkgs.libxkbcommon
        pkgs.openssl
        pkgs.cargo-nextest
      ];
    in
    {
      packages.rust = fnx.combine [
        fnx.stable.cargo
        fnx.stable.clippy
        fnx.stable.rust-analyzer
        fnx.stable.rust-src
        fnx.stable.rustc
        fnx.complete.rustfmt
        fnx.targets.wasm32-unknown-unknown.stable.rust-std
      ];
      devShells = rec {
        default = stable;
        stable = pkgs.mkShell {
          name = "stable";
          packages = generalPkgs ++ [ self'.packages.rust ];
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath generalPkgs;
        };
      };
    };
}
