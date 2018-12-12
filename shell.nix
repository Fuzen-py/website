with import <nixpkgs> {};
pkgs.mkShell {
    buildInputs = [
        rust.cargo
    ];
}