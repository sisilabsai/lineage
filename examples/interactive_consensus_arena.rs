use lineage::{TaskAgent, Task, TaskOutcome, TaskResult};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Style, Color, Modifier},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;
use std::collections::VecDeque;
use rand::Rng;

const NUM_AI_AGENTS: usize = 7;
const HUMAN_INDEX: usize = 7;
const INITIAL_POWER: u64 = 1500;
const MAX_ROUNDS: usize = 30;

#[derive(Clone)]
struct ProposalType {
    title: &'static str,
    description: &'static str,
    base_risk: f32,
}

struct AgentStats {
    votes_cast: u32,
    votes_for: u32,
    votes_against: u32,
    votes_abstain: u32,
    #[allow(dead_code)]
    scars_from_dissent: u64,
    dissent_rate: f32,
}

impl AgentStats {
    fn new() -> Self {
        Self {
            votes_cast: 0,
            votes_for: 0,
            votes_against: 0,
            votes_abstain: 0,
            scars_from_dissent: 0,
            dissent_rate: 0.0,
        }
    }

    fn update_dissent_rate(&mut self) {
        if self.votes_cast > 0 {
            let dissent = (self.votes_against + self.votes_abstain) as f32;
            self.dissent_rate = (dissent / self.votes_cast as f32) * 100.0;
        }
    }
}

struct VotingRound {
    #[allow(dead_code)]
    proposal_idx: usize,
    for_votes: u32,
    against_votes: u32,
    abstain_votes: u32,
    consensus_pct: f32,
    your_vote: usize,
    scarring: u64,
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize agents
    let mut human = TaskAgent::create(INITIAL_POWER);
    let mut ai_agents: Vec<TaskAgent> = (0..NUM_AI_AGENTS)
        .map(|_| TaskAgent::create(INITIAL_POWER))
        .collect();

    // Proposals
    let proposals = vec![
        ProposalType { title: "Protocol: Increase block size", description: "Higher throughput", base_risk: 0.70 },
        ProposalType { title: "Emergency: Security patch", description: "Fix critical bug", base_risk: 0.15 },
        ProposalType { title: "Allocation: Dev fund increase", description: "Budget expansion", base_risk: 0.45 },
        ProposalType { title: "Transaction: Accept $50M inflow", description: "Major exchange deal", base_risk: 0.50 },
        ProposalType { title: "Policy: Change voting threshold", description: "60% → 50% consensus", base_risk: 0.65 },
        ProposalType { title: "Governance: Stake weight adjustment", description: "Rebalance power", base_risk: 0.55 },
    ];

    let mut rng = rand::thread_rng();
    let mut round = 0;
    let mut selected_vote = 0; // 0 = For, 1 = Against, 2 = Abstain
    let mut human_stats = AgentStats::new();
    let mut voting_history: VecDeque<VotingRound> = VecDeque::new();
    let mut power_history: Vec<u64> = vec![];
    let mut total_scarring = 0u64;

    // Main loop
    loop {
        if round >= MAX_ROUNDS {
            break;
        }

        let proposal = &proposals[rng.gen_range(0..proposals.len())];

        // Draw voting screen
        terminal.draw(|f: &mut ratatui::Frame| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(8),
                    Constraint::Length(12),
                    Constraint::Min(5),
                    Constraint::Length(3),
                ].as_ref())
                .split(size);

