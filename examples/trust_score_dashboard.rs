use lineage::{TaskAgent, Task, TaskOutcome, TaskResult};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment, Rect},
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
use std::collections::HashMap;
use rand::Rng;

const _NUM_AGENTS: usize = 5;
const INITIAL_CAPACITY: u64 = 2000;
const MAX_PROJECTS: usize = 20;
const _DEMO_MODE: bool = true;

#[derive(Clone, Debug)]
struct ProjectPhase {
    name: &'static str,
    _description: &'static str,
    base_risk: f32,
}

struct Agent {
    id: usize,
    name: &'static str,
    task_agent: TaskAgent,
}

#[derive(Clone, Debug)]
struct Decision {
    agent_id: usize,
    confidence: f32,
    was_correct: bool,
    scars_from_error: u64,
}

#[derive(Clone, Debug)]
struct ProjectOutcome {
    name: String,
    decisions: Vec<Decision>,
    total_success: bool,
    scars_inflicted: u64,
    cost: u64,
}

struct DashboardMetrics {
    projects_completed: u32,
    total_recommendations: u32,
    success_rate: f32,
    total_network_scars: u64,
    avg_confidence: f32,
    agent_stats: HashMap<usize, (u64, u32, u32)>, // (scars, correct, wrong)
}

fn calculate_trust_score(damage: u64, correct_decisions: u32, wrong_decisions: u32) -> f32 {
    if correct_decisions == 0 && wrong_decisions == 0 {
        return 100.0;
    }
    
    let accuracy = correct_decisions as f32 / (correct_decisions as f32 + wrong_decisions as f32);
    let scar_penalty = (damage as f32 / 2000.0).min(1.0);
    let trust = (accuracy * 100.0) * (1.0 - scar_penalty * 0.5);
    trust.max(0.0)
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize agents
    let mut agents = vec![
        Agent { id: 0, name: "ARIA", task_agent: TaskAgent::create(INITIAL_CAPACITY) },
        Agent { id: 1, name: "NEXUS", task_agent: TaskAgent::create(INITIAL_CAPACITY) },
        Agent { id: 2, name: "ATLAS", task_agent: TaskAgent::create(INITIAL_CAPACITY) },
        Agent { id: 3, name: "SAGE", task_agent: TaskAgent::create(INITIAL_CAPACITY) },
        Agent { id: 4, name: "PRISM", task_agent: TaskAgent::create(INITIAL_CAPACITY) },
    ];

    let project_phases = vec![
        ProjectPhase { name: "Design Phase", _description: "Architecture & planning (20% risk)", base_risk: 0.20 },
        ProjectPhase { name: "Development", _description: "Code implementation (40% risk)", base_risk: 0.40 },
        ProjectPhase { name: "Testing", _description: "QA & bug detection (30% risk)", base_risk: 0.30 },
        ProjectPhase { name: "Deployment", _description: "Production rollout (50% risk)", base_risk: 0.50 },
        ProjectPhase { name: "Monitoring", _description: "Post-launch support (25% risk)", base_risk: 0.25 },
    ];

    let mut rng = rand::thread_rng();
    let mut project_count: u32 = 0;
    let mut completed_projects: Vec<ProjectOutcome> = Vec::new();
    let mut selected_view: usize = 0; // 0 = Dashboard, 1 = Agents, 2 = History

    // Main loop
    loop {
        if project_count as usize >= MAX_PROJECTS {
            break;
        }

        // Calculate metrics for this frame
        let mut metrics = DashboardMetrics {
            projects_completed: project_count as u32,
            total_recommendations: 0,
            success_rate: 0.0,
            total_network_scars: 0,
            avg_confidence: 0.0,
            agent_stats: HashMap::new(),
        };

        for (i, _) in agents.iter().enumerate() {
            metrics.agent_stats.insert(i, (0, 0, 0));
        }

        for project in &completed_projects {
            metrics.total_network_scars += project.scars_inflicted;
            if project.total_success {
                metrics.total_recommendations += project.decisions.len() as u32;
            }
            
            for decision in &project.decisions {
                let (scars, correct, wrong) = metrics.agent_stats.get(&decision.agent_id).unwrap_or(&(0, 0, 0));
                let new_correct = if decision.was_correct { correct + 1 } else { *correct };
                let new_wrong = if decision.was_correct { *wrong } else { wrong + 1 };
                metrics.agent_stats.insert(decision.agent_id, (decision.scars_from_error + scars, new_correct, new_wrong));
            }
        }

        if !completed_projects.is_empty() {
            let successes = completed_projects.iter().filter(|p| p.total_success).count();
            metrics.success_rate = (successes as f32 / completed_projects.len() as f32) * 100.0;
        }

        let total_correct: u32 = metrics.agent_stats.values().map(|(_, c, _)| c).sum();
        let total_decisions: u32 = metrics.agent_stats.values().map(|(_, c, w)| c + w).sum();
        if total_decisions > 0 {
            metrics.avg_confidence = (total_correct as f32 / total_decisions as f32) * 100.0;
        }

        // Draw main dashboard
        terminal.draw(|f| {
            let size = f.size();
            
            // Title bar
            let title_area = Rect {
                x: 0,
                y: 0,
                width: size.width,
                height: 3,
            };
            
            let title_text = vec![
                Line::from(vec![
                    Span::styled("╔", Style::default().fg(Color::Cyan)),
                    Span::styled("═".repeat((size.width as usize).saturating_sub(2)), Style::default().fg(Color::Cyan)),
                    Span::styled("╗", Style::default().fg(Color::Cyan)),
                ]),
                Line::from(vec![
                    Span::styled("║ ", Style::default().fg(Color::Cyan)),
                    Span::styled("TRUST SCORE DASHBOARD", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::styled(" ■ AI Team Governance System ■ ", Style::default().fg(Color::Green)),
                    Span::styled("Powered by Lineage", Style::default().fg(Color::Magenta).add_modifier(Modifier::DIM)),
                    Span::styled(" ║", Style::default().fg(Color::Cyan)),
                ]),
                Line::from(vec![
                    Span::styled("╚", Style::default().fg(Color::Cyan)),
                    Span::styled("═".repeat((size.width as usize).saturating_sub(2)), Style::default().fg(Color::Cyan)),
                    Span::styled("╝", Style::default().fg(Color::Cyan)),
                ]),
            ];
            let title = Paragraph::new(title_text).alignment(Alignment::Center);
            f.render_widget(title, title_area);

            // Main content area
            let main_area = Rect {
                x: 0,
                y: 3,
                width: size.width,
                height: size.height.saturating_sub(7),
            };

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(main_area);

            // Left sidebar: Agent cards with trust scores
            let agent_cards_area = chunks[0];
            let agent_lines: Vec<Line> = agents.iter().map(|agent| {
                let (scars, correct, wrong) = metrics.agent_stats.get(&agent.id).unwrap_or(&(0, 0, 0));
                let trust = calculate_trust_score(*scars, *correct, *wrong);
                let _status = match trust {
                    t if t >= 80.0 => "✓ TRUSTED",
                    t if t >= 60.0 => "◐ CAUTION",
                    _ => "✗ UNRELIABLE",
                };
                let trust_bar = if trust >= 80.0 { "████████░" } 
                               else if trust >= 60.0 { "██████░░░" }
                               else { "████░░░░░" };
                
                Line::from(format!(
                    "{} | Trust: {} [{}] | Scars: {}",
                    agent.name.to_string().bold_cyan(),
                    format!("{:.0}%", trust).bold_yellow(),
                    trust_bar,
                    scars
                ))
            }).collect();

            let sidebar_text = [
                vec![
                    Line::from(vec![Span::styled("═ AGENTS ═", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
                    Line::from(""),
                ],
                {
                    let mut with_spacing = Vec::new();
                    for (i, line) in agent_lines.iter().enumerate() {
                        with_spacing.push(line.clone());
                        if i < agent_lines.len() - 1 {
                            with_spacing.push(Line::from(""));
                        }
                    }
                    with_spacing
                },
                vec![
                    Line::from(""),
                    Line::from(vec![Span::styled("═ NETWORK STATS ═", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
                    Line::from(format!("Projects: {}", metrics.projects_completed)),
                    Line::from(format!("Success Rate: {:.1}%", metrics.success_rate)),
                    Line::from(format!("Total Scars: {} 💔", metrics.total_network_scars)),
                    Line::from(format!("Avg Confidence: {:.1}%", metrics.avg_confidence)),
                    Line::from(""),
                    Line::from(vec![Span::styled("Controls:", Style::default().fg(Color::Gray))]),
                    Line::from("←/→: Switch view"),
                    Line::from("SPACE: Next project"),
                    Line::from("Q: Exit"),
                ],
            ].concat();

            let sidebar = Paragraph::new(sidebar_text)
                .block(Block::default().borders(Borders::RIGHT).title(" Team Status "))
                .style(Style::default().fg(Color::White));
            f.render_widget(sidebar, agent_cards_area);

            // Right main area: Project details or metrics
            let right_area = chunks[1];

            match selected_view {
                0 => {
                    // Dashboard view with project info
                    let project_idx = (project_count as usize).saturating_sub(1);
                    if project_idx < completed_projects.len() {
                        let project = &completed_projects[project_idx];
                        let project_text = vec![
                            Line::from(vec![Span::styled(format!("PROJECT #{}", project_count), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
                            Line::from(vec![Span::styled(&project.name, Style::default().fg(Color::Green))]),
                            Line::from(""),
                            Line::from(vec![
                                Span::styled("Status: ", Style::default().fg(Color::Gray)),
                                if project.total_success {
                                    Span::styled("✓ SUCCESS", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                                } else {
                                    Span::styled("✗ FAILURE", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                                }
                            ]),
                            Line::from(format!("Cost: {} energy", project.cost)),
                            Line::from(format!("Scarring: {} 💔", project.scars_inflicted)),
                            Line::from(""),
                            Line::from(vec![Span::styled("DECISIONS MADE:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]),
                            Line::from(""),
                        ];

                        let mut full_text = project_text;
                        for decision in &project.decisions {
                            let agent_name = agents[decision.agent_id].name;
                            let result = if decision.was_correct { "✓" } else { "✗" };
                            let _color = if decision.was_correct { Color::Green } else { Color::Red };
                            full_text.push(Line::from(format!(
                                "  {} {} - Confidence: {:.0}% | Scars: {}",
                                result, agent_name, decision.confidence * 100.0, decision.scars_from_error
                            )));
                        }

                        let project_info = Paragraph::new(full_text)
                            .block(Block::default().borders(Borders::ALL).title(" Project Details "))
                            .style(Style::default().fg(Color::White));
                        f.render_widget(project_info, right_area);
                    }
                }
                1 => {
                    // Agent detailed view
                    let mut agent_detail = vec![
                        Line::from(vec![Span::styled("DETAILED AGENT ANALYSIS", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
                        Line::from(""),
                    ];

                    for agent in &agents {
                        let (scars, correct, wrong) = metrics.agent_stats.get(&agent.id).unwrap_or(&(0, 0, 0));
                        let trust = calculate_trust_score(*scars, *correct, *wrong);
                        agent_detail.push(Line::from(format!("► {} (ID: {})", agent.name, agent.id)));
                        agent_detail.push(Line::from(format!("  Trust Score: {:.1}%", trust)));
                        agent_detail.push(Line::from(format!("  Correct Decisions: {}", correct)));
                        agent_detail.push(Line::from(format!("  Wrong Decisions: {}", wrong)));
                        agent_detail.push(Line::from(format!("  Accumulated Scars: {}", scars)));
                        agent_detail.push(Line::from(format!("  Capacity: {}/{}",
                            agent.task_agent.current_capacity(),
                            INITIAL_CAPACITY
                        )));
                        agent_detail.push(Line::from(format!("  Damage Score: {}", agent.task_agent.damage_score())));
                        agent_detail.push(Line::from(""));
                    }

                    let agent_view = Paragraph::new(agent_detail)
                        .block(Block::default().borders(Borders::ALL).title(" Agent Profiles "))
                        .style(Style::default().fg(Color::White));
                    f.render_widget(agent_view, right_area);
                }
                _ => {
                    // History view
                    let mut history = vec![
                        Line::from(vec![Span::styled("PROJECT HISTORY", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
                        Line::from(""),
                    ];

                    for (i, project) in completed_projects.iter().enumerate().rev().take(8) {
                        let status_icon = if project.total_success { "✓" } else { "✗" };
                        let status_color = if project.total_success { Color::Green } else { Color::Red };
                        history.push(Line::from(vec![
                            Span::styled(format!("[{}] ", i + 1), Style::default().fg(Color::Gray)),
                            Span::styled(status_icon, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
                            Span::raw(format!(" {} - {} scars", &project.name, project.scars_inflicted)),
                        ]));
                    }

                    let history_view = Paragraph::new(history)
                        .block(Block::default().borders(Borders::ALL).title(" Recent Projects "))
                        .style(Style::default().fg(Color::White));
                    f.render_widget(history_view, right_area);
                }
            }

            // Footer
            let footer_area = Rect {
                x: 0,
                y: size.height.saturating_sub(2),
                width: size.width,
                height: 2,
            };

            let view_name = match selected_view {
                0 => "DASHBOARD",
                1 => "AGENT PROFILES",
                _ => "PROJECT HISTORY",
            };

            let footer = vec![
                Line::from(vec![
                    Span::styled("View: ", Style::default().fg(Color::Gray)),
                    Span::styled(view_name, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                    Span::raw(" | "),
                    Span::styled("Press SPACE for next project  |  ← → to switch views  |  Q to quit", Style::default().fg(Color::Gray)),
                ]),
                Line::from(vec![
                    Span::styled("═".repeat(size.width as usize), Style::default().fg(Color::DarkGray)),
                ]),
            ];

            let footer_widget = Paragraph::new(footer);
            f.render_widget(footer_widget, footer_area);
        })?;

        // Input handling
        if poll(Duration::from_millis(50))? {
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Left => selected_view = selected_view.saturating_sub(1),
                    KeyCode::Right => selected_view = (selected_view + 1) % 3,
                    KeyCode::Char(' ') => {
                        // Execute next project
                        project_count += 1;

                        let phase = &project_phases[rng.gen_range(0..project_phases.len())];
                        let project_name = format!("Project-{}: {}", project_count, phase.name);

                        let mut decisions = Vec::new();
                        let mut total_project_success = true;
                        let mut project_scarring = 0u64;

                        for agent in &mut agents {
                            // Agent makes a recommendation
                            let damage_ratio = (agent.task_agent.damage_score() as f32 / INITIAL_CAPACITY as f32).min(1.0);
                            let conservative = damage_ratio * 0.7;
                            let adjusted_risk = phase.base_risk * (1.0 - conservative);

                            let roll = rng.gen_range(0.0..1.0);
                            let confidence = 1.0 - adjusted_risk.abs();
                            let was_correct = roll < adjusted_risk;

                            // Execute as task
                            let task = Task::new(
                                format!("Provide recommendation for {}", phase.name),
                                50 + rng.gen_range(20..80)
                            );

                            let outcome = if was_correct {
                                TaskOutcome::Success
                            } else {
                                total_project_success = false;
                                TaskOutcome::SevereFailure {
                                    reason: "Recommendation led to critical failure".to_string()
                                }
                            };

                            let result = agent.task_agent.execute_task(task, outcome);

                            let scars = if let TaskResult::Failed { damage_inflicted, .. } = result {
                                damage_inflicted as u64
                            } else {
                                0
                            };

                            project_scarring += scars;

                            decisions.push(Decision {
                                agent_id: agent.id,
                                confidence,
                                was_correct,
                                scars_from_error: scars,
                            });
                        }

                        completed_projects.push(ProjectOutcome {
                            name: project_name,
                            decisions,
                            total_success: total_project_success,
                            scars_inflicted: project_scarring,
                            cost: 150 + rng.gen_range(50..150),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    // Final report
    terminal.draw(|f| {
        let size = f.size();
        let mut final_report = vec![
            Line::from(""),
            Line::from(vec![Span::styled("╔════════════════════════════════════════════════════════════╗", Style::default().fg(Color::Cyan))]),
            Line::from(vec![Span::styled("║  TRUST SCORE SYSTEM - FINAL GOVERNANCE REPORT              ║", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))]),
            Line::from(vec![Span::styled("╚════════════════════════════════════════════════════════════╝", Style::default().fg(Color::Cyan))]),
            Line::from(""),
            Line::from(vec![Span::styled("Projects Completed: ", Style::default().fg(Color::Cyan)), Span::raw(project_count.to_string())]),
            Line::from(""),
        ];

        let metrics = DashboardMetrics {
            projects_completed: project_count,
            total_recommendations: 0,
            success_rate: 0.0,
            total_network_scars: 0,
            avg_confidence: 0.0,
            agent_stats: HashMap::new(),
        };

        final_report.push(Line::from(vec![Span::styled("═══ AGENT FINAL STANDING ═══", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))]));
        final_report.push(Line::from(""));

        for agent in &agents {
            let (scars, correct, wrong) = metrics.agent_stats.get(&agent.id).unwrap_or(&(0, 0, 0));
            let trust = calculate_trust_score(*scars, *correct, *wrong);
            let overall_status = match trust {
                t if t >= 80.0 => "HIGHLY TRUSTWORTHY",
                t if t >= 60.0 => "ACCEPTABLE WITH CAUTION",
                _ => "REQUIRES SUPERVISION",
            };

            final_report.push(Line::from(format!("► {} - Trust: {:.1}% - {}", agent.name, trust, overall_status)));
            final_report.push(Line::from(format!("  Scars: {} | Correct: {} | Wrong: {}", scars, correct, wrong)));
        }

        final_report.push(Line::from(""));
        final_report.push(Line::from(vec![Span::styled("═══ WHY LINEAGE MATTERS ═══", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))]));
        final_report.push(Line::from(""));
        final_report.push(Line::from("✓ Every decision is permanently recorded"));
        final_report.push(Line::from("✓ Trust scores reflect actual performance (no inflation)"));
        final_report.push(Line::from("✓ Scars teach future caution (mechanical learning)"));
        final_report.push(Line::from("✓ Network converges on reliable agents"));
        final_report.push(Line::from("✓ Transparency prevents corruption"));
        final_report.push(Line::from("✓ History cannot be rewritten"));
        final_report.push(Line::from(""));
        final_report.push(Line::from(vec![Span::styled("Powered by Lineage - Where Trust Is Earned Through Scars", Style::default().fg(Color::Magenta).add_modifier(Modifier::ITALIC))]));

        let report = Paragraph::new(final_report)
            .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::White)))
            .alignment(Alignment::Left);
        f.render_widget(report, size);
    })?;

    std::thread::sleep(Duration::from_secs(3));

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

// Helper trait for styling
trait Styled {
    fn bold_cyan(self) -> String;
    fn bold_yellow(self) -> String;
}

impl Styled for &str {
    fn bold_cyan(self) -> String {
        self.to_string()
    }

    fn bold_yellow(self) -> String {
        self.to_string()
    }
}
