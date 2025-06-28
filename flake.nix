{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.treefmt-nix.flakeModule ];
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          pkgs,
          system,
          self',
          ...
        }:
        {
          treefmt.programs = {
            nixfmt.enable = true;
            rustfmt.enable = true;
            prettier.enable = true;
            taplo.enable = true;
          };

          packages.default = import ./nix/build.nix {
            inherit pkgs system;
            inherit (inputs) fenix crane;
          };

          devShells.default = pkgs.mkShell {
            inputsFrom = self'.packages.default.buildInputs;
            buildInputs = with pkgs; [
              dioxus-cli
              wasm-bindgen-cli
              lld
            ];
          };
        };
    };
}
