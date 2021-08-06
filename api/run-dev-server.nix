{ pkgs }:
let
  config = {
    listen_addr = "127.0.0.1:8080";

    server_cert = builtins.readFile ./dev/server.pem;
    server_key = builtins.readFile ./dev/server-key.pem;
    client_ca_cert = builtins.readFile ./dev/ca.pem;
  };
  configFile = pkgs.writeText "config" (builtins.toJSON config);
in
pkgs.writeShellScriptBin "run-dev-server" ''
  set -x
  exec ./target/debug/api ${configFile}
''
