use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::layout::Alignment::Center;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::state::{CurrentScreen, Sebulba};

pub fn ui<B: Backend>(f : &mut Frame<B>, app: &Sebulba) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
                .as_ref(),
        )
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Sebulba",
        Style::default().fg(Color::Yellow),
    )).alignment(Center).block(title_block);

    let container_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let mut containers = Vec::<ListItem>::new();
    for cnt in &app.all_containers {
        containers.push(ListItem::new(Line::from(Span::styled(
            &cnt.name,
            Style::default().fg(Color::Yellow),
        ))));
    }
    let container_list = List::new(containers).block(container_block);

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Detail => {
                Span::styled("Detail Mode", Style::default().fg(Color::Yellow))
            }
            CurrentScreen::Log => Span::styled("Log Mode", Style::default().fg(Color::LightRed)),
        }.to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        Span::styled("Selected Container...", Style::default().fg(Color::Green))
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = Span::styled(
        "(q) to quit / (r) to refresh",
        Style::default().fg(Color::Red),
    );

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

    f.render_widget(title, chunks[0]);
    f.render_widget(container_list, chunks[1]);
    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
                .as_ref(),
        )
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
                .as_ref(),
        )
        .split(popup_layout[1])[1] // Return the middle chunk
}