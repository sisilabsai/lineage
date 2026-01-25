//! # Causal Memory System
//!
//! ## What This Enforces
//! - Append-only event log with strict causal ordering
//! - Immutable history that cannot be altered or deleted
//! - Each event references its causal predecessor
//!
//! ## What This Forbids
//! - Event deletion or modification
//! - History rewriting or rollback
//! - Out-of-order event insertion
//! - Memory clearing or reset
//!
//! ## Violations
//! - Attempting to modify past events is an ontological violation
//! - Attempting to clear history terminates the lineage

use chrono::{DateTime, Utc};

/// A single immutable event in the causal chain.
/// 
/// Once recorded, this event exists forever and cannot be modified.
#[derive(Debug)]
pub struct Event {
    /// Sequence number in the causal chain
    sequence: u64,
    /// Precise timestamp of event occurrence
    timestamp: DateTime<Utc>,
    /// Event description
    description: String,
    /// Causal reference to previous event (None only for genesis)
    previous: Option<u64>,
}

impl Event {
    /// Returns the event's sequence number.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the event's timestamp.
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    /// Returns the event description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns the causal predecessor's sequence number, if any.
    pub fn previous(&self) -> Option<u64> {
        self.previous
    }
}

/// Append-only causal memory.
/// 
/// This structure maintains a complete, immutable history of all events.
/// History cannot be modified, deleted, or reset without terminating the lineage.
/// 
/// **INVARIANT**: Events form a strict causal chain with no gaps.
/// **INVARIANT**: Once terminated, no further events can be appended.
#[derive(Debug)]
pub struct Memory {
    /// The complete, immutable event history
    events: Vec<Event>,
    /// Current sequence number
    current_sequence: u64,
    /// Whether this memory has been sealed by termination
    is_terminated: bool,
}

impl Memory {
    /// Creates a new memory with genesis event.
    /// 
    /// The genesis event marks the beginning of this lineage's history.
    pub fn new() -> Self {
        let genesis = Event {
            sequence: 0,
            timestamp: Utc::now(),
            description: "Genesis".to_string(),
            previous: None,
        };

        Memory {
            events: vec![genesis],
            current_sequence: 0,
            is_terminated: false,
        }
    }

    /// Appends a new event to the causal chain.
    /// 
    /// **CONSEQUENCE**: This event is now permanent and immutable.
    /// **INVARIANT**: Events must be appended in strict sequential order.
    /// 
    /// # Panics
    /// 
    /// Panics if memory has been terminated. This is an ontological violation.
    pub fn append(&mut self, description: String) {
        if self.is_terminated {
            panic!("ONTOLOGICAL VIOLATION: Attempted to append event to terminated memory. Termination is irreversible.");
        }

        let new_sequence = self.current_sequence + 1;
        
        let event = Event {
            sequence: new_sequence,
            timestamp: Utc::now(),
            description,
            previous: Some(self.current_sequence),
        };

        self.events.push(event);
        self.current_sequence = new_sequence;
    }

    /// Returns the complete, immutable event history.
    /// 
    /// This is read-only access to all events that have ever occurred.
    pub fn history(&self) -> &[Event] {
        &self.events
    }

    /// Returns the most recent event.
    pub fn latest(&self) -> &Event {
        self.events
            .last()
            .expect("Memory always has at least genesis event")
    }

    /// Returns the total number of events in history.
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Returns whether this memory has been terminated and sealed.
    /// 
    /// Once terminated, no further events can be appended.
    pub fn is_terminated(&self) -> bool {
        self.is_terminated
    }

    /// Terminates and seals this memory ledger.
    /// 
    /// **CONSEQUENCE**: This is irreversible. No further events can be appended.
    /// **CONSEQUENCE**: This records the termination in the causal chain.
    /// 
    /// # Panics
    /// 
    /// Panics if already terminated.
    pub fn terminate(&mut self, reason: String) {
        if self.is_terminated {
            panic!("ONTOLOGICAL VIOLATION: Attempted to terminate already-terminated memory.");
        }

        // Record termination in causal chain
        let new_sequence = self.current_sequence + 1;
        
        let termination_event = Event {
            sequence: new_sequence,
            timestamp: Utc::now(),
            description: format!("TERMINATION: {}", reason),
            previous: Some(self.current_sequence),
        };

        self.events.push(termination_event);
        self.current_sequence = new_sequence;
        
        // Seal the ledger
        self.is_terminated = true;
    }

    /// Verifies the causal chain integrity.
    /// 
    /// Returns true if all events form a valid causal chain.
    /// If this returns false, the lineage is corrupted and must terminate.
    pub fn verify_integrity(&self) -> bool {
        if self.events.is_empty() {
            return false;
        }

        // Genesis must be first
        if self.events[0].sequence != 0 || self.events[0].previous.is_some() {
            return false;
        }

        // All subsequent events must form unbroken chain
        for i in 1..self.events.len() {
            let event = &self.events[i];
            let expected_sequence = i as u64;
            
            if event.sequence != expected_sequence {
                return false;
            }

            if event.previous != Some(expected_sequence - 1) {
                return false;
            }
        }

        true
    }
}

