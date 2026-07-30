# sz-rust-sdk-ffi

[![CI](https://github.com/brianmacy/sz-rust-sdk-ffi/actions/workflows/ci.yml/badge.svg)](https://github.com/brianmacy/sz-rust-sdk-ffi/actions/workflows/ci.yml)
[![Security Audit](https://github.com/brianmacy/sz-rust-sdk-ffi/actions/workflows/security.yml/badge.svg)](https://github.com/brianmacy/sz-rust-sdk-ffi/actions/workflows/security.yml)
[![Documentation](https://github.com/brianmacy/sz-rust-sdk-ffi/actions/workflows/docs.yml/badge.svg)](https://brianmacy.github.io/sz-rust-sdk-ffi/)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

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

## Usage

```toml
[dependencies]
sz-rust-sdk-ffi = { git = "https://github.com/brianmacy/sz-rust-sdk-ffi", rev = "<commit-sha>" }

[build-dependencies]
# default-features = false is important here: a build.rs generating the error taxonomy has no
# business linking FFI bindings, and on Windows an unused `ffi` feature's `SzHelper_free`
# reference isn't reliably dead-stripped, causing an unresolved-external link failure.
sz-rust-sdk-ffi = { git = "https://github.com/brianmacy/sz-rust-sdk-ffi", rev = "<commit-sha>", default-features = false, features = ["codegen"] }
```

Pin `rev` to a full 40-character commit SHA, not a floating branch/tag — this crate has no
stability guarantees between commits. A `vX.Y.Z` tag on the pinned commit is provided as a
human-readable marker only.

```rust
// build.rs
fn main() {
    let out = sz_rust_sdk_ffi::codegen::generate_error_taxonomy("szerrors.json".as_ref())
        .expect("codegen failed");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    std::fs::write(format!("{out_dir}/errors_generated.rs"), out).unwrap();
}
```

```rust
// lib.rs
include!(concat!(env!("OUT_DIR"), "/errors_generated.rs"));
// -> pub enum SzErrorCategory { .. }, pub fn classify(code: i32) -> SzErrorCategory, ...
```

## Regenerating bindings

```sh
cargo run --example generate_bindings
```

Requires an installed Senzing SDK (auto-detected via `SENZING_SDK_PATH`, `SENZING_DIR`, or the
standard Homebrew/Linux install locations). Regenerate after upgrading to a new Senzing SDK
version, then commit `src/bindings.rs`.

## Development

Unlike its consumers, this crate needs no live Senzing engine to build or test — the FFI
declarations resolve at a *consumer's* final-link time, and the `codegen` module only parses JSON.

```sh
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt -- --check
cargo deny check     # license/advisory/source policy (dev-dependency: cargo install cargo-deny)
cargo vet            # supply-chain review status (dev-dependency: cargo install cargo-vet)
```

## Consumers

- [`sz-rust-sdk`](https://github.com/brianmacy/sz-rust-sdk) — the standalone Rust SDK.
- `senzing-sys` in [`G2`](https://github.com/Senzing/G2) (`dev/apps/g2/rust/senzing-sys`) — Senzing's
  in-tree Rust binding layer (JNI, PyO3, C#).

## License

Apache-2.0
