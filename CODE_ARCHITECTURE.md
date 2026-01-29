╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║              INTERACTIVE CONSENSUS ARENA - CODE STRUCTURE                 ║
║                                                                           ║
║                     "MORE THAN POWERFUL" Implementation                   ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝

═══════════════════════════════════════════════════════════════════════════════

📐 CODE ARCHITECTURE

File: examples/interactive_consensus_arena.rs (423 lines)

┌─ IMPORTS (Lines 1-20) ─────────────────────────────────────────────────────┐
│ use lineage::{TaskAgent, Task, TaskOutcome, TaskResult}                   │
│ use ratatui::{ ... }  // Terminal UI framework                           │
│ use crossterm::{ ... }  // Cross-platform terminal handling              │
│ use std::{ io, time::Duration, collections::VecDeque }                   │
│ use rand::Rng  // Random number generation                               │
│                                                                           │
│ → Core Lineage integration + Professional TUI stack                      │
└─────────────────────────────────────────────────────────────────────────────┘

┌─ CONSTANTS (Lines 22-25) ──────────────────────────────────────────────────┐
│ const NUM_AI_AGENTS: usize = 7;    // 7 AI agents to vote alongside you   │
│ const HUMAN_INDEX: usize = 7;      // You are agent #7                    │
│ const INITIAL_POWER: u64 = 1500;   // Your starting energy/capacity      │
│ const MAX_ROUNDS: usize = 30;      // 30 voting rounds total              │
│                                                                           │
│ → Balanced numbers for engagement and replayability                      │
└─────────────────────────────────────────────────────────────────────────────┘

┌─ PROPOSAL TYPE STRUCT (Lines 27-31) ───────────────────────────────────────┐
│ #[derive(Clone)]                                                          │
│ struct ProposalType {                                                    │
│     title: &'static str,          // e.g., "Protocol: Increase block size"│
│     description: &'static str,    // e.g., "Higher throughput"           │
│     base_risk: f32,               // Risk 0.15 (15%) to 0.70 (70%)       │
│ }                                                                         │
│                                                                           │
│ → Defines 6 proposal types rotating through voting rounds                │
│ → Color-coded based on risk level (green/yellow/red)                     │
└─────────────────────────────────────────────────────────────────────────────┘

┌─ AGENT STATS STRUCT (Lines 33-42) ─────────────────────────────────────────┐
│ struct AgentStats {                                                      │
│     votes_cast: u32,               // Total votes you've cast             │
│     votes_for: u32,                // # of FOR votes                     │
│     votes_against: u32,            // # of AGAINST votes                 │
│     votes_abstain: u32,            // # of ABSTAIN votes                │
│     scars_from_dissent: u64,       // Total scars from dissenting        │
│     dissent_rate: f32,             // Percentage of non-FOR votes        │
│ }                                                                         │
│                                                                           │
│ impl AgentStats {                                                        │
│     fn update_dissent_rate(&mut self) { ... }  // Calculate percentage  │
│ }                                                                         │
│                                                                           │
│ → Tracks your complete voting record across all 30 rounds               │
│ → Used for final report generation                                      │
└─────────────────────────────────────────────────────────────────────────────┘

┌─ VOTING ROUND STRUCT (Lines 63-71) ────────────────────────────────────────┐
│ struct VotingRound {                                                     │
│     proposal_idx: usize,           // Which proposal type                │
│     for_votes: u32,                // # agents voting FOR               │
│     against_votes: u32,            // # agents voting AGAINST           │
│     abstain_votes: u32,            // # agents voting ABSTAIN           │
│     consensus_pct: f32,            // Consensus percentage (0-100)      │
│     your_vote: usize,              // Your vote: 0=FOR, 1=AGAINST, 2=ABSTAIN
│     scarring: u64,                 // Network scarring this round       │
│ }                                                                         │
│                                                                           │
│ → Records complete history of each voting round                         │
│ → Stored in VecDeque for recent history display                         │
└─────────────────────────────────────────────────────────────────────────────┘

═══════════════════════════════════════════════════════════════════════════════

🔧 MAIN FUNCTION FLOW (Lines 73-423)

