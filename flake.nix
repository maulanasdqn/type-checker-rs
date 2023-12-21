{
  description = "TypeChecker RUST";

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
        default = type-checker;
        type-checker = pkgs.mkShell {
          name = "type-checker";
          packages = with pkgs; [
            openssl
            fontconfig
            rust-bin.beta.latest.default
            cargo-nextest
            rustup
            #self.packages.aarch64-darwin.type-checker
          ];
          #inputsFrom = [ self.packages.aarch64-darwin.type-checker ];
        };
      };

      #packages.aarch64-darwin = rec {
      #  default = type-checker;
      #  type-checker = pkgs.rustPlatform.buildRustPackage {
      #    pname = "type-checker";
      #    version = "v0.1.0";
      #    doCheck = true;
      #    src = ./.;
      #    nativeBuildInputs = with pkgs.darwin.apple_sdk.frameworks; [
      #      Security
      #      CoreFoundation
      #      CoreServices
      #      SystemConfiguration
      #    ];
      #    cargoLock = { lockFile = ./Cargo.lock; };
      #  };
      #};
    };
}
