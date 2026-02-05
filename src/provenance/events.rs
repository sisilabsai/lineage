use chrono::{DateTime, Utc};

use crate::provenance::{CustodianId, MetadataHash};

#[derive(Debug, Clone)]
pub enum CustodyEventType {
    Genesis {
        custodian: CustodianId,
        label: String,
        metadata_hash: MetadataHash,
    },
    Transfer {
        from: CustodianId,
        to: CustodianId,
    },
    Inspection {
        notes: String,
    },
    Seal {
        reason: String,
    },
}

#[derive(Debug, Clone)]
pub struct CustodyEvent {
    pub event_type: CustodyEventType,
    pub timestamp: DateTime<Utc>,
    pub prev_hash: Option<String>,
    pub hash: String,
}

impl CustodyEventType {
    pub fn digest_string(&self) -> String {
        match self {
            CustodyEventType::Genesis {
                custodian,
                label,
                metadata_hash,
            } => format!(
                "GENESIS|{}|{}|{}",
                custodian.as_str(),
                label,
                metadata_hash.as_hex()
            ),
            CustodyEventType::Transfer { from, to } => {
                format!("TRANSFER|{}|{}", from.as_str(), to.as_str())
            }
            CustodyEventType::Inspection { notes } => format!("INSPECTION|{}", notes),
            CustodyEventType::Seal { reason } => format!("SEAL|{}", reason),
        }
    }
}
