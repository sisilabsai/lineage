//! Provenance module: immutable chain-of-custody with verifiable history.

mod asset;
mod events;
mod verify;
mod vault;

pub use asset::{Asset, AssetId, CustodianId, MetadataHash};
pub use events::{CustodyEvent, CustodyEventType};
pub use verify::{VerifyReport, VerifyStatus};
pub use vault::{ProvenanceConfig, ProvenanceError, ProvenanceVault};
