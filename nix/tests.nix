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
            pkgs.ripgrep
            pkgs.clang
          ];
          text = ''
            coverage=$(RUSTDOCFLAGS='-Z unstable-options --show-coverage' cargo doc 2>&1 | rg "Documenting ifc_rs " -A 9999)
            echo Fully documented: 
            echo "$coverage" | rg "100.0%"
            echo Needs further documentation: 
            echo "$coverage" | rg --invert-match "100.0%"
          '';
        };
      };
    };
}
