{ pkgs ? import <nixpkgs> {} }:

with pkgs;

stdenv.mkDerivation {
  name = "sendgrid-amqp-bridge";
  buildInputs = [
    pkg-config
    openssl
  ];
}
