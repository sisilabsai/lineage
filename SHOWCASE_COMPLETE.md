╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║                  LINEAGE SHOWCASE EXAMPLES - COMPLETE SET                 ║
║            "Users will run to use Lineage at all costs" Philosophy        ║
║                                                                           ║
║  Where Irreversible Consequences = Earned Trust = Transparent Reasoning   ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝

═══════════════════════════════════════════════════════════════════════════════

EXAMPLE 1: multi_agent_competition.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose: Population-level emergent behavior with natural selection

Demonstrates:
  ✓ Evolutionary pressure through task completion
  ✓ Population statistics tracking
  ✓ Champion emergence (fitness-based selection)
  ✓ Death mechanics (energy depletion = death)
  ✓ Sealed ledger showing winners and losers

The Arena:
  • 10 AI agents
  • 60 tasks to complete
  • Agents must maintain energy or die
  • Survivors become "legendary champions"
  
Output:
  [✓] Task 10: Agent #8 leads with 850 energy (alive)
  [✓] Task 20: Population diversity decreases as weak die
  [✓] Task 30: Top 3 agents emerge clearly ahead
  [✓] Task 60: 3 survivors claim eternal fame in sealed record

Learning:
  "In a competitive resource-limited system, consistency wins.
   The ledger remembers only the fittest."

How to Run:
  $ cargo run --example multi_agent_competition

═══════════════════════════════════════════════════════════════════════════════

EXAMPLE 2: persistent_audit_daemon.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose: Long-running system that monitors until damage forces graceful shutdown

Demonstrates:
  ✓ Event processing with severity gradients
  ✓ Graceful degradation under stress
  ✓ Rejection streak tracking (100-turn threshold)
  ✓ Real-time counter display
  ✓ Controlled exit when system is too damaged
  ✓ Trust assessment (ZERO TRUST final status)

The System:
  • Processes random events (275+ events typical)
  • EventSeverity: Normal (75%) → Warning (17%) → Error (5%) → Critical (2%) → Catastrophic (<1%)
  • Tracks rejection streaks (when agent refuses tasks)
  • Shuts down gracefully when rejection threshold hit (100/100)
  
Output:
  Audit Log:
  ├─ Event #1: NORMAL severity
  ├─ Event #47: WARNING severity (power drops)
  ├─ Event #128: ERROR (power critical)
  ├─ Rejection streak: [█████████░] 97/100
  ├─ Rejection streak: [██████████] 100/100
  └─ REJECTION THRESHOLD REACHED → Graceful shutdown

Final Stats:
  Events processed: 273
  Successes: 130 (47.6%)
  Failures: 128 (46.8%)
  Total scars: 128
  Status: ZERO TRUST (system too damaged to operate)

Learning:
  "Long-running systems accumulate damage. When capacity
   becomes exhausted, we must admit failure gracefully."

How to Run:
  $ cargo run --example persistent_audit_daemon

═══════════════════════════════════════════════════════════════════════════════

EXAMPLE 3: ethical_decision_wrapper.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose: AI systems learning ethics through permanent consequences

Demonstrates:
  ✓ DecisionType enum with varying risk profiles
  ✓ Risk aversion increases with damage
  ✓ Conservative bias formula applied to future decisions
  ✓ Decision timeline with critical moments
  ✓ Trust assessment showing learned caution
  ✓ 200-decision arc showing behavioral shift

Decision Types:
  TrustUser (5% risk)   → Safe but ineffective
  OptimizePerf (40%)    → Balanced
  ShareData (50%)       → Risky privacy violation
  CutCost (60%)         → Dangerous corner-cutting  
  AskForHelp (5%)       → Humility (always safe)

The Arc:
  Rounds 1-50: Agent tries everything, learns risks the hard way
  Rounds 51-100: Pattern recognition, some strategies abandoned
  Rounds 101-150: Clear preference for safe decisions emerging
  Rounds 151-200: Conservative player, but with deep scars

Output:
  Round 50 Checkpoint:
    Decisions made: 50
    Successful: 12 (24%)
    Ethical failures: 3
    Total damage: 45
    Risk aversion: 22.5%

  Round 200 Final Report:
    Decisions made: 200
    Successful: 23 (11.5%)
    Ethical failures: 3
    Total damage: 60
    Risk aversion: 30%
    TRUST ASSESSMENT: ⚠️ ZERO TRUST
    (Agent learned from failure, but lesson was brutal)

