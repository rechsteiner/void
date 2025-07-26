{ pkgs ? import <nixpkgs> {} }:

with pkgs; mkShell {
    buildInputs = [
        wasm-pack
        watchexec
        nodejs
        esbuild
        rust-analyzer
    ];
}
