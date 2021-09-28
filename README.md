# VOID

A code-driven space program to explore the great void.

## Installation

Void requires the nightly version of Rust:

```
rustup toolchain install nightly
rustup override set nightly
```

## How to run the project

- Run the continous WASM build with `cargo watch -w "src" -s "wasm-pack build"`
- Front-end continous build `npm run serve`

## How to play

Enter code to control the ship

### Example ship code

Find examples of ship programs in the [examples folder](./examples/)
