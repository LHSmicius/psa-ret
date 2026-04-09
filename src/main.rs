use log::debug;
use ratatui::Terminal;
use ratatui::crossterm::event;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal;
use ratatui::prelude;
use std::io;

mod app;
pub mod bus;
pub mod config;
mod ui;

fn main() {
    // Setup terminal
    terminal::enable_raw_mode().expect("Failed to enable RAW terminal mode.");
    let mut stdout = io::stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )
    .expect("Failed to setup terminal.");

    let backend = prelude::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to create Ratatui terminal.");

    let mut app = app::App::new();
    app.load_can_messages();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    terminal::disable_raw_mode().expect("Failed to disable RAW terminal mode.");
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture
    )
    .expect("Failed to set back terminal to the pre-app settings.");
    terminal
        .show_cursor()
        .expect("Failed to set back terminal's cursor.");

    if let Ok(app_res) = res {
        debug!("App exited with bool val: {}", app_res);
    } else if let Err(err) = res {
        debug!("{err:?}");
    }
}

fn run_app<B: prelude::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut app::App,
) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if let event::Event::Key(key) = event::read()? {
            // dbg!(key.code)
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.active_screen {
                app::ActiveScreen::CanBus => match key.code {
                    event::KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    event::KeyCode::Char('n') => {
                        app.edit_window = Some(app::EditWindow::NewCanMsg);
                        app.active_screen = app::ActiveScreen::Editing;
                    }
                    event::KeyCode::Char('e') => {
                        app.edit_window = Some(app::EditWindow::EditCanMsg);
                        app.active_screen = app::ActiveScreen::Editing;
                    }
                    event::KeyCode::Up | event::KeyCode::Char('k') => {
                        app.previous_message();
                    }
                    event::KeyCode::Down | event::KeyCode::Char('j') => {
                        app.next_message();
                    }
                    _ => {}
                },
                app::ActiveScreen::Editing => match key.code {
                    event::KeyCode::Char('q') => {
                        return Ok(false);
                    }
                    _ => {}
                },
            }
        }
    }
}
