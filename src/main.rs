use std::io;

use crossterm::{event, execute};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{prelude::{CrosstermBackend, Terminal}};
use ratatui::backend::Backend;

use crate::state::{CurrentScreen, Sebulba};
use crate::ui::ui;

pub mod state;
pub mod ui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr))?;

    let mut sebulba = Sebulba::new();
    let res = run_app(&mut terminal, &mut sebulba);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            println!("Bye!");
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut Sebulba) -> io::Result<bool> {

    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('r') => { app.list_files() }
                    KeyCode::Char('q') => { return Ok(false) }
                    _ => {}
                },
                CurrentScreen::Detail => match key.code {
                    KeyCode::Char('q') => app.current_screen = CurrentScreen::Main,
                    _ => {}
                },
                CurrentScreen::Log => match key.code {
                    KeyCode::Char('q') => app.current_screen = CurrentScreen::Main,
                    _ => {}
                },
            }
        }
    }
}