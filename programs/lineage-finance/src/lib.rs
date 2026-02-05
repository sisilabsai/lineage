//! Lineage Finance Solana Program
//! 
//! Bridges Solana blockchain with the Lineage trading agent system.
//! Serves as the on-chain settlement and oracle layer for:
//! - Agent creation backed by Solana accounts
//! - Trade execution with Pyth price oracle data
//! - Capital management via SPL tokens
//! - Agent lifecycle (birth, trading, death, resurrection)
//! - Evolutionary spawning with genetic mutation
//!
//! This program is a **blockchain interface** to the Lineage finance library
//! at ../src/finance. Off-chain agents use Pyth prices and this program
//! to execute trades with cryptographic finality.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use pyth_sdk_solana::load_price_feed_from_account;

declare_id!("LineageNqytQrZJgFYYJrmKBrGfVJKVZJnHwkzZCRwJqz");

#[program]
pub mod lineage_finance {
    use super::*;

    /// Initialize a new trading agent account on-chain
    ///
    /// Creates a Solana account that represents a trading agent,
    /// backed by the Lineage finance library. Agent trades are settled
    /// on-chain with cryptographic proof via this account.
    ///
    /// # Integration with /src/finance
    /// - AgentId maps to Solana pubkey
    /// - Capital is tracked in lamports/SPL tokens
    /// - Trade history is recorded in events (queryable on-chain)
    /// - Scars and lifecycle match the Rust library exactly
    pub fn create_agent(
        ctx: Context<CreateAgent>,
        name: String,
        initial_capital: u64,
        model_weights: Vec<u8>, // Serialized weights from finance::Agent
    ) -> Result<()> {
        require!(name.len() <= 50, ErrorCode::NameTooLong);
        require!(initial_capital > 0, ErrorCode::InvalidCapital);
        require!(model_weights.len() <= 1024, ErrorCode::ModelWeightsTooLarge);

        let agent = &mut ctx.accounts.agent;
        agent.authority = ctx.accounts.authority.key();
        agent.name = name;
        agent.capital = initial_capital;
        agent.model_weights = model_weights;
        agent.total_trades = 0;
        agent.winning_trades = 0;
        agent.losing_trades = 0;
        agent.scars = 0;
        agent.current_loss_streak = 0;
        agent.creation_time = Clock::get()?.unix_timestamp;
        agent.last_trade_time = 0;
        agent.is_alive = true;
        agent.generation = 1;
        agent.parent_id = None;

        msg!("Agent {} created with {} capital", agent.name, initial_capital);
        Ok(())
    }

    /// Execute a trade with real-time Pyth oracle price
    ///
    /// Executes a trade order using Pyth's BTC-USD price feed.
    /// All trade results are irreversible once executed (matching
    /// the trade.rs immutability guarantee).
    ///
    /// # Integration with /src/finance
    /// - Uses Pyth price as oracle source (replaces market_data provider)
    /// - Applies scars for losses > 5% (matching scars.rs logic)
    /// - Records trade in on-chain events (event history)
    /// - Updates capital immediately and irreversibly
    pub fn execute_trade(
        ctx: Context<ExecuteTrade>,
        position_type: u8, // 0 = long, 1 = short
        position_size: u64,
        expected_slippage_bps: u16, // 0-10000 = 0-100%
    ) -> Result<()> {
        let agent = &mut ctx.accounts.agent;
        
        require!(agent.is_alive, ErrorCode::AgentDead);
        require!(position_size > 0 && position_size <= agent.capital, ErrorCode::InvalidPositionSize);
        require!(position_type <= 1, ErrorCode::InvalidPositionType);

        // Load Pyth price feed for BTC-USD
        let price_feed = load_price_feed_from_account(&ctx.accounts.pyth_account)?;
        let current_price = price_feed
            .get_current_price()
            .ok_or(ErrorCode::PriceUnavailable)?;

        require!(current_price.price > 0, ErrorCode::InvalidPrice);

        let now = Clock::get()?.unix_timestamp;
        let time_since_last = now.saturating_sub(agent.last_trade_time);
        require!(time_since_last >= 5, ErrorCode::TradeCooldown);

        // Simulate trade outcome (in production: connect to actual execution)
        let pnl = simulate_trade_outcome(
            position_type,
            position_size,
            current_price.price as u64,
            expected_slippage_bps,
        );

        let pnl_percentage = (pnl * 10000) / position_size as i64;
        let is_win = pnl > 0;

        // Update agent state - matching finance::Agent exactly
        if is_win {
            agent.capital = agent.capital.saturating_add(pnl as u64);
            agent.winning_trades = agent.winning_trades.saturating_add(1);
            agent.current_loss_streak = 0;
        } else {
            let loss = (-pnl) as u64;
            agent.capital = agent.capital.saturating_sub(loss);
            agent.losing_trades = agent.losing_trades.saturating_add(1);
            agent.current_loss_streak = agent.current_loss_streak.saturating_add(1);

            // Apply scar for losses > 5% (matching scars.rs::ScarSeverity::Major)
            if pnl_percentage < -500 {
                agent.scars = agent.scars.saturating_add(1);
                
                // Death after 10+ scars (matching agent.rs TerminallyDamaged)
                if agent.scars >= 10 {
                    agent.is_alive = false;
                }
            }
        }

        agent.total_trades = agent.total_trades.saturating_add(1);
        agent.last_trade_time = now;

        // Emit event (queryable on-chain, matches trade.rs Trade struct)
        emit!(TradeExecuted {
            agent: agent.key(),
            position_type,
            position_size,
            pnl,
            is_win,
            btc_price: current_price.price as u64,
            scars_applied: !is_win && pnl_percentage < -500,
        });

        Ok(())
    }

