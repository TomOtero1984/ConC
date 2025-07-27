{
  description = "ConC build environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            pkgs.rustup
            pkgs.cargo
            pkgs.pkg-config
            pkgs.openssl
            pkgs.cmake
            pkgs.wasm-pack
            pkgs.nodejs_20
          ];

          shellHook = ''
            rustup default stable
            rustup target add wasm32-unknown-unknown
            echo "✅ ConC devShell ready on ${system}"
          '';
        };

        # Placeholder default package so `nix build` works
        packages.default = pkgs.hello;
      });
}
