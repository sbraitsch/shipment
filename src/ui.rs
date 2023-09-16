use std::rc::Rc;

use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::layout::Alignment::{Center, Right};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::state::{Container, Mode, Shipment};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut Shipment) {
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
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(app.theme.primary))
        .style(Style::default());

    let primary_color = app.theme.primary;

    let title = Paragraph::new(Text::styled(
        "Welcome to Shipment, 1v1?",
        Style::default().fg(primary_color),
    )).alignment(Center).block(title_block);

    f.render_widget(title, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(chunks[1]);

    let current_screen = app.mode.clone();

    match current_screen {
        Mode::Main(c) => {
            render_selection_list(f, app, &main_chunks);
            render_file_content(f, app, &chunks, main_chunks, c);
        }
    }

    let current_navigation_text = match &app.mode {
        Mode::Main(Some(c)) => {
            if let Err(msg) = &app.info {
                vec![Span::styled(msg, Style::default().fg(Color::LightRed))]
            } else {
                vec![Span::styled("Press Enter for Details", Style::default().fg(primary_color)),
                     Span::styled(" | ", Style::default().fg(Color::White)),
                     Span::styled(&c.name, Style::default().fg(Color::LightYellow))]
            }
        }
        _ => { vec![Span::styled("Critical Error", Style::default().fg(Color::LightRed))] }
    }.to_owned();

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default());

    let current_keys_hint = vec![
        Span::styled("Q: Quit", Style::default().fg(primary_color)),
        Span::styled(" | ", Style::default().fg(Color::White)),
        Span::styled("L: Toggle Logs", Style::default().fg(primary_color)),
        Span::styled(" | ", Style::default().fg(Color::White)),
        Span::styled("TAB: Next Container", Style::default().fg(primary_color)),
        Span::styled(" | ", Style::default().fg(Color::White)),
        Span::styled("↑↓: Move in Logs", Style::default().fg(primary_color))];

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).alignment(Right).block(Block::default());

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[2]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);
}

fn render_file_content<B: Backend>(f: &mut Frame<B>, app: &mut Shipment, chunks: &Rc<[Rect]>, main_chunks: Rc<[Rect]>, c: Option<Container>) {
    let view_height = chunks[1].height as usize - 2;
    let view: Vec<ListItem>;
    if let Some(container) = c {
        let lines: Vec<&str> = container.logs.lines().collect();
        let line_count = lines.len();

        if line_count < view_height || app.offset > (line_count.wrapping_sub(view_height)) {
            if app.offset > 0 { app.offset -= app.offset };
        }

        let upper_bound = (app.offset + view_height).clamp(0, line_count);
        view = lines[app.offset..upper_bound].iter().map(|line|
            ListItem::new(
                Line::from(Span::styled(String::from(*line), Style::default().fg(app.theme.primary)))
            )
        ).collect();
    } else {
        view = vec![
            ListItem::new(Line::from(Span::styled("REEEEEEEEEEE", Style::default().fg(app.theme.primary)))
        )]
    }

    f.render_widget(List::new(view).block(Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.primary))
    ), main_chunks[1]);
}

fn render_selection_list<B: Backend>(f: &mut Frame<B>, app: &mut Shipment, main_chunks: &Rc<[Rect]>) {
    let mut containers = Vec::<ListItem>::new();

    for (idx, cnt) in app.all_containers.iter().enumerate() {
        let mut style = Style::default().fg(Color::Gray);
        if let Some(selected_idx) = app.selected_idx {
            if selected_idx == idx {
                style = Style::default().fg(app.theme.primary)
            }
        };
        containers.push(ListItem::new(Line::from(Span::styled(
            &cnt.name,
            style,
        ))));
    }
    let container_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(app.theme.primary))
        .style(Style::default());
    let container_list = List::new(containers).block(container_block);
    f.render_widget(container_list, main_chunks[0]);
}

#[allow(dead_code)]
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