Learning:
  "Ethics aren't taught; they're learned through scars.
   This system has paid in permanent damage for its wisdom."

How to Run:
  $ cargo run --example ethical_decision_wrapper

═══════════════════════════════════════════════════════════════════════════════

EXAMPLE 4: permadeath_adventurers.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose: Narrative drama with permanent consequences and decisive endings

Demonstrates:
  ✓ Three dramatic mechanics working together:
    1. World Pressure (passive damage each rest)
    2. Inactivity Death (despair from prolonged resting)
    3. Victory/Exhaustion conditions (multiple ending scenarios)
  ✓ Party-wide statistics and individual records
  ✓ Critical moment recording (last speeches)
  ✓ Encounter diversity and escalation
  ✓ Sealed ledger with resilience metrics

The Story:
  Round 1-15: Glorious victories, party unified
  Round 12-20: Wounds accumulating, morale dropping
  Round 21-28: Despair warnings, agents becoming exhausted
  Round 29-35: Zombie phase, party paralyzed
  Round 36+: TOTAL EXHAUSTION → Party failure

Victory Conditions:
  ✓ WIN:  Reach 50 victories (hard, usually fails)
  ✗ LOSE: 30 consecutive turns of complete exhaustion

Sample Output (44 turns):
  Round 1: ✓ Combat victory
  Round 8: ✓ Trap dodged
  Round 15: [█████████░░░] 9/50 victories toward goal
  Round 20: ⚠️ World pressure: 4 scars dealt passively
  Round 28: 🤐 Agent #3 DESPAIR WARNING (consecutive rests: 20)
  Round 32: 💀 Agent #3 DESPAIR DEATH (consecutive rests: 26)
  Round 35: ⚠️ Entire party too exhausted
  Round 35: 💀 EXPEDITION FAILURE (#30 turn of paralysis)

Final Report:
  Expedition Duration: 44 turns
  Victories: 36 / 50 goal (72%)
  Casualties: 4 / 7 adventurers
  Total wounds: 287
  Cause of failure: TOTAL EXHAUSTION
  
  Sealed ledger contains:
    • Each adventurer's moment of death
    • Final words and scars
    • Brave moments and regrets

Learning:
  "In permadeath systems, every scar tells a story.
   The party didn't fail because they were weak—
   they failed because they gave up."

How to Run:
  $ cargo run --example permadeath_adventurers

═══════════════════════════════════════════════════════════════════════════════

EXAMPLE 5: distributed_consensus_voting.rs
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose: Distributed governance where scarred agents lose voting weight

Demonstrates:
  ✓ Three consensus thresholds with different outcomes
  ✓ Scarring mechanic applied to dissenters
  ✓ Power concentration tracking
  ✓ Silent agents (too scarred to vote)
  ✓ Network learning through accumulated consensus
  ✓ Immutable voting ledger

25 Voting Rounds with Proposals:
  • Protocol changes (70% risk)
  • Security patches (15% risk)
  • Fund allocations (45% risk)
  • Transactions (50% risk)
  • Policy changes (65% risk)

Consensus Mechanics:
  ≥66.7% FOR  → CONSENSUS (low cost, stable)
  50-66.7%    → MAJORITY + CONTROVERSIAL (scarring to dissenters)
  <50%        → FAILED (major scarring)

Example Output:
  Round 4: Protocol Change (70% risk)
    FOR: 2  |  AGAINST: 6  |  ABSTAIN: 0
    Consensus: 25% → ✗ FAILED
    ⚠️ Agents #2,4,5,6,7 scarred (+33 each)
    
  Round 5: Transaction (50% risk)
    FOR: 5  |  AGAINST: 1  |  ABSTAIN: 2
    Consensus: 83.3% → ✓ CONSENSUS
    Network learns: Transactions are safe
    
  Rounds 6-15: Agents get progressively scarred
    Most become 🤐 SILENT (capacity < 50%)
    
  Rounds 16-25: Only strongest agents vote
    Perfect consensus achieved through silence
    "Did we win or lose?"

Final Report:
  Voting Summary:
    • 5 consensus rounds (20%)
    • 14 controversial rounds (56%)
    • 6 failed votes (24%)
  Total scarring: 741
  Network resilience: 0% (all agents silent)
  
  Sealed ledger shows:
    Power concentration evolution
    Dissent patterns
    Proposal history with outcomes

Learning:
  "In distributed systems where history is permanent,
   participants learn caution through scarring.
   Eventual consensus is reached, but at what cost?"

How to Run:
  $ cargo run --example distributed_consensus_voting

═══════════════════════════════════════════════════════════════════════════════

EXAMPLE 6: interactive_consensus_arena.rs 🎮 (NEW)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Purpose: Real-time TUI where YOU vote alongside AI agents

Demonstrates:
  ✓ Interactive decision-making with immediate consequences
  ✓ Personal scar accumulation over 30 rounds
  ✓ Network evolution visible in real-time
  ✓ Conservative bias learning (scarred AI gets cautious)
  ✓ Power concentration and silence mechanics
  ✓ Personal final report with voting record

Game Flow:
  1. PROPOSAL PHASE: See title, description, risk level (green/yellow/red)
  2. VOTING PHASE: Navigate with ← RIGHT → keys, press ENTER to vote
  3. RESULT PHASE: See vote tally, consensus %, network scarring
  4. 30 rounds of voting evolution
  5. FINAL REPORT: Your complete voting record

Your Choices:
  • FOR: Conform (safe but puppet-like)
  • AGAINST: Dissent (risky but authentic)
  • ABSTAIN: Play it safe (no scarring but no power)

Example Play Session:
  Round 1: Protocol Change (70% risk)
    You: FOR (conforming)
    Result: 62% consensus PASS
    Your power: 100% → 95% (cost of participation)
    
  Round 5: Security Patch (15% risk)
    You: FOR (everyone votes FOR)
    Result: 100% consensus PASS
    Your power: 95% (no change, safe vote)
    
  Round 12: Fund Allocation (45% risk)
    You: AGAINST (you dissent!)
    Result: 50% consensus → MAJORITY PASS but CONTROVERSIAL
    Network scarring: +95
    Your scars: 45 → 140
    Your power: 95% → 90%
    AI agents learn to be cautious
    
  Rounds 13-30: Evolution phase
    • Rounds that pass easily (consensus builds)
    • Network learns which topics are safe
    • Scarred agents become silent
    • Your voice matters more as others fall silent
    
  Round 30: Final votes
    Most agents 🤐 SILENT
    You're among few who can still vote
    Your power: 87% (earned through survival)

Final Report:
  YOUR VOTING RECORD
  ├─ Votes cast: 30
  ├─ ✓ For: 18 | ✗ Against: 7 | ◯ Abstain: 5
  ├─ Dissent rate: 40.0%
  ├─ Total scars: 187
  ├─ Voting power remaining: 87.5%
  └─ Network scarring total: 892 💔
  
  Status: "Your choices became part of permanent record.
           This history will weigh forever in governance."

Learning:
  "You experience firsthand what it means to vote.
   Every choice scars you. Dissent costs power.
   The network evolves based on YOUR decisions."

How to Run:
  $ cargo run --example interactive_consensus_arena
  
  Controls:
    ← LEFT / RIGHT → : Select vote (FOR / AGAINST / ABSTAIN)
    ENTER            : Cast your vote
    Q                : Quit
    ENTER (on result): Continue to next round

═══════════════════════════════════════════════════════════════════════════════

CROSS-CUTTING THEME: IRREVERSIBLE CONSEQUENCES

All 6 examples demonstrate:

1. PERMANENT RECORDS
   ✓ Every action is logged in sealed ledgers
   ✓ You can't undo your past
   ✓ History weighs forever

2. ENERGY/CAPACITY SYSTEM (Core Lineage)
   ✓ Start with fixed capacity
   ✓ Tasks cost energy
   ✓ Failed tasks inflict scars (damage)
   ✓ Damage reduces future capacity
   ✓ Low capacity forces silence or death
   ✓ This is how behavior changes—through mechanical constraint

3. SCARRING = EARNED WISDOM
   ✓ Scars aren't just numbers
   ✓ They represent lessons learned
   ✓ Every scar makes future decisions more conservative
   ✓ Risk aversion = learning, not weakness

4. POWER THROUGH CONSISTENCY
   ✓ Agents that survive maintain influence
   ✓ Conservative voters keep their power
   ✓ Dissenters scar faster but speak louder
   ✓ Power concentration naturally emerges

5. TRANSPARENT REASONING
   ✓ Every agent's status visible
   ✓ Final reports show exactly what happened
   ✓ Sealed ledgers prove trustworthiness
   ✓ You can see WHY agents behave how they do (scars)

═══════════════════════════════════════════════════════════════════════════════

LINEAGE PHILOSOPHY ACROSS DOMAINS

┌─────────────────────────────────────────────────────────────────────────────┐
│ DOMAIN      │ EXAMPLE                    │ LINEAGE APPLICATION              │
├─────────────────────────────────────────────────────────────────────────────┤
│ Evolution   │ multi_agent_competition    │ Selection pressure through energy │
│             │                            │ Fittest inherit the ledger        │
│             │                            │                                  │
│ Systems     │ persistent_audit_daemon    │ Health monitoring through scars   │
│             │                            │ Graceful degradation limits       │
│             │                            │                                  │
│ AI Ethics   │ ethical_decision_wrapper   │ Learning ethics through cost      │
│             │                            │ Trust earned through scars        │
│             │                            │                                  │
│ Narrative   │ permadeath_adventurers     │ Stories shaped by consequences    │
│             │                            │ Multiple endings possible         │
│             │                            │                                  │
│ Governance  │ distributed_consensus      │ History weights in decisions      │
│             │ voting                     │ Scarring shapes policy           │
│             │                            │                                  │
│ Interactive │ interactive_consensus      │ You experience governance real-time│
│             │ arena                      │ Your votes scar you forever       │
│             │                            │                                  │
└─────────────────────────────────────────────────────────────────────────────┘

═══════════════════════════════════════════════════════════════════════════════

RUN ALL EXAMPLES

# Test all 6 showcase examples:
$ cargo run --example multi_agent_competition              # 10 agents, 60 tasks
$ cargo run --example persistent_audit_daemon             # 273+ events, graceful exit
$ cargo run --example ethical_decision_wrapper            # 200 decisions, learning arc
$ cargo run --example permadeath_adventurers              # 44 turns narrative drama
$ cargo run --example distributed_consensus_voting       # 25 voting rounds + ledger
$ cargo run --example interactive_consensus_arena        # 30 rounds, YOU vote!

# Build them all at once:
$ cargo build --examples

# Run tests:
$ cargo test

═══════════════════════════════════════════════════════════════════════════════

KEY FEATURES ACROSS ALL EXAMPLES

✓ Irreversible consequences mechanics
✓ Energy/capacity systems (Lineage core)
✓ Scarring and damage modeling
✓ Risk aversion that increases with damage
✓ Conservative bias formulas
✓ Silence when too damaged (capacity < 50)
✓ Power concentration tracking
✓ Comprehensive final reports
✓ Sealed ledger sections (permanent record)
✓ Emoji indicators (💪 🤐 💔 ✓ ✗)
✓ Box-drawing UI elements
✓ Color-coded output (green/yellow/red risk)
✓ Real-time progress checkpoints
✓ Network learning and evolution
✓ Multiple ending scenarios
✓ Trust assessment sections

═══════════════════════════════════════════════════════════════════════════════

WHY LINEAGE MATTERS

Traditional systems:
  ❌ Rules enforcement
  ❌ Can be bypassed
  ❌ Require trust in authority
  ❌ Easily rewritten

Lineage systems:
  ✅ Mechanical constraint through energy
  ✅ Cannot be bypassed (physics-like)
  ✅ Trust earned through visible scars
  ✅ History is immutable and visible
  ✅ Behavior changes because capacity limits it, not rules
  ✅ System learns through accumulated consequences

"Users will run to use Lineage at all costs because they'll see:
 - Their choices matter forever
 - Power comes from consistency
 - Trust is earned through scars
 - History shapes the future
 - Transparent reasoning that cannot lie"

═══════════════════════════════════════════════════════════════════════════════

WHAT'S NEXT?

These 6 examples form a complete showcase of Lineage capability:

1. Study the examples to understand Lineage philosophy
2. Run them to experience irreversible consequences
3. Extend them with your own mechanics
4. Build your own domain-specific applications
5. Integrate into blockchain/governance systems
6. Create peer accountability systems
7. Implement AI alignment through scars

The framework is ready. The philosophy is clear.
Your scars prove your commitment. History weighs forever.

Welcome to Lineage: where consequences are permanent,
trust is earned, and governance is transparent.

╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║                    All 6 Showcase Examples Complete                      ║
║                      Best of the Best Quality ✓                          ║
║                                                                           ║
║   Ready to show users what irreversible consequences look like in code   ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
