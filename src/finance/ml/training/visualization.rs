//! Training Visualization and Logging
//! Exports metrics to CSV, generates plots, tracks performance

use std::fs::File;
use std::io::Write;
use crate::finance::ml::errors::Result;

/// Episode metrics for logging
#[derive(Debug, Clone)]
pub struct EpisodeLog {
    pub episode: u32,
    pub total_reward: f32,
    pub average_reward: f32,
    pub training_loss: f32,
    pub epsilon: f32,
    pub buffer_size: usize,
    pub best_loss: f32,
    pub avg_capital: u64,
    pub avg_win_rate: f32,
}

/// Training metrics recorder
pub struct MetricsRecorder {
    /// All episode logs
    episodes: Vec<EpisodeLog>,
    
    /// Output CSV path
    csv_path: String,
    
    /// Plot output path
    plot_path: String,
}

impl MetricsRecorder {
    pub fn new(csv_path: String, plot_path: String) -> Self {
        Self {
            episodes: Vec::new(),
            csv_path,
            plot_path,
        }
    }
    
    /// Record an episode
    pub fn log_episode(&mut self, log: EpisodeLog) {
        self.episodes.push(log);
    }
    
    /// Export to CSV file
    pub fn export_csv(&self) -> Result<()> {
        let mut file = File::create(&self.csv_path)?;
        
        // Header
        writeln!(
            file,
            "episode,total_reward,avg_reward,loss,epsilon,buffer_size,best_loss,avg_capital,avg_win_rate"
        )?;
        
        // Data rows
        for log in &self.episodes {
            writeln!(
                file,
                "{},{},{},{},{},{},{},{},{}",
                log.episode,
                log.total_reward,
                log.average_reward,
                log.training_loss,
                log.epsilon,
                log.buffer_size,
                log.best_loss,
                log.avg_capital,
                log.avg_win_rate
            )?;
        }
        
        println!("✓ Metrics exported to CSV: {}", self.csv_path);
        Ok(())
    }
    
    /// Generate ASCII plot of loss curve
    pub fn plot_loss_curve(&self) -> String {
        if self.episodes.is_empty() {
            return "No data to plot".to_string();
        }
        
        let mut plot = String::new();
        plot.push_str("Training Loss Over Episodes\n");
        plot.push_str("============================\n\n");
        
        // Find min/max for scaling
        let min_loss = self.episodes.iter()
            .map(|e| e.training_loss)
            .fold(f32::INFINITY, f32::min);
        
        let max_loss = self.episodes.iter()
            .map(|e| e.training_loss)
            .fold(f32::NEG_INFINITY, f32::max);
        
        let range = (max_loss - min_loss).max(0.1);
        let height = 15;
        
        // Plot bars
        for log in self.episodes.iter().step_by((self.episodes.len() / 30).max(1)) {
            let normalized = (log.training_loss - min_loss) / range;
            let bar_height = ((1.0 - normalized) * height as f32) as usize;
            
            plot.push_str(&format!("Ep {:3} | ", log.episode));
            for _ in 0..bar_height {
                plot.push('█');
            }
            plot.push_str(&format!(" {:.4}\n", log.training_loss));
        }
        
        plot.push_str("\nMin: ");
        plot.push_str(&format!("{:.6} | ", min_loss));
        plot.push_str("Max: ");
        plot.push_str(&format!("{:.6}\n", max_loss));
        
        plot
    }
    
    /// Generate ASCII plot of reward curve
    pub fn plot_reward_curve(&self) -> String {
        if self.episodes.is_empty() {
            return "No data to plot".to_string();
        }
        
        let mut plot = String::new();
        plot.push_str("Total Reward Over Episodes\n");
        plot.push_str("============================\n\n");
        
        // Find min/max for scaling
        let min_reward = self.episodes.iter()
            .map(|e| e.total_reward)
            .fold(f32::INFINITY, f32::min);
        
        let max_reward = self.episodes.iter()
            .map(|e| e.total_reward)
            .fold(f32::NEG_INFINITY, f32::max);
        
        let range = (max_reward - min_reward).max(1.0);
        let height = 15;
        
        // Plot bars
        for log in self.episodes.iter().step_by((self.episodes.len() / 30).max(1)) {
            let normalized = (log.total_reward - min_reward) / range;
            let bar_height = (normalized * height as f32) as usize;
            
            plot.push_str(&format!("Ep {:3} | ", log.episode));
            for _ in 0..bar_height {
                plot.push('▓');
            }
            plot.push_str(&format!(" {:.2}\n", log.total_reward));
        }
        
        plot.push_str("\nMin: ");
        plot.push_str(&format!("{:.2} | ", min_reward));
        plot.push_str("Max: ");
        plot.push_str(&format!("{:.2}\n", max_reward));
        
        plot
    }
    
