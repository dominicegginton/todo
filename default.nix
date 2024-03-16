{
  pkgs,
  rustPlatform,
  ...
}:
with rustPlatform;
  buildRustPackage rec {
    name = "todo";
    pname = "todo";
    version = "0.1.0";
    src = ./.;
    cargoSha256 = "sha256-AcZcnDZrIso+TKDCKnXDXbPgjKG4SUhsBLq1li7hRsI=";

    nativeBuildInputs = with pkgs; [
      rustc # rust compiler
      cargo # cargo package manager
      pkg-config # for finding dependencies
      patchelf # for fixing rpath
    ];
  }
