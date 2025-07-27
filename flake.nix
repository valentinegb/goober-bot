{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    {

      packages =
        nixpkgs.lib.genAttrs (nixpkgs.lib.remove "x86_64-freebsd" nixpkgs.lib.systems.flakeExposed)
          (system: {
            goober-bot = nixpkgs.legacyPackages.${system}.rustPlatform.buildRustPackage {
              pname = "goober-bot";
              version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
              src = ./.;
              cargoLock = {
                lockFile = ./Cargo.lock;
                allowBuiltinFetchGit = true;
              };
            };

            default = self.packages.${system}.goober-bot;
          });

    };
}
