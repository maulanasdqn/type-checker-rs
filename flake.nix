{
  description =
    "Programa de linha de comando para navegar entre os desafios, corrigí-los e dar dicas";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      pkgs = import nixpkgs {
        system = "aarch64-darwin";
        overlays = [ (import rust-overlay) ];
      };
    in {
      devShells.aarch64-darwin = rec {
        default = pescarte-desafios;
        pescarte-desafios = pkgs.mkShell {
          name = "pescarte-desafios";
          packages = with pkgs; [
            rust-bin.stable.latest.default
            cargo-nextest
            self.packages.aarch64-darwin.pescarte-desafios
          ];
          inputsFrom = [ self.packages.aarch64-darwin.pescarte-desafios ];
          shellHook =
            ''alias pescarte-desafios="./target/debug/pescarte-desafios"'';
        };
      };

      packages.aarch64-darwin = rec {
        default = pescarte-desafios;
        pescarte-desafios = pkgs.rustPlatform.buildRustPackage {
          pname = "pescarte-desafios";
          version = "v0.1.0";
          doCheck = true;
          src = ./.;
          nativeBuildInputs = with pkgs.darwin.apple_sdk; [
            frameworks.CoreFoundation
            frameworks.CoreServices
            frameworks.SystemConfiguration
          ];
          singleStep = true;
          cargoLock = { lockFile = ./Cargo.lock; };
          meta = with pkgs.lib; {
            homepage = "htpps://github.com/peapescarte/desafios/blob/main/cli";
            license = licenses.bsd3;
            maintainers = [ maintainers.zoedsoupe ];
          };
        };
      };
    };
}
