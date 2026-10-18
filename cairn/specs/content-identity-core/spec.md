## Purpose

Define a narrow product-neutral BLAKE3 framing boundary based on Lattice's encoding.

## Requirements

### Requirement: Framing is deterministic
r[content_identity_core.framing] Versioned domains, tags, lengths, ordered sequences, deterministic maps, and option markers MUST produce full-width BLAKE3 digests.

#### Scenario: Same framed input repeats
r[content_identity_core.framing.scenario.repeat]
- GIVEN the same domain and framed values
- WHEN identity generation repeats
- THEN the digest MUST be identical

### Requirement: Invalid identity input fails closed
r[content_identity_core.validation] Empty or unversioned domains, security-sensitive domains, duplicate map keys, and length overflow MUST fail.

#### Scenario: Security domain is supplied
r[content_identity_core.validation.scenario.security]
- GIVEN a vault, signature, credential, password, secret, or KDF domain
- WHEN domain validation runs
- THEN the domain MUST be rejected

### Requirement: Product authority remains outside
r[content_identity_core.boundary] The library MUST NOT own product domains, projections, persistence, migration, cache policy, evidence roles, authorization, or trust.

#### Scenario: Consumer uses shared framing
r[content_identity_core.boundary.scenario.consumer]
- GIVEN a consumer has an eligible identity
- WHEN it uses shared framing
- THEN it MUST retain its domain and projection authority

### Requirement: Radicle is canonical
r[content_identity_core.publication] Consumers MUST pin the canonical read-only Radicle source and exact reviewed object without automatic fallback.

#### Scenario: Source differs
r[content_identity_core.publication.scenario.reject]
- GIVEN a branch, sibling path, GitHub URL, unknown RID, or different object
- WHEN source admission runs
- THEN admission MUST fail
