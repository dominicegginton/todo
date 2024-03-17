# todo

```ocaml
Built With Nix
```

A suckless todo list manager written in Rust and built with Nix.

## Installation

#### Nix Shell

```sh
nix shell github:dominicegginton/todo#todo
```

#### Nix Flake

```nix
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    todo.url = "github:dominicegginton/todo";
  };

  outputs = {nixpkgs, todo, ...}: {
    nixosConfigurations = {
      example = nixpkgs.lib.nixosSystem rec {
        system = "x86_64-linux";

        modules = [
          # Directly referance the default package from the github:dominicegginton/todo flake
          {
            environment.systemPackages = [todo.defaultPackage.${system}];
          }
          # Or apply the overlay and referance the package from nixpkgs
          {
            environment.systemPackages = let
              pkgs = import nixpkgs {
                system = "x86_64-linux";
                overlays = [ todo.overlays.default ];
              };
            in with pkgs; [todo];
          }
        ];
      };
    };
  };
}
```
