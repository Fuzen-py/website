{ nixpkgs ? import <nixpkgs> {} }:
nixpkgs.callPackage ./website.nix {}