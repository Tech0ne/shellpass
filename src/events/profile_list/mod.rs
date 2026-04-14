use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, state::State},
    errors::Result,
};

pub fn handle(app: &mut App, key: KeyEvent) -> Result<()> {
    let count = app.current_profile_count();

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            if count > 0 {
                app.selected = (app.selected + 1).min(count - 1);
            }
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
        KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
            if count > 0 {
                let index = app.selected;
                app.state = State::EntryList {
                    profile_index: index,
                };
                app.selected = 0;
                app.scroll = 0;
            }
        }
        KeyCode::Char('n') => {
            app.profile_name_input.clear();
            app.state = State::EditProfile {
                profile_index: None,
            };
        }
        KeyCode::Char('r') => {
            let name = app
                .vault
                .as_ref()
                .and_then(|v| v.profiles.get(app.selected))
                .map(|p| p.name.clone())
                .unwrap_or_default();
            app.profile_name_input = name;
            let index = app.selected;
            app.state = State::EditProfile {
                profile_index: Some(index),
            };
        }
        KeyCode::Char('d') => {
            let index = app.selected;
            if let Some(vault) = &mut app.vault {
                vault.profiles.remove(index);
                app.dirty = true;
                if app.selected > 0 && app.selected >= vault.profiles.len() {
                    app.selected -= 1;
                }
                app.ntfy_info("Profile deleted");
            }
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Some(vault) = &app.vault {
                vault.save(&app.vault_path, &app.vault_pass)?;
                app.ntfy_info("Saved vault successfully");
                app.dirty = false;
            }
        }
        KeyCode::Char('x') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Some(vault) = &app.vault {
                vault.save(&app.vault_path, &app.vault_pass)?;
                app.dirty = false;
                app.quit = true;
            }
        }
        _ => {}
    }

    Ok(())
}