┌─ INITIALIZATION (Lines 73-104) ────────────────────────────────────────────┐
│ 1. Enable raw mode (terminal takes input)                               │
│ 2. Set up CrosstermBackend for Terminal UI                             │
│ 3. Create human agent: TaskAgent::create(1500)                         │
│ 4. Create 7 AI agents: Vec of TaskAgent::create(1500)                  │
│ 5. Define 6 ProposalTypes with varying risks (15%-70%)                 │
│ 6. Initialize tracking:                                               │
│    • human_stats: AgentStats::new()                                   │
│    • voting_history: VecDeque<VotingRound>                            │
│    • power_history: Vec<u64>                                          │
│    • total_scarring: u64                                              │
└─────────────────────────────────────────────────────────────────────────────┘

┌─ MAIN LOOP: 30 ROUNDS (Lines 106-352) ─────────────────────────────────────┐
│                                                                           │
│ for round 0..30:                                                       │
│                                                                           │
│   ┌─ PROPOSAL PHASE ─────────────────────────────────────────────────┐  │
│   │ 1. Select random proposal from 6 types                          │  │
│   │ 2. Draw proposal display (title, description, risk level)       │  │
│   │ 3. Wait for user input (← LEFT / RIGHT → to navigate)           │  │
│   │                                                                 │  │
│   │ Display:                                                        │  │
│   │ Round 5 / 30 ━ Protocol: Increase block size                   │  │
│   │ Risk Level: 70% 🔴 (RED - DANGEROUS)                           │  │
│   │                                                                 │  │
│   │ Show 3 vote options with navigation:                           │  │
│   │ [FOR]  AGAINST  ABSTAIN  ← select with arrow keys              │  │
│   │ ^^^^                                                             │  │
│   │ currently highlighted (yellow background)                       │  │
│   │                                                                 │  │
│   │ Show all 8 agents' status:                                      │  │
│   │ YOU  | 💪 78% | 💔  278 | Active                               │  │
│   │ AI#0 | 💪 100%| 💔    0 | Active                               │  │
│   │ AI#1 | 💪 32% | 💔  988 | 🤐 SILENT                            │  │
│   │ ...                                                             │  │
│   └─────────────────────────────────────────────────────────────────┘  │
│                                                                           │
│   ┌─ INPUT HANDLING ─────────────────────────────────────────────────┐  │
│   │ while user hasn't pressed ENTER:                               │  │
│   │   if LEFT pressed:  selected_vote = max(0, selected_vote - 1) │  │
│   │   if RIGHT pressed: selected_vote = (selected_vote + 1) % 3   │  │
│   │   if ENTER pressed: proceed to voting phase                    │  │
│   │   if Q pressed:     exit completely                            │  │
│   │                                                                │  │
│   │ (Non-blocking with 100ms poll intervals)                       │  │
│   └─────────────────────────────────────────────────────────────────┘  │
│                                                                           │
│   ┌─ VOTING PHASE (Lines 210-290) ────────────────────────────────────┐  │
│   │                                                                 │  │
│   │ 1. Record your vote                                           │  │
│   │    human_stats.votes_cast += 1                                │  │
│   │    if selected_vote == 0: human_stats.votes_for += 1         │  │
│   │    else if selected_vote == 1: human_stats.votes_against += 1│  │
│   │    else: human_stats.votes_abstain += 1                       │  │
│   │                                                                 │  │
│   │ 2. YOUR VOTE COSTS ENERGY                                    │  │
│   │    task = Task::new("Cast consensus vote", 30 + rng(10..50)) │  │
│   │    outcome = match selected_vote {                            │  │
│   │        0 => TaskOutcome::Success,  // FOR is safe             │  │
│   │        1 => TaskOutcome::SevereFailure { ... },  // AGAINST   │  │
│   │        _ => TaskOutcome::RecoverableFailure { ... },  // ABSTAIN
│   │    }                                                           │  │
│   │    human.execute_task(task, outcome)  ← THIS APPLIES SCARS!   │  │
│   │                                                                 │  │
│   │ 3. AI AGENTS VOTE                                             │  │
│   │    for each of 7 AI agents:                                  │  │
│   │                                                                 │  │
│   │    a) Calculate conservative bias from their damage:          │  │
│   │       damage_ratio = agent.damage_score() / 1500              │  │
│   │       conservative_bias = damage_ratio * 0.8                  │  │
│   │       adjusted_risk = proposal.base_risk * (1.0 - bias)       │  │
│   │                                                                 │  │
│   │    b) If agent.capacity < 50: SILENT (vote abstain)          │  │
│   │       else: vote based on adjusted_risk                       │  │
│   │       roll = rng.gen_range(0.0..1.0)                          │  │
│   │       if roll < adjusted_risk: FOR                            │  │
│   │       else if roll < 0.6: AGAINST                             │  │
│   │       else: ABSTAIN                                            │  │
│   │                                                                 │  │
│   │    c) Execute their vote as a task (costs them energy too)   │  │
│   │       Dissenters on controversial votes get scarred!          │  │
│   │                                                                 │  │
│   │ 4. CALCULATE CONSENSUS                                       │  │
│   │    consensus % = for_votes / total_votes * 100                │  │
│   │    if consensus >= 66.67: ✓ CONSENSUS (safe)                 │  │
│   │    else if consensus >= 50: ✓ MAJORITY (controversial)       │  │
│   │    else: ✗ FAILED (major division)                            │  │
│   │                                                                 │  │
│   │ 5. APPLY SCARRING                                             │  │
│   │    On controversial votes (50-67%):                           │  │
│   │      dissenters get TaskOutcome::SevereFailure                │  │
│   │      causing scarring damage (damage_inflicted as u64)        │  │
│   │      Their capacity reduces, power drops                       │  │
│   │    Network learns: "This topic is divisive"                   │  │
│   │                                                                 │  │
│   │ 6. RECORD THE ROUND                                           │  │
│   │    voting_history.push_back(VotingRound {                    │  │
│   │        for_votes, against_votes, abstain_votes,              │  │
│   │        consensus_pct, your_vote, scarring                    │  │
│   │    })                                                           │  │
│   │                                                                 │  │
│   └─────────────────────────────────────────────────────────────────┘  │
│                                                                           │
│   ┌─ RESULT DISPLAY (Lines 302-340) ──────────────────────────────────┐  │
│   │ Full-screen result modal shows:                               │  │
│   │                                                                 │  │
│   │ ┌────────────────────────────────────────────────────────────┐  │  │
│   │ │            ✓ MAJORITY PASSED CONTROVERSIAL               │  │  │
│   │ │                                                             │  │  │
│   │ │  FOR: 5  |  AGAINST: 2  |  ABSTAIN: 1                    │  │  │
│   │ │  Consensus: 62.5%                                         │  │  │
│   │ │                                                             │  │  │
│   │ │  Network Scarring This Round: 127 💔                      │  │  │
│   │ │  Your Influence: 76.2%                                    │  │  │
│   │ │                                                             │  │  │
│   │ │  Press ENTER to continue...                               │  │  │
│   │ └────────────────────────────────────────────────────────────┘  │  │
│   │                                                                 │  │
│   │ Wait for ENTER key to proceed to next round                  │  │
│   │                                                                 │  │
│   └─────────────────────────────────────────────────────────────────┘  │
│                                                                           │
│ next round...                                                           │
│                                                                           │
└─────────────────────────────────────────────────────────────────────────────┘

