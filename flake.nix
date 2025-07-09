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
    proc-flake.url = "github:srid/proc-flake";
    flake-root.url = "github:srid/flake-root";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.rust-flake.flakeModules.default
        inputs.rust-flake.flakeModules.nixpkgs
        inputs.proc-flake.flakeModule
        inputs.flake-root.flakeModule
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
          config,
          ...
        }:
        {
          proc.groups.run.processes = {
            dioxus-serve.command = "dx serve --port 42069";
            tailwind.command = "tailwindcss --watch -i ./input.css -o ./assets/tailwind.css";
          };

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
              config.proc.groups.run.package
              dioxus-cli
              wasm-bindgen-cli
              lld
              bacon
              tailwindcss_4
              watchman
            ];
          };
        };
    };
}
