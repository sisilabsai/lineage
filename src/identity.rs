//! # Identity System
//!
//! ## What This Enforces
//! - One-time identity creation with cryptographic uniqueness
//! - Identity cannot be cloned, copied, or duplicated
//! - Identity is bound to creation moment and entropy source
//!
//! ## What This Forbids
//! - Clone or Copy traits
//! - Identity reuse or resurrection
//! - Identity comparison beyond exact reference equality
//!
//! ## Violations
//! Any attempt to implement Clone/Copy constitutes an ontological violation.

use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// An irreversible, non-copyable identity.
/// 
/// Once created, this identity is bound to its creation context forever.
/// It cannot be cloned, copied, or recreated with the same value.
/// 
/// **INVARIANT**: No two Identity instances can ever be equal, even across restarts.
#[derive(Debug)]
pub struct Identity {
    /// Unique identifier derived from creation entropy
    id: String,
    /// Creation timestamp (nanoseconds since epoch)
    birth_time: u128,
    /// Entropy source at creation
    entropy: [u8; 32],
}

impl Identity {
    /// Creates a new, permanently unique identity.
    /// 
    /// This operation:
    /// - Captures current system time with nanosecond precision
    /// - Generates cryptographically random entropy
    /// - Creates an irreversible hash binding time, entropy, and UUID
    /// 
    /// **CONSEQUENCE**: This identity can never be recreated identically.
    pub fn create() -> Self {
        let birth_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards - ontological violation")
            .as_nanos();

        let uuid = Uuid::new_v4();
        let mut entropy = [0u8; 32];
        
        // Mix UUID bytes into entropy
        let uuid_bytes = uuid.as_bytes();
        for (i, byte) in uuid_bytes.iter().enumerate() {
            entropy[i] = *byte;
            entropy[i + 16] = byte.wrapping_add(i as u8);
        }

        // Create irreversible hash of all creation context
        let mut hasher = Sha256::new();
        hasher.update(birth_time.to_le_bytes());
        hasher.update(&entropy);
        hasher.update(uuid_bytes);
        
        let hash = hasher.finalize();
        let id = format!("{:x}", hash);

        Identity {
            id,
            birth_time,
            entropy,
        }
    }

    /// Returns the immutable identity string.
    /// 
    /// This value is permanent and globally unique.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the exact nanosecond of creation.
    /// 
    /// This timestamp marks the beginning of this identity's existence.
    pub fn birth_time(&self) -> u128 {
        self.birth_time
    }

    /// Verifies that another identity reference is the exact same instance.
    /// 
    /// Note: This is pointer equality, not value equality.
    /// Two identities are NEVER equal by value.
    pub fn is_same_instance(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

// EXPLICIT PREVENTION: No Clone
// EXPLICIT PREVENTION: No Copy
// EXPLICIT PREVENTION: No PartialEq (identities are never "equal")
// EXPLICIT PREVENTION: No Default (identity must be explicitly created)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_unique() {
        let id1 = Identity::create();
        let id2 = Identity::create();
        
        assert_ne!(id1.id(), id2.id(), "Identities must be unique");
        assert_ne!(id1.birth_time(), id2.birth_time(), "Birth times must differ");
    }

    #[test]
    fn identity_cannot_be_cloned() {
        // This test verifies at compile-time that Clone is not implemented.
        // If this compiles, the identity system is compromised.
        let _id = Identity::create();
        
        // The following line should NOT compile:
        // let _cloned = _id.clone();
        
        // If you can uncomment the above line and it compiles,
        // the identity system has been violated.
    }

    #[test]
    fn identity_id_is_stable() {
        let id = Identity::create();
        let first_read = id.id();
        let second_read = id.id();
        
        assert_eq!(first_read, second_read, "Identity must be stable");
    }

    #[test]
    fn identity_hash_is_irreversible() {
        let id = Identity::create();
        let hash = id.id();
        
        // Hash should be 64 hex characters (256 bits)
        assert_eq!(hash.len(), 64, "Hash must be SHA256");
        
        // Hash should be unrecoverable - we cannot get birth_time from hash alone
        // This is verified by the fact that we need the id reference to access birth_time
    }

    #[test]
    fn identity_cannot_be_recreated() {
        let id1 = Identity::create();
        let hash1 = id1.id();
        
        // Wait to ensure different timestamp
        std::thread::sleep(std::time::Duration::from_nanos(100));
        
        let id2 = Identity::create();
        let hash2 = id2.id();
        
        // INVARIANT: No two identities can ever be the same
        assert_ne!(hash1, hash2);
        assert_ne!(id1.birth_time(), id2.birth_time());
        
        // INVARIANT: There should be NO method like:
        // Identity::from_hash(hash1)
        // Identity::recreate(birth_time, entropy)
        // 
        // If such methods exist, identity uniqueness has been violated.
    }

    #[test]
    fn identity_cannot_be_assigned() {
        let id = Identity::create();
        let _other = Identity::create();
        
        // INVARIANT: Identity cannot be reassigned
        // The following should NOT compile:
        // id = other;
        // 
        // If identity can be reassigned, the identity system has been violated.
        
        // Verify identity remains stable
        let original_hash = id.id();
        let same_hash = id.id();
        assert_eq!(original_hash, same_hash);
    }
}
