{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      naersk' = pkgs.callPackage naersk { };
      nativeBuildInputs = with pkgs; [ pkg-config glibc gtk3 ];

    in flake-utils.lib.eachDefaultSystem (system: rec {
      defaultPackage = naersk'.buildPackage {
        inherit nativeBuildInputs;
        src = ./.;
      };

      devShell = pkgs.mkShell {
        inherit nativeBuildInputs;
        name = "rust-env";
        src = ./.;
      };

      apps.default = {
        type = "app";
        program = "${defaultPackage}/bin/gtk-hello-world";
      };
    });
}
