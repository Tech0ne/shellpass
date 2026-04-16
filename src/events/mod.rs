mod edit_entry;
mod edit_profile;
mod entry_detail;
mod entry_list;
mod profile_list;
mod unlock;

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};

use crate::{
    app::{App, state::State},
    errors::{Error, Result},
};

pub fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(Duration::from_millis(50)).map_err(|e| Error::crossterm(e))? {
        match event::read().map_err(|e| Error::crossterm(e))? {
            Event::Key(key) => handle_key(app, key)?,
            Event::Mouse(mouse) => handle_mouse(app, mouse)?,
            _ => {}
        }
    }

    Ok(())
}

fn handle_mouse(app: &mut App, mouse: MouseEvent) -> Result<()> {
    match app.state {
        State::EditEntry {
            profile_index,
            entry_index,
        } => edit_entry::mouse::handle(app, mouse, profile_index, entry_index),

        State::EditProfile { profile_index } => {
            edit_profile::mouse::handle(app, mouse, profile_index)
        }

        State::EntryDetail {
            profile_index,
            entry_index,
        } => entry_detail::mouse::handle(app, mouse, profile_index, entry_index),

        State::EntryList { profile_index } => entry_list::mouse::handle(app, mouse, profile_index),

        State::ProfileList => profile_list::mouse::handle(app, mouse),

        State::Unlock => unlock::mouse::handle(app, mouse),
    }
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
        } => edit_entry::key::handle(app, key, profile_index, entry_index),

        State::EditProfile { profile_index } => edit_profile::key::handle(app, key, profile_index),

        State::EntryDetail {
            profile_index,
            entry_index,
        } => entry_detail::key::handle(app, key, profile_index, entry_index),

        State::EntryList { profile_index } => entry_list::key::handle(app, key, profile_index),

        State::ProfileList => profile_list::key::handle(app, key),

        State::Unlock => unlock::key::handle(app, key),
    }
}
