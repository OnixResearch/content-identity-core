// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2026 Onix Research
// Earlier source grants remain effective; see NOTICE.

#![no_std]
#![forbid(unsafe_code)]

//! Product-neutral BLAKE3 identity framing based on Lattice's tagged encoding.
//!
//! Consumers own domain labels, canonical projections, persistence, migration,
//! trust, access control, evidence roles, and all security-sensitive hashing.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use core::fmt;

use serde::{Deserialize, Serialize};

pub const ALGORITHM: &str = "blake3-256";
pub const LATTICE_TAGGED_LE_V1: &str = "lattice-tagged-le.v1";

const VERSION_MARKERS: &[&str] = &[":v", ".v", "-v"];
const SECRET_DOMAIN_MARKERS: &[&str] = &[
    "credential",
    "kdf",
    "password",
    "secret",
    "signature",
    "vault",
];
const TRUE_MARKER: &[u8] = b"true";
const FALSE_MARKER: &[u8] = b"false";
const SOME_MARKER: &[u8] = b"some";
const NONE_MARKER: &[u8] = b"none";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityError {
    EmptyDomain,
    UnversionedDomain { domain: String },
    SecuritySensitiveDomain { domain: String },
    LengthOverflow,
    DuplicateMapKey { key: String },
}

impl fmt::Display for IdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDomain => formatter.write_str("identity domain must not be empty"),
            Self::UnversionedDomain { domain } => {
                write!(
                    formatter,
                    "identity domain must end with a recognized version suffix: {domain}"
                )
            }
            Self::SecuritySensitiveDomain { domain } => write!(
                formatter,
                "security-sensitive hashing is outside content identity: {domain}"
            ),
            Self::LengthOverflow => formatter.write_str("identity input length exceeds u64"),
            Self::DuplicateMapKey { key } => {
                write!(formatter, "identity map contains duplicate key: {key}")
            }
        }
    }
}

impl core::error::Error for IdentityError {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Domain(String);

impl Domain {
    pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(IdentityError::EmptyDomain);
        }
        if !has_version_suffix(trimmed) {
            return Err(IdentityError::UnversionedDomain {
                domain: trimmed.to_string(),
            });
        }
        let lowered = trimmed.to_ascii_lowercase();
        if SECRET_DOMAIN_MARKERS
            .iter()
            .any(|marker| lowered.contains(marker))
        {
            return Err(IdentityError::SecuritySensitiveDomain {
                domain: trimmed.to_string(),
            });
        }
        Ok(Self(trimmed.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Digest([u8; blake3::OUT_LEN]);

impl Digest {
    #[must_use]
    pub const fn from_bytes(bytes: [u8; blake3::OUT_LEN]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; blake3::OUT_LEN] {
        &self.0
    }

    #[must_use]
    pub fn to_hex(self) -> String {
        blake3::Hash::from_bytes(self.0).to_hex().to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityMetadata {
    pub algorithm: String,
    pub domain: String,
    pub canonicalization: String,
    pub digest: String,
}

pub struct IdentityHasher {
    domain: Domain,
    hasher: blake3::Hasher,
}

impl IdentityHasher {
    #[must_use]
    pub fn new(domain: Domain) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(domain.as_str().as_bytes());
        Self { domain, hasher }
    }

    pub fn update_raw(&mut self, value: &[u8]) -> &mut Self {
        self.hasher.update(value);
        self
    }

    pub fn update_len_le(&mut self, value: usize) -> Result<&mut Self, IdentityError> {
        let value = u64::try_from(value).map_err(|_| IdentityError::LengthOverflow)?;
        self.hasher.update(&value.to_le_bytes());
        Ok(self)
    }

    pub fn update_tagged_bytes(
        &mut self,
        tag: &[u8],
        value: &[u8],
    ) -> Result<&mut Self, IdentityError> {
        self.hasher.update(tag);
        self.update_len_le(value.len())?;
        self.hasher.update(value);
        Ok(self)
    }

    pub fn update_tagged_str(
        &mut self,
        tag: &[u8],
        value: &str,
    ) -> Result<&mut Self, IdentityError> {
        self.update_tagged_bytes(tag, value.as_bytes())
    }

    pub fn update_tagged_u64_le(&mut self, tag: &[u8], value: u64) -> &mut Self {
        self.hasher.update(tag);
        self.hasher.update(&value.to_le_bytes());
        self
    }

    pub fn update_tagged_i64_be(&mut self, tag: &[u8], value: i64) -> &mut Self {
        self.hasher.update(tag);
        self.hasher.update(&value.to_be_bytes());
        self
    }

    pub fn update_tagged_bool(&mut self, tag: &[u8], value: bool) -> &mut Self {
        self.hasher.update(tag);
        self.hasher
            .update(if value { TRUE_MARKER } else { FALSE_MARKER });
        self
    }

    pub fn update_optional_str(
        &mut self,
        tag: &[u8],
        value: Option<&str>,
    ) -> Result<&mut Self, IdentityError> {
        self.hasher.update(tag);
        match value {
            Some(value) => {
                self.hasher.update(SOME_MARKER);
                self.update_tagged_str(b"value", value)?;
            }
            None => {
                self.hasher.update(NONE_MARKER);
            }
        }
        Ok(self)
    }

    pub fn update_ordered_strs(
        &mut self,
        tag: &[u8],
        values: &[&str],
    ) -> Result<&mut Self, IdentityError> {
        self.hasher.update(tag);
        self.update_len_le(values.len())?;
        for value in values {
            self.update_tagged_str(b"item", value)?;
        }
        Ok(self)
    }

    pub fn update_sorted_strs(
        &mut self,
        tag: &[u8],
        values: &[&str],
    ) -> Result<&mut Self, IdentityError> {
        let mut sorted = values.to_vec();
        sorted.sort_unstable();
        self.update_ordered_strs(tag, &sorted)
    }

    pub fn update_sorted_map(
        &mut self,
        tag: &[u8],
        entries: &[(&str, &str)],
    ) -> Result<&mut Self, IdentityError> {
        let mut sorted = BTreeMap::new();
        for (key, value) in entries {
            if sorted.insert(*key, *value).is_some() {
                return Err(IdentityError::DuplicateMapKey {
                    key: (*key).to_string(),
                });
            }
        }
        self.hasher.update(tag);
        self.update_len_le(sorted.len())?;
        for (key, value) in sorted {
            self.update_tagged_str(b"key", key)?;
            self.update_tagged_str(b"value", value)?;
        }
        Ok(self)
    }

    pub fn update_hashed_part(&mut self, tag: &[u8], value: &[u8]) -> &mut Self {
        let mut part = blake3::Hasher::new();
        part.update(tag);
        part.update(value);
        self.hasher.update(part.finalize().as_bytes());
        self
    }

    #[must_use]
    pub fn finish(self) -> Digest {
        Digest::from_bytes(*self.hasher.finalize().as_bytes())
    }

    #[must_use]
    pub fn finish_metadata(self) -> IdentityMetadata {
        let domain = self.domain.as_str().to_string();
        let digest = self.finish().to_hex();
        IdentityMetadata {
            algorithm: ALGORITHM.to_string(),
            domain,
            canonicalization: LATTICE_TAGGED_LE_V1.to_string(),
            digest,
        }
    }
}

fn has_version_suffix(value: &str) -> bool {
    VERSION_MARKERS.iter().any(|marker| {
        value.rsplit_once(marker).is_some_and(|(_, suffix)| {
            !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())
        })
    })
}
