{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
   buildInputs = with pkgs; [
      rustc
      cargo
      pkg-config
      openssl
   ];

   PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
