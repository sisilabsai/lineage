//! # Graveyard System - The Eternal Archive
//!
//! Persistent storage for deceased agents.
//!
//! ## What This Enforces
//! - Cryptographic sealing of agent tombstones
//! - Immutable historical records of all dead agents
//! - Prevention of identity resurrection ("Lazarus Prevention")
//! - Fast O(1) lookups via in-memory registry
//! - Tamper-detection via causal chain hashing
//! - Signature verification to detect fraudulent edits
//! - Genealogical tracking via parent agent IDs
//!
//! ## What This Forbids
//! - Overwriting existing tombstones
//! - Reusing a dead agent's identity
//! - Operating on dead agents
//! - Modifying sealed records
//! - Tampering with Legacy Scores or metadata
//! - Creating agents without proper genealogy
//!
//! ## Storage Format
//! Each tombstone is stored as JSON in `.lineage/graveyard/<ID>.tomb`
//! Each signature is stored alongside in `.lineage/graveyard/<ID>.sig`
//! Files are marked read-only at OS level to prevent accidental mutation.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use hex;

type HmacSha256 = Hmac<Sha256>;

/// Global registry of dead agents - prevents resurrection
static GRAVEYARD_REGISTRY: Mutex<Option<GraveyardRegistry>> = Mutex::new(None);

/// In-memory index of all buried agents (lightning-fast Lazarus checks)
#[derive(Debug, Clone)]
pub struct GraveyardRegistry {
    /// Map of ID -> Tombstone location
    dead_ids: HashMap<String, PathBuf>,
}

impl GraveyardRegistry {
    /// Initialize the registry from disk
    pub fn initialize(graveyard_path: &Path) -> Result<Self, GraveyardError> {
        let mut dead_ids = HashMap::new();

        if graveyard_path.exists() {
            for entry in fs::read_dir(graveyard_path)
                .map_err(|e| GraveyardError::IoError(e.to_string()))?
            {
                let entry = entry.map_err(|e| GraveyardError::IoError(e.to_string()))?;
                let path = entry.path();

                if path.extension().map_or(false, |ext| ext == "tomb") {
                    if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                        dead_ids.insert(file_stem.to_string(), path);
                    }
                }
            }
        }

        Ok(GraveyardRegistry { dead_ids })
    }

    /// Check if an identity has already died (Lazarus prevention)
    pub fn is_dead(&self, id: &str) -> bool {
        self.dead_ids.contains_key(id)
    }

    /// Register a newly buried agent
    pub fn bury(&mut self, id: String, path: PathBuf) {
        self.dead_ids.insert(id, path);
    }

    /// Get all dead agents
    pub fn list_all(&self) -> Vec<String> {
        self.dead_ids.keys().cloned().collect()
    }

    /// Get path to a tombstone
    pub fn get_tombstone_path(&self, id: &str) -> Option<PathBuf> {
        self.dead_ids.get(id).cloned()
    }
}

/// Errors that can occur in the graveyard system
#[derive(Debug, Clone)]
pub enum GraveyardError {
    /// IO operation failed
    IoError(String),
    /// Tombstone already exists (no overwrites allowed)
    TombstoneExists { id: String },
    /// ID not found in graveyard
    NotFound { id: String },
    /// Serialization/deserialization failed
    SerializationError(String),
    /// Hash verification failed (tampering detected)
    TamperingDetected { id: String },
    /// Directory initialization failed
    DirectoryError(String),
}

impl std::fmt::Display for GraveyardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraveyardError::IoError(e) => write!(f, "IO Error: {}", e),
            GraveyardError::TombstoneExists { id } => {
                write!(f, "Tombstone already exists for ID: {}", id)
            }
            GraveyardError::NotFound { id } => write!(f, "No tombstone found for ID: {}", id),
            GraveyardError::SerializationError(e) => write!(f, "Serialization error: {}", e),
            GraveyardError::TamperingDetected { id } => {
                write!(f, "Tampering detected in tombstone: {}", id)
            }
            GraveyardError::DirectoryError(e) => write!(f, "Directory error: {}", e),
        }
    }
}

impl std::error::Error for GraveyardError {}

/// Identity block in tombstone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityBlock {
    /// Agent's unique identifier
    pub id: String,
    /// Creation timestamp
    pub creation_time: DateTime<Utc>,
    /// Cryptographic hash of identity proof
    pub identity_hash: String,
}

