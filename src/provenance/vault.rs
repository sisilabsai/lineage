use std::collections::HashMap;

use chrono::Utc;
use sha2::{Digest, Sha256};

use crate::lineage::{Lineage, OperationResult};
use crate::provenance::asset::{Asset, AssetId, CustodianId, MetadataHash};
use crate::provenance::events::{CustodyEvent, CustodyEventType};
use crate::provenance::verify::{verify_chain, VerifyReport};

#[derive(Debug, Clone)]
pub struct ProvenanceConfig {
    pub initial_energy: u64,
    pub operation_cost: u64,
}

impl Default for ProvenanceConfig {
    fn default() -> Self {
        Self {
            initial_energy: 1_000_000,
            operation_cost: 1,
        }
    }
}

pub struct ProvenanceVault {
    lineage: Lineage,
    config: ProvenanceConfig,
    assets: HashMap<AssetId, Asset>,
}

impl ProvenanceVault {
    pub fn new() -> Self {
        Self::with_config(ProvenanceConfig::default())
    }

    pub fn with_config(config: ProvenanceConfig) -> Self {
        Self {
            lineage: Lineage::create(config.initial_energy),
            config,
            assets: HashMap::new(),
        }
    }

    pub fn create_asset(
        &mut self,
        label: String,
        metadata_hash: MetadataHash,
        custodian: String,
    ) -> Result<AssetId, ProvenanceError> {
        self.consume_energy("Create asset", self.config.operation_cost)?;

        let custodian = CustodianId::new(custodian);
        let mut asset = Asset::new(label, metadata_hash.clone(), custodian.clone());
        let asset_id = AssetId::from_identity(asset.id());

        let event = self.build_event(None, CustodyEventType::Genesis {
            custodian,
            label: asset.label().to_string(),
            metadata_hash,
        });
        asset.push_event(event);

        self.assets.insert(asset_id.clone(), asset);
        Ok(asset_id)
    }

    pub fn transfer(
        &mut self,
        asset_id: &AssetId,
        from: String,
        to: String,
        energy_cost: u64,
    ) -> Result<(), ProvenanceError> {
        let (current_custodian, prev_hash, sealed) = {
            let asset = self
                .assets
                .get(asset_id)
                .ok_or(ProvenanceError::AssetNotFound)?;
            (
                asset.current_custodian().as_str().to_string(),
                asset.events().last().map(|e| e.hash.clone()),
                asset.is_sealed(),
            )
        };

        if sealed {
            return Err(ProvenanceError::AssetSealed);
        }

        if current_custodian != from {
            return Err(ProvenanceError::CustodianMismatch {
                expected: current_custodian,
                provided: from,
            });
        }

        self.consume_energy("Transfer custody", energy_cost)?;

        let from_id = CustodianId::new(current_custodian);
        let to_id = CustodianId::new(to);
        let event = self.build_event(prev_hash, CustodyEventType::Transfer {
            from: from_id,
            to: to_id.clone(),
        });
        let asset = self
            .assets
            .get_mut(asset_id)
            .ok_or(ProvenanceError::AssetNotFound)?;
        asset.push_event(event);
        asset.set_custodian(to_id);
        Ok(())
    }

    pub fn record_event(
        &mut self,
        asset_id: &AssetId,
        event_type: CustodyEventType,
    ) -> Result<(), ProvenanceError> {
        let (prev_hash, sealed) = {
            let asset = self
                .assets
                .get(asset_id)
                .ok_or(ProvenanceError::AssetNotFound)?;
            (
                asset.events().last().map(|e| e.hash.clone()),
                asset.is_sealed(),
            )
        };

        if sealed {
            return Err(ProvenanceError::AssetSealed);
        }

        self.consume_energy("Record custody event", self.config.operation_cost)?;

        let event = self.build_event(prev_hash, event_type);
        let asset = self
            .assets
            .get_mut(asset_id)
            .ok_or(ProvenanceError::AssetNotFound)?;
        asset.push_event(event);
        Ok(())
    }

    pub fn seal(&mut self, asset_id: &AssetId, reason: String) -> Result<(), ProvenanceError> {
        let (prev_hash, sealed) = {
            let asset = self
                .assets
                .get(asset_id)
                .ok_or(ProvenanceError::AssetNotFound)?;
            (
                asset.events().last().map(|e| e.hash.clone()),
                asset.is_sealed(),
            )
        };

        if sealed {
            return Err(ProvenanceError::AssetSealed);
        }

        self.consume_energy("Seal asset", self.config.operation_cost)?;

        let event = self.build_event(prev_hash, CustodyEventType::Seal { reason: reason.clone() });
        let asset = self
            .assets
            .get_mut(asset_id)
            .ok_or(ProvenanceError::AssetNotFound)?;
        asset.push_event(event);
        asset.seal(reason);
        Ok(())
    }

    pub fn asset(&self, asset_id: &AssetId) -> Option<&Asset> {
        self.assets.get(asset_id)
    }

    pub fn verify(&self, asset_id: &AssetId) -> Result<VerifyReport, ProvenanceError> {
        let asset = self
            .assets
            .get(asset_id)
            .ok_or(ProvenanceError::AssetNotFound)?;
        Ok(verify_chain(asset))
    }

    fn consume_energy(&mut self, description: &str, cost: u64) -> Result<(), ProvenanceError> {
        match self.lineage.perform_operation(description.to_string(), cost) {
            OperationResult::Success { .. } => Ok(()),
            OperationResult::InsufficientEnergy { required, available } => {
                Err(ProvenanceError::InsufficientEnergy { required, available })
            }
            OperationResult::Dead => Err(ProvenanceError::VaultTerminated),
            OperationResult::OntologicalViolation { reason } => {
                Err(ProvenanceError::OntologicalViolation(reason))
            }
        }
    }

    fn build_event(
        &self,
        prev_hash: Option<String>,
        event_type: CustodyEventType,
    ) -> CustodyEvent {
        let timestamp = Utc::now();
        let mut hasher = Sha256::new();
        if let Some(prev) = &prev_hash {
            hasher.update(prev.as_bytes());
        }
        hasher.update(event_type.digest_string().as_bytes());
        hasher.update(timestamp.to_rfc3339().as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        CustodyEvent {
            event_type,
            timestamp,
            prev_hash,
            hash,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProvenanceError {
    AssetNotFound,
    AssetSealed,
    CustodianMismatch { expected: String, provided: String },
    InsufficientEnergy { required: u64, available: u64 },
    VaultTerminated,
    OntologicalViolation(String),
}

impl std::fmt::Display for ProvenanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvenanceError::AssetNotFound => write!(f, "Asset not found"),
            ProvenanceError::AssetSealed => write!(f, "Asset is sealed"),
            ProvenanceError::CustodianMismatch { expected, provided } => write!(
                f,
                "Custodian mismatch (expected {}, provided {})",
                expected, provided
            ),
            ProvenanceError::InsufficientEnergy { required, available } => write!(
                f,
                "Insufficient energy: required {}, available {}",
                required, available
            ),
            ProvenanceError::VaultTerminated => write!(f, "Provenance vault is terminated"),
            ProvenanceError::OntologicalViolation(reason) => write!(f, "{}", reason),
        }
    }
}

impl std::error::Error for ProvenanceError {}
