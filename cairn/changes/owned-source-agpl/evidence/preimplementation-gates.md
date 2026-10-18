# Preimplementation licensing gate evidence

All commands ran from `/home/brittonr/git/OnixResearch/content-identity-core`
using `/tmp/kiln-agpl-cairn-a22-target/debug/cairn`, built by the parent from
unmodified exact Cairn `a22ea2bff65f16abec4f0f7ba2d7ddc14dc35871` source
`/nix/store/n35psykrlygqazck2lavw09vg37vqli7-source` (source NAR
`sha256-KrCOV4I/R7ET3Se9uN9lLv1XmfnGWYBMTMMEgs5Ey64=`).

The unchanged complete owning policy was explicitly selected with
`--root . --policy cairn-policy/generated/cairn-policy.json`. Existing classic
`cairn/` lifecycle content was retained without migration. Policy hash:
`03014c51aac92f53696ea78c0ec6901711b6ece1d112a644e7b0e9a2f5b317f0`.

| Command | Observed result | Receipt hash |
|---|---|---|
| `cairn gate proposal owned-source-agpl` | PASS, valid true, no issues | `ebc2873799c1d24a84bbc0e303d40bd20a082c5fc1e190f547deebf2bbfe0652` |
| `cairn gate design owned-source-agpl` | PASS, valid true, no issues | `f26f05a71e8b6a381fa959d295d5eba1e9bd606090ac8af13c6d132a3f6bf1c7` |
| `cairn gate tasks owned-source-agpl` | PASS, valid true, no issues | `54e1b6b5da4c7ba9126cd1d63ed51d65046e416a2bc8af5d48d76a74b565462f` |

All three gates ran before product metadata edits. They establish structural
preimplementation admission only, not legal correctness, publication, or
release readiness. The BLAKE3 inventory correction reflects the existing
Cargo declaration `=1.8.6`; the dependency itself was not changed. Historical
extraction authorization, source identities, dependency pins, and prior grants
remain intact. The full primary AGPL text, explicit current-source notice, and
historical MIT text accompany the updated metadata.

Final coordinated licensing and packaging verification remains parent-owned
and pending. No builds, tests, lint, formatters, or broad validation ran during
implementation. The change remains open with verification unchecked.