    /// Generate summary statistics
    pub fn summary_stats(&self) -> String {
        if self.episodes.is_empty() {
            return "No episodes logged".to_string();
        }
        
        let mut summary = String::new();
        summary.push_str("═══════════════════════════════════════\n");
        summary.push_str("      Training Summary Statistics\n");
        summary.push_str("═══════════════════════════════════════\n\n");
        
        summary.push_str(&format!("Total Episodes: {}\n", self.episodes.len()));
        
        let avg_reward: f32 = self.episodes.iter()
            .map(|e| e.total_reward)
            .sum::<f32>() / self.episodes.len() as f32;
        summary.push_str(&format!("Average Reward: {:.2}\n", avg_reward));
        
        let avg_loss: f32 = self.episodes.iter()
            .map(|e| e.training_loss)
            .sum::<f32>() / self.episodes.len() as f32;
        summary.push_str(&format!("Average Loss: {:.6}\n", avg_loss));
        
        let best_loss = self.episodes.iter()
            .map(|e| e.best_loss)
            .fold(f32::INFINITY, f32::min);
        summary.push_str(&format!("Best Loss: {:.6}\n", best_loss));
        
        let final_epsilon = self.episodes.last()
            .map(|e| e.epsilon)
            .unwrap_or(0.0);
        summary.push_str(&format!("Final Epsilon: {:.3}\n", final_epsilon));
        
        let avg_capital: u64 = self.episodes.iter()
            .map(|e| e.avg_capital)
            .sum::<u64>() / self.episodes.len() as u64;
        summary.push_str(&format!("Average Capital: ${}\n", avg_capital));
        
        summary.push_str("\n═══════════════════════════════════════\n");
        
        summary
    }
    
    /// Save plot to file
    pub fn save_plots(&self) -> Result<()> {
        let mut file = File::create(&self.plot_path)?;
        
        writeln!(file, "{}", self.plot_loss_curve())?;
        writeln!(file, "\n")?;
        writeln!(file, "{}", self.plot_reward_curve())?;
        writeln!(file, "\n")?;
        writeln!(file, "{}", self.summary_stats())?;
        
        println!("✓ Plots saved to: {}", self.plot_path);
        Ok(())
    }
    
    /// Get episode count
    pub fn episode_count(&self) -> usize {
        self.episodes.len()
    }
}

/// Export results in multiple formats
pub struct ResultsExporter {
    metrics_recorder: MetricsRecorder,
}

impl ResultsExporter {
    pub fn new(base_path: &str) -> Self {
        Self {
            metrics_recorder: MetricsRecorder::new(
                format!("{}/training_metrics.csv", base_path),
                format!("{}/training_plots.txt", base_path),
            ),
        }
    }
    
    /// Export all results
    pub fn export_all(&self) -> Result<()> {
        self.metrics_recorder.export_csv()?;
        self.metrics_recorder.save_plots()?;
        Ok(())
    }
    
    /// Get metrics recorder for logging
    pub fn metrics_mut(&mut self) -> &mut MetricsRecorder {
        &mut self.metrics_recorder
    }
    
    /// Display summary on console
    pub fn display_summary(&self) {
        println!("{}", self.metrics_recorder.summary_stats());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metrics_recorder_csv() {
        let mut recorder = MetricsRecorder::new(
            "test_metrics.csv".to_string(),
            "test_plots.txt".to_string(),
        );
        
        recorder.log_episode(EpisodeLog {
            episode: 1,
            total_reward: 50.0,
            average_reward: 50.0,
            training_loss: 0.5,
            epsilon: 0.9,
            buffer_size: 100,
            best_loss: 0.5,
            avg_capital: 10500,
            avg_win_rate: 55.0,
        });
        
        assert_eq!(recorder.episode_count(), 1);
    }
    
    #[test]
    fn test_plot_generation() {
        let mut recorder = MetricsRecorder::new(
            "test_metrics.csv".to_string(),
            "test_plots.txt".to_string(),
        );
        
        for i in 0..10 {
            recorder.log_episode(EpisodeLog {
                episode: i,
                total_reward: (i as f32) * 10.0,
                average_reward: (i as f32) * 10.0,
                training_loss: 1.0 / ((i + 1) as f32),
                epsilon: 1.0 * 0.95_f32.powi(i as i32),
                buffer_size: (i as usize) * 100,
                best_loss: 1.0 / ((i + 1) as f32),
                avg_capital: 10000 + (i as u64) * 500,
                avg_win_rate: 50.0 + (i as f32),
            });
        }
        
        let loss_plot = recorder.plot_loss_curve();
        assert!(loss_plot.contains("Training Loss"));
        
        let reward_plot = recorder.plot_reward_curve();
        assert!(reward_plot.contains("Total Reward"));
    }
}
