{ pkgs }:
let
  config = {
    listen_addr = "127.0.0.1:8080";
  };
  configFile = pkgs.writeText "config" (builtins.toJSON config);
in
pkgs.writeShellScriptBin "run-dev-server" ''
  set -x
  exec ./target/debug/api ${configFile}
''
