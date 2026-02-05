use std::io;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use lineage::{
    GovernanceConfig, GovernanceCouncil, ProposalId, ProposalOutcome, ProposalRisk, VoteChoice,
};

struct ProposalState {
    id: ProposalId,
    title: String,
    risk: ProposalRisk,
    round: u32,
    last_outcome: Option<ProposalOutcome>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut council = GovernanceCouncil::new(GovernanceConfig::default());
    let user_id = council.add_member("You".to_string(), 1200);
    let _ai1 = council.add_member("AI-1".to_string(), 1200);
    let _ai2 = council.add_member("AI-2".to_string(), 1200);
    let _ai3 = council.add_member("AI-3".to_string(), 1200);
    let _ai4 = council.add_member("AI-4".to_string(), 1200);

    let mut state = new_proposal(&mut council, 1);

    loop {
        terminal.draw(|frame| draw_ui(frame, &council, &state, &user_id))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('f') => {
                        state = handle_vote(&mut council, &state, &user_id, VoteChoice::For);
                    }
                    KeyCode::Char('a') => {
                        state = handle_vote(&mut council, &state, &user_id, VoteChoice::Against);
                    }
                    KeyCode::Char('s') => {
                        state = handle_vote(&mut council, &state, &user_id, VoteChoice::Abstain);
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn handle_vote(
    council: &mut GovernanceCouncil,
    state: &ProposalState,
    user_id: &str,
    choice: VoteChoice,
) -> ProposalState {
    let _ = council.vote(state.id.clone(), user_id, choice);

    let ai_ids: Vec<String> = council
        .member_ids()
        .into_iter()
        .filter(|id| id != user_id)
        .collect();

    let mut rng = rand::thread_rng();
    for ai_id in ai_ids {
        let roll: f64 = rng.r#gen();
        let ai_choice = if roll < 0.55 {
            VoteChoice::For
        } else if roll < 0.85 {
            VoteChoice::Against
        } else {
            VoteChoice::Abstain
        };
        let _ = council.vote(state.id.clone(), &ai_id, ai_choice);
    }

    let outcome = council.close(state.id.clone()).ok();
    let next_round = state.round + 1;
    let mut next_state = new_proposal(council, next_round);
    next_state.last_outcome = outcome;
    next_state
}

fn new_proposal(council: &mut GovernanceCouncil, round: u32) -> ProposalState {
    let mut rng = rand::thread_rng();
    let risk = match rng.gen_range(0..3) {
        0 => ProposalRisk::Low,
        1 => ProposalRisk::Medium,
        _ => ProposalRisk::High,
    };

    let titles = [
        "Increase validator penalties",
        "Reduce block rewards",
        "Adopt new treasury policy",
        "Freeze unstable markets",
        "Emergency governance patch",
        "Raise quorum threshold",
    ];

    let title = titles[rng.gen_range(0..titles.len())].to_string();
    let id = council.propose(title.clone(), risk, 60);

    ProposalState {
        id,
        title,
        risk,
        round,
        last_outcome: None,
    }
}

fn draw_ui(
    frame: &mut ratatui::Frame<'_>,
    council: &GovernanceCouncil,
    state: &ProposalState,
    user_id: &str,
) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(6),
            Constraint::Min(6),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let header = Paragraph::new("Consensus Arena Live")
        .block(Block::default().borders(Borders::ALL).title("Lineage Governance"))
        .style(Style::default().fg(Color::Cyan));
    frame.render_widget(header, layout[0]);

    let risk_label = match state.risk {
        ProposalRisk::Low => "LOW",
        ProposalRisk::Medium => "MEDIUM",
        ProposalRisk::High => "HIGH",
    };

    let outcome_line = match state.last_outcome {
        Some(ProposalOutcome::Passed) => "Last outcome: PASSED",
        Some(ProposalOutcome::Failed) => "Last outcome: FAILED",
        Some(ProposalOutcome::NoQuorum) => "Last outcome: NO QUORUM",
        None => "Last outcome: -",
    };

    let proposal = Paragraph::new(vec![
        Line::from(format!("Round {}", state.round)),
        Line::from(format!("Proposal: {}", state.title)),
        Line::from(format!("Risk: {}", risk_label)),
        Line::from(outcome_line),
        Line::from("Vote: [f] For  [a] Against  [s] Abstain  [q] Quit"),
    ])
    .block(Block::default().borders(Borders::ALL).title("Proposal"));
    frame.render_widget(proposal, layout[1]);

    let mut items: Vec<ListItem> = Vec::new();
    for member_id in council.member_ids() {
        let name = council.member_name(&member_id).unwrap_or("Unknown");
        let energy = council.member_energy(&member_id).unwrap_or(0);
        let damage = council.member_damage(&member_id).unwrap_or(0);
        let marker = if member_id == user_id { "*" } else { " " };
        let line = Line::from(vec![
            Span::styled(marker, Style::default().fg(Color::Yellow)),
            Span::raw(format!(" {} | energy: {:4} | damage: {:3}", name, energy, damage)),
        ]);
        items.push(ListItem::new(line));
    }

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Council Members"));
    frame.render_widget(list, layout[2]);

    let footer = Paragraph::new("Votes cost energy. Dissent can scar.")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, layout[3]);
}
