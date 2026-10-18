// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Onix Research
// Earlier source grants remain effective; see NOTICE.

use content_identity_core::{Domain, IdentityError, IdentityHasher, LATTICE_TAGGED_LE_V1};

const DOMAIN: &str = "node-execution:v1";
const OTHER_DOMAIN: &str = "schedule-occurrence:v1";
const EXPECTED_DIGEST: &str = "6c7df34ed228dcd26c1d6232e64ff5b002b890f820a12acb27ea606f1e5976e4";
const NODE_ID: u64 = 7;
const COMMAND: &str = "printf hello";

fn legacy_lattice_sample_digest() -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(DOMAIN.as_bytes());
    hasher.update(b"node-id");
    hasher.update(&NODE_ID.to_le_bytes());
    hasher.update(b"command");
    let command_len = u64::try_from(COMMAND.len()).expect("command length fits u64");
    hasher.update(&command_len.to_le_bytes());
    hasher.update(COMMAND.as_bytes());
    hasher.update(b"pipe-stdin");
    hasher.update(b"true");
    hasher.finalize().to_hex().to_string()
}

fn sample_digest(domain: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut hasher = IdentityHasher::new(Domain::new(domain)?);
    hasher
        .update_tagged_u64_le(b"node-id", NODE_ID)
        .update_tagged_str(b"command", COMMAND)?
        .update_tagged_bool(b"pipe-stdin", true);
    Ok(hasher.finish().to_hex())
}

#[test]
fn lattice_encoding_has_a_fixed_positive_vector() -> Result<(), Box<dyn std::error::Error>> {
    let digest = sample_digest(DOMAIN)?;

    assert_eq!(digest, EXPECTED_DIGEST);
    assert_eq!(digest, legacy_lattice_sample_digest());
    Ok(())
}

#[test]
fn metadata_names_the_exact_encoding() -> Result<(), Box<dyn std::error::Error>> {
    let metadata = IdentityHasher::new(Domain::new(DOMAIN)?).finish_metadata();

    assert_eq!(metadata.domain, DOMAIN);
    assert_eq!(metadata.canonicalization, LATTICE_TAGGED_LE_V1);
    assert_eq!(metadata.digest.len(), blake3::OUT_LEN * 2);
    Ok(())
}

#[test]
fn wrong_domain_and_order_change_the_digest() -> Result<(), Box<dyn std::error::Error>> {
    let expected = sample_digest(DOMAIN)?;
    let other_domain = sample_digest(OTHER_DOMAIN)?;
    let mut reordered = IdentityHasher::new(Domain::new(DOMAIN)?);
    reordered
        .update_tagged_str(b"command", COMMAND)?
        .update_tagged_u64_le(b"node-id", NODE_ID)
        .update_tagged_bool(b"pipe-stdin", true);

    assert_ne!(expected, other_domain);
    assert_ne!(expected, reordered.finish().to_hex());
    Ok(())
}

#[test]
fn sorted_map_is_order_independent() -> Result<(), Box<dyn std::error::Error>> {
    let mut first = IdentityHasher::new(Domain::new(DOMAIN)?);
    first.update_sorted_map(b"env", &[("B", "2"), ("A", "1")])?;
    let mut second = IdentityHasher::new(Domain::new(DOMAIN)?);
    second.update_sorted_map(b"env", &[("A", "1"), ("B", "2")])?;

    assert_eq!(first.finish(), second.finish());
    Ok(())
}

#[test]
fn invalid_domains_and_duplicate_keys_fail_closed() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(Domain::new(""), Err(IdentityError::EmptyDomain));
    assert_eq!(
        Domain::new("lattice-schedule-occurrence-v1")?.as_str(),
        "lattice-schedule-occurrence-v1"
    );
    assert_eq!(
        Domain::new("seaglass.node-config.v1")?.as_str(),
        "seaglass.node-config.v1"
    );
    assert!(matches!(
        Domain::new("node-execution"),
        Err(IdentityError::UnversionedDomain { .. })
    ));
    assert!(matches!(
        Domain::new("vault-stream:v1"),
        Err(IdentityError::SecuritySensitiveDomain { .. })
    ));

    let mut hasher = IdentityHasher::new(Domain::new(DOMAIN)?);
    assert!(matches!(
        hasher.update_sorted_map(b"env", &[("A", "1"), ("A", "2")]),
        Err(IdentityError::DuplicateMapKey { .. })
    ));
    Ok(())
}
