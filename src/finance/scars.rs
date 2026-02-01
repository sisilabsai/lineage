//! Financial Scar System - Permanent penalties from losses
//!
//! Scars represent permanent damage from trading losses:
//! - Accumulate from drawdowns exceeding thresholds
//! - Increase future trading costs (scar cost multiplier)
//! - Cannot be healed or removed
//! - Lead to agent death when excessive

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Impact of a financial scar on future operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScarImpact {
    /// Cost multiplier increase (1.05 = 5% increase)
    pub cost_multiplier: f32,
    
    /// Maximum leverage reduction percentage
    pub leverage_reduction: f32,
    
    /// Trust score penalty
    pub trust_penalty: f32,
    
    /// Resource access restrictions
    pub access_restrictions: Vec<String>,
}

impl ScarImpact {
    pub fn from_severity(severity: u32) -> Self {
        ScarImpact {
            cost_multiplier: 1.0 + (severity as f32 * 0.05),
            leverage_reduction: severity as f32 * 10.0,
            trust_penalty: severity as f32 * 5.0,
            access_restrictions: match severity {
                0 => vec![],
                1..=2 => vec!["premium_feeds".to_string()],
                3..=4 => vec!["premium_feeds".to_string(), "margin_trading".to_string()],
                _ => vec!["premium_feeds".to_string(), "margin_trading".to_string(), "leverage".to_string()],
            },
        }
    }
}

/// A single financial scar from a loss event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialScar {
    /// When the scar was inflicted
    pub timestamp: DateTime<Utc>,
    
    /// Type of loss that caused scar
    pub loss_type: LossType,
    
    /// Percentage of capital lost
    pub loss_percentage: f32,
    
    /// Severity level (1-5)
    pub severity: u32,
    
    /// Trade ID that caused the scar (if applicable)
    pub trade_id: Option<u64>,
    
    /// Immutable impact this scar has
    pub impact: ScarImpact,
    
    /// Description of what happened
    pub description: String,
}

/// Type of financial loss
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LossType {
    /// Individual trade loss
    TradeLoss,
    
    /// Liquidation from leverage collapse
    Liquidation,
    
    /// Fee burden exceeding profit
    FeeBurden,
    
    /// Stop-loss triggered
    StopLossTriggered,
    
    /// Slippage during execution
    ExecutionSlippage,
    
    /// Black swan market event
    MarketCrash,
}

/// Accumulated damage record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialDamage {
    /// Total scars accumulated
    pub scar_count: u32,
    
    /// Combined cost multiplier
    pub cumulative_cost_multiplier: f32,
    
    /// Total leverage reduction
    pub cumulative_leverage_reduction: f32,
    
    /// Combined trust penalty
    pub cumulative_trust_penalty: f32,
    
    /// All scars (history, immutable)
    pub scar_history: Vec<FinancialScar>,
    
    /// Maximum allowable scars before death
    pub max_scars_before_death: u32,
}

impl FinancialDamage {
    /// Create new damage record
    pub fn new(max_scars: u32) -> Self {
        FinancialDamage {
            scar_count: 0,
            cumulative_cost_multiplier: 1.0,
            cumulative_leverage_reduction: 0.0,
            cumulative_trust_penalty: 0.0,
            scar_history: Vec::new(),
            max_scars_before_death: max_scars,
        }
    }
    
    /// Inflict a new scar (permanent)
    pub fn inflict_scar(&mut self, scar: FinancialScar) -> Result<(), String> {
        if self.scar_count >= self.max_scars_before_death {
            return Err("Too many scars, agent is dead".to_string());
        }
        
        self.cumulative_cost_multiplier *= scar.impact.cost_multiplier;
        self.cumulative_leverage_reduction += scar.impact.leverage_reduction;
        self.cumulative_trust_penalty += scar.impact.trust_penalty;
        
        self.scar_count += 1;
        self.scar_history.push(scar);
        
        if self.scar_count >= self.max_scars_before_death {
            return Err("Agent has exceeded maximum scars".to_string());
        }
        
        Ok(())
    }
    
    /// Check if agent should die from scars
    pub fn is_terminal(&self) -> bool {
        self.scar_count >= self.max_scars_before_death
    }
    
    /// Get restricted resources due to scars
    pub fn get_restrictions(&self) -> Vec<String> {
        let mut restrictions = Vec::new();
        for scar in &self.scar_history {
            for restriction in &scar.impact.access_restrictions {
                if !restrictions.contains(restriction) {
                    restrictions.push(restriction.clone());
                }
            }
        }
        restrictions
    }
    
    /// Calculate adjusted leverage allowed
    pub fn get_adjusted_leverage(&self, base_leverage: f32) -> f32 {
        let reduction_percentage = self.cumulative_leverage_reduction / 100.0;
        (base_leverage * (1.0 - reduction_percentage)).max(1.0)
    }
    
    /// Get audit record of all scars
    pub fn get_audit_record(&self) -> String {
        let mut record = format!("Scar History ({} total):\n", self.scar_count);
        for (i, scar) in self.scar_history.iter().enumerate() {
            record.push_str(&format!(
                "  #{}: [{:?}] {:.2}% loss - {} (Severity: {})\n",
                i + 1,
                scar.loss_type,
                scar.loss_percentage,
                scar.description,
                scar.severity,
            ));
        }
        record
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scar_creation() {
        let impact = ScarImpact::from_severity(2);
        assert!(impact.cost_multiplier > 1.0);
        assert!(impact.leverage_reduction > 0.0);
    }

    #[test]
    fn test_damage_accumulation() {
        let mut damage = FinancialDamage::new(5);
        
        let scar = FinancialScar {
            timestamp: Utc::now(),
            loss_type: LossType::TradeLoss,
            loss_percentage: 5.0,
            severity: 1,
            trade_id: Some(1),
            impact: ScarImpact::from_severity(1),
            description: "Bad trade".to_string(),
        };
        
        assert!(damage.inflict_scar(scar).is_ok());
        assert_eq!(damage.scar_count, 1);
    }
}
