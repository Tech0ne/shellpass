use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, state::State},
    errors::{Error, Result},
};

mod edit_entry;
mod edit_profile;
mod entry_detail;
mod entry_list;
mod profile_list;
mod unlock;

pub fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(Duration::from_millis(50)).map_err(|e| Error::crossterm(e))? {
        if let Event::Key(key) = event::read().map_err(|e| Error::crossterm(e))? {
            handle_key(app, key)?;
        }
    }

    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) -> Result<()> {
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.quit = true;
        return Ok(());
    }
    match app.state {
        State::EditEntry {
            profile_index,
            entry_index,
        } => edit_entry::handle(app, key, profile_index, entry_index),

        State::EditProfile { profile_index } => edit_profile::handle(app, key, profile_index),

        State::EntryDetail {
            profile_index,
            entry_index,
        } => entry_detail::handle(app, key, profile_index, entry_index),

        State::EntryList { profile_index } => entry_list::handle(app, key, profile_index),

        State::ProfileList => profile_list::handle(app, key),

        State::Unlock => unlock::handle(app, key),
    }
}