┌─ FINAL REPORT (Lines 356-410) ─────────────────────────────────────────────┐
│                                                                           │
│ After all 30 rounds complete, display comprehensive report:             │
│                                                                           │
│ YOUR VOTING RECORD                                                      │
│ ├─ Votes Cast: 30                                                      │
│ ├─ ✓ For: 18  |  ✗ Against: 7  |  ◯ Abstain: 5                       │
│ ├─ Dissent Rate: 40%                                                   │
│ ├─ Total Scars: 187                                                    │
│ ├─ Voting Power Remaining: 87.5%                                       │
│ ├─ Network Scarring Total: 892 💔                                      │
│ └─ Status: "Your choices became part of permanent record.              │
│           This history will weigh forever in governance."              │
│                                                                           │
│ RECENT VOTING HISTORY (last 8 rounds):                                 │
│ ├─ Round 23: FOR / 83.3% consensus / 0 scarring                       │
│ ├─ Round 24: AGAINST / 25% consensus / 165 scarring 🔴                │
│ ├─ Round 25: ABSTAIN / 60% consensus / 50 scarring                    │
│ ... (8 rounds shown)                                                    │
│                                                                           │
│ NETWORK RESILIENCE:                                                    │
│ ├─ Agents still participating: 2 / 8                                   │
│ ├─ Agents silent: 6 / 8                                                │
│ └─ Governance effectiveness: 25%                                       │
│                                                                           │
│ Wait for any keypress to exit                                          │
│                                                                           │
└─────────────────────────────────────────────────────────────────────────────┘

