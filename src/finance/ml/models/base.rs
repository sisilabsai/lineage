//! Base neural network layer types for pure Rust ML
//!
//! Provides matrix operations and activation functions using ndarray.

#[cfg(feature = "ml")]
use ndarray::{Array1, Array2};

/// Simple dense layer for neural networks
#[cfg(feature = "ml")]
#[derive(Clone, Debug)]
pub struct DenseLayer {
    pub weights: Array2<f32>,
    pub bias: Array1<f32>,
}

#[cfg(feature = "ml")]
impl DenseLayer {
    /// Create a new dense layer with input_size and output_size
    pub fn new(input_size: usize, output_size: usize) -> Self {
        use ndarray_rand::RandomExt;
        use rand_distr::Normal;

        let dist = Normal::new(0.0, 0.1).expect("valid distribution");
        let weights = Array2::random((input_size, output_size), dist);
        let bias = Array1::zeros(output_size);

        Self { weights, bias }
    }

    /// Forward pass through the layer
    pub fn forward(&self, input: &Array1<f32>) -> Array1<f32> {
        self.weights.t().dot(input) + &self.bias
    }

    /// Apply ReLU activation
    pub fn relu(x: &Array1<f32>) -> Array1<f32> {
        x.mapv(|v| v.max(0.0))
    }

    /// Apply sigmoid activation
    pub fn sigmoid(x: &Array1<f32>) -> Array1<f32> {
        x.mapv(|v| 1.0 / (1.0 + (-v).exp()))
    }

    /// Linear activation (identity)
    pub fn linear(x: &Array1<f32>) -> Array1<f32> {
        x.clone()
    }
}

/// Activation function types
#[cfg(feature = "ml")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Activation {
    ReLU,
    Sigmoid,
    Linear,
}

#[cfg(feature = "ml")]
impl Activation {
    pub fn apply(&self, x: &Array1<f32>) -> Array1<f32> {
        match self {
            Activation::ReLU => DenseLayer::relu(x),
            Activation::Sigmoid => DenseLayer::sigmoid(x),
            Activation::Linear => DenseLayer::linear(x),
        }
    }
}

/// Helper functions for neural network operations
pub mod network_utils {
    use std::collections::HashMap;
    
    /// Normalize prices for neural network input
    pub fn normalize_prices(prices: &HashMap<String, f64>) -> Vec<f32> {
        let mut price_values: Vec<f32> = prices
            .values()
            .map(|&p| (p.ln()) as f32)  // Log prices for stability
            .collect();
        
        if price_values.is_empty() {
            return vec![0.0];
        }
        
        // Standardize: (x - mean) / std
        let mean = price_values.iter().sum::<f32>() / price_values.len() as f32;
        let variance = price_values.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f32>() / price_values.len() as f32;
        let std = variance.sqrt();
        
        if std > 0.0 {
            price_values.iter_mut().for_each(|x| {
                *x = (*x - mean) / std;
            });
        }
        
        price_values
    }
}
