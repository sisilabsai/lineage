use crate::provenance::Asset;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyStatus {
    Valid,
    Invalid,
}

#[derive(Debug, Clone)]
pub struct VerifyReport {
    pub status: VerifyStatus,
    pub errors: Vec<String>,
}

impl VerifyReport {
    pub fn valid() -> Self {
        Self {
            status: VerifyStatus::Valid,
            errors: Vec::new(),
        }
    }

    pub fn invalid(errors: Vec<String>) -> Self {
        Self {
            status: VerifyStatus::Invalid,
            errors,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.status == VerifyStatus::Valid
    }
}

pub(crate) fn verify_chain(asset: &Asset) -> VerifyReport {
    use sha2::{Digest, Sha256};

    let mut errors = Vec::new();
    let mut previous_hash: Option<String> = None;

    for (index, event) in asset.events().iter().enumerate() {
        if event.prev_hash != previous_hash {
            errors.push(format!(
                "Event {} prev hash mismatch (expected {:?}, got {:?})",
                index, previous_hash, event.prev_hash
            ));
        }

        let mut hasher = Sha256::new();
        if let Some(prev) = &event.prev_hash {
            hasher.update(prev.as_bytes());
        }
        hasher.update(event.event_type.digest_string().as_bytes());
        hasher.update(event.timestamp.to_rfc3339().as_bytes());
        let computed = format!("{:x}", hasher.finalize());

        if computed != event.hash {
            errors.push(format!(
                "Event {} hash mismatch (expected {}, got {})",
                index, computed, event.hash
            ));
        }

        previous_hash = Some(event.hash.clone());
    }

    if errors.is_empty() {
        VerifyReport::valid()
    } else {
        VerifyReport::invalid(errors)
    }
}
