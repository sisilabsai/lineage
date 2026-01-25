# Pressure Log: Trust Degradation System

**Domain**: Credential and trust management for privileged actors  
**Date**: 2026-01-22  
**Status**: Doctrine extraction from completed system

---

## A. Where We Wanted to Cheat

### 1. "Let's add good behavior credits"

**Temptation**: After N successful operations, restore one revoked capability.

**Why it felt necessary**: Actors who make one mistake early shouldn't be permanently crippled. They should be able to "work their way back" to full trust through demonstrated reliability.

**Why it felt reasonable**: Real organizations do this. Probation periods exist. People rehabilitate.

**Reality**: This would make violations transactional. "I can commit 3 unauthorized accesses if I do 30 clean operations." Trust becomes a balance sheet, not a consequence record.

### 2. "Let's add administrator override"

**Temptation**: Add `force_restore_capability(actor_id, capability, reason)` for emergency situations.

**Why it felt necessary**: What if revocation happens due to system error? What if we need an operator to have access RIGHT NOW for an incident?

**Why it felt reasonable**: Every access control system has break-glass procedures. Inflexibility is operational suicide.

**Reality**: Override mechanisms communicate that rules are suggestions. If trust can be manually restored, the entire enforcement model becomes theater. We'd be building a system we don't actually trust.

### 3. "Let's add capability suspension instead of revocation"

**Temptation**: `suspend_capability(duration)` that temporarily disables but can be automatically restored.

**Why it felt necessary**: Permanent revocation feels too harsh for minor violations. Let actors "sit in timeout" and return.

**Why it felt reasonable**: Graduated response. Temporary measures for temporary problems.

**Reality**: Suspension is just delayed forgiveness. It treats violations as temporary states rather than permanent facts. If the capability returns, the violation didn't actually matter.

### 4. "Let's add violation expiration"

**Temptation**: Scars older than 90 days don't count toward damage score.

**Why it felt necessary**: Past violations shouldn't haunt actors forever. People change. Systems should reflect growth.

**Why it felt reasonable**: Legal systems have statutes of limitations. Credit reports drop old marks. Time-based forgiveness is civilized.

**Reality**: Expiration erases causal history. The violation happened. Its consequences are real. Making it "not count" is rewriting the past, not acknowledging change.

### 5. "Let's add re-certification"

**Temptation**: Actors can submit to audit and regain capabilities if they pass.

**Why it felt necessary**: Provides hope. Gives actors a path forward. Shows the system isn't vindictive.

**Why it felt reasonable**: Driver's licenses can be reinstated. Professional credentials can be regained. Redemption is possible.

**Reality**: Re-certification suggests identity can be reset. But the actor's identity is tied to their Lineage. You can't re-certify without creating a new identity, which is just identity laundering.

### 6. "Let's add warning mode before revocation"

**Temptation**: First violation triggers warning. Second violation revokes capability.

**Why it felt necessary**: Actors deserve a chance to correct before permanent consequences.

**Why it felt reasonable**: Human systems use warnings. "First offense" leniency is standard practice.

**Reality**: If the first violation doesn't matter, it's not actually a violation. Warning mode communicates that rules don't apply until the second time. Either the action is a violation or it isn't.

---

## B. What Lineage Forced Instead

### 1. Capabilities became a shrinking-only set

We couldn't add "good behavior credits" because TrustProfile has no `add_capability()` method. The only operation is `revoke_capabilities()`.

**Decision that disappeared**: Numeric trust scores. There's no score that goes up and down. There's only a set that shrinks.

**Upstream change**: Instead of designing a "trust recovery" mechanism, we designed violation severity to match capability importance. Violations are classified by which capabilities they should revoke, not by how many points to subtract.

### 2. No configuration options exist

We couldn't add administrator override because there's no configuration layer. The system has no concept of "admin mode" or "emergency access."

**Decision that disappeared**: Role-based bypass. We can't have "admins who can restore trust" because there's no mechanism to inject that concept.

**Upstream change**: Trust operations are deterministic. `record_violation(type)` always revokes the same capabilities. No human-in-the-loop, no contextual exceptions.

### 3. Violations must inflict scars immediately

We couldn't add suspension or warnings because `record_violation()` calls `lineage.record_error()`, which inflicts permanent scars.