    /// Spawn offspring with genetic mutation
    ///
    /// Creates a child agent by mutating parent's model weights.
    /// Matches spawning.rs mutation logic exactly.
    pub fn spawn_offspring(
        ctx: Context<SpawnOffspring>,
        offspring_name: String,
        mutation_magnitude: u16,
    ) -> Result<()> {
        let parent = &ctx.accounts.parent_agent;
        
        require!(parent.is_alive, ErrorCode::CannotSpawnFromDeadAgent);
        require!(parent.total_trades >= 10, ErrorCode::InsufficientTradeHistory);
        require!(offspring_name.len() <= 50, ErrorCode::NameTooLong);
        require!(mutation_magnitude <= 5000, ErrorCode::InvalidMutationRate);

        let mutated_weights = apply_genetic_mutation(
            &parent.model_weights,
            mutation_magnitude,
        )?;

        let offspring = &mut ctx.accounts.offspring_agent;
        offspring.authority = ctx.accounts.authority.key();
        offspring.name = offspring_name;
        offspring.capital = parent.capital / 2;
        offspring.model_weights = mutated_weights;
        offspring.total_trades = 0;
        offspring.winning_trades = 0;
        offspring.losing_trades = 0;
        offspring.scars = 0;
        offspring.current_loss_streak = 0;
        offspring.creation_time = Clock::get()?.unix_timestamp;
        offspring.last_trade_time = 0;
        offspring.is_alive = true;
        offspring.generation = parent.generation.saturating_add(1);
        offspring.parent_id = Some(parent.key());

        emit!(OffspringSpawned {
            parent: parent.key(),
            offspring: offspring.key(),
            generation: offspring.generation,
        });

        Ok(())
    }

    /// Resurrect a dead agent via SPL token stake
    ///
    /// Matches agent.rs resurrection mechanics: costs 100 tokens per scar.
    /// Resets metrics but preserves lineage/generation counter.
    pub fn resurrect(
        ctx: Context<Resurrect>,
        new_capital: u64,
    ) -> Result<()> {
        let agent = &mut ctx.accounts.agent;
        
        require!(!agent.is_alive, ErrorCode::AgentAlreadyAlive);
        require!(new_capital > 0, ErrorCode::InvalidCapital);

        // Resurrection cost: 100 tokens per scar
        let cost = (agent.scars as u64) * 100_000_000;

        let token_balance = ctx.accounts.user_token_account.amount;
        require!(token_balance >= cost, ErrorCode::InsufficientTokensForResurrection);

        // Transfer tokens
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, cost)?;

        // Reset agent (matching agent.rs Lineage reset)
        agent.capital = new_capital;
        agent.is_alive = true;
        agent.total_trades = 0;
        agent.winning_trades = 0;
        agent.losing_trades = 0;
        agent.scars = 0;
        agent.current_loss_streak = 0;
        agent.generation = agent.generation.saturating_add(1);

        emit!(AgentResurrected {
            agent: agent.key(),
            cost,
            generation: agent.generation,
        });

        Ok(())
    }
}

// ============================================================================
// ACCOUNTS
// ============================================================================

#[account]
#[derive(Default)]
pub struct Agent {
    /// Authority/owner (maps to finance::Agent::authority)
    pub authority: Pubkey,
    
    /// Agent name (finance::Agent.strategy_name)
    pub name: String,
    
    /// Current capital in lamports (finance::Agent.capital)
    pub capital: u64,
    
    /// Serialized ML weights (finance::Agent.model_weights)
    pub model_weights: Vec<u8>,
    
    /// Total trades executed (finance::Agent.metrics.total_trades)
    pub total_trades: u64,
    
    /// Winning trades (calculated in dashboard)
    pub winning_trades: u64,
    
    /// Losing trades (calculated in dashboard)
    pub losing_trades: u64,
    
    /// Scar count (finance::Agent.lineage.scars)
    pub scars: u32,
    
    /// Current loss streak (for win rate calc)
    pub current_loss_streak: u32,
    
    /// When account was created
    pub creation_time: i64,
    
    /// Last trade execution time
    pub last_trade_time: i64,
    
    /// Alive status (finance::FinanceAgentStatus)
    pub is_alive: bool,
    
