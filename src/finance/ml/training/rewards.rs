//! Reward calculation for ML training
//! Converts financial metrics into RL rewards

use crate::finance::agent::AgentMetrics;

/// Calculate training reward from agent metrics
/// This reward signal drives the learning process
pub struct RewardCalculator {
    /// Scaling factor for ROI reward
    roi_scale: f32,
    
    /// Penalty for drawdowns
    drawdown_penalty: f32,
    
    /// Penalty per scar
    scar_penalty: f32,
    
    /// Bonus for improving win rate
    win_rate_bonus: f32,
}

impl RewardCalculator {
    /// Create a new reward calculator with default weights
    pub fn new() -> Self {
        Self {
            roi_scale: 10.0,           // 1% ROI = +10 reward
            drawdown_penalty: 5.0,     // 1% drawdown = -5 reward
            scar_penalty: 1.0,         // Each scar = -1 reward
            win_rate_bonus: 2.0,       // 1% win rate improvement = +2 reward
        }
    }
    
    /// Calculate immediate reward from current state
    /// 
    /// # Arguments
    /// * `previous_metrics` - Metrics from previous step
    /// * `current_metrics` - Metrics from current step
    /// 
    /// # Returns
    /// Reward signal for this step
    pub fn calculate_immediate_reward(
        &self,
        previous_metrics: &AgentMetrics,
        current_metrics: &AgentMetrics,
    ) -> f32 {
        let mut reward = 0.0;
        
        // Reward for capital growth
        if current_metrics.capital > previous_metrics.capital {
            let capital_gain = current_metrics.capital as f32 - previous_metrics.capital as f32;
            let roi = capital_gain / previous_metrics.capital as f32;
            reward += roi * self.roi_scale;
        } else if current_metrics.capital < previous_metrics.capital {
            // Penalty for capital loss
            let capital_loss = previous_metrics.capital as f32 - current_metrics.capital as f32;
            let loss_ratio = capital_loss / previous_metrics.capital as f32;
            reward -= loss_ratio * self.roi_scale * 2.0; // Double penalty for losses
        }
        
        // Penalty for increasing drawdown
        if current_metrics.current_drawdown > previous_metrics.current_drawdown {
            let drawdown_increase = current_metrics.current_drawdown - previous_metrics.current_drawdown;
            reward -= drawdown_increase * self.drawdown_penalty;
        }
        
        // Penalty for new scars
        if current_metrics.scar_count > previous_metrics.scar_count {
            let new_scars = (current_metrics.scar_count - previous_metrics.scar_count) as f32;
            reward -= new_scars * self.scar_penalty;
        }
        
        // Reward for improved win rate
        if current_metrics.win_rate > previous_metrics.win_rate {
            let win_rate_improvement = current_metrics.win_rate - previous_metrics.win_rate;
            reward += (win_rate_improvement / 100.0) * self.win_rate_bonus;
        }
        
        reward
    }
    
    /// Calculate cumulative episode reward
    /// Focuses on terminal metrics (final capital, final drawdown, scars)
    pub fn calculate_episode_reward(
        &self,
        initial_capital: u64,
        final_metrics: &AgentMetrics,
    ) -> f32 {
        let mut reward = 0.0;
        
        // Main signal: total return on investment
        let roi = (final_metrics.capital as f32 / initial_capital as f32) - 1.0;
        reward += roi * self.roi_scale * 100.0; // 100x scaling for episode reward
        
        // Penalize high drawdown
        reward -= final_metrics.max_drawdown * self.drawdown_penalty;
        
        // Penalize scars (accumulated damage)
        reward -= final_metrics.scar_count as f32 * self.scar_penalty * 10.0;
        
        // Bonus for consistent profitability
        if final_metrics.win_rate > 50.0 {
            let excess_wins = final_metrics.win_rate - 50.0;
            reward += excess_wins * self.win_rate_bonus;
        }
        
        // Heavy penalty for bankruptcy
        if final_metrics.capital < (initial_capital / 2) {
            reward -= 1000.0;
        }
        
        reward
    }
    
    /// Set custom reward weights
    pub fn with_weights(
        roi_scale: f32,
        drawdown_penalty: f32,
        scar_penalty: f32,
        win_rate_bonus: f32,
    ) -> Self {
        Self {
            roi_scale,
            drawdown_penalty,
            scar_penalty,
            win_rate_bonus,
        }
    }
}

impl Default for RewardCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_metrics(capital: u64, win_rate: f32, scar_count: u32, drawdown: f32) -> AgentMetrics {
        AgentMetrics {
            capital,
            total_trades: 10,
            win_rate,
            max_drawdown: drawdown,
            current_drawdown: drawdown,
            peak_capital: capital,
            total_fees_paid: 0,
            scar_count,
            trust_score: 50.0,
            generation: 0,
        }
    }
    
    #[test]
    fn test_profit_reward() {
        let calc = RewardCalculator::new();
        let prev = create_metrics(10000, 50.0, 0, 0.0);
        let curr = create_metrics(11000, 50.0, 0, 0.0); // 10% gain
        
        let reward = calc.calculate_immediate_reward(&prev, &curr);
        assert!(reward > 0.0, "Profitable trade should give positive reward");
    }
    
    #[test]
    fn test_loss_penalty() {
        let calc = RewardCalculator::new();
        let prev = create_metrics(10000, 50.0, 0, 0.0);
        let curr = create_metrics(9000, 50.0, 0, 5.0); // 10% loss
        
        let reward = calc.calculate_immediate_reward(&prev, &curr);
        assert!(reward < 0.0, "Loss should give negative reward");
    }
    
    #[test]
    fn test_scar_penalty() {
        let calc = RewardCalculator::new();
        let prev = create_metrics(10000, 50.0, 0, 0.0);
        let curr = create_metrics(10000, 50.0, 1, 5.0); // New scar
        
        let reward = calc.calculate_immediate_reward(&prev, &curr);
        assert!(reward < 0.0, "Scar should give negative reward");
    }
}
