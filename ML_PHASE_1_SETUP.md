# ML Implementation Phase 1: Environment Setup Guide

## Quick Reference: Getting libtorch & tch-rs Running

### Environment: Windows 10/11 + Rust

---

## Step 1: Verify libtorch Installation

```powershell
PS D:\Projects\Lineage> Get-Item .\libtorch

    Directory: D:\Projects\Lineage

Mode                 LastWriteTime         Length Name
----                 -----------           ------ ----
d-----        1/15/2026   2:30 PM                libtorch

# Verify structure
PS> ls .\libtorch
    bin/
    build-hash
    build-version
    include/
    lib/
    share/

# Verify lib files exist (CPU version)
PS> ls .\libtorch\lib | Select-Object Name
    torch.lib
    torch_cpu.lib
    caffe2.lib
```

**Status**: ✓ libtorch extracted successfully

---

## Step 2: Set Environment Variables (Windows)

```powershell
# Option A: Temporary (current session only)
$env:LIBTORCH = "D:\Projects\Lineage\libtorch"
$env:PATH = "$env:LIBTORCH\lib;$env:PATH"

# Verify
echo $env:LIBTORCH
# Output: D:\Projects\Lineage\libtorch

# Option B: Permanent (System-wide)
# Control Panel → System → Environment Variables
# Add: LIBTORCH = D:\Projects\Lineage\libtorch
```

---

## Step 3: Update Cargo.toml

### Current Cargo.toml
```toml
[dependencies]
# ... existing ...

# ADD THIS:
tch = "0.15"

[features]
default = []
ml = []  # ML feature flag (optional for non-ML users)
```

### With ML Feature Flag (Recommended)
```toml
[dependencies]
# ... existing ...
tch = { version = "0.15", optional = true }

[features]
default = []
ml = ["tch"]  # Only include tch-rs when ml feature enabled
```

---

## Step 4: Create build.rs (Optional but Recommended)

**File**: `build.rs` (project root)

```rust
fn main() {
    #[cfg(feature = "ml")]
    {
        let libtorch_path = std::env::var("LIBTORCH")
            .expect(
                "LIBTORCH environment variable not set!\n\
                 Please set: LIBTORCH=D:\\Projects\\Lineage\\libtorch\n\
                 Or download from: https://download.pytorch.org/libtorch/cpu/"
            );
        
        println!("cargo:rustc-env=LIBTORCH_PATH={}", libtorch_path);
        println!("cargo:rustc-link-search=native={}/lib", libtorch_path);
        println!("cargo:rustc-link-lib=dylib=torch");
        println!("cargo:rustc-link-lib=dylib=torch_cpu");
    }
}
```

---

## Step 5: Test Installation

### Test 1: Verify Compilation
```bash
# Without ML (should always work)
cargo check

# With ML feature
cargo check --features ml
```

### Test 2: Simple Tensor Test

Create `src/finance/ml/mod.rs`:

```rust
#[cfg(feature = "ml")]
pub mod models;

#[cfg(feature = "ml")]
pub mod traits;

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "ml")]
    fn test_libtorch_basic() {
        use tch::Kind;
        let t = tch::Tensor::randn(&[3, 4], (Kind::Float, tch::Device::Cpu));
        assert_eq!(t.size(), vec![3, 4]);
    }
}
```

Run test:
```bash
cargo test --features ml test_libtorch_basic -- --nocapture
```

**Expected Output**:
```
test tests::test_libtorch_basic ... ok
```

---

## Step 6: Create ML Module Structure

```bash
# PowerShell
New-Item -ItemType Directory -Force -Path src\finance\ml\models
New-Item -ItemType Directory -Force -Path src\finance\ml\training
New-Item -ItemType Directory -Force -Path src\finance\ml\evolution
New-Item -ItemType Directory -Force -Path src\finance\ml\integration

# Create files
@(
    'src/finance/ml/mod.rs',
    'src/finance/ml/traits.rs',
    'src/finance/ml/models/mod.rs',
    'src/finance/ml/models/base.rs',
    'src/finance/ml/models/q_net.rs',
    'src/finance/ml/training/mod.rs',
    'src/finance/ml/training/replay_buffer.rs',
    'src/finance/ml/training/optimizer.rs',
    'src/finance/ml/evolution/mod.rs',
    'src/finance/ml/evolution/mutation.rs',
    'src/finance/ml/integration/mod.rs',
    'src/finance/ml/integration/agent_lifecycle.rs'
) | ForEach-Object { New-Item -ItemType File -Force -Path $_ }
```

---

## Step 7: Stub Out Module Files

### src/finance/ml/mod.rs
```rust
#![allow(dead_code)]  // During development

#[cfg(feature = "ml")]
pub mod traits;

#[cfg(feature = "ml")]
pub mod models;

#[cfg(feature = "ml")]
pub mod training;

#[cfg(feature = "ml")]
pub mod evolution;

#[cfg(feature = "ml")]
pub mod integration;

#[cfg(feature = "ml")]
pub mod errors {
    use thiserror::Error;
    
    #[derive(Error, Debug)]
    pub enum MlError {
        #[error("Tensor operation failed: {0}")]
        TensorError(String),
        
        #[error("Model serialization failed: {0}")]
        SerializationError(String),
        
        #[error("Training error: {0}")]
        TrainingError(String),
        
        #[error("Mutation error: {0}")]
        MutationError(String),
    }
}
```

