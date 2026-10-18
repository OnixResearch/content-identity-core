# content-identity-core

`content-identity-core` provides `no_std` product-neutral BLAKE3 framing based on Lattice's tagged encoding.

Consumers own versioned domains, canonical projections, persistence, migrations, cache policy, evidence roles, authorization, and trust.

Security-sensitive hashing is excluded. Do not use this crate for vault streams, signatures, credentials, password hashing, or KDFs.

## Encoding

`lattice-tagged-le.v1` uses caller-owned byte tags, little-endian `u64` lengths, explicit option markers, deterministic map order, and full-width BLAKE3 digests.

## Source

Canonical Radicle RID: `rad:zA14jhTdwST4PxPBdsryc4zS7Kpq`.

Reviewed object: `7f55597b5dc879b7601856e8d7fd0dbacaa2a498`.

Read-only adapter: `https://seed.radicle.garden/zA14jhTdwST4PxPBdsryc4zS7Kpq.git`.

GitHub and sibling paths are not dependency fallbacks. See `SOURCE.md` and `docs/radicle-publication.md`.

## Checks

```sh
cargo fmt --all -- --check
cargo test --all-targets
cargo check --lib --target wasm32-unknown-unknown
cargo clippy --all-targets -- -D warnings
nix flake check -L
```

## License

Current and new owned code, contracts, schemas, fixtures, templates, documentation, and policy are licensed under **AGPL-3.0-or-later**. See `LICENSE` for the complete license and `NOTICE` for the current grant, attribution, and historical/third-party boundaries.

`LICENSE-MIT` retains the original extraction grant; it does not offer MIT terms for new owned contributions. Prior grants remain effective for content they already cover, including byte-identical content retained here. This source preparation does not publish a revision or change the canonical RID, reviewed objects, or consumer pins.