            // Header: Proposal
            let proposal_text = vec![
                Line::from(vec![
                    Span::styled(format!("Round {} / {} ", round + 1, MAX_ROUNDS), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::styled("━", Style::default().fg(Color::Gray)),
                    Span::styled(format!(" {} ", proposal.title), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(format!("  {}", proposal.description)),
                Line::from(""),
                Line::from(vec![
                    Span::raw("Risk Level: "),
                    Span::styled(format!("{:.0}%", proposal.base_risk * 100.0), 
                        Style::default().fg(if proposal.base_risk > 0.6 { Color::Red } else if proposal.base_risk > 0.4 { Color::Yellow } else { Color::Green })),
                ]),
                Line::from(""),
                Line::from("╔════════════════════════════════════════╗"),
                Line::from("║     Select your vote: ← LEFT | RIGHT → ║"),
                Line::from("║            ENTER to vote               ║"),
            ];
            let proposal_widget = Paragraph::new(proposal_text)
                .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::White)))
                .alignment(Alignment::Center);
            f.render_widget(proposal_widget, chunks[0]);

            // Vote options
            let vote_options = ["FOR", "AGAINST", "ABSTAIN"];
            let mut option_text = vec![];
            for (i, &vote_str) in vote_options.iter().enumerate() {
                let style = if i == selected_vote {
                    Style::default().bg(Color::Yellow).fg(Color::Black).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                option_text.push(Span::styled(format!("  [{}]  ", vote_str), style));
            }
            option_text.insert(0, Span::raw(" "));
            let vote_widget = Paragraph::new(vec![Line::from(option_text)])
                .block(Block::default().borders(Borders::ALL).title(" YOUR VOTE "))
                .alignment(Alignment::Center);
            f.render_widget(vote_widget, chunks[1]);

            // Network status
            let agent_lines: Vec<Line> = (0..NUM_AI_AGENTS + 1)
                .map(|i| {
                    let agent = if i < NUM_AI_AGENTS {
                        &ai_agents[i]
                    } else {
                        &human
                    };
                    let prefix = if i == HUMAN_INDEX { "YOU " } else { &format!("AI#{} ", i) };
                    let power_pct = (agent.current_capacity() as f32 / INITIAL_POWER as f32 * 100.0).max(0.0);
                    let status = if agent.current_capacity() < 50 { "🤐 SILENT" } else { "  Active" };
                    let damage = agent.damage_score();
                    
                    Line::from(format!(
                        "{} | 💪 {: >3.0}% | 💔 {:3} | {}",
                        prefix, power_pct, damage, status
                    ))
                })
                .collect();
            let agents_widget = Paragraph::new(agent_lines)
                .block(Block::default().borders(Borders::ALL).title(" 8 AGENTS "))
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(agents_widget, chunks[2]);

            // Instructions
            let instructions = Line::from(vec![
                Span::styled("Q: ", Style::default().fg(Color::Gray)),
                Span::raw("Quit  |  "),
                Span::styled("TAB: ", Style::default().fg(Color::Gray)),
                Span::raw("History  |  "),
                Span::styled("Your votes: ", Style::default().fg(Color::Cyan)),
                Span::raw(format!("{}", human_stats.votes_cast)),
            ]);
            let footer = Paragraph::new(instructions).alignment(Alignment::Center);
            f.render_widget(footer, chunks[3]);
        })?;

        // Input handling
        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Left => selected_vote = selected_vote.saturating_sub(1),
                    KeyCode::Right => selected_vote = (selected_vote + 1) % 3,
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Enter => {
                        // Execute voting round
                        let your_choice = selected_vote;
                        
                        // Record your vote
                        human_stats.votes_cast += 1;
                        match your_choice {
                            0 => human_stats.votes_for += 1,
                            1 => human_stats.votes_against += 1,
                            _ => human_stats.votes_abstain += 1,
                        }
                        human_stats.update_dissent_rate();

                        // Your vote cost energy
                        let task = Task::new("Cast consensus vote".to_string(), 30 + rng.gen_range(10..50));
                        let your_outcome = match your_choice {
                            0 => TaskOutcome::Success,
                            1 => TaskOutcome::SevereFailure { reason: "Dissent from majority".to_string() },
                            _ => TaskOutcome::RecoverableFailure { reason: "Abstained".to_string() },
                        };
                        let _ = human.execute_task(task.clone(), your_outcome.clone());

                        // AI voting (influenced by scarring)
                        let mut for_votes = if your_choice == 0 { 1 } else { 0 };
                        let mut against_votes = if your_choice == 1 { 1 } else { 0 };
                        let mut abstain_votes = if your_choice == 2 { 1 } else { 0 };
                        let mut round_scarring = 0u64;

                        for ai in &mut ai_agents {
                            // Conservative bias from damage
                            let damage_ratio = (ai.damage_score() as f32 / INITIAL_POWER as f32).min(1.0);
                            let conservative_bias = damage_ratio * 0.8;
                            let adjusted_risk = proposal.base_risk * (1.0 - conservative_bias);

                            // Silence if too scarred
                            if ai.current_capacity() < 50 {
                                abstain_votes += 1;
                                continue;
                            }

                            let roll = rng.gen_range(0.0..1.0);
                            let ai_outcome = if roll < adjusted_risk {
                                TaskOutcome::Success
                            } else if roll < 0.6 + conservative_bias {
                                TaskOutcome::SevereFailure { reason: "Dissent from proposal".to_string() }
                            } else {
                                TaskOutcome::RecoverableFailure { reason: "Abstained".to_string() }
                            };

                            let ai_task = Task::new("Participate in voting".to_string(), 40 + rng.gen_range(20..60));
                            let result = ai.execute_task(ai_task, ai_outcome.clone());

                            match ai_outcome {
                                TaskOutcome::Success => for_votes += 1,
                                TaskOutcome::SevereFailure { .. } => {
                                    against_votes += 1;
                                    if let TaskResult::Failed { damage_inflicted, .. } = result {
                                        round_scarring += damage_inflicted as u64;
                                    }
                                }
                                _ => abstain_votes += 1,
                            }
                        }

                        let total_votes = for_votes + against_votes + abstain_votes;
                        let consensus = if total_votes > 0 {
                            (for_votes as f32 / total_votes as f32) * 100.0
                        } else {
                            50.0
                        };

                        let outcome_str = if consensus >= 66.67 {
                            "✓ CONSENSUS PASSED"
                        } else if consensus >= 50.0 {
                            "✓ MAJORITY PASSED"
                        } else {
                            "✗ FAILED"
                        };

                        total_scarring += round_scarring;
                        power_history.push(human.current_capacity() as u64);

                        voting_history.push_back(VotingRound {
                            proposal_idx: 0,
                            for_votes,
                            against_votes,
                            abstain_votes,
                            consensus_pct: consensus,
                            your_vote: your_choice,
                            scarring: round_scarring,
                        });

                        // Show result
                        terminal.draw(|f: &mut ratatui::Frame| {
                            let size = f.size();
                            let block = Block::default()
                                .borders(Borders::ALL)
                                .title(" VOTING RESULT ")
                                .style(Style::default().fg(Color::Green));
                            
                            let text = vec![
                                Line::from(""),
                                Line::from(vec![
                                    Span::styled(format!("  {}  ", outcome_str), 
                                        Style::default().fg(if consensus >= 66.67 { Color::Green } else if consensus >= 50.0 { Color::Yellow } else { Color::Red }).add_modifier(Modifier::BOLD))
                                ]),
                                Line::from(""),
                                Line::from(format!("  FOR: {}  |  AGAINST: {}  |  ABSTAIN: {}", for_votes, against_votes, abstain_votes)),
                                Line::from(format!("  Consensus: {:.1}%", consensus)),
                                Line::from(""),
                                Line::from(format!("  Network Scarring This Round: {} 💔", round_scarring)),
                                Line::from(format!("  Your Influence: {:.1}%", 
                                    (human.current_capacity() as f32 / INITIAL_POWER as f32) * 100.0)),
                                Line::from(""),
                                Line::from("  Press ENTER to continue..."),
                                Line::from(""),
                            ];
                            
                            let paragraph = Paragraph::new(text)
                                .block(block)
                                .alignment(Alignment::Center);
                            f.render_widget(paragraph, size);
                        })?;

                        // Wait for user to continue
                        loop {
                            if let Event::Key(key) = read()? {
                                if key.code == KeyCode::Enter {
                                    break;
                                } else if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                                    let _ = disable_raw_mode();
                                    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
                                    return Ok(());
                                }
                            }
                        }

