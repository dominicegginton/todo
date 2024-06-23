{pkgs}:
with pkgs.rustPlatform;
with pkgs.lib;
with builtins;
let
  cargoToml = fromTOML (readFile ./Cargo.toml);
in
  buildRustPackage {
    pname =  cargoToml.package.name;
    version = cargoToml.package.version;
    src = cleanSource ./.;
    cargoSha256 = "sha256-ow2im5UdpyiUhXqdlUqTf1AQYx1ElGQKlATGdujLQOc=";
    nativeBuildInputs = with pkgs; [
      rustc
      cargo
    ];
  }
