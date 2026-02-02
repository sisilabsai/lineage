//! Example: Training a Q-Learning agent in a simulated market environment
//! 
//! This example demonstrates:
//! - Creating a neural network model
//! - Running a training coordinator
//! - Collecting experiences from trading episodes
//! - Computing rewards and losses
//! - Training on batches from experience replay

#[cfg(feature = "ml")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use lineage::finance::ml::models::q_net::SimpleQNet;
    use lineage::finance::ml::training::{QLearningTrainer, RewardCalculator};
    use lineage::finance::agent::AgentMetrics;
    
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Q-Learning Agent Training Example");
    println!("═══════════════════════════════════════════════════════════════\n");
    
    // Initialize the neural network
    // Input: 5 features (market prices + agent state)
    // Hidden: 64 neurons
    // Output: 4 possible actions (buy, sell, hold, etc.)
    let model = SimpleQNet::new(5, 64)?;
    println!("[*] Created neural network with 5 inputs, 64 hidden, 4 outputs");
    
    // Initialize the Q-Learning trainer
    let mut trainer = QLearningTrainer::new(model);
    println!("[*] Created Q-Learning trainer");
    println!("    - Gamma (discount): {}", trainer.gamma);
    println!("    - Learning rate: {}", trainer.learning_rate);
    println!("    - Batch size: {}\n", trainer.batch_size);
    
    // Initialize reward calculator
    let reward_calc = RewardCalculator::new();
    println!("[*] Initialized reward calculator\n");
    
    // Simulate training episodes
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Simulating Training Episodes");
    println!("═══════════════════════════════════════════════════════════════\n");
    
    for episode in 0..10 {
        println!("Episode {}/10", episode + 1);
        println!("-----------");
        
        // Simulate a trading episode
        // In a real scenario, this would involve actual market data and agent decisions
        let mut episode_reward = 0.0;
        let mut trades_in_episode = 0;
        
        // Initial agent metrics
        let mut prev_metrics = AgentMetrics {
            capital: 10_000,
            total_trades: 0,
            win_rate: 50.0,
            max_drawdown: 0.0,
            current_drawdown: 0.0,
            peak_capital: 10_000,
            total_fees_paid: 0,
            scar_count: 0,
            trust_score: 50.0,
            generation: 0,
        };
        
        // Simulate 20 trading steps per episode
        for step in 0..20 {
            // Simulate market state (random prices)
            let market_prices: Vec<f32> = (0..5)
                .map(|_| rand::random::<f32>() * 100.0)
                .collect();
            
            // Simulate trader decision (0=Buy, 1=Sell, 2=Hold)
            let action = step % 3;  // Only 3 actions supported by SimpleQNet
            
            // Simulate the result of the trade
            let mut curr_metrics = prev_metrics.clone();
            
            // Randomly apply profit or loss
            if rand::random::<bool>() {
                // Profitable trade
                let gain = rand::random::<f32>() * 500.0;
                curr_metrics.capital += gain as u64;
                curr_metrics.total_trades += 1;
                curr_metrics.win_rate = (curr_metrics.win_rate + 1.0) / 2.0;
                trades_in_episode += 1;
            } else {
                // Losing trade
                let loss = rand::random::<f32>() * 200.0;
                curr_metrics.capital = curr_metrics.capital.saturating_sub(loss as u64);
                curr_metrics.current_drawdown += loss / 10_000.0;
                curr_metrics.scar_count += 1;
                trades_in_episode += 1;
            }
            
            // Calculate immediate reward
            let immediate_reward = reward_calc.calculate_immediate_reward(&prev_metrics, &curr_metrics);
            episode_reward += immediate_reward;
            
            // Store experience in replay buffer
            trainer.remember_experience(
                market_prices,
                action,
                immediate_reward,
                vec![0.0; 5],  // next_state simplified
                false,         // not terminal yet
            );
            
            prev_metrics = curr_metrics;
        }
        
        // End of episode - calculate terminal reward
        let terminal_reward = reward_calc.calculate_episode_reward(10_000, &prev_metrics);
        episode_reward += terminal_reward;
        
        // Train on a batch
        if let Ok(loss) = trainer.train_step() {
            println!("  Trades: {} | Reward: {:.2} | Loss: {:.6}", 
                     trades_in_episode, 
                     episode_reward, 
                     loss);
        }
        
        // Print final capital
        println!("  Final Capital: ${}\n", prev_metrics.capital);
    }
    
    // Print final training statistics
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Training Summary");
    println!("═══════════════════════════════════════════════════════════════\n");
    
    let stats = trainer.get_stats();
    println!("Training Statistics:");
    println!("  {}\n", stats);
    
    println!("Replay Buffer Status:");
    println!("  Capacity: {}", stats.replay_capacity);
    println!("  Current Size: {}", stats.buffer_size);
    println!("  Utilization: {:.1}%\n", 
             (stats.buffer_size as f32 / stats.replay_capacity as f32) * 100.0);
    
    println!("Model Training Progress:");
    println!("  Total Steps: {}", stats.training_steps);
    println!("  Average Loss: {:.6}", stats.total_loss);
    
    println!("\n✓ Training example completed successfully!");
    Ok(())
}

#[cfg(not(feature = "ml"))]
fn main() {
    println!("This example requires the 'ml' feature to be enabled.");
    println!("Run with: cargo run --example training_loop_example --features ml");
}
