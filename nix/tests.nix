{ inputs, ... }:
{
  perSystem =
    {
      self',
      pkgs,
      system,
      ...
    }:
    {
      packages = {
        exec-tests = pkgs.writeShellApplication {
          name = "run-tests";
          runtimeInputs = [
            self'.packages.rust
            pkgs.cargo-nextest
            pkgs.clang
          ];
          text = ''
            cargo nextest --version
            cargo nextest run --release
            echo SUCCESS
          '';
        };
        exec-deps-check = pkgs.writeShellApplication {
          name = "check-deps";
          runtimeInputs = [
            self'.packages.rust
            pkgs.cargo-machete
            pkgs.clang
          ];
          text = ''
            cargo machete --version
            cargo machete
            echo SUCCESS
          '';
        };
        exec-doc-coverage = pkgs.writeShellApplication {
          name = "check-docs";
          runtimeInputs = [
            self'.packages.rust-nightly
            pkgs.clang
          ];
          text = ''
            RUSTDOCFLAGS='-Z unstable-options --show-coverage' cargo doc
            echo SUCCESS
          '';
        };
      };
    };
}
