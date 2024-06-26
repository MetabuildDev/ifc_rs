{ inputs, ... }:
{
  perSystem =
    {
      self',
      pkgs,
      lib,
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

            pkgs.git
            pkgs.ripgrep
            pkgs.clang
            pkgs.bc
          ];
          text = ''
            coverage=$(RUSTDOCFLAGS='-Z unstable-options --show-coverage' cargo doc 2>&1 | rg "Documenting ifc_rs " -A 9999)
            echo Fully documented: 
            echo "$coverage" | rg "100.0%"
            echo Needs further documentation: 
            echo "$coverage" | rg --invert-match "100.0%"

            branch=$(git rev-parse --abbrev-ref HEAD)
            base=$(git merge-base HEAD origin/main)
            echo "branch: $branch"

            echo "checkout base"
            git checkout "$base"
            echo "calculate percentage before"
            before=$(RUSTDOCFLAGS='-Z unstable-options --show-coverage' cargo doc 2>&1 | rg "Documenting ifc_rs " -A 9999 | rg Total | cut -d '|' --fields=4 | xargs)
            echo "checkout branch"
            git checkout "$branch"
            echo "calculate percentage after"
            after=$(RUSTDOCFLAGS='-Z unstable-options --show-coverage' cargo doc 2>&1 | rg "Documenting ifc_rs " -A 9999 | rg Total | cut -d '|' --fields=4 | xargs)

            echo "before: $before, after: $after"

            a=$(echo "$after" | awk '{print $1/100}')
            b=$(echo "$before" | awk '{print $1/100}')

            if echo "$a>=$b" | bc -l; then
              echo NO DOCUMENTATION REGRESSIONS
              exit 0
            else 
              echo REGRESSIONS, PLEASE CHECK EVALUATION ABOVE
              exit 1
            fi
          '';
        };
      };
    };
}
