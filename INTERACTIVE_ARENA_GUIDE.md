# INTERACTIVE CONSENSUS ARENA

**A real-time TUI voting system where YOU are a Lineage agent voting alongside AI agents. Your choices scar you permanently.**

---

## 🎮 How to Run

```bash
cargo run --example interactive_consensus_arena
```

The TUI will launch in full-screen mode. Press `Q` to quit at any time.

---

## 🎯 Game Flow

### 1. **PROPOSAL PHASE**
- A proposal appears with title, description, and **risk level** (color-coded)
- Risk levels: 
  - 🟢 Green (15%): Safe proposals (emergency patches)
  - 🟡 Yellow (45-55%): Medium risk (transactions, allocations)
  - 🔴 Red (65-70%): High risk (protocol changes, policy shifts)

### 2. **VOTING PHASE**
- **Navigate**: Use `← LEFT` and `RIGHT →` arrow keys
- **Select**: Highlight `FOR`, `AGAINST`, or `ABSTAIN`
- **Vote**: Press `ENTER` to cast your vote

Your vote options:
- **FOR**: Agree with proposal → Safe but conformist
- **AGAINST**: Dissent → Risky if you're wrong, scarring!
- **ABSTAIN**: Play it safe → No scarring but no power either

### 3. **VOTING RESULT PHASE**
Shows:
- **Vote tally** (FOR / AGAINST / ABSTAIN counts)
- **Consensus percentage** (what % voted FOR)
- **Outcome**: CONSENSUS (67%+) | MAJORITY (50-67%) | FAILED (<50%)
- **Network scarring** from this round
- **Your influence %** (power remaining)

The network scars dissenters on controversial votes!

### 4. **EVOLUTION PHASE**
After 30 voting rounds:
- **Final report** shows your complete voting record
- **Dissent rate**: How often you voted against proposals
- **Total scars**: Cumulative damage from controversy
- **Voting power**: Your remaining influence (starts at 100%)
- **Recent history**: Last 8 voting rounds

---

## 🔧 The Mechanics

### Your Voting Power
```
Power % = (current_capacity / 1500) * 100%
```

- Starts at **100%** with 1500 energy
- Decreases as you accumulate scars
- When power drops below ~3%, you become 🤐 **SILENT** (cannot vote)

### Conservative Bias
AI agents learn caution through scarring:
```
damage_ratio = damage_score / 1500
conservative_bias = damage_ratio * 0.8
adjusted_risk = proposal_risk * (1.0 - conservative_bias)
```

**Scarred agents vote MORE conservatively.** They've learned to avoid dissent!

### Vote Outcomes
- **Consensus** (67%+ FOR): Low cost, everyone wins
- **Majority** (50-67% FOR): **CONTROVERSIAL** - dissenters take scarring
- **Failed** (<50% FOR): **MAJOR DIVISION** - large scarring to minority

---

## 📊 Agent Status Display

```
YOU | 💪 85% | 💔  125 | Active
AI#0 | 💪 92% | 💔  78 | Active
AI#1 | 💪 50% | 💔  750 | 🤐 SILENT
```

Legend:
- **💪 85%**: Voting power remaining (energy capacity)
- **💔 125**: Total scars accumulated
- **Active / 🤐 SILENT**: Can you vote? (Silent if <50% power)

---

## 🧠 Strategic Insights

### You'll Learn:
1. **Dissent is costly** - Voting against wastes energy every time
2. **Majority changes minds** - What's controversial changes with each proposal
3. **Scarring is permanent** - You never fully heal, only more careful
4. **Power concentrates** - Consistent voters retain influence
5. **Silence spreads** - As more agents scar, the network becomes quieter

### The Network's Evolution:
- **Early rounds**: Lots of debate, divided votes
- **Mid rounds**: Cautious agents grow silent, consensus emerges
- **Late rounds**: Only the unscarred can vote, decisions become unanimous

**This is how Lineage governs: not through rules, but through irreversible consequences.**

---

## 🏆 Winning Strategies

### Conservative Play
- Vote **FOR** every proposal → Keep power → Maintain influence
- Problem: You become a puppet, no real power
- Result: Low scars, high power, but no meaningful voice

### Principled Play
- Vote your conscience → Accept the scars
- Problem: You'll scar faster than conservative players
- Result: Low influence by end game, but integrity

### Balanced Play
- Vote **AGAINST** only on risky proposals (70%+ risk)
- Vote **FOR** on safe proposals to conserve power
- Vote **ABSTAIN** when unsure
- Result: Moderate scars, moderate power, meaningful participation

### Adaptive Play
- Start conservative, learn proposal patterns
- As you scar, become more strategic about when to dissent
- Result: Peak influence mid-game, meaningful late-game voice

---

## 🔬 What's Actually Happening

### Under the Hood:

1. **Your vote is a Lineage task** - It costs energy (30-80 energy)
2. **Dissent is a failure** - When you vote AGAINST and it fails, you get TaskOutcome::SevereFailure
3. **Scars are real** - Each failed dissent inflicts damage_score += cost
4. **Power is capacity** - As damage_score grows, your current_capacity shrinks
5. **Silence is enforced** - Once capacity < 50, you physically cannot participate

### The Network's Learning:
```
round 1: Lots of voting, diverse opinions
round 5: Some agents scarred, becoming cautious
round 10: Most agents silent or following consensus
round 15: Only strongest voices heard
round 30: Pure consensus (or complete paralysis)
```

---

## 🎨 TUI Features

- **Live agent status**: See all 8 agents' power and scars in real time
- **Color-coded proposals**: Risk level at a glance
- **Vote confirmation**: Never accidentally press ENTER
- **Beautiful borders**: Box-drawing characters for professional feel
- **Emoji indicators**: 💪 power, 💔 scars, 🤐 silence, ✓ success, ✗ failure
- **Responsive layout**: Adapts to terminal size

---

## 📈 The Data

After 30 rounds, your **Final Report** includes:

- **Votes cast**: Total participation count
- **Vote breakdown**: FOR / AGAINST / ABSTAIN split
- **Dissent rate**: % of votes that weren't FOR
- **Total scars**: Permanent damage accumulated
- **Power remaining**: Your current influence
- **Network scarring**: Total damage inflicted across all rounds
- **Recent history**: Last 8 rounds for pattern analysis

---

## 🔐 Lineage Philosophy in Action

This example demonstrates:

✅ **Irreversible Consequences** - Your votes scar you forever
✅ **Earned Trust** - Consistency builds influence over time
✅ **Transparent Reasoning** - See exactly why agents became silent
✅ **Permanent Records** - "This history is permanently recorded in the blockchain"
✅ **Natural Learning** - System organically learns caution from disagreement

**Every vote you cast becomes part of the immutable ledger.**
**History weighs forever in governance.**

---

## 🚀 Next Steps

1. **Run the arena**: `cargo run --example interactive_consensus_arena`
2. **Experiment**: Try different voting strategies (conservative vs principled)
3. **Watch the network evolve**: See how scarring shapes consensus
4. **Understand governance**: Why permanent records matter for trustless systems
5. **Extend it**: Add persistence, leader boards, or blockchain-style validation

---

**Welcome to distributed governance where consequences are final.**
**Your scars are your credibility. Choose wisely.**
