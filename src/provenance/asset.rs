use chrono::{DateTime, Utc};

use crate::identity::Identity;
use crate::provenance::events::CustodyEvent;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssetId(String);

impl AssetId {
    pub fn from_identity(identity: &Identity) -> Self {
        Self(identity.id().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustodianId(String);

impl CustodianId {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetadataHash(String);

impl MetadataHash {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let hash = hasher.finalize();
        Self(format!("{:x}", hash))
    }

    pub fn as_hex(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Asset {
    id: Identity,
    label: String,
    metadata_hash: MetadataHash,
    created_at: DateTime<Utc>,
    current_custodian: CustodianId,
    sealed: bool,
    seal_reason: Option<String>,
    events: Vec<CustodyEvent>,
}

impl Asset {
    pub fn new(label: String, metadata_hash: MetadataHash, custodian: CustodianId) -> Self {
        Self {
            id: Identity::create(),
            label,
            metadata_hash,
            created_at: Utc::now(),
            current_custodian: custodian,
            sealed: false,
            seal_reason: None,
            events: Vec::new(),
        }
    }

    pub fn id(&self) -> &Identity {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn metadata_hash(&self) -> &MetadataHash {
        &self.metadata_hash
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn current_custodian(&self) -> &CustodianId {
        &self.current_custodian
    }

    pub fn is_sealed(&self) -> bool {
        self.sealed
    }

    pub fn seal_reason(&self) -> Option<&str> {
        self.seal_reason.as_deref()
    }

    pub fn events(&self) -> &[CustodyEvent] {
        &self.events
    }

    pub(crate) fn push_event(&mut self, event: CustodyEvent) {
        self.events.push(event);
    }

    pub(crate) fn set_custodian(&mut self, custodian: CustodianId) {
        self.current_custodian = custodian;
    }

    pub(crate) fn seal(&mut self, reason: String) {
        self.sealed = true;
        self.seal_reason = Some(reason);
    }
}
