{ nixpkgs ? <nixpkgs> }:

let
  pkgs = import nixpkgs {
    overlays = [ ];
  };
in
pkgs.mkShell {
  buildInputs = [
    pkgs.wasm-pack
    pkgs.nodejs-12_x
  ];
}
