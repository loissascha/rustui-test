use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::{stdout, Result};
use std::process::exit;

fn main() -> Result<()> {
    // Enter an alternate screen and enable raw mode for terminal interaction.
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    // Initialize the terminal with a Crossterm backend.
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // Initialize the state for a list and define the list items.
    let mut state = ListState::default();
    let items = ["Test", "Quit"];
    state.select(Some(0)); // Start with the first item selected.

    // Main event loop.
    loop {
        // Draw UI components on each iteration.
        terminal.draw(|frame| {
            let area = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(items.len() as u16 + 1), Constraint::Min(0)])
                .split(area);
            let list = List::new(items)
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
                .highlight_symbol("->")
                .repeat_highlight_symbol(true);
            let command_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black).fg(Color::White))
                .title("Terminal");
            // Render the list and a paragraph widget.
            frame.render_stateful_widget(list, area, &mut state);
            frame.render_widget(
                Paragraph::new("Press 'q' to quit")
                    .style(Style::default().bg(Color::DarkGray).fg(Color::White).add_modifier(Modifier::BOLD)),
                chunks[0],
            );
            frame.render_widget(
                command_block,
                chunks[1],
            );

        })?;

        // Handle keyboard input.
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match key.kind {
                    KeyEventKind::Press | KeyEventKind::Repeat => match key.code {
                        // move down in list
                        KeyCode::Down | KeyCode::Char('j') => {
                            let selected = state.selected().unwrap_or(0);
                            state.select(Some((selected + 1).min(items.len() - 1)));
                        }
                        // move up in list
                        KeyCode::Up | KeyCode::Char('k') => {
                            let selected = state.selected().unwrap_or(0);
                            state.select(Some(selected.saturating_sub(1)));
                        }
                        KeyCode::Enter => {
                            // Handle the enter key
                            if let Some(index) = state.selected() {
                                match items[index] {
                                    "Test" => {
                                        use std::process::Command;
                                        println!("Launching terminal running test command...");
                                        Command::new("sh")
                                            .arg("-c")
                                            .arg("")
                                            .spawn()
                                            .expect("error");
                                    }
                                    "Quit" => {
                                        break;
                                    }
                                    _ => (),
                                }
                            }
                        }
                        KeyCode::Char('q') => {
                            break;
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
    }

    // Exit the alternate screen and disable raw mode before exiting.
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}