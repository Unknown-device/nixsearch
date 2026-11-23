{
  description = "NixOS package search TUI";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSystem = f:
        nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forEachSystem (pkgs: rec {
        default = nixsearch;
        nixsearch = pkgs.rustPlatform.buildRustPackage {
          pname = "nixsearch";
          version = "0.1.0";
          src = self;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = [ pkgs.makeWrapper ];
          postInstall = ''
            wrapProgram $out/bin/nixsearch \
              --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.nix-search-cli ]}
          '';

          meta = with pkgs.lib; {
            description = "TUI for searching nixpkgs";
            license = licenses.mit;
            mainProgram = "nixsearch";
          };
        };
      });

      apps = nixpkgs.lib.genAttrs systems (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/nixsearch";
          meta = { description = "TUI for searching nixpkgs"; };
        };
      });

      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [ cargo rustc rustfmt clippy nix-search-cli ];
        };
      });
    };
}
