# sz-rust-sdk-ffi

Senzing low-level FFI/marshaling core: C-ABI bindgen bindings + `str`<->C marshaling helpers +
error-taxonomy codegen, shared by [`sz-rust-sdk`](https://github.com/brianmacy/sz-rust-sdk) and
Senzing's in-tree Rust binding layer (`senzing-sys` in the `G2` product repo).

## Why this crate exists

Both consumers need to call the same Senzing C ABI and classify the same Senzing error codes.
Before this crate existed, each vendored its own copy of the bindgen output and its own
hand-rolled error-code classification, and the two copies silently drifted. This crate factors out
exactly the parts that are safe to share:

- **`bindings`** — bindgen-generated `extern "C"` declarations for the Senzing C ABI, checked in
  and regenerated manually (the C ABI is stable; see `examples/generate_bindings.rs`).
- **`helpers`** — `str`<->C marshaling helpers (buffer ownership/free via `SzHelper_free`).
- **`codegen`** (feature `codegen`) — a build-time helper that turns a `szerrors.json` file (shipped
  with every Senzing SDK install) into generated Rust source for an `SzErrorCategory` enum and a
  `classify(code)` function. Each consumer supplies its own `szerrors.json` path — they may
  legitimately track different Senzing versions — and compiles its own copy of the generated enum.
  Only the *generation logic* is shared, not a single runtime type or a single error API.

This crate intentionally does **not** define a public error/exception type. Consumers keep their
own error model and build it on top of the classification `codegen` produces.

## Regenerating bindings

```sh
cargo run --example generate_bindings
```

Requires an installed Senzing SDK (auto-detected via `SENZING_SDK_PATH`, `SENZING_DIR`, or the
standard Homebrew/Linux install locations). Regenerate after upgrading to a new Senzing SDK
version, then commit `src/bindings.rs`.

## License

Apache-2.0