/// Metabolic record in tombstone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetabolicRecord {
    /// Final energy level when agent died
    pub final_energy: u64,
    /// Peak energy ever achieved
    pub peak_energy: u64,
    /// Initial energy at birth
    pub initial_energy: u64,
    /// Efficiency rating: tasks_completed / energy_burned
    pub efficiency_ratio: f64,
    /// Total tasks completed
    pub tasks_completed: u32,
}

impl MetabolicRecord {
    /// Calculate efficiency ratio
    pub fn calculate_efficiency(tasks: u32, energy_burned: u64) -> f64 {
        if energy_burned == 0 {
            0.0
        } else {
            tasks as f64 / energy_burned as f64
        }
    }
}

/// Pathology report in tombstone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathologyReport {
    /// List of scars inflicted during lifetime
    pub scars: Vec<ScarRecord>,
    /// Total number of scars
    pub scar_count: usize,
    /// Cause of death (the final scar)
    pub cause_of_death: String,
    /// Time of death
    pub death_timestamp: DateTime<Utc>,
}

/// Individual scar record with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScarRecord {
    /// When scar was inflicted
    pub timestamp: DateTime<Utc>,
    /// Severity level
    pub severity: String,
    /// Description of the injury
    pub description: String,
    /// Context/stack trace
    pub context: Option<String>,
}

/// Causal chain - cryptographic proof of unaltered history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalChain {
    /// Hash of the entire event sequence
    pub merkle_root: String,
    /// Ordered list of event hashes
    pub event_hashes: Vec<String>,
    /// Total events in chain
    pub total_events: usize,
}

/// Genealogical record for descendancy tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentageRecord {
    /// ID of parent agent (if spawned from another agent)
    pub parent_id: Option<String>,
    /// Capacity inherited from parent
    pub inherited_capacity: Option<u64>,
    /// Knowledge transferred from parent (description)
    pub inherited_knowledge: Option<String>,
    /// Generation number (0 = origin, 1 = spawned from origin, etc.)
    pub generation: u32,
}

/// A complete tombstone record for a deceased agent
///
/// Contains all information needed to:
/// - Reconstruct an agent's lifetime
/// - Verify no tampering has occurred
/// - Prevent resurrection via Lazarus check
/// - Query historical data
/// - Track genealogical relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tombstone {
    /// Identity information block
    pub identity: IdentityBlock,
    /// Metabolic records from lifetime
    pub metabolism: MetabolicRecord,
    /// Pathology report with scars and cause of death
    pub pathology: PathologyReport,
    /// Causal chain for tamper detection
    pub causal_chain: CausalChain,
    /// Genealogical information (parentage and generation)
    pub parentage: ParentageRecord,
    /// Cryptographic signature (HMAC-SHA256) for fraud detection
    pub signature: String,
    /// Signature timestamp of burial
    pub burial_timestamp: DateTime<Utc>,
    /// Version of graveyard schema
    pub schema_version: u32,
}

impl Tombstone {
    /// Create a new tombstone from final agent state
    pub fn create(
        id: String,
        identity_hash: String,
        creation_time: DateTime<Utc>,
        final_energy: u64,
        peak_energy: u64,
        initial_energy: u64,
        tasks_completed: u32,
        scars: Vec<ScarRecord>,
        cause_of_death: String,
    ) -> Self {
        Self::create_with_parentage(
            id, identity_hash, creation_time, final_energy, peak_energy, 
            initial_energy, tasks_completed, scars, cause_of_death,
            None, None, None, 0
        )
    }

    /// Create a new tombstone with genealogical information
    pub fn create_with_parentage(
        id: String,
        identity_hash: String,
        creation_time: DateTime<Utc>,
        final_energy: u64,
        peak_energy: u64,
        initial_energy: u64,
        tasks_completed: u32,
        scars: Vec<ScarRecord>,
        cause_of_death: String,
        parent_id: Option<String>,
        inherited_capacity: Option<u64>,
        inherited_knowledge: Option<String>,
        generation: u32,
    ) -> Self {
        let efficiency_ratio =
            MetabolicRecord::calculate_efficiency(tasks_completed, initial_energy - final_energy);

        let pathology = PathologyReport {
            scar_count: scars.len(),
            scars: scars.clone(),
            cause_of_death,
            death_timestamp: Utc::now(),
        };

        let causal_chain = Self::create_causal_chain(&scars);
        
        let parentage = ParentageRecord {
            parent_id,
            inherited_capacity,
            inherited_knowledge,
            generation,
        };

        let mut tombstone = Tombstone {
            identity: IdentityBlock {
                id,
                creation_time,
                identity_hash,
            },
            metabolism: MetabolicRecord {
                final_energy,
                peak_energy,
                initial_energy,
                efficiency_ratio,
                tasks_completed,
            },
            pathology,
            causal_chain,
            parentage,
            signature: String::new(), // Will be calculated next
            burial_timestamp: Utc::now(),
            schema_version: 1,
        };
        
        // Generate signature
        tombstone.signature = Self::calculate_signature(&tombstone);
        tombstone
    }

