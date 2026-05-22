{
  description = "at32f4xx-pac SVD->Rust generation environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        # `form` is the rust crate from svd2rust ecosystem (splits the giant
        # mod.rs into per-peripheral files). Nixpkgs has a `form` package but
        # it is the unrelated symbolic-math tool, so build from crates.io.
        form = pkgs.rustPlatform.buildRustPackage rec {
          pname = "form";
          version = "0.13.0";
          src = pkgs.fetchCrate {
            inherit pname version;
            sha256 = "0sf5g2msd5dzyaaqwnzghkfmgjc9x2mpawlp6cacvqzv4c9lgvpg";
          };
          cargoHash = "sha256-ItNBQKye1GD01KFBubMLxksv8OCWIxya/LlZ9g6Jdg8=";
          doCheck = false;
        };

        # Toolchain pinned to match what is already baked into committed
        # sources (svd2rust v0.37.1 — see header doc-comment in src/at32fXXX/mod.rs).
        genTools = [
          pkgs.svd2rust   # 0.37.1 in current nixpkgs
          pkgs.svdtools
          form
          pkgs.cargo
          pkgs.rustc
          pkgs.rustfmt
          pkgs.unzip
          pkgs.gnused
          pkgs.findutils
        ];
      in {
        devShells.default = pkgs.mkShell {
          packages = genTools;
          shellHook = ''
            echo "at32f4xx-pac generation shell"
            echo "  svd2rust: $(svd2rust --version 2>&1 | head -1)"
            echo "  svdtools: $(svdtools --version 2>&1 | head -1)"
            echo "  form:     $(form --version 2>&1 | head -1)"
            echo "Run: ./generate.sh"
          '';
        };

        # `nix run .#generate` -> regenerate src/ from PACKs+patches in-place.
        apps.generate = {
          type = "app";
          program = toString (pkgs.writeShellScript "at32f4xx-pac-generate" ''
            export PATH=${pkgs.lib.makeBinPath genTools}:$PATH
            exec ./generate.sh "$@"
          '');
        };

        packages.form = form;
      });
}
