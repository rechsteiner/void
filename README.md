# VOID

A toy, code-driven space game to explore the great void. Built in Rust
with a custom entity-component system and scripting interpreter,
players program their spacecraft using a purpose-built language to
navigate through space.

## Installation

Void requires the nightly version of Rust:

```
rustup toolchain install nightly
rustup override set nightly
```

## How to run the project

- Run the continous WASM build with `make watch`
- Front-end continous build `make serve`

## How to play

Enter code to control the ship.

### Example ship code

Find examples of ship programs in the [examples folder](./examples/)

## Authors

- Martin Rechsteiner ([@rechsteiner](https://github.com/rechsteiner/))
- Alexander Vanvik ([@avanvik](https://github.com/avanvik))
