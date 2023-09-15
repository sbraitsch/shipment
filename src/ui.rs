use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::layout::Alignment::Center;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::state::{CurrentScreen, Sebulba};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &Sebulba) {
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

    let primary_color = app.theme.pastel_blue;

    let title = Paragraph::new(Text::styled(
        "Sebulba",
        Style::default().fg(primary_color),
    )).alignment(Center).block(title_block);

    f.render_widget(title, chunks[0]);


    match &app.current_screen {
        CurrentScreen::Main => {
            let container_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());
            let mut containers = Vec::<ListItem>::new();

            for (idx, cnt) in app.all_containers.iter().enumerate() {
                let mut style = Style::default().fg(Color::Gray);
                if let Some(selected_idx) = app.selected {
                    if selected_idx == idx {
                        style = Style::default().fg(Color::LightYellow)
                    }
                };
                containers.push(ListItem::new(Line::from(Span::styled(
                    &cnt.name,
                    style,
                ))));
            }
            let container_list = List::new(containers).block(container_block);
            f.render_widget(container_list, chunks[1]);
        }
        CurrentScreen::Detail(c) => {
            let view_height = chunks[1].height as usize;
            let lines: Vec<&str> = c.logs.lines().collect();
            let mut windows = lines.windows(view_height);
            let view: Vec<ListItem> = if lines.len() < view_height {
                lines.iter().map(|line|
                    ListItem::new(
                        Line::from(Span::styled(String::from(*line), Style::default().fg(app.theme.mint)))
                    )
                ).collect()
            } else {
                windows.nth(app.offset).unwrap_or(windows.last().unwrap()).iter().map(|line|
                    ListItem::new(
                        Line::from(Span::styled(String::from(*line), Style::default().fg(app.theme.mint)))
                    )
                ).collect()
            };

            f.render_widget(List::new(view).block(Block::default().borders(Borders::ALL)), chunks[1]);
        }
        CurrentScreen::Log(_) => {}
    }


    let selected_name = if let Some(idx) = app.selected {
        &app.all_containers[idx].name
    } else { "" };

    let current_navigation_text = match &app.current_screen {
        CurrentScreen::Main => {
            if let Err(msg) = &app.info {
                vec![Span::styled(msg, Style::default().fg(Color::LightRed))]
            } else {
                vec![Span::styled("Press Enter for Details", Style::default().fg(primary_color)),
                     Span::styled(" | ", Style::default().fg(Color::White)),
                     Span::styled(selected_name, Style::default().fg(Color::LightYellow))]
            }
        }
        CurrentScreen::Detail(c) => {
            vec![Span::styled("Details for", Style::default().fg(primary_color)),
                 Span::styled(" | ", Style::default().fg(Color::White)),
                 Span::styled(&c.name, Style::default().fg(Color::LightYellow))]
        }
        CurrentScreen::Log(c) => {
            vec![Span::styled("Logs for", Style::default().fg(primary_color)),
                 Span::styled(" | ", Style::default().fg(Color::White)),
                 Span::styled(&c.name, Style::default().fg(Color::LightYellow))]
        }
    }.to_owned();

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = Span::styled(
        "(q) to quit / (r) to refresh",
        Style::default().fg(primary_color),
    );

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[2]);

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