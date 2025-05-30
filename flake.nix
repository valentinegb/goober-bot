{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/b366c9ee8ee169da482aa724eed85a63824f4448";
  };

  outputs =
    { self, nixpkgs }:
    {

      packages = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (system: {
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

      formatter = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
        system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style
      );

      devShells = nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [ clippy ];
            inputsFrom = [ self.packages.${system}.goober-bot ];
          };
        }
      );

    };
}