═══════════════════════════════════════════════════════════════════════════════

⚙️ CORE MECHANICS IMPLEMENTATION

1. ENERGY COST OF VOTING (Lines 215-217)
   ─────────────────────────────────────
   let task = Task::new("Cast consensus vote".to_string(), 30 + rng.gen_range(10..50));
   let _ = human.execute_task(task.clone(), your_outcome.clone());
   
   → Each vote costs 30-80 energy from your capacity
   → Tasks execute with proper Lineage tracking

2. SCARRING FROM DISSENT (Lines 263-267)
   ──────────────────────────────────────
   let result = ai.execute_task(ai_task, ai_outcome.clone());
   if let TaskResult::Failed { damage_inflicted, .. } = result {
       against_votes += 1;
       if let TaskResult::Failed { damage_inflicted, .. } = result {
           round_scarring += damage_inflicted as u64;
   
   → Dissenters get SevereFailure outcome
   → Damage is extracted from TaskResult
   → Network scarring accumulates

3. CONSERVATIVE BIAS LEARNING (Lines 244-252)
   ──────────────────────────────────────────
   let damage_ratio = (ai.damage_score() as f32 / INITIAL_POWER as f32).min(1.0);
   let conservative_bias = damage_ratio * 0.8;
   let adjusted_risk = proposal.base_risk * (1.0 - conservative_bias);
   
   → More scars = higher conservative_bias
   → Higher bias = lower adjusted_risk
   → Lower risk = more likely to vote FOR (safe)
   → Scarring teaches caution mechanically

4. SILENCE MECHANICS (Lines 254-257)
   ─────────────────────────────────
   if ai.current_capacity() < 50 {
       abstain_votes += 1;
       continue;  // Don't process AI voting
   
   → Capacity < 50 triggers forced silence
   → Silent agents automatically abstain
   → They cannot influence voting anymore

5. CONSENSUS CALCULATION (Lines 277-282)
   ───────────────────────────────────
   let total_votes = for_votes + against_votes + abstain_votes;
   let consensus = if total_votes > 0 {
       (for_votes as f32 / total_votes as f32) * 100.0
   } else {
       50.0
   };
   
   → Calculate consensus percentage
   → All agents count (including silent ones as abstain)

═══════════════════════════════════════════════════════════════════════════════

🎨 TUI RENDERING

┌─ PROPOSAL DISPLAY (Lines 119-143) ─────────────────────────────────────────┐
│ terminal.draw(|f: &mut ratatui::Frame| {                                │
│     let size = f.size();                                               │
│     let chunks = Layout::default()                                    │
│         .direction(Direction::Vertical)                              │
│         .constraints([                                              │
│             Constraint::Length(8),    // Proposal area              │
│             Constraint::Length(12),   // Vote selector              │
│             Constraint::Min(5),       // Agent list                 │
│             Constraint::Length(3),    // Instructions               │
│         ])                                                           │
│         .split(size);                                              │
│                                                                     │
│     // Proposal block with title, description, risk                │
│     let proposal_text = vec![...];                                 │
│     let proposal_widget = Paragraph::new(proposal_text)           │
│         .block(Block::default()                                   │
│             .borders(Borders::ALL)                                │
│             .style(Style::default().fg(Color::White))           │
│         )                                                           │
│         .alignment(Alignment::Center);                            │
│     f.render_widget(proposal_widget, chunks[0]);                 │
│                                                                     │
│     // Vote selector with highlighting                             │
│     let vote_options = ["FOR", "AGAINST", "ABSTAIN"];             │
│     let mut option_text = vec![];                                 │
│     for (i, &vote_str) in vote_options.iter().enumerate() {      │
│         let style = if i == selected_vote {                       │
│             Style::default()                                      │
│                 .bg(Color::Yellow)                               │
│                 .fg(Color::Black)                                │
│                 .add_modifier(Modifier::BOLD)                   │
│         } else {                                                  │
│             Style::default().fg(Color::White)                    │
│         };                                                         │
│         option_text.push(Span::styled(format!("  [{}]  ", vote_str), style));
│     }                                                              │
│     // Render vote selector                                       │
│                                                                     │
│     // Agent status with power and scars                           │
│     let agent_lines: Vec<Line> = (0..NUM_AI_AGENTS + 1)          │
│         .map(|i| {                                               │
│             let agent = if i < NUM_AI_AGENTS { &ai_agents[i] } else { &human };
│             let power_pct = (agent.current_capacity() as f32 / INITIAL_POWER as f32 * 100.0).max(0.0);
│             let status = if agent.current_capacity() < 50 { "🤐 SILENT" } else { "  Active" };
│             Line::from(format!(                                   │
│                 "{} | 💪 {: >3.0}% | 💔 {:3} | {}",              │
│                 prefix, power_pct, damage, status                │
│             ))                                                     │
│         })                                                         │
│         .collect();                                              │
│                                                                     │
│ })?;  // terminal.draw() block                                    │
│                                                                     │
│ → Professional layout with 4 sections                             │
│ → Color-coded (cyan, yellow, white, gray)                         │
│ → Real-time updates every 100ms                                   │
│ → Box borders (╔ ║ ╚ ═) for structure                             │
│ → Emoji indicators (💪 💔 🤐) for status                           │
└─────────────────────────────────────────────────────────────────────────────┘

═══════════════════════════════════════════════════════════════════════════════

📊 DATA STRUCTURES FLOWCHART

Round 0:
  user_input ──→ selected_vote (0=FOR, 1=AGAINST, 2=ABSTAIN)
         ↓
  Execute task for human ──→ apply energy cost, possible scarring
         ↓
  AI voting calculation (for each of 7 agents):
    agent.damage_score() ──→ damage_ratio
                      ↓
                conservative_bias = damage_ratio * 0.8
                      ↓
                adjusted_risk = proposal.base_risk * (1.0 - bias)
                      ↓
              roll random number ──→ vote decision
                      ↓
              Execute task for AI ──→ apply energy cost, possible scarring
         ↓
  Tally votes ──→ calculate consensus %
         ↓
  Record in VotingRound struct ──→ add to voting_history
         ↓
  Display result ──→ show vote tally, consensus, scarring

Round 1-29: Repeat with evolved agent states

Round 30 → Final Report:
  human_stats ──→ display votes cast, dissent rate, scars
  voting_history ──→ show recent 8 rounds
  all agents ──→ show participation status

═══════════════════════════════════════════════════════════════════════════════

🔑 KEY IMPLEMENTATION DETAILS

Type Safety:
  ✓ All u64 for capacity/damage (proper Lineage types)
  ✓ Explicit Frame type for terminal closures: |f: &mut ratatui::Frame|
  ✓ Proper error handling with Result types
  ✓ Dead code attributes where intentional: #[allow(dead_code)]

Performance:
  ✓ Non-blocking input polling (100ms intervals)
  ✓ Efficient terminal redraws only when needed
  ✓ VecDeque for efficient recent history access
  ✓ Single-pass vote tallying

User Experience:
  ✓ Clear visual feedback (highlighted selection)
  ✓ Immediate result display after voting
  ✓ Progress tracking (round X / 30)
  ✓ Color-coded risk levels
  ✓ Status indicators (Active / 🤐 SILENT)
  ✓ Comprehensive final report

═══════════════════════════════════════════════════════════════════════════════

This is "more than powerful" because:

1. It's not just code—it's a Lineage philosophy demonstration
2. Every mechanic serves a purpose (teaching irreversible consequences)
3. The TUI is professional and responsive
4. AI behavior is sophisticated (conservative bias learning)
5. The 30-round arc creates an emergent narrative
6. Your choices genuinely matter and scar you forever
7. The final report proves your history is permanent
8. You experience why trustless systems need visible scars

$ cargo run --example interactive_consensus_arena

Vote. Scar. Learn. Reflect.
