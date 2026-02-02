#![allow(dead_code)]
/// ANSI color codes for beautiful terminal output

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";

// Bright Colors
pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_GREEN: &str = "\x1b[92m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const BRIGHT_BLUE: &str = "\x1b[94m";
pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
pub const BRIGHT_CYAN: &str = "\x1b[96m";
pub const BRIGHT_WHITE: &str = "\x1b[97m";

// Standard Colors
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

// Background Colors
pub const BG_RED: &str = "\x1b[41m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BG_BLUE: &str = "\x1b[44m";

pub struct Color;

impl Color {
    /// Format text with color
    pub fn text(text: &str, color: &str) -> String {
        format!("{}{}{}", color, text, RESET)
    }

    /// Format header with gradient effect
    pub fn header(text: &str) -> String {
        format!("{}{}{}", BOLD, text, RESET)
    }

    /// Format success message
    pub fn success(text: &str) -> String {
        format!("{}{}{}", BRIGHT_GREEN, text, RESET)
    }

    /// Format error message
    pub fn error(text: &str) -> String {
        format!("{}{}{}", BRIGHT_RED, text, RESET)
    }

    /// Format info message
    pub fn info(text: &str) -> String {
        format!("{}{}{}", BRIGHT_CYAN, text, RESET)
    }

    /// Format warning message
    pub fn warning(text: &str) -> String {
        format!("{}{}{}", BRIGHT_YELLOW, text, RESET)
    }

    /// Format highlight
    pub fn highlight(text: &str) -> String {
        format!("{}{}{}", BRIGHT_MAGENTA, text, RESET)
    }

    /// Format agent 1 (balanced) - green
    pub fn agent_1(text: &str) -> String {
        format!("{}{}{}", BRIGHT_GREEN, text, RESET)
    }

    /// Format agent 2 (trend) - cyan
    pub fn agent_2(text: &str) -> String {
        format!("{}{}{}", BRIGHT_CYAN, text, RESET)
    }

    /// Format agent 3 (momentum) - yellow
    pub fn agent_3(text: &str) -> String {
        format!("{}{}{}", BRIGHT_YELLOW, text, RESET)
    }

    /// Format agent 4 (volatility) - red
    pub fn agent_4(text: &str) -> String {
        format!("{}{}{}", BRIGHT_RED, text, RESET)
    }

    /// Format agent 5 (conservative) - white
    pub fn agent_5(text: &str) -> String {
        format!("{}{}{}", WHITE, text, RESET)
    }

    /// Color by rank
    pub fn by_rank(text: &str, rank: usize) -> String {
        match rank {
            1 => Self::agent_1(text),
            2 => Self::agent_2(text),
            3 => Self::agent_3(text),
            4 => Self::agent_4(text),
            _ => Self::agent_5(text),
        }
    }
}
