# Wasm package compiled from rust
To build the package, just run `make build`. The wasm binary will be placed under
`target/wasm32-unknown-unknown/release/wasm-example.wasm`. In my toolchain (stable 1.42)
The final .wasm size was 94989 bytes. This can probably be reduced further if we make the crate `no_std`

Please review all the files in this repo, including the `.cargo` directory.

You may need to install the `wasm32-unknown-unknown` if you haven't done so before:
```bash
rustup +stable target add wasm32-unknown-unknown
```