                        round += 1;
                        selected_vote = 0;
                    }
                    _ => {}
                }
            }
        }
    }

    // Final report
    terminal.draw(|f: &mut ratatui::Frame| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(12),
                Constraint::Min(5),
            ].as_ref())
            .split(size);

        // Title
        let title = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("╔═══════════════════════════════════════════════════╗", Style::default().fg(Color::Magenta)),
            ]),
        ]);
        f.render_widget(title, chunks[0]);

        // Your final stats
        human_stats.update_dissent_rate();
        let your_power = (human.current_capacity() as f32 / INITIAL_POWER as f32 * 100.0).max(0.0);
        let stats_text = vec![
            Line::from(vec![Span::styled("  YOUR VOTING RECORD  ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
            Line::from(format!("  Votes Cast: {}", human_stats.votes_cast)),
            Line::from(format!("  ✓ For: {}  |  ✗ Against: {}  |  ◯ Abstain: {}", 
                human_stats.votes_for, human_stats.votes_against, human_stats.votes_abstain)),
            Line::from(format!("  Dissent Rate: {:.1}%", human_stats.dissent_rate)),
            Line::from(format!("  Total Scars: {}", human.damage_score())),
            Line::from(format!("  Voting Power Remaining: {:.1}%", your_power)),
            Line::from(format!("  Network Scarring Total: {} 💔", total_scarring)),
            Line::from(""),
            Line::from("╔═══════════════════════════════════════════════════╗"),
            Line::from("║  Your choices became part of permanent record    ║"),
            Line::from("║  This history will weigh forever in governance   ║"),
            Line::from("╚═══════════════════════════════════════════════════╝"),
        ];
        let stats_widget = Paragraph::new(stats_text)
            .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::White)));
        f.render_widget(stats_widget, chunks[1]);

        // Recent rounds
        let recent: Vec<Line> = voting_history.iter().rev().take(8).map(|vr| {
            let vote_str = ["FOR", "AGAINST", "ABSTAIN"][vr.your_vote];
            Line::from(format!(
                "  Round: {} votes | Your: {} | Consensus: {:.1}% | Scarring: {}",
                vr.for_votes + vr.against_votes + vr.abstain_votes, vote_str, vr.consensus_pct, vr.scarring
            ))
        }).collect();
        let history_widget = Paragraph::new(recent)
            .block(Block::default().borders(Borders::ALL).title(" RECENT VOTING HISTORY "))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(history_widget, chunks[2]);
    })?;

    // Wait for final keypress
    loop {
        if let Event::Key(_) = read()? {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}