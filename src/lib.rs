//! `sz-rust-sdk-ffi` — Senzing low-level FFI/marshaling core.
//!
//! Shared by [`sz-rust-sdk`](https://github.com/brianmacy/sz-rust-sdk) and Senzing's in-tree Rust
//! binding layer (`senzing-sys` in the `G2` repo), so the raw C-ABI bindings and marshaling
//! helpers have exactly one source instead of two independently-drifting vendored copies.
//!
//! ## What this crate is
//! - [`bindings`] (feature `ffi`, default-on) — bindgen-generated `extern "C"` declarations for the
//!   Senzing C ABI (checked in, regenerated manually via `cargo run --example generate_bindings`).
//! - [`helpers`] (feature `ffi`, default-on) — `str`<->C marshaling helpers (buffer ownership/free
//!   via `SzHelper_free`).
//! - [`logging`] (feature `ffi`) — bindgen declarations for `libSzLogging.h` (SzLogging callbacks,
//!   levels, formats, default output), as released in Senzing 4.5.0 (unchanged since 4.3.0).
//! - [`codegen`] (feature `codegen`) — a build-time helper that turns a `szerrors.json` file into
//!   generated Rust source for an `SzErrorCategory` enum + `classify(code)` taxonomy. Each
//!   consumer supplies its own `szerrors.json` path (they may legitimately track different Senzing
//!   versions) and compiles its own copy of the generated enum — only the *generation logic* is
//!   shared, not a single runtime type.
//!
//! A build-dependency consumer that only needs `codegen` (a build.rs generating the error
//! taxonomy) should depend with `default-features = false, features = ["codegen"]` to avoid
//! compiling `ffi` at all -- see the `ffi` feature's doc comment in `Cargo.toml` for why this
//! matters on Windows.
//!
//! ## What this crate is NOT
//! It does not define a public error/exception type. Consumers keep their own error model
//! (`senzing-sys::SzError` in G2, `sz_rust_sdk::error::SzError` in the Rust SDK) and build it on
//! top of the classification this crate's codegen produces.

#[cfg(feature = "ffi")]
// Generated code: silence the lints bindgen output trips (it's not hand-written).
#[allow(
    clippy::all,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
pub mod bindings;

#[cfg(feature = "ffi")]
pub mod helpers;

/// bindgen-generated declarations for `libSzLogging.h` (feature `ffi`): callback registration,
/// unregistration, levels, the default formatter and the default output.
///
/// Generated from the Senzing 4.5.0 header, whose nine `SzLogging_*` functions are unchanged since
/// 4.3.0. Only functions present in the released header are declared: a declaration for a symbol
/// missing from libSz can break linking (notably on Windows). Kept in its own module because its
/// `SzLogLevel` / `SzLogFormat` / `SzLogCallback` names would clash with consumer types of the
/// same names. Regenerated with the rest by `cargo run --example generate_bindings`.
#[cfg(feature = "ffi")]
// Generated code: silence the lints bindgen output trips (it's not hand-written).
#[allow(
    clippy::all,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    rustdoc::broken_intra_doc_links
)]
pub mod logging;

#[cfg(feature = "codegen")]
pub mod codegen;
