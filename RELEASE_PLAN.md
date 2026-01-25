# Release Plan: Lineage

**Status**: Technically complete, philosophically disruptive  
**Date**: 2026-01-22  
**Approach**: Quiet, filtration-oriented, anti-promotional

---

## What to Release

### 1. Source Code

**Format**: Single Git repository, no hosting platform promotion

**Contents**:
- Complete implementation (7 modules, ~1200 lines)
- Full test suite (177 tests, all passing)
- Cargo.toml with minimal dependencies
- No CI/CD configuration
- No issue templates
- No pull request templates

**What to omit**:
- Quickstart guide
- Example applications
- Docker images
- Package manager distribution
- Star/fork encouragement

**README content**:
- System definition (what it is)
- What it is NOT (prominently)
- Build and test commands only
- Links to formal documentation
- "No support, no roadmap" statement

---

### 2. Technical Paper

**Format**: Markdown or PDF, academic tone

**File**: PAPER.md (already complete)

**Contents**:
- Problem definition (resettable systems erase consequence)
- Six primitives with formal definitions
- Enforcement mechanisms (type system + runtime)
- Related work comparison
- Implementation statistics
- Limitations section (prominently)

**What to omit**:
- Future work section
- Call for contributors
- "Join us" language
- Vision statements

---

### 3. Documentation Set

**Files to include**:
- README.md (system documentation, normative language)
- FREEZE.md (ontological freeze declaration)
- CONTRIBUTING.md (ontological review before code review)
- DOCTRINE.md (seven consolidated principles)
- PAPER.md (technical paper)
- TRUST_SYSTEM.md (trust degradation system)
- AGENT_SYSTEM.md (autonomous agent system)
- PRESSURE_LOG_TRUST.md (trust pressure log)
- PRESSURE_LOG_AGENT.md (agent pressure log)
- FEEDBACK.md (feedback classification guide)
- EXTENSION_PROTOCOL.md (three-question evaluation)
- ANNOUNCEMENT.md (quiet release announcement)

**What to omit**:
- Getting started guides
- Tutorial content
- Video walkthroughs
- Community guidelines (beyond CONTRIBUTING.md)
- FAQ (forces pre-emptive answers)

---

### 4. Supplementary Artifacts

**Optional**:
- BOOK_OUTLINE.md (evidence of larger thinking, not a promise)
- Test output logs (proof of claims)
- Benchmark disclaimers (we did not optimize)

**What to omit**:
- Roadmap
- Changelog (version 1.0.0, frozen ontology)
- Migration guides (nothing to migrate from)
- Comparison matrices (not competing)

---

## Where to Release

### Primary Format: Static Archive

**Recommended**:
- Git repository with clear LICENSE file
- Tarball/ZIP download option
- Self-hosted or minimal platform (no social features)

**Why**: Control, permanence, no engagement metrics

---

### Secondary Format: Technical Paper Distribution

**Recommended**:
- arXiv preprint (computer science, software engineering)
- Personal website/blog (single post, no comments)
- Direct sharing with researchers (no broadcast)

**Why**: Reaches technical audience, avoids social amplification

---

### Tertiary Format: Discussion Forum Post

**If required**:
- HN, Lobsters, or similar (single announcement, walk away)
- Title: "Lineage: Irreversible state, finite resources, permanent consequences"
- Body: Link to repository, link to paper, one-sentence summary, no call to action

**Why**: Acceptable for discovery, but post-and-exit approach

---

## What NOT to Say Publicly

### 1. Feature Language

**Avoid**:
- "Introducing Lineage..."
- "Now you can..."
- "Built for developers who need..."
- Feature bullet lists with checkmarks

**Why**: Frames as product seeking adoption

---

### 2. Call to Action

**Avoid**:
- "Try it out"
- "Star the repo if you find this interesting"
- "Join the discussion"
- "Contributions welcome"

**Why**: Invites shallow engagement

---

### 3. Vision or Future

**Avoid**:
- "We believe..."
- "The future of software..."
- "Imagine a world where..."
- "Coming soon: ..."

**Why**: Promises progression, implies momentum campaign

---

### 4. Community Building

**Avoid**:
- "Growing community"
- "Contributors wanted"
- "We're building together"
- "Join our Discord/Slack"

**Why**: Creates expectation of ongoing social engagement

---

### 5. Comparison Framing

**Avoid**:
- "Unlike X, Lineage..."
- "Better than Y because..."
- Feature comparison tables
- "Why we're different"

**Why**: Positions as competitor in market

---

### 6. Sympathy Appeals

**Avoid**:
- "We understand your pain"
- "Frustrated with X? Try..."
- "Finally, a solution that..."
- "Tired of Y?"

**Why**: Marketing language, implies sales pitch

---

## What TO Say (Minimally)

### Release Announcement Text (Template)

