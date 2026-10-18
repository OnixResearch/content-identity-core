// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Onix Research
// Earlier source grants remain effective; see NOTICE.

const FORBIDDEN_TERMS: [&str; 11] = [
    "seaglass.pipeline",
    "lattice.workflow",
    "keyed_hash",
    "password_hash",
    "vault_stream",
    "signature(",
    "evidence_identity",
    "authorization",
    "path = \"../",
    "github.com/OnixResearch",
    "std::",
];

fn forbidden_term<'a>(candidate: &'a str, forbidden: &'a [&str]) -> Option<&'a str> {
    forbidden
        .iter()
        .copied()
        .find(|term| candidate.contains(term))
}

#[test]
fn shared_source_excludes_product_and_security_authority() {
    let candidate = concat!(include_str!("../Cargo.toml"), include_str!("../src/lib.rs"));

    assert_eq!(forbidden_term(candidate, &FORBIDDEN_TERMS), None);
}

#[test]
fn guard_rejects_security_and_sibling_coupling() {
    assert_eq!(
        forbidden_term("blake3::keyed_hash(key, value)", &FORBIDDEN_TERMS),
        Some("keyed_hash")
    );
    assert_eq!(
        forbidden_term("helper = { path = \"../helper\" }", &FORBIDDEN_TERMS),
        Some("path = \"../")
    );
    assert_eq!(
        forbidden_term("use std::fs;", &FORBIDDEN_TERMS),
        Some("std::")
    );
}
