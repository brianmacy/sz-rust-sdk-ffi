# CLAUDE.md

Guidance for Claude Code (or any agent) working in this repository.

## What this crate is

`sz-rust-sdk-ffi` is a **narrow, low-level** shared building block — bindgen C-ABI declarations,
`str`<->C marshaling helpers, and a `szerrors.json`-driven error-taxonomy codegen helper. It has
exactly two consumers: [`sz-rust-sdk`](https://github.com/brianmacy/sz-rust-sdk) and Senzing's
in-tree Rust binding layer (`senzing-sys` in the `G2` product repo). See `README.md` for the "why."

## Non-goals — do not add these here

- **No public error/exception type.** Each consumer keeps its own error model
  (`sz_rust_sdk::error::SzError`, `senzing_sys::error::SzError`) built on top of this crate's
  `codegen::classify()` output. Do not try to unify those types into this crate — they intentionally
  have different shapes (one is a rich ~15-variant hierarchy, the other a flat struct) and unifying
  them would be a breaking API change for `sz-rust-sdk`'s existing users.
- **No runtime dependency on a specific Senzing version's `szerrors.json`.** The `codegen` module
  takes a path and returns generated source text; it does not embed or ship a `szerrors.json` of its
  own. Each consumer supplies its own copy (they may legitimately track different Senzing SDK
  versions) and compiles its own generated `SzErrorCategory` enum.
- **No SDK-level application logic.** Return-code checking macros, `ResponseBuffer`-style handle
  wrappers, retry/backoff policy, etc. belong in the consumer, not here.

## Conventions

- **Bindgen output is checked in, not build-time generated.** Run
  `cargo run --example generate_bindings` manually after a Senzing header change or SDK upgrade,
  review the diff, and commit `src/bindings.rs`.
- **`Cargo.lock` IS committed** (same as sz-rust-sdk) and `security.yml`'s `vet` job runs
  `cargo vet --locked`. Without a committed lockfile the job regenerated one each run, so every new
  upstream patch release drifted past the versions in `supply-chain/config.toml` and the weekly
  scheduled audit went red. Dependabot (`.github/dependabot.yml`) proposes lockfile bumps; each bump
  must update the matching `supply-chain` entry in the same PR.
- **Dependents pin `rev` to a full 40-character commit SHA**, never a floating branch/tag. Tag the
  commit with a human-readable `vX.Y.Z` marker for convenience, but the tag is not what's pinned.
- **License policy** (`deny.toml`): MIT, Apache-2.0, Unlicense, Unicode-3.0, Zlib only — matches the
  Senzing product's own third-party dependency policy. Run `cargo deny check` before adding any new
  dependency.
- **`cargo vet`**: run `cargo vet` after any dependency change; if it fails, either add a
  certification/exemption (`cargo vet certify`) with a real justification, or reconsider the
  dependency. Don't blanket-exempt without looking at what you're exempting.

## Verification

No live Senzing engine is needed to build, test, or lint this crate — see `README.md`'s
"Development" section for the exact commands. If you ever add code that DOES need to link against
`libSz` at test time, that's a signal it belongs in a consumer, not here.
