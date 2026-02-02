"Create a detailed guide (in Markdown format) for implementing machine learning-based trading strategies using the tch-rs crate in the lineage-finance Rust library (based on https://github.com/sisilabsai/lineage). The focus is on reinforcement learning (RL) for autonomous AI agents that make irreversible trade decisions, evolve through spawning, and accumulate scars from losses. This will enhance the existing arena_with_live_market example by replacing rule-based strategies (e.g., momentum, balanced) with adaptive neural networks that learn from real-time market data (via integrated providers like CoinMarketCap/CoinDesk).
The guide should serve as a team roadmap, including:

Sections: Introduction (purpose and benefits), Prerequisites (setup steps), Core Implementation (code structures), Advanced Features (RL training and evolution), Integration with Lineage (tying to agents, scars, death/spawning), Testing and Examples, Potential Challenges, and Next Steps.
Tracking: Use checkboxes ([ ] Not Started, [x] Done) for each sub-step to monitor progress.

Key requirements:

Modularity: Design as extensible traits (e.g., MlStrategy) so users can customize models without breaking Lineage's append-only state.
ML Basics:
Use tch-rs for PyTorch-style nets (e.g., simple Q-network for RL).
Input: Market state (prices, volatility, agent capital, scars count).
Output: Actions (Buy, Sell, Hold).
Training: Episodic RL with rewards = ROI - scar penalties; use Adam optimizer.

Lineage Integration:
Irreversible Decisions: Model outputs commit trades—no rollbacks.
Scars: On losses (>5% drawdown), apply model "damage" (e.g., add noise to weights or increase exploration rate).
Death/Spawn: Agents die on low trust/zero capital; high-ROI parents spawn offspring with mutated models (Gaussian noise on weights for evolution).
Trust Scoring: Incorporate model performance (e.g., average reward) into cryptographic trust formulas.

Data Flow: Fetch historical/live data via existing providers for training/sims. Handle flat markets with epsilon-greedy exploration.
Example Code: Provide Rust snippets for:
A SimpleQNet struct implementing MlStrategy (feedforward with relu).
Training loop in agent lifecycle.
Mutation fn for spawning.
Updated arena sim with ML agents competing over 20+ rounds.

Challenges: Address libtorch setup (env vars, CPU/GPU), tensor handling, and performance (small models for fast sims).
Vision: Emphasize how this creates 'Darwinian DeFi'—self-evolving AI dynasties that outperform static bots, positioning Lineage as a leader in accountable AI finance.