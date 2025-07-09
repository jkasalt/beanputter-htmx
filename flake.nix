{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-flake.url = "github:juspay/rust-flake";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
      ];
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

          rust-project = {
            crates."beanputter-htmx".crane.args = {
              nativeBuildInputs = with pkgs; [
                pkg-config
              ];
              buildInputs = with pkgs; [
                glib
                cairo
                pango
              ];
            };
          };

          packages.default = self'.packages.beanputter-htmx;

          devShells.default = pkgs.mkShell {
            inputsFrom = [ self'.devShells.rust ];
            buildInputs = with pkgs; [
              dioxus-cli
              wasm-bindgen-cli
              lld
              bacon
              tailwindcss_4
            ];
          };
        };
    };
}
