{
  description = "nixek api devshell and utils";

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    devShell = import ./shell.nix { inherit pkgs; };

    run-dev-server = import ./api/run-dev-server.nix { inherit pkgs; };
  };
}
