use std::io::{Result};
use crossterm::event::Event;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    prelude::*,
    widgets::*,
    layout::*,
    style::Stylize,
    DefaultTerminal,
    Frame,
    Terminal,
};
use crate::{Widget, StatefulWidget};

fn render_frame(frame: &mut Frame) {
    let main_frame = Layout::default()
        .direction(Direction::Vertical)// Vertical : Column Layout
        .constraints(
            vec![
                Constraint::Fill(1), // Grid with 2 columns
                Constraint::Percentage(50), // Grid with 50% width
            ]
        )
        .split(frame.area());

    //// Top Layout
    let top_frame = Layout::default()
        .direction(Direction::Horizontal) // Horizontal : Row Layout
        .constraints(
            vec![
                Constraint::Fill(1), // Grid with 2 columns
                Constraint::Percentage(80), // Grid with 80% width
            ]
        )
        .split(main_frame[0]);

    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::new()
                    .title("Menu")
                    .borders(Borders::ALL)
            ), top_frame[0],
    );

    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::new()
                    .title("Used CPU, Memory [%]")
                    .borders(Borders::ALL)
            ), top_frame[1],
    );

    //// Bottom Layout
    let bottom_frame = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            vec![
                Constraint::Percentage(100), // Grid with 100% width
            ]
        )
        .split(main_frame[1]);

    frame.render_widget(
        Paragraph::new("")
            .block(
                Block::new()
                    .title("Resource")
                    .borders(Borders::ALL)),
        bottom_frame[0],
    );
}

pub fn display() -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    loop {
        terminal.draw(|f| render_frame(f))?;
        std::thread::sleep(std::time::Duration::from_millis(100));

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    ratatui::restore();
    Ok(())
}