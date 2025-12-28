# Goober Bot, the Discord bot
# Copyright (C) 2025  Valentine Briese
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published
# by the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.
#
# You may contact me via electronic mail at <valentinegb@icloud.com>.

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
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
              meta.mainProgram = "goober-bot";
            };

            default = self.packages.${system}.goober-bot;
          });

      nixosModules = {
        goober-bot =
          {
            lib,
            config,
            pkgs,
            ...
          }:
          {
            options = {
              services.goober-bot = {
                enable = lib.mkEnableOption "goober-bot";
                token = lib.mkOption { type = lib.types.str; };
              };
            };
            config = lib.mkIf config.services.goober-bot.enable {
              systemd.services.goober-bot = {
                wantedBy = [ "multi-user.target" ];
                after = [ "network.target" ];
                environment.GOOBER_BOT_DISCORD_TOKEN = config.services.goober-bot.token;
                serviceConfig = {
                  ExecStart = lib.getExe self.packages.${pkgs.stdenv.hostPlatform.system}.goober-bot;
                  Restart = "always";
                };
              };
            };
          };

        default = self.nixosModules.goober-bot;
      };

    };
}