### src/finance/ml/traits.rs (from Internal Document - Phase 2)
```rust
#[cfg(feature = "ml")]
use async_trait::async_trait;
#[cfg(feature = "ml")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "ml")]
#[derive(Clone, Debug)]
pub struct MarketState {
    pub prices: Vec<f32>,
    pub volatility: Vec<f32>,
    pub agent_capital: f32,
    pub scar_count: u32,
    pub win_loss_ratio: f32,
    pub timestamp: u64,
}

#[cfg(feature = "ml")]
#[async_trait]
pub trait MlStrategy: Send + Sync {
    async fn predict(&self, state: &MarketState) -> TradeDecision;
    fn update_weights(&mut self, gradients: &[f32]) -> Result<(), crate::finance::ml::errors::MlError>;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(&mut self, data: &[u8]) -> Result<(), crate::finance::ml::errors::MlError>;
    fn metadata(&self) -> ModelMetadata;
    fn mutate(&mut self, mutation_rate: f32) -> Result<(), crate::finance::ml::errors::MlError>;
}

#[cfg(feature = "ml")]
#[derive(Clone, Debug)]
pub struct TradeDecision {
    pub action: TradeAction,
    pub confidence: f32,
    pub amount: u64,
    pub model_id: String,
}

#[cfg(feature = "ml")]
#[derive(Clone, Debug)]
pub enum TradeAction {
    Buy,
    Sell,
    Hold,
}

#[cfg(feature = "ml")]
#[derive(Clone, Debug)]
pub struct ModelMetadata {
    pub name: String,
    pub version: String,
    pub parent_id: Option<String>,
    pub mutation_count: u32,
    pub generations: u32,
}
```

### src/finance/ml/models/mod.rs
```rust
#[cfg(feature = "ml")]
pub mod base;

#[cfg(feature = "ml")]
pub mod q_net;
```

### src/finance/ml/training/mod.rs
```rust
#[cfg(feature = "ml")]
pub mod replay_buffer;

#[cfg(feature = "ml")]
pub mod optimizer;
```

### src/finance/ml/evolution/mod.rs
```rust
#[cfg(feature = "ml")]
pub mod mutation;
```

### src/finance/ml/integration/mod.rs
```rust
#[cfg(feature = "ml")]
pub mod agent_lifecycle;
```

---

## Step 8: Update src/finance/mod.rs

Add ML module export:

```rust
// existing code...
pub mod agent;
pub mod traits;
pub mod data_providers;
pub mod market_data;
pub mod metrics;
pub mod scars;
pub mod spawning;
pub mod trade;
pub mod trust_scoring;
pub mod visualization;
pub mod advanced;

// ADD THIS:
#[cfg(feature = "ml")]
pub mod ml;
```

---

## Step 9: First Compile Test

```bash
# No ML (should work immediately)
cargo check

# With ML (tests libtorch linking)
cargo check --features ml

# If errors occur, see troubleshooting below
```

**Expected Output**:
```
   Compiling lineage-rs v0.2.1
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 15.23s
```

---

## Troubleshooting

### Error: "Cannot find libtorch"
```
error: linking with `link.exe` failed: exit code: 1104
  = note: LINK : fatal error LNK1104: cannot open file 'torch.lib'
```

**Solution**:
```powershell
# Verify LIBTORCH is set
echo $env:LIBTORCH

# If empty, set it
$env:LIBTORCH = "D:\Projects\Lineage\libtorch"

# Verify lib files exist
ls "$env:LIBTORCH\lib"
```

### Error: "feature ml not found"
```
error: feature `ml` not found in this workspace
```

**Solution**: Run with feature flag:
```bash
cargo check --features ml
# NOT: cargo check --all-features
```

### Error: "torch.lib not found after linking"
```
error[E0391]: cannot determine resolution for this import
```

**Solution**: Update `build.rs` to point to correct paths:
```rust
println!("cargo:rustc-link-search=native={}/lib", libtorch_path);
println!("cargo:rustc-link-search=native={}/lib64", libtorch_path);  // Try both
```

---

## Verification Checklist

- [ ] `LIBTORCH` environment variable set
- [ ] `libtorch/lib/torch.lib` exists
- [ ] `Cargo.toml` has `tch = "0.15"` dependency
- [ ] `build.rs` created and configured
- [ ] ML module structure created (src/finance/ml/)
- [ ] `cargo check --features ml` passes without errors
- [ ] `cargo test --features ml` runs successfully
- [ ] No linking errors about torch.lib or torch_cpu.lib

---

## Next Phase: Core Models

Once Phase 1 is complete, proceed to:
1. Implement `SimpleQNet` in `src/finance/ml/models/q_net.rs`
2. Add training loop in `src/finance/ml/training/optimizer.rs`
3. Create integration hooks in `src/finance/ml/integration/agent_lifecycle.rs`

See `ML_IMPLEMENTATION_INTERNAL.md` → Phase 2 for detailed code.

---

## Quick Command Reference

```bash
# Development
cargo check --features ml
cargo build --features ml
cargo test --features ml

# Release build (optimized)
cargo build --release --features ml

# Run with ML arena (once implemented)
cargo run --example arena_with_ml_agents --release --features ml

# Benchmark
cargo bench --features ml --bench ml_performance
```

---

**Status**: Phase 1 Ready for Implementation  
**Maintainer**: Lineage Team  
**Last Updated**: February 2, 2026
