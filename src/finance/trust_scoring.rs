//! Trust Scoring System - Cryptographic reputation and access control
//!
//! Trust scores are computed from performance history and determine:
//! - Access to premium data feeds
//! - Maximum leverage allowed
//! - Capital allocation in competitions
//! - Resource priority and fees

use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use hex::encode;

/// Performance score based on trading metrics
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PerformanceScore {
    /// Win rate percentage (0-100)
    pub win_rate: f32,
    
    /// Profit factor (avg win / avg loss)
    pub profit_factor: f32,
    
    /// Sharpe ratio equivalent (return / volatility)
    pub sharpe_equivalent: f32,
    
    /// Maximum drawdown (0-100, lower is better)
    pub max_drawdown: f32,
    
    /// Trade count consistency
    pub consistency: f32,
    
    /// Scar damage penalty (0-100, 0 is best)
    pub damage_penalty: f32,
}

impl PerformanceScore {
    /// Create performance score from agent metrics
    pub fn from_agent_metrics(
        win_rate: f32,
        max_drawdown: f32,
        total_trades: u64,
        scar_count: u32,
    ) -> Self {
        let profit_factor = if win_rate > 50.0 {
            1.0 + ((win_rate - 50.0) / 50.0) * 0.5
        } else {
            0.5 + (win_rate / 50.0) * 0.5
        };
        
        let sharpe = (win_rate / 100.0) / (max_drawdown.max(1.0) / 100.0);
        
        let consistency = if total_trades > 100 {
            1.0
        } else {
            total_trades as f32 / 100.0
        };
        
        let damage_penalty = (scar_count as f32) * 10.0;
        
        PerformanceScore {
            win_rate,
            profit_factor,
            sharpe_equivalent: sharpe,
            max_drawdown,
            consistency,
            damage_penalty: damage_penalty.min(100.0),
        }
    }
    
    /// Calculate composite score (0-100)
    pub fn composite_score(&self) -> f32 {
        let win_component = self.win_rate;
        let drawdown_component = (100.0 - self.max_drawdown) * 0.5;
        let consistency_component = self.consistency * 100.0;
        let sharpe_component = (self.sharpe_equivalent.min(5.0) / 5.0) * 100.0;
        
        let mut score = win_component * 0.35 +
            drawdown_component * 0.25 +
            consistency_component * 0.20 +
            sharpe_component * 0.20;
        
        // Apply damage penalty
        score = (score * (1.0 - (self.damage_penalty / 200.0))).max(0.0);
        
        score.min(100.0)
    }
}

/// Trust grant result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustGrant {
    /// Full access
    Gold { score: f32 },
    
    /// Premium features available
    Silver { score: f32 },
    
    /// Standard access
    Bronze { score: f32 },
    
    /// Minimal access, restricted
    Restricted { score: f32, reason: String },
}

/// Trust formula for computing final trust score
pub struct TrustFormula {
    /// Weight for performance history (0-1)
    pub performance_weight: f32,
    
    /// Weight for consistency (0-1)
    pub consistency_weight: f32,
    
    /// Weight for damage/scars (0-1)
    pub damage_weight: f32,
    
    /// Weight for time in market (0-1)
    pub longevity_weight: f32,
}

impl Default for TrustFormula {
    fn default() -> Self {
        TrustFormula {
            performance_weight: 0.40,
            consistency_weight: 0.25,
            damage_weight: 0.20,
            longevity_weight: 0.15,
        }
    }
}

impl TrustFormula {
    /// Compute trust score using the formula
    pub fn compute_trust_score(
        &self,
        perf_score: &PerformanceScore,
        days_trading: u64,
    ) -> f32 {
        let longevity = (days_trading as f32 / 365.0).min(1.0);
        
        let score = perf_score.composite_score() * self.performance_weight +
            perf_score.consistency * 100.0 * self.consistency_weight +
            (100.0 - perf_score.damage_penalty) * self.damage_weight +
            longevity * 100.0 * self.longevity_weight;
        
        score.min(100.0)
    }
    
    /// Compute cryptographic trust hash
    pub fn compute_trust_hash(
        agent_id: &str,
        score: f32,
        timestamp: u64,
    ) -> String {
        let data = format!("{}:{}:{}", agent_id, score, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        encode(hasher.finalize())
    }
    
    /// Determine access tier based on score
    pub fn determine_grant(score: f32) -> TrustGrant {
        match score {
            80.0..=100.0 => TrustGrant::Gold { score },
            60.0..=79.99 => TrustGrant::Silver { score },
            40.0..=59.99 => TrustGrant::Bronze { score },
            _ => TrustGrant::Restricted {
                score,
                reason: format!("Score {:.2} below minimum", score),
            },
        }
    }
}

/// Complete trust record for an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustRecord {
    /// Agent ID
    pub agent_id: String,
    
