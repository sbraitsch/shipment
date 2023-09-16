use std::rc::Rc;

use ratatui::backend::Backend;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::layout::Alignment::{Center, Right};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::state::{Container, CurrentScreen, Sebulba};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut Sebulba) {
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
        .border_style(Style::default().fg(app.theme.pastel_blue))
        .style(Style::default());

    let primary_color = app.theme.pastel_blue;

    let title = Paragraph::new(Text::styled(
        "Welcome to SEBULBA, the not-so-friendly Pod Manager",
        Style::default().fg(primary_color),
    )).alignment(Center).block(title_block);

    f.render_widget(title, chunks[0]);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(chunks[1]);

    let current_screen = app.current_screen.clone();

    match current_screen {
        CurrentScreen::Main => {
            render_selection_list(f, app, &main_chunks);
            f.render_widget(
                Paragraph::new(
                    Text::styled(
                        "No Selection",
                        Style::default().fg(Color::DarkGray)
                    )
                )
                    .alignment(Center).block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::DarkGray))),
                main_chunks[1]);
        }
        CurrentScreen::Detail(c) | CurrentScreen::File(c) => {
            render_selection_list(f, app, &main_chunks);
            render_file_content(f, app, &chunks, main_chunks, c);
        }
        CurrentScreen::Log(_) => {}
    }


    let selected_name = if let Some(idx) = app.selected_idx {
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
        CurrentScreen::Detail(c) | CurrentScreen::File(c) | CurrentScreen::Log(c) => {
            vec![Span::styled("Viewing", Style::default().fg(Color::White)),
                 Span::styled(" | ", Style::default().fg(primary_color)),
                 Span::styled(&c.name, Style::default().fg(primary_color))]
        }
    }.to_owned();

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default());

    let current_keys_hint = vec![
        Span::styled("Q: Quit / Go Back", Style::default().fg(primary_color)),
        Span::styled(" | ", Style::default().fg(Color::White)),
        Span::styled("D: Container Details", Style::default().fg(primary_color)),
        Span::styled(" | ", Style::default().fg(Color::White)),
        Span::styled("L: Container Logs", Style::default().fg(primary_color)),
        Span::styled(" | ", Style::default().fg(Color::White)),
        Span::styled("TAB: Switch Panes", Style::default().fg(primary_color)),
        Span::styled(" | ", Style::default().fg(Color::White)),
        Span::styled("↑↓: Move", Style::default().fg(primary_color))];

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).alignment(Right).block(Block::default());

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[2]);

    f.render_widget(mode_footer, footer_chunks[0]);
    f.render_widget(key_notes_footer, footer_chunks[1]);
}

fn render_file_content<B: Backend>(f: &mut Frame<B>, app: &mut Sebulba, chunks: &Rc<[Rect]>, main_chunks: Rc<[Rect]>, c: Container) {
    let view_height = chunks[1].height as usize - 2;
    let lines: Vec<&str> = c.logs.lines().collect();
    let line_count = lines.len();

    if line_count < view_height || app.offset > (line_count.wrapping_sub(view_height)) {
        if app.offset > 0 { app.offset = app.offset - 1 };
    }

    let upper_bound = (app.offset + view_height).clamp(0, line_count);
    let view: Vec<ListItem> = lines[app.offset..upper_bound].iter().map(|line|
        ListItem::new(
            Line::from(Span::styled(String::from(*line), Style::default().fg(app.theme.pastel_blue)))
        )
    ).collect();

    f.render_widget(List::new(view).block(Block::default()
        .borders(Borders::ALL)
        .border_style(
            match app.current_screen {
                CurrentScreen::File(_) => Style::default().fg(app.theme.pastel_blue),
                _ => Style::default()
            }
        )), main_chunks[1]);
}

fn render_selection_list<B: Backend>(f: &mut Frame<B>, app: &mut Sebulba, main_chunks: &Rc<[Rect]>) {
    let mut containers = Vec::<ListItem>::new();

    for (idx, cnt) in app.all_containers.iter().enumerate() {
        let mut style = Style::default().fg(Color::Gray);
        if let Some(selected_idx) = app.selected_idx {
            if selected_idx == idx {
                style = Style::default().fg(app.theme.pastel_blue)
            }
        };
        containers.push(ListItem::new(Line::from(Span::styled(
            &cnt.name,
            style,
        ))));
    }
    let container_block = Block::default()
        .borders(Borders::ALL)
        .border_style(
            match app.current_screen {
                CurrentScreen::Detail(_) | CurrentScreen::Main => Style::default().fg(app.theme.pastel_blue),
                _ => Style::default()
            }
        ).style(Style::default());
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