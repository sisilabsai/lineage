//! Trade Execution System - Irreversible financial operations
//!
//! Trades in this system:
//! - Cannot be rolled back or cancelled after execution
//! - Consume energy (capital) on fees and losses
//! - Are recorded permanently in agent history
//! - Can inflict scars from losses

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Direction of a trade execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeDirection {
    /// Buy/long position
    Buy,
    /// Sell/short position
    Sell,
}

/// Execution error types (all result in energy consumption)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionError {
    /// Insufficient capital for trade
    InsufficientCapital { required: u64, available: u64 },
    
    /// Loss exceeded maximum allowed drawdown
    DrawdownExceeded { loss_percentage: f32 },
    
    /// Trade rejected by risk management
    RiskExceeded { reason: String },
    
    /// Agent capacity exceeded
    CapacityExceeded { reason: String },
}

/// Result of trade execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeResult {
    /// Trade executed and profitable
    Success { pnl: i64, fees: u64 },
    
    /// Trade executed but resulted in loss
    Loss { loss: u64, fees: u64, drawdown: f32 },
    
    /// Trade rejected (still may consume energy from validation)
    Rejected { error: ExecutionError, energy_consumed: u64 },
}

/// Core trade operation descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// Unique trade ID (timestamp + seq)
    pub id: u64,
    
    /// When trade was executed
    pub timestamp: DateTime<Utc>,
    
    /// Buy or Sell
    pub direction: TradeDirection,
    
    /// Asset being traded
    pub asset: String,
    
    /// Size in base units
    pub size: u64,
    
    /// Entry price
    pub entry_price: f32,
    
    /// Exit price (if closed, otherwise none)
    pub exit_price: Option<f32>,
    
    /// Leverage used (1.0 = no leverage)
    pub leverage: f32,
    
    /// Fee percentage applied
    pub fee_percentage: f32,
    
    /// Actual fees paid
    pub fees_paid: u64,
}

impl Trade {
    /// Create a new trade descriptor
    pub fn new(
        id: u64,
        direction: TradeDirection,
        asset: String,
        size: u64,
        entry_price: f32,
        leverage: f32,
        fee_percentage: f32,
    ) -> Self {
        Trade {
            id,
            timestamp: Utc::now(),
            direction,
            asset,
            size,
            entry_price,
            exit_price: None,
            leverage,
            fee_percentage,
            fees_paid: 0,
        }
    }
    
    /// Close the trade at exit price and calculate P&L
    pub fn close(&mut self, exit_price: f32) -> (i64, u64) {
        self.exit_price = Some(exit_price);
        
        let entry_value = (self.size as f32 * self.entry_price * self.leverage) as i64;
        let exit_value = (self.size as f32 * exit_price * self.leverage) as i64;
        
        let pnl = match self.direction {
            TradeDirection::Buy => exit_value - entry_value,
            TradeDirection::Sell => entry_value - exit_value,
        };
        
        let fees = ((pnl.abs() as f32) * (self.fee_percentage / 100.0)) as u64;
        self.fees_paid = fees;
        
        (pnl, fees)
    }
    
    /// Calculate drawdown percentage
    pub fn calculate_drawdown(&self) -> Option<f32> {
        self.exit_price.map(|exit_price| {
            let price_change = (exit_price - self.entry_price) / self.entry_price;
            let drawdown = match self.direction {
                TradeDirection::Buy => -price_change,
                TradeDirection::Sell => price_change,
            };
            drawdown * 100.0
        })
    }
}

/// Trade operation with validation and execution
pub struct TradeOperation {
    /// Trade being executed
    pub trade: Trade,
    
    /// Fees to be paid
    pub fee_cost: u64,
    
    /// Additional capital requirement (margin, collateral)
    pub capital_requirement: u64,
}

impl TradeOperation {
    /// Validate trade is feasible
    pub fn validate(&self, available_capital: u64, max_leverage: f32) -> Result<(), ExecutionError> {
        if self.trade.leverage > max_leverage {
            return Err(ExecutionError::RiskExceeded {
                reason: format!("Leverage {} exceeds max {}", self.trade.leverage, max_leverage),
            });
        }
        
        if self.capital_requirement > available_capital {
            return Err(ExecutionError::InsufficientCapital {
                required: self.capital_requirement,
                available: available_capital,
            });
        }
        
        Ok(())
    }
    
    /// Execute the trade (irreversible)
    pub fn execute(&mut self, exit_price: f32) -> TradeResult {
        let (pnl, fees) = self.trade.close(exit_price);
        self.fee_cost = fees;
        
        match self.trade.calculate_drawdown() {
            Some(drawdown) if drawdown > 5.0 => {
                TradeResult::Loss {
                    loss: drawdown.abs() as u64,
                    fees,
                    drawdown,
                }
            }
            Some(drawdown) if pnl < 0 => {
                TradeResult::Loss {
                    loss: pnl.abs() as u64,
                    fees,
                    drawdown,
                }
            }
            _ if pnl > 0 => {
                TradeResult::Success {
                    pnl,
                    fees,
                }
            }
            _ => {
                TradeResult::Loss {
                    loss: 0,
                    fees,
                    drawdown: 0.0,
                }
            }
        }
    }
    
    /// Calculate total energy cost (capital to consume)
    pub fn total_energy_cost(&self) -> u64 {
        self.capital_requirement + self.fee_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_creation() {
        let trade = Trade::new(1, TradeDirection::Buy, "BTC".to_string(), 1, 50000.0, 1.0, 0.1);
        assert_eq!(trade.direction, TradeDirection::Buy);
        assert_eq!(trade.asset, "BTC");
    }

    #[test]
    fn test_trade_pnl_calculation() {
        let mut trade = Trade::new(1, TradeDirection::Buy, "BTC".to_string(), 1, 50000.0, 1.0, 0.1);
        let (pnl, fees) = trade.close(51000.0);
        
        assert!(pnl > 0); // Profitable trade
        assert!(fees > 0); // Fees charged
    }
}
