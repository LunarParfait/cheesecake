{
  description = "cheesecake CLI";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {

    defaultPackage.${system} =
      pkgs.stdenv.mkDerivation {
        name = "cake";
        src = self;
        nativeBuildInputs = with pkgs; [ cargo pkg-config openssl ];
        buildInputs = with pkgs; [ ];
        buildPhase = "make build";
        installPhase = "make install";
        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      };

    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [ cargo rustfmt bacon rust-analyzer pkg-config
      openssl ];
      PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    };
  };
}

