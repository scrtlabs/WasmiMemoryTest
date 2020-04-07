We import memory from rust to WASM, then try to see what happens when WASM code imports memory vs. exports memory.

According to wasmi it doesn't matter, according to WASM it does, so lets see what happens.

Run with:

```bash
(cd rust_contract && make) && cargo run
```
