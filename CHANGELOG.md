# Changelog

All notable changes to this crate are documented here.

## [0.1.0] - 2026-07-30

Initial release. Extracted from:
- The FFI/marshaling layer (`bindings`, `helpers`) vendored into `senzing-sys`
  (Senzing's in-tree Rust binding layer, in the `G2` product repo) from
  `sz-rust-sdk` during the 4.4 binding-layer work, which had since drifted from
  its origin (different visibility, different bindgen flags).
- The error-taxonomy codegen (`codegen::generate_error_taxonomy`) that
  `senzing-sys` already generated cleanly from `szerrors.json`, now shared so
  `sz-rust-sdk` can drive its own richer `SzError` enum from the same
  classification decision instead of independently re-implementing it.

Both `senzing-sys` and `sz-rust-sdk` depend on this crate via a git dependency
pinned to a full commit SHA.
