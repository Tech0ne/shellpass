use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, clip_timer::ClipTimer, edit_mode::Mode, entry::FocusedField, state::State},
    clipboard::copy_to_clipboard,
    errors::Result,
};

pub fn copy_field(app: &mut App, profile_index: usize, entry_index: usize, field: FocusedField) {
    let entry = match app
        .vault
        .as_ref()
        .and_then(|v| v.profiles.get(profile_index))
        .and_then(|p| p.entries.get(entry_index))
    {
        Some(e) => e.clone(),
        None => return,
    };

    let (value, label) = match field {
        FocusedField::Username => (entry.username.clone(), "Username"),
        FocusedField::Password => (entry.password.clone(), "Password"),
        FocusedField::Website => (entry.website.clone(), "Website"),
        FocusedField::RawData => (entry.raw_data.clone(), "Custom Data"),
    };

    if value.is_empty() {
        app.ntfy_info("Field is empty");
        return;
    }

    if let Some(clipboard) = &mut app.clipboard {
        if let Ok(_) = copy_to_clipboard(clipboard, &value) {
            if field == FocusedField::Password {
                app.clip_timer = Some(ClipTimer::new(label));
                app.ntfy_info("Password copied - clears in 10s");
            } else {
                app.ntfy_info(format!("{} copied to clipboard", label));
            }
        } else {
            app.ntfy_error("Failed to write to clipboad");
        }
    } else {
        app.ntfy_error("Failed to handle clipboard. Missing display server, maybe ?");
    }
}

pub fn handle(
    app: &mut App,
    key: KeyEvent,
    profile_index: usize,
    entry_index: usize,
) -> Result<()> {
    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            if let Some(f) = &mut app.entry_form {
                f.focused_field = match f.focused_field {
                    FocusedField::Username => FocusedField::Password,
                    FocusedField::Password => FocusedField::Website,
                    FocusedField::Website => FocusedField::RawData,
                    FocusedField::RawData => FocusedField::RawData,
                }
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if let Some(f) = &mut app.entry_form {
                f.focused_field = match f.focused_field {
                    FocusedField::Username => FocusedField::Username,
                    FocusedField::Password => FocusedField::Username,
                    FocusedField::Website => FocusedField::Password,
                    FocusedField::RawData => FocusedField::Website,
                }
            }
        }
        KeyCode::Char('g') => {
            if let Some(f) = &mut app.entry_form {
                f.focused_field = FocusedField::Username;
            }
        }
        KeyCode::Char('G') => {
            if let Some(f) = &mut app.entry_form {
                f.focused_field = FocusedField::RawData;
            }
        }
        KeyCode::Char('i')
        | KeyCode::Char('I')
        | KeyCode::Char('o')
        | KeyCode::Char('O')
        | KeyCode::Char('a')
        | KeyCode::Char('A')
        | KeyCode::Char('e')
        | KeyCode::Char('E')
        | KeyCode::Char('r')
        | KeyCode::Char('R') => {
            app.mode = Mode::Insert;
            app.state = State::EditEntry {
                profile_index,
                entry_index: Some(entry_index),
            };
        }
        KeyCode::Esc | KeyCode::Char('h') | KeyCode::Left => {
            app.entry_form = None;
            app.selected = entry_index;
            app.state = State::EntryList { profile_index };
        }
        KeyCode::Enter => {
            if let Some(f) = &app.entry_form {
                copy_field(app, profile_index, entry_index, f.focused_field);
            }
        }
        _ => {}
    }

    Ok(())
}
