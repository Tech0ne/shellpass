mod app;
mod cli_args;
mod clipboard;
mod default_vault_dir_path;
mod errors;
mod errors_builder;
mod events;
mod ui;
mod vault;

use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::{fs, io};

use crate::{
    app::App,
    default_vault_dir_path::default_vault_dir_path,
    errors::{Error, Result},
};

fn main() -> std::io::Result<()> {
    if let Err(e) = run() {
        eprintln!("[FATAL ERROR]: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn run() -> Result<()> {
    let cli = cli_args::CliArgs::parse();

    let vault_path = cli.vault_dir.unwrap_or(default_vault_dir_path());

    if let Some(dir_path) = vault_path.parent() {
        fs::create_dir_all(dir_path).map_err(|e| Error::mkdir(e))?;
    }

    let is_new_vault = !vault_path.exists();

    let mut app = App::new(vault_path, is_new_vault)?;

    enable_raw_mode().map_err(|e| Error::crossterm(e))?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture,).map_err(|e| Error::crossterm(e))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|e| Error::crossterm(e))?;

    let result = run_app(&mut terminal, &mut app);

    while event::poll(std::time::Duration::from_millis(0)).unwrap_or(false) {
        let _ = event::read();
    }

    execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen,
    )
    .map_err(|e| Error::crossterm(e))?;
    terminal.show_cursor().map_err(|e| Error::crossterm(e))?;
    disable_raw_mode().map_err(|e| Error::crossterm(e))?;

    if let Some(vault) = app.vault {
        vault.save(&app.vault_path, &app.vault_pass)?;
    }

    result?;

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal
            .draw(|f| ui::render(f, app))
            .map_err(|e| Error::crossterm(e))?;
        events::handle_events(app)?;
        app.tick();
        if app.quit {
            if app.dirty {
                app.ntfy_error("Please save before quiting !");
                app.quit = false;
            } else {
                break;
            }
        }
    }
    Ok(())
}
