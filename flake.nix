{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/4faa5f5321320e49a78ae7848582f684d64783e9";
  };

  outputs =
    { self, nixpkgs }:
    {

      packages = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (system: {
        goober-bot = nixpkgs.legacyPackages.${system}.rustPlatform.buildRustPackage {
          pname = "goober-bot";
          version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;

          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;
        };
        default = self.packages.${system}.goober-bot;
      });

      formatter = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
        system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style
      );

      devShells = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (system: {
        default = nixpkgs.legacyPackages.${system}.mkShell {
          inputsFrom = [ self.packages.${system}.goober-bot ];
        };
      });

    };
}