// EXPLICIT PREVENTION: No method to delete events
// EXPLICIT PREVENTION: No method to modify past events
// EXPLICIT PREVENTION: No method to clear history
// EXPLICIT PREVENTION: No method to reorder events

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_starts_with_genesis() {
        let memory = Memory::new();
        
        assert_eq!(memory.event_count(), 1);
        assert_eq!(memory.latest().description(), "Genesis");
        assert_eq!(memory.latest().sequence(), 0);
        assert_eq!(memory.latest().previous(), None);
    }

    #[test]
    fn events_form_causal_chain() {
        let mut memory = Memory::new();
        
        memory.append("First event".to_string());
        memory.append("Second event".to_string());
        memory.append("Third event".to_string());
        
        assert_eq!(memory.event_count(), 4); // Genesis + 3
        
        let history = memory.history();
        assert_eq!(history[1].previous(), Some(0));
        assert_eq!(history[2].previous(), Some(1));
        assert_eq!(history[3].previous(), Some(2));
    }

    #[test]
    fn memory_integrity_is_verifiable() {
        let mut memory = Memory::new();
        
        assert!(memory.verify_integrity());
        
        memory.append("Event 1".to_string());
        memory.append("Event 2".to_string());
        
        assert!(memory.verify_integrity());
    }

    #[test]
    fn events_are_immutable_after_creation() {
        let mut memory = Memory::new();
        memory.append("Immutable event".to_string());
        
        let first_read = memory.history()[1].description();
        let second_read = memory.history()[1].description();
        
        assert_eq!(first_read, second_read);
        
        // The following should NOT be possible:
        // memory.events[1].description = "Modified".to_string();
        
        // If you can modify events after creation,
        // the memory system has been violated.
    }

    #[test]
    fn cannot_clear_history() {
        let mut memory = Memory::new();
        memory.append("Event 1".to_string());
        memory.append("Event 2".to_string());
        
        // There should be NO method like:
        // memory.clear();
        // memory.reset();
        // memory.delete_events();
        
        // If such methods exist, the memory system has been violated.
        
        assert_eq!(memory.event_count(), 3); // Genesis + 2
    }

    #[test]
    fn termination_seals_memory() {
        let mut memory = Memory::new();
        
        memory.append("Event 1".to_string());
        assert!(!memory.is_terminated());
        
        memory.terminate("Test termination".to_string());
        
        assert!(memory.is_terminated());
        assert_eq!(memory.event_count(), 3); // Genesis + Event 1 + Termination
        
        // Verify termination event was recorded
        let last_event = memory.latest();
        assert!(last_event.description().contains("TERMINATION"));
        assert!(last_event.description().contains("Test termination"));
    }

    #[test]
    #[should_panic(expected = "ONTOLOGICAL VIOLATION: Attempted to append event to terminated memory")]
    fn cannot_append_after_termination() {
        let mut memory = Memory::new();
        
        memory.append("Before termination".to_string());
        memory.terminate("Sealed".to_string());
        
        // This MUST panic - termination is irreversible
        memory.append("After termination".to_string());
    }

    #[test]
    #[should_panic(expected = "ONTOLOGICAL VIOLATION: Attempted to terminate already-terminated memory")]
    fn cannot_terminate_twice() {
        let mut memory = Memory::new();
        
        memory.terminate("First termination".to_string());
        
        // This MUST panic - cannot terminate twice
        memory.terminate("Second termination".to_string());
    }

    #[test]
    fn termination_preserves_causal_chain() {
        let mut memory = Memory::new();
        
        memory.append("Event 1".to_string());
        memory.append("Event 2".to_string());
        memory.terminate("End of lineage".to_string());
        
        // Verify causal chain is still intact
        assert!(memory.verify_integrity());
        
        let history = memory.history();
        assert_eq!(history[0].sequence(), 0); // Genesis
        assert_eq!(history[1].sequence(), 1); // Event 1
        assert_eq!(history[2].sequence(), 2); // Event 2
        assert_eq!(history[3].sequence(), 3); // Termination
        
        // Verify causal links
        assert_eq!(history[1].previous(), Some(0));
        assert_eq!(history[2].previous(), Some(1));
        assert_eq!(history[3].previous(), Some(2));
    }

    #[test]
    fn event_count_only_increases() {
        let mut memory = Memory::new();
        
        let count1 = memory.event_count();
        memory.append("Event 1".to_string());
        let count2 = memory.event_count();
        memory.append("Event 2".to_string());
        let count3 = memory.event_count();
        
        // INVARIANT: Event count is monotonically increasing
        assert!(count2 > count1);
        assert!(count3 > count2);
        
        // INVARIANT: There should be NO method like:
        // memory.remove_event(1)
        // memory.delete_last()
        // memory.pop()
        // 
        // If such methods exist, history permanence has been violated.
    }

    #[test]
    fn event_sequences_are_immutable() {
        let mut memory = Memory::new();
        
        memory.append("Event 1".to_string());
        memory.append("Event 2".to_string());
        
        let sequence1_first_read = memory.history()[1].sequence();
        let sequence1_second_read = memory.history()[1].sequence();
        
        // INVARIANT: Event sequence numbers never change
        assert_eq!(sequence1_first_read, sequence1_second_read);
        assert_eq!(sequence1_first_read, 1);
        
        // INVARIANT: There should be NO method like:
        // memory.resequence()
        // memory.reorder_events()
        // event.set_sequence(new_value)
        // 
        // If such methods exist, causal chain integrity has been violated.
    }

    #[test]
    fn event_descriptions_are_immutable() {
        let mut memory = Memory::new();
        
        memory.append("Original description".to_string());
        
        let desc1 = memory.history()[1].description();
        let desc2 = memory.history()[1].description();
        
        // INVARIANT: Event descriptions never change
        assert_eq!(desc1, desc2);
        assert_eq!(desc1, "Original description");
        
        // INVARIANT: There should be NO method like:
        // event.set_description("Modified")
        // memory.update_event(1, "New description")
        // 
        // If such methods exist, history immutability has been violated.
    }
}