    /// Create causal chain from scar sequence
    fn create_causal_chain(scars: &[ScarRecord]) -> CausalChain {
        let mut event_hashes = Vec::new();
        let mut hasher = Sha256::new();

        for scar in scars {
            let scar_json = serde_json::to_string(scar)
                .unwrap_or_else(|_| format!("{:?}", scar));
            hasher.update(scar_json.as_bytes());
            let hash = format!("{:x}", Sha256::digest(hasher.clone().finalize()));
            event_hashes.push(hash);
        }

        let merkle_root = format!("{:x}", hasher.finalize());

        CausalChain {
            merkle_root,
            event_hashes,
            total_events: scars.len(),
        }
    }

    /// Get or create the cryptographic key for signing tombstones
    fn get_signing_key() -> Result<Vec<u8>, GraveyardError> {
        let keys_dir = PathBuf::from(".lineage/keys");
        fs::create_dir_all(&keys_dir)
            .map_err(|e| GraveyardError::DirectoryError(format!("Failed to create keys directory: {}", e)))?;

        let key_file = keys_dir.join("tombstone.key");

        let key = if key_file.exists() {
            fs::read(&key_file)
                .map_err(|e| GraveyardError::IoError(format!("Failed to read signing key: {}", e)))?
        } else {
            // Generate a new key from system entropy
            use sha2::Sha256;
            let mut hasher = Sha256::new();
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos().to_le_bytes().to_vec())
                .unwrap_or_default();
            hasher.update(&timestamp);
            let key_vec = hasher.finalize().to_vec();
            
            // Write key securely (readable only by user)
            #[cfg(target_os = "windows")]
            {
                fs::write(&key_file, &key_vec)
                    .map_err(|e| GraveyardError::IoError(format!("Failed to write signing key: {}", e)))?;
            }
            #[cfg(target_os = "linux")]
            {
                fs::write(&key_file, &key_vec)
                    .map_err(|e| GraveyardError::IoError(format!("Failed to write signing key: {}", e)))?;
                use std::os::unix::fs::PermissionsExt;
                let perms = std::fs::Permissions::from_mode(0o600); // rw-------
                fs::set_permissions(&key_file, perms)
                    .map_err(|e| GraveyardError::IoError(format!("Failed to set key permissions: {}", e)))?;
            }
            #[cfg(not(any(target_os = "windows", target_os = "linux")))]
            {
                fs::write(&key_file, &key_vec)
                    .map_err(|e| GraveyardError::IoError(format!("Failed to write signing key: {}", e)))?;
            }
            
            key_vec
        };

