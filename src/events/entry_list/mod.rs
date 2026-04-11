use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, entry::Entry, state::State},
    errors::Result,
};

pub fn handle(app: &mut App, key: KeyEvent, profile_index: usize) -> Result<()> {
    let count = app.current_entry_count();

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app.selected = (app.selected + 1).min(count - 1);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.selected = app.selected.saturating_sub(1);
        }
        KeyCode::Char('g') => {
            app.selected = 0;
        }
        KeyCode::Char('G') => {
            app.selected = count - 1;
        }
        KeyCode::Esc | KeyCode::Char('h') | KeyCode::Left => {
            app.state = State::ProfileList;
            app.selected = profile_index;
        }
        KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
            app.state = State::EntryDetail {
                profile_index,
                entry_index: app.selected,
            };
            app.detail_selected = 0;
        }
        KeyCode::Char('n') => {
            app.entry_form = Some(Entry::new());
            app.state = State::EditEntry {
                profile_index,
                entry_index: None,
            };
        }
        KeyCode::Char('e') | KeyCode::Char('r') => {
            let form = app
                .vault
                .as_ref()
                .and_then(|v| v.profiles.get(profile_index))
                .and_then(|p| p.entries.get(app.selected))
                .map(|e| Entry::from(e));
            if let Some(f) = form {
                app.entry_form = Some(f);
                app.state = State::EditEntry {
                    profile_index,
                    entry_index: Some(app.selected),
                };
            }
        }
        KeyCode::Char('d') => {
            if let Some(vault) = &mut app.vault {
                if let Some(profile) = vault.profiles.get_mut(profile_index) {
                    profile.entries.remove(app.selected);
                    app.dirty = true;
                    if app.selected > 0 && app.selected >= profile.entries.len() {
                        app.selected -= 1;
                    }
                    app.ntfy_info("Entry deleted");
                }
            }
        }
        _ => {}
    }

    Ok(())
}
