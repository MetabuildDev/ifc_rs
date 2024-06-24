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
      };
    };
}