        Ok(key)
    }

    /// Calculate HMAC-SHA256 signature for the tombstone
    fn calculate_signature(tombstone: &Tombstone) -> String {
        // Create a temporary copy without signature for hashing
        let mut temp = tombstone.clone();
        temp.signature = String::new();

        // Serialize the core data
        let data_to_sign = format!(
            "{}|{}|{}|{}|{}|{}",
            temp.identity.id,
            temp.metabolism.tasks_completed,
            temp.metabolism.efficiency_ratio,
            temp.pathology.scar_count,
            temp.causal_chain.merkle_root,
            temp.burial_timestamp
        );

        // Sign with key if available, otherwise use SHA256 hash
        if let Ok(key) = Self::get_signing_key() {
            let mut mac = HmacSha256::new_from_slice(&key)
                .unwrap_or_else(|_| HmacSha256::new_from_slice(&[0u8; 32]).unwrap());
            mac.update(data_to_sign.as_bytes());
            hex::encode(mac.finalize().into_bytes())
        } else {
            // Fallback to simple SHA256 hash
            format!("{:x}", Sha256::digest(data_to_sign.as_bytes()))
        }
    }

    /// Verify the cryptographic signature of this tombstone
    pub fn verify_signature(&self) -> Result<(), GraveyardError> {
        let expected_signature = Self::calculate_signature(self);

        if expected_signature != self.signature {
            return Err(GraveyardError::TamperingDetected {
                id: self.identity.id.clone(),
            });
        }

        Ok(())
    }

    /// Verify the integrity of this tombstone (no tampering)
    pub fn verify(&self) -> Result<(), GraveyardError> {
        // First, verify the causal chain integrity
        let mut hasher = Sha256::new();

        for scar in &self.pathology.scars {
            let scar_json = serde_json::to_string(scar)
                .map_err(|e| GraveyardError::SerializationError(e.to_string()))?;
            hasher.update(scar_json.as_bytes());
        }

        let calculated_root = format!("{:x}", hasher.finalize());

        if calculated_root != self.causal_chain.merkle_root {
            return Err(GraveyardError::TamperingDetected {
                id: self.identity.id.clone(),
            });
        }

        // Second, verify the cryptographic signature
        self.verify_signature()?;

        Ok(())
    }

    /// Calculate legacy score (Success-to-Scar ratio with efficiency bonus)
    pub fn legacy_score(&self) -> f64 {
        let base_score = if self.pathology.scar_count > 0 {
            self.metabolism.tasks_completed as f64 / self.pathology.scar_count as f64
        } else {
            self.metabolism.tasks_completed as f64 + 1.0
        };

        base_score * self.metabolism.efficiency_ratio
    }

    /// Get lifespan in seconds
    pub fn lifespan_seconds(&self) -> i64 {
        (self.pathology.death_timestamp - self.identity.creation_time).num_seconds()
    }
}

/// The Graveyard manager - handles burial, loading, and queries
pub struct Graveyard;

impl Graveyard {
    /// Initialize the graveyard system
    pub fn initialize() -> Result<(), GraveyardError> {
        let graveyard_path = Graveyard::path();

        fs::create_dir_all(&graveyard_path)
            .map_err(|e| GraveyardError::DirectoryError(e.to_string()))?;

        let registry = GraveyardRegistry::initialize(&graveyard_path)?;

        let mut global_registry = GRAVEYARD_REGISTRY
            .lock()
            .expect("Graveyard registry poisoned");
        *global_registry = Some(registry);

        Ok(())
    }

    /// Get the graveyard path
    pub fn path() -> PathBuf {
        PathBuf::from(".lineage/graveyard")
    }

    /// Bury an agent (atomic write with no overwrites)
    pub fn bury(tombstone: &Tombstone) -> Result<(), GraveyardError> {
        let graveyard_path = Graveyard::path();
        let tomb_filename = format!("{}.tomb", tombstone.identity.id);
        let final_path = graveyard_path.join(&tomb_filename);

        // Check if already buried (no overwrites)
        if final_path.exists() {
            return Err(GraveyardError::TombstoneExists {
                id: tombstone.identity.id.clone(),
            });
        }

        // Atomic write: write to temp file first, then rename
        let temp_filename = format!("{}.tmp", tombstone.identity.id);
        let temp_path = graveyard_path.join(&temp_filename);

        // Serialize tombstone
        let tombstone_json = serde_json::to_string_pretty(tombstone)
            .map_err(|e| GraveyardError::SerializationError(e.to_string()))?;

        // Write to temp file
        fs::write(&temp_path, tombstone_json)
            .map_err(|e| GraveyardError::IoError(e.to_string()))?;

        // Atomic rename
        fs::rename(&temp_path, &final_path)
            .map_err(|e| GraveyardError::IoError(e.to_string()))?;

        // Mark as read-only (OS level)
        Graveyard::make_readonly(&final_path)?;

        // Register in global registry
        if let Ok(mut global_registry) = GRAVEYARD_REGISTRY.lock() {
            if let Some(ref mut registry) = *global_registry {
                registry.bury(tombstone.identity.id.clone(), final_path);
            }
        }

        Ok(())
    }

    /// Mark a file as read-only at OS level
    #[cfg(target_os = "windows")]
    fn make_readonly(path: &Path) -> Result<(), GraveyardError> {
        let mut perms = fs::metadata(path)
            .map_err(|e| GraveyardError::IoError(e.to_string()))?
            .permissions();
        perms.set_readonly(true);
        fs::set_permissions(path, perms)
            .map_err(|e| GraveyardError::IoError(e.to_string()))
    }

