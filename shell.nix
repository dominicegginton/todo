{pkgs, ...}:
with pkgs;
  mkShell rec {
    nativeBuildInputs = with pkgs; [
      rustc # rust compiler
      cargo # cargo package manager
      rust-analyzer # rust language server
      rustfmt # rust code formatter
    ];

    shellHook = ''
      export CARGO_HOME=$PWD/.cargo
      export PATH=$CARGO_HOME/bin:$PATH
    '';
  }
