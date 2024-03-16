{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    supportedSystems = ["x86_64-linux" "i686-linux" "x86_64-darwin"];

    forAllSystems = f:
      nixpkgs.lib.genAttrs supportedSystems (system: f system);

    nixpkgsFor = forAllSystems (system:
      import nixpkgs {
        inherit system;

        overlays = [self.overlays.default];
      });
  in {
    overlays = {
      default = final: prev: let
        pkgs = final.pkgs;
        rustPlatform = final.rustPlatform;
      in {
        todo = import ./default.nix {inherit pkgs rustPlatform;};
      };
    };

    packages = forAllSystems (system: let
      pkgs = nixpkgsFor.${system};
    in {
      inherit (pkgs) todo;

      default = pkgs.todo;
    });

    formatter = forAllSystems (
      system: let
        pkgs = nixpkgsFor.${system};
      in
        pkgs.alejandra
    );

    devShells = forAllSystems (system: let
      pkgs = nixpkgsFor.${system};
    in {
      default = import ./shell.nix {inherit pkgs;};
    });
  };
}
