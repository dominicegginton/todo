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
    cargoSha256 = "sha256-cPJ+KRTRN52LQnOzsnEhURxysBzrIKXJs41rf1WyxSQ=";

    nativeBuildInputs = with pkgs; [
      rustc # rust compiler
      cargo # cargo package manager
    ];
  }