    /// Current trust score
    pub current_score: f32,
    
    /// Trust grant level
    pub grant: TrustGrant,
    
    /// Cryptographic proof of score
    pub trust_hash: String,
    
    /// When this record was computed
    pub timestamp: u64,
    
    /// Previous scores (history)
    pub score_history: Vec<f32>,
    
    /// Permissions granted
    pub permissions: Vec<TrustPermission>,
}

/// Permission types granted by trust system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustPermission {
    /// Access to premium market data feeds
    PremiumDataFeed,
    
    /// Allowed to use margin/leverage
    MarginTrading,
    
    /// Can participate in private pools
    PrivatePoolAccess,
    
    /// Can vote on governance
    GovernanceVoting,
    
    /// Can spawn offspring
    Spawning,
    
    /// Can participate in competitions
    Competition,
    
    /// Can perform complex derivative trades
    DerivativeTrading,
}

/// Result of trust operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustResult {
    /// Trust check passed
    Authorized {
        score: f32,
        permissions: Vec<TrustPermission>,
    },
    
    /// Trust check failed
    Denied {
        score: f32,
        reason: String,
        required_score: f32,
    },
    
    /// Temporary ban due to violations
    TemporaryBan {
        reason: String,
        duration_hours: u32,
    },
    
    /// Permanent revocation
    Revoked {
        reason: String,
    },
}

impl TrustRecord {
    /// Create new trust record
    pub fn new(agent_id: String, initial_score: f32, _formula: &TrustFormula) -> Self {
        let grant = TrustFormula::determine_grant(initial_score);
        let trust_hash = TrustFormula::compute_trust_hash(&agent_id, initial_score, 0);
        
        let permissions = match &grant {
            TrustGrant::Gold { .. } => vec![
                TrustPermission::PremiumDataFeed,
                TrustPermission::MarginTrading,
                TrustPermission::PrivatePoolAccess,
                TrustPermission::GovernanceVoting,
                TrustPermission::Spawning,
                TrustPermission::Competition,
                TrustPermission::DerivativeTrading,
            ],
            TrustGrant::Silver { .. } => vec![
                TrustPermission::MarginTrading,
                TrustPermission::Spawning,
                TrustPermission::Competition,
            ],
            TrustGrant::Bronze { .. } => vec![
                TrustPermission::Competition,
            ],
            TrustGrant::Restricted { .. } => vec![],
        };
        
        TrustRecord {
            agent_id,
            current_score: initial_score,
            grant,
            trust_hash,
            timestamp: 0,
            score_history: vec![initial_score],
            permissions,
        }
    }
    
    /// Update trust score
    pub fn update_score(&mut self, new_score: f32, _formula: &TrustFormula) {
        self.current_score = new_score;
        self.score_history.push(new_score);
        self.grant = TrustFormula::determine_grant(new_score);
        self.trust_hash = TrustFormula::compute_trust_hash(
            &self.agent_id,
            new_score,
            self.score_history.len() as u64,
        );
        
        // Update permissions based on new grant
        self.permissions = match &self.grant {
            TrustGrant::Gold { .. } => vec![
                TrustPermission::PremiumDataFeed,
                TrustPermission::MarginTrading,
                TrustPermission::PrivatePoolAccess,
                TrustPermission::GovernanceVoting,
                TrustPermission::Spawning,
                TrustPermission::Competition,
                TrustPermission::DerivativeTrading,
            ],
            TrustGrant::Silver { .. } => vec![
                TrustPermission::MarginTrading,
                TrustPermission::Spawning,
                TrustPermission::Competition,
            ],
            TrustGrant::Bronze { .. } => vec![
                TrustPermission::Competition,
            ],
            TrustGrant::Restricted { .. } => vec![],
        };
    }
    
    /// Check if agent has permission
    pub fn has_permission(&self, permission: TrustPermission) -> bool {
        self.permissions.contains(&permission)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_score() {
        let score = PerformanceScore::from_agent_metrics(65.0, 10.0, 50, 1);
        assert!(score.composite_score() > 0.0);
    }

    #[test]
    fn test_trust_formula() {
        let formula = TrustFormula::default();
        let perf = PerformanceScore::from_agent_metrics(70.0, 5.0, 100, 0);
        let trust = formula.compute_trust_score(&perf, 365);
        assert!(trust > 0.0);
    }

    #[test]
    fn test_trust_grant_levels() {
        assert!(matches!(TrustFormula::determine_grant(85.0), TrustGrant::Gold { .. }));
        assert!(matches!(TrustFormula::determine_grant(65.0), TrustGrant::Silver { .. }));
        assert!(matches!(TrustFormula::determine_grant(30.0), TrustGrant::Restricted { .. }));
    }
}
