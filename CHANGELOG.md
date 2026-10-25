# Changelog

All notable changes to this crate are documented here.

## [Unreleased]

CI-only fix: the weekly scheduled Security Audit `vet` job had failed since 2026-08-10 because
it ran `cargo generate-lockfile` on every run, resolving the newest upstream patch releases
(e.g. libc 0.2.190, syn 3.0.6), which were not in `supply-chain/config.toml`. `Cargo.lock` is now
committed (pinned to the vetted versions) and the job runs `cargo vet --locked`. Added
`.github/dependabot.yml` (cargo + github-actions, daily, 21-day cooldown) matching sz-rust-sdk.
No source changes.

## [0.1.2] - 2026-07-30

CI-only fix: `cargo vet` always runs `cargo metadata --locked` internally regardless of whether
`--locked` is passed to `cargo vet` itself, so the security-audit workflow failed outright with no
committed `Cargo.lock` (this is a library crate, no lockfile committed per convention). Added a
`cargo generate-lockfile` step before it. No source changes.

## [0.1.1] - 2026-07-30

Split the `ffi` (bindings + helpers) and `codegen` features apart (`ffi` is now its own
default-on feature, separate from `codegen`). Fixes a Windows-only link failure: a
`codegen`-only consumer (a build.rs / build-dependency) that also compiled the unconditional
`bindings`/`helpers` modules would fail to link with `unresolved external symbol
SzHelper_free`, because MSVC does not reliably dead-strip an unreferenced-at-runtime extern
from a dependency crate the way macOS/Linux linkers do. Build-dependency consumers should now
use `default-features = false, features = ["codegen"]` — see README.

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
