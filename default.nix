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
    cargoSha256 = "sha256-VK4DNjErLe3kniLH7eBRBfc8NFLeIXHk7+E9tKTuPvk=";

    nativeBuildInputs = with pkgs; [
      rustc # rust compiler
      cargo # cargo package manager
    ];
  }
