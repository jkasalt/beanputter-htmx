{
  pkgs,
  fenix,
  crane,
  system,
}:

let
  target = "wasm32-unknown-unknown";
  toolchain =
    with fenix.packages.${system};
    combine [
      (default.toolchain.withComponents ["rustfmt"])
      targets.${target}.stable.rust-std
    ];
  craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
in
craneLib.buildPackage { src = ../.;}