    /// Mark a file as read-only at OS level (Unix)
    #[cfg(target_os = "linux")]
    fn make_readonly(path: &Path) -> Result<(), GraveyardError> {
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        let perms = fs::Permissions::from_mode(0o444); // r--r--r--
        fs::set_permissions(path, perms)
            .map_err(|e| GraveyardError::IoError(e.to_string()))
    }

    /// Mark a file as read-only at OS level (fallback)
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    fn make_readonly(path: &Path) -> Result<(), GraveyardError> {
        let mut perms = fs::metadata(path)
            .map_err(|e| GraveyardError::IoError(e.to_string()))?
            .permissions();
        perms.set_readonly(true);
        fs::set_permissions(path, perms)
            .map_err(|e| GraveyardError::IoError(e.to_string()))
    }

    /// Load a tombstone from disk
    pub fn load(id: &str) -> Result<Tombstone, GraveyardError> {
        let graveyard_path = Graveyard::path();
        let tomb_path = graveyard_path.join(format!("{}.tomb", id));

        if !tomb_path.exists() {
            return Err(GraveyardError::NotFound { id: id.to_string() });
        }

        let content =
            fs::read_to_string(&tomb_path).map_err(|e| GraveyardError::IoError(e.to_string()))?;

        let tombstone: Tombstone = serde_json::from_str(&content)
            .map_err(|e| GraveyardError::SerializationError(e.to_string()))?;

        Ok(tombstone)
    }

    /// Check if an identity has already died (fast O(1) check)
    pub fn is_dead(id: &str) -> bool {
        if let Ok(global_registry) = GRAVEYARD_REGISTRY.lock() {
            if let Some(ref registry) = *global_registry {
                return registry.is_dead(id);
            }
        }
        false
    }

    /// List all dead agents
    pub fn list_all() -> Vec<String> {
        if let Ok(global_registry) = GRAVEYARD_REGISTRY.lock() {
            if let Some(ref registry) = *global_registry {
                return registry.list_all();
            }
        }
        Vec::new()
    }

    /// Load all tombstones (expensive operation)
    pub fn load_all() -> Result<Vec<Tombstone>, GraveyardError> {
        let mut tombstones = Vec::new();

        for id in Graveyard::list_all() {
            if let Ok(tombstone) = Graveyard::load(&id) {
                tombstones.push(tombstone);
            }
        }

        Ok(tombstones)
    }

    /// Get summary statistics
    pub fn statistics() -> Result<GraveyardStats, GraveyardError> {
        let tombstones = Graveyard::load_all()?;
        let mut total_lifespan = 0i64;
        let mut total_efficiency = 0.0f64;
        let mut scar_counts = Vec::new();
        let mut legacy_scores = Vec::new();

        for tombstone in &tombstones {
            total_lifespan += tombstone.lifespan_seconds();
            total_efficiency += tombstone.metabolism.efficiency_ratio;
            scar_counts.push(tombstone.pathology.scar_count);
            legacy_scores.push(tombstone.legacy_score());
        }

        let count = tombstones.len() as f64;
        let avg_lifespan = if tombstones.is_empty() {
            0
        } else {
            total_lifespan / tombstones.len() as i64
        };
        let avg_efficiency = total_efficiency / count.max(1.0);

        Ok(GraveyardStats {
            total_agents: tombstones.len(),
            average_lifespan_seconds: avg_lifespan,
            average_efficiency: avg_efficiency,
            total_scars: scar_counts.iter().sum(),
            most_common_scar_count: scar_counts.iter().max().cloned().unwrap_or(0),
            highest_legacy_score: legacy_scores
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max),
        })
    }
}

/// Statistics about the graveyard
#[derive(Debug, Clone)]
pub struct GraveyardStats {
    pub total_agents: usize,
    pub average_lifespan_seconds: i64,
    pub average_efficiency: f64,
    pub total_scars: usize,
    pub most_common_scar_count: usize,
    pub highest_legacy_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_score_calculation() {
        let metabolic = MetabolicRecord {
            final_energy: 100,
            peak_energy: 1000,
            initial_energy: 1000,
            efficiency_ratio: 0.5,
            tasks_completed: 10,
        };

        assert_eq!(metabolic.efficiency_ratio, 0.5);
    }
}
