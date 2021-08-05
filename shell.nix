{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [
    protobuf
  ];
  shellHook = ''
    export PROTOC=${protobuf}/bin/protoc
  '';
}
