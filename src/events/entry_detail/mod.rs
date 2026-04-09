use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, clip_timer::ClipTimer, state::State},
    clipboard::copy_to_clipboard,
    errors::Result,
};

fn copy_field(app: &mut App, profile_index: usize, entry_index: usize, field: usize) {
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
        0 => (entry.username.clone(), "Username"),
        1 => (entry.password.clone(), "Password"),
        2 => (entry.website.clone(), "Website"),
        n => match entry.custom_fields.get(n - 3) {
            Some(f) => (f.val.clone(), f.key.as_str()),
            None => return,
        },
    };

    if value.is_empty() {
        app.ntfy_info("Field is empty");
        return;
    }

    match arboard::Clipboard::new() {
        Ok(mut clipboard) => {
            if let Ok(_) = copy_to_clipboard(&mut clipboard, &value) {
                if field == 1 {
                    app.clip_timer = Some(ClipTimer::new(label));
                    app.ntfy_info("Password copied - clears in 10s");
                } else {
                    app.ntfy_info(format!("{} copied to clipboard", label));
                }
            } else {
                app.ntfy_error("Failed to write to clipboad");
            }
        }
        Err(_) => {
            app.ntfy_error("Clipboard unavailable");
        }
    }
}

pub fn handle(
    app: &mut App,
    key: KeyEvent,
    profile_index: usize,
    entry_index: usize,
) -> Result<()> {
    let field_count = app
        .vault
        .as_ref()
        .and_then(|v| v.profiles.get(profile_index))
        .and_then(|p| p.entries.get(entry_index))
        .map(|e| 3 + e.custom_fields.len())
        .unwrap_or(3);

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app.selected = (app.selected + 1).min(field_count - 1);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.selected += field_count;
            app.selected -= 1;
            app.selected %= field_count;
        }
        KeyCode::Char('g') => {
            app.selected = 0;
        }
        KeyCode::Char('G') => {
            app.selected = field_count - 1;
        }
        KeyCode::Esc | KeyCode::Char('h') | KeyCode::Left => {
            app.state = State::EntryList { profile_index };
        }
        KeyCode::Enter => {}
        _ => {}
    }

    Ok(())
}
