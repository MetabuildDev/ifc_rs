{ inputs, ... }:
{
  perSystem =
    { pkgs, system, ... }:
    let
      fnx = inputs.fenix.packages.${system};
      mkName = name: "Bevy (" + name + ") dev shell";
      mkRustDeriv =
        fnx-version: extra-components:
        let
          std-components = [
            fnx-version.cargo
            fnx-version.clippy
            fnx-version.rust-analyzer
            fnx-version.rust-src
            fnx-version.rustc

            # it's generally recommended to use nightly rustfmt
            fnx.complete.rustfmt
          ];
          all-components = std-components ++ extra-components;
        in
        fnx.combine all-components;

      stableRust = mkRustDeriv fnx.stable [ fnx.targets.wasm32-unknown-unknown.stable.rust-std ];
      nightlyRust = mkRustDeriv fnx.complete [ fnx.targets.wasm32-unknown-unknown.latest.rust-std ];

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
      nightlyPkgs = [ ];

      mkBevyShell =
        { name, packages }:
        pkgs.mkShell {
          name = mkName name;
          inherit packages;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath generalPkgs;
        };
    in
    {
      devShells = rec {
        default = stable;

        stable = mkBevyShell {
          name = "stable";
          packages = generalPkgs ++ [ stableRust ];
        };

        nightly = mkBevyShell {
          name = "nightly";
          packages = generalPkgs ++ nightlyPkgs ++ [ nightlyRust ];
        };

        # wip: make it actually faster
        fast = mkBevyShell {
          name = "fast nightly (wip)";
          packages = generalPkgs ++ nightlyPkgs ++ [ nightlyRust ];
        };
      };
    };
}