    /// Generation number (finance::Agent.generation)
    pub generation: u64,
    
    /// Parent agent pubkey (finance::Agent.parent_id)
    pub parent_id: Option<Pubkey>,
}

// ============================================================================
// CONTEXTS
// ============================================================================

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateAgent<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Agent>() + 50 + 1024, // Extra space for dynamic fields
        seeds = [b"agent", authority.key().as_ref()],
        bump
    )]
    pub agent: Account<'info, Agent>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTrade<'info> {
    #[account(mut)]
    pub agent: Account<'info, Agent>,
    
    /// Pyth price feed for BTC-USD
    pub pyth_account: AccountInfo<'info>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SpawnOffspring<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Agent>() + 50 + 1024,
    )]
    pub offspring_agent: Account<'info, Agent>,
    
    pub parent_agent: Account<'info, Agent>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Resurrect<'info> {
    #[account(mut)]
    pub agent: Account<'info, Agent>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

// ============================================================================
// EVENTS
// ============================================================================

#[event]
pub struct TradeExecuted {
    pub agent: Pubkey,
    pub position_type: u8,    // 0 = long, 1 = short
    pub position_size: u64,
    pub pnl: i64,
    pub is_win: bool,
    pub btc_price: u64,       // Current BTC price from Pyth
    pub scars_applied: bool,
    pub agent_alive: bool,
}

#[event]
pub struct OffspringSpawned {
    pub parent: Pubkey,
    pub offspring: Pubkey,
    pub generation: u64,
    pub inherited_capital: u64,
}

#[event]
pub struct AgentResurrected {
    pub agent: Pubkey,
    pub resurrection_cost: u64,
    pub new_capital: u64,
    pub generation: u64,
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Simulate trade outcome based on market conditions
/// In production, this connects to actual price execution
fn simulate_trade_outcome(
    position_type: u8,
    position_size: u64,
    current_price: u64,
    exit_condition: u16,
) -> i64 {
    // Simplified simulation: 60% win rate with randomized outcomes
    // In production: Connect to actual market price oracle
    let win_probability = 6000; // 60%
    let random = ((current_price as u128 * position_size as u128) % 10000) as u16;
    
    let is_win = random < win_probability;
    
    if is_win {
        // Win: 1-5% profit
        let profit_percent = ((random % 400) as u64 + 100) * position_size / 10000;
        profit_percent as i64
    } else {
        // Loss: 0.5-2% loss
        let loss_percent = ((random % 150) as u64 + 50) * position_size / 10000;
        -(loss_percent as i64)
    }
}

/// Apply genetic mutation to model weights
/// Matches spawning.rs::apply_genetic_mutation logic
fn apply_genetic_mutation(
    parent_weights: &[u8],
    mutation_magnitude: u16,
) -> Result<Vec<u8>> {
    require!(mutation_magnitude <= 5000, ErrorCode::InvalidMutationRate);

    let mut mutated = parent_weights.to_vec();
    
    for weight in mutated.iter_mut() {
        // Mutation: randomly adjust each weight by up to mutation_magnitude basis points
        let random_factor = ((*weight as u16).wrapping_mul(12345)) % 10000;
        if random_factor < mutation_magnitude {
            let adjustment = ((*weight as u16 * mutation_magnitude) / 10000) as u8;
            *weight = weight.saturating_add(adjustment);
        }
    }
    
    Ok(mutated)
}

// ============================================================================
// ERROR CODES
// ============================================================================

#[error_code]
pub enum ErrorCode {
    #[msg("Agent name exceeds 50 character limit")]
    NameTooLong,

    #[msg("Invalid capital amount (must be > 0)")]
    InvalidCapital,

    #[msg("Model weights exceed 1KB limit")]
    ModelWeightsTooLarge,

    #[msg("Agent is dead and cannot execute trades (FinanceAgentStatus::TerminallyDamaged)")]
    AgentDead,

    #[msg("Insufficient capital for position size")]
    InsufficientCapital = 10,

    #[msg("Invalid position size")]
    InvalidPositionSize,

    #[msg("Invalid position type (must be 0=long or 1=short)")]
    InvalidPositionType,

    #[msg("Pyth price feed is unavailable")]
    PriceUnavailable,

    #[msg("Price data is invalid or zero")]
    InvalidPrice,

    #[msg("Trade cooldown not met (5 second minimum between trades)")]
    TradeCooldown,

    #[msg("Cannot spawn offspring from dead agent")]
    CannotSpawnFromDeadAgent = 20,

    #[msg("Parent agent has insufficient trade history (requires 10+ trades)")]
    InsufficientTradeHistory,

    #[msg("Invalid mutation rate for genetic algorithm")]
    InvalidMutationRate,

    #[msg("Agent is already alive (cannot resurrect alive agent)")]
    AgentAlreadyAlive,

    #[msg("Insufficient SPL tokens for resurrection cost (100 per scar)")]
    InsufficientTokensForResurrection,
}