**Decision that disappeared**: Graduated responses. There's no "warn, then suspend, then revoke" pipeline. Violation → scar → revocation happens atomically.

**Upstream change**: Violation types are binary. Either it's a violation (inflicts scar) or it isn't (no record). We had to decide up front what constitutes an actual violation, not what constitutes a "warning-worthy action."

### 4. History is permanent and visible

We couldn't add expiration because Memory is append-only. Events stay in the causal chain forever.

**Decision that disappeared**: Violation forgetting. We can't make old violations "not count" because they're permanently in the record.

**Upstream change**: Damage score calculation is cumulative forever. We designed severity levels knowing they accumulate unbounded. Fatal violations (100 damage) must be truly fatal because there's no reset.

### 5. Identity cannot be recreated

We couldn't add re-certification because each TrustedActor has exactly one Lineage identity, which cannot be cloned or reset.

**Decision that disappeared**: Identity refresh. We can't issue a "clean identity" to a rehabilitated actor.

**Upstream change**: Trust is permanently tied to identity. If an actor loses all capabilities, that identity is operationally dead even if biologically alive. We had to accept that some actors will become non-functional.

### 6. No retry or recovery mechanisms

We couldn't add "try again" logic because failed operations consume energy and inflict scars.

**Decision that disappeared**: Optimistic trust granting. We can't "give them capability to try, then revoke if they fail." Granting a capability is permanent until violation.

**Upstream change**: Capability checks happen BEFORE operations, not after. We enforce preconditions, not react to outcomes.

---

## C. What This Reveals About Software Culture

### 1. We assume forgiveness is a technical feature, not a policy choice

**Conventional trust systems**: Include time-based expiration, score recovery, re-certification by default.

**What Lineage exposes**: These aren't technical necessities. They're policy decisions that hide behind "user-friendly" design. We build forgiveness into systems because questioning it is uncomfortable.

**Dishonest practice**: Calling override mechanisms "emergency access" when they're really "we don't trust our own rules."

### 2. We treat rehabilitation as compulsory

**Conventional trust systems**: Assume violators must have a path to redemption. Systems without recovery are called "harsh" or "unforgiving."

**What Lineage exposes**: Rehabilitation is a social policy, not an engineering requirement. Software can enforce permanent consequence if we choose.

**Dishonest practice**: Framing irreversibility as a failure of empathy rather than an enforcement choice.

### 3. We confuse "second chances" with consequence avoidance

**Conventional trust systems**: Allow violations to not count if followed by good behavior.

**What Lineage exposes**: If a violation can be erased by later compliance, it wasn't a violation—it was a suggestion. Real consequences don't depend on what happens next.

**Dishonest practice**: Marketing temporary restrictions as "accountability" when they're really just waiting periods.

### 4. We build override mechanisms because we don't believe our own rules

**Conventional trust systems**: Include admin bypass, manual restore, policy exceptions.

**What Lineage exposes**: If a rule needs exceptions, it's poorly designed. If a system requires override, its constraints are performative.

**Dishonest practice**: Implementing "strict" policies with "emergency" backdoors, then acting surprised when backdoors become standard practice.

### 5. We use "flexibility" as justification for consequence erasure

**Conventional trust systems**: Describe forgiveness mechanisms as "pragmatic" or "realistic."

**What Lineage exposes**: Flexibility is often just avoiding accountability infrastructure. Irreversible systems aren't inflexible—they're honest about what counts.

**Dishonest practice**: Calling consequence evasion "practical engineering" instead of acknowledging we're avoiding hard choices.

### 6. We treat identity as mutable when convenient

**Conventional trust systems**: Allow trust to be "reset" while preserving the same identity/username.

**What Lineage exposes**: If trust can be reset, identity is cosmetic. Real identity is inseparable from history.

**Dishonest practice**: Maintaining the illusion of identity continuity while erasing consequence history. The actor is "the same person" when beneficial, but has a "clean slate" when violations matter.

---

## D. One Sentence of Doctrine

**Trust systems with forgiveness mechanisms exist to protect institutions from accountability, not to rehabilitate violators.**

---

## Signature

This pressure log documents ontological friction during development of the Trust Degradation System (2026-01-22).

The temptations listed were real. The constraints were absolute. The dishonest practices are pervasive.

No appeals.
