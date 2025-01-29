{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    let
      overlay = final: prev: {
        kibadda = (prev.kibadda or { }) // {
          passmenu = final.pkgs.rustPlatform.buildRustPackage {
            name = "passmenu";
            cargoHash = "sha256-NI87bRgwyxkdQZ3UKQ+qAfMSkjc/TNuqTEFu6yW10zk=";
            src = self;
          };
        };
      };

      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    flake-utils.lib.eachSystem supportedSystems (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            overlay
          ];
        };
      in
      {
        packages = rec {
          default = passmenu;
          inherit (pkgs.kibadda) passmenu;
        };

        devShells = {
          default = pkgs.mkShell {
            name = "passmenu-development-shell";
            buildInputs = with pkgs; [
              cargo
              rustc
              rustfmt
              rustPackages.clippy
            ];
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          };
        };
      }
    )
    // {
      overlays.default = overlay;
    };
}
