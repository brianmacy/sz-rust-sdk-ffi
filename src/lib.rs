//! `sz-rust-sdk-ffi` — Senzing low-level FFI/marshaling core.
//!
//! Shared by [`sz-rust-sdk`](https://github.com/brianmacy/sz-rust-sdk) and Senzing's in-tree Rust
//! binding layer (`senzing-sys` in the `G2` repo), so the raw C-ABI bindings and marshaling
//! helpers have exactly one source instead of two independently-drifting vendored copies.
//!
//! ## What this crate is
//! - [`bindings`] — bindgen-generated `extern "C"` declarations for the Senzing C ABI (checked in,
//!   regenerated manually via `cargo run --example generate_bindings`).
//! - [`helpers`] — `str`<->C marshaling helpers (buffer ownership/free via `SzHelper_free`).
//! - [`codegen`] (feature `codegen`) — a build-time helper that turns a `szerrors.json` file into
//!   generated Rust source for an `SzErrorCategory` enum + `classify(code)` taxonomy. Each
//!   consumer supplies its own `szerrors.json` path (they may legitimately track different Senzing
//!   versions) and compiles its own copy of the generated enum — only the *generation logic* is
//!   shared, not a single runtime type.
//!
//! ## What this crate is NOT
//! It does not define a public error/exception type. Consumers keep their own error model
//! (`senzing-sys::SzError` in G2, `sz_rust_sdk::error::SzError` in the Rust SDK) and build it on
//! top of the classification this crate's codegen produces.

// Generated code: silence the lints bindgen output trips (it's not hand-written).
#[allow(
    clippy::all,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
pub mod bindings;

pub mod helpers;

#[cfg(feature = "codegen")]
pub mod codegen;