```
Lineage: Irreversible State, Finite Resources, Permanent Consequences

Software systems typically permit reset, rollback, and recovery. 
Lineage removes these capabilities.

A reference implementation in Rust demonstrating that:
- Identity cannot be cloned
- History cannot be erased
- Resources cannot be replenished
- Errors accumulate permanently
- Death is irreversible

Code: [link]
Paper: [link]
Documentation: [link]

This is not a framework. This is not a solution. This is an existence proof.

177 tests pass. The ontology is frozen. No roadmap exists.
```

**Length**: ~100 words  
**Tone**: Declarative, uninviting  
**Purpose**: Filter for serious readers

---

## How to Respond to Misunderstanding

### Pattern 1: "How do I use this in my project?"

**Response**:
```
Lineage is a reference implementation, not a library. 
See PAPER.md for architectural principles.
```

**Why**: Deflects usage questions, redirects to theory

---

### Pattern 2: "Will you add [feature]?"

**Response**:
```
The ontology is frozen. See FREEZE.md.
Extensions must be compositional. See EXTENSION_PROTOCOL.md.
```

**Why**: Cites documentation, closes conversation

---

### Pattern 3: "This is too harsh for real users"

**Response**:
```
See ANNOUNCEMENT.md, "Suitability" section.
```

**Why**: Acknowledges domain mismatch without defending

---

### Pattern 4: "Can you explain [concept] more?"

**Response**:
```
See PAPER.md, Section [X].
```

**Why**: Documentation-first, no personal explanation duty

---

### Pattern 5: "Why no [conventional feature]?"

**Response**:
```
See PRESSURE_LOG_[DOMAIN].md for development constraints.
See DOCTRINE.md for principles.
```

**Why**: Points to evidence, not opinion

---

### Pattern 6: "This is just [existing thing]"

**Response**:
```
See PAPER.md, Section 7 (Related Work) for distinctions.
```

**Why**: Comparison already documented, no debate needed

---

### Pattern 7: "I found a bug / issue"

**Response** (if ontological violation):
```
File issue with: reproduction, expected vs. actual behavior, 
which invariant was violated.
```

**Response** (if feature request):
```
See FREEZE.md. The ontology is frozen.
```

**Response** (if support request):
```
No support is provided. See README.md.
```

**Why**: Acknowledge real bugs, deflect everything else

---

## Response Frequency

**Recommended**:
- Initial release: Respond to first ~5-10 substantive questions with documentation links
- After 48 hours: Stop responding to general questions
- Ongoing: Only respond to ontological violation reports

**Why**: Limited engagement signals seriousness, filters casual interest

---

## What Success Looks Like

**Not**:
- High star count
- Many contributors
- Active community

**Yes**:
- Researchers cite the paper
- Engineers reference the principles in their own designs
- A few serious forks preserve the ontology
- Quiet understanding among small group

**Metric**: Quality of engagement, not quantity

---

## What Failure Looks Like

**Signs**:
- "Awesome lists" inclusion
- Framework comparison blog posts
- Tutorial YouTube videos
- "Getting started with Lineage" articles

**Response**: None. Let misunderstanding exist. Correct only if directly asked.

---

## Long-Term Maintenance

**Plan**:
- Version 1.0.0 remains indefinitely
- Bug fixes that preserve invariants: 1.0.X
- Compositional extensions (if any): 1.X.0
- Ontological changes: Not versioned, new system with new name

**What NOT to do**:
- Respond to feature pressure
- Accept contributions weakening constraints
- Create "ecosystem"
- Maintain "momentum"

---

## Summary

**Release**:
- Code + Paper + Docs
- No quickstart, no tutorials, no promotion

**Announce**:
- Single factual post
- ~100 words
- Links only

**Respond**:
- Documentation links for 48 hours
- Then silence except for bugs

**Measure**:
- Ignore metrics
- Note serious citations/references
- Accept low adoption as correct filtration

**Maintain**:
- Bug fixes only
- Reject weakening contributions
- No roadmap ever

---

## Release Checklist

- [ ] All 177 tests passing
- [ ] Documentation complete and accurate
- [ ] LICENSE file present
- [ ] ANNOUNCEMENT.md reflects reality
- [ ] Repository has no issue templates
- [ ] Repository has no "good first issue" labels
- [ ] README.md contains "No support" statement
- [ ] FREEZE.md is final and signed
- [ ] Version is 1.0.0 (ONTOLOGY.CAPABILITY.FIX)
- [ ] No TODO comments in code
- [ ] No promises in any documentation

---

## Signature

This release plan prioritizes filtration over adoption.

If few people understand, that is acceptable.  
If no one uses it, that is acceptable.  
If it is cited in academic work, that is sufficient.

The system exists. The evidence is public. The constraints are documented.

What happens next is not our concern.
