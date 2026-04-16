use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, state::State},
    errors::Result,
    vault::vault_data::profile::Profile,
};

pub fn handle(app: &mut App, key: KeyEvent, profile_index: Option<usize>) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            let name = app.profile_name_input.trim().to_string();
            if name.is_empty() {
                app.ntfy_error("Name cannot be empty");
                return Ok(());
            }
            if let Some(vault) = &mut app.vault {
                if vault.profiles.iter().any(|p| p.name == name) {
                    app.ntfy_error(format!("A profile named \"{name}\" already exists !"));
                    return Ok(());
                }
                if let Some(index) = profile_index {
                    if let Some(p) = vault.profiles.get_mut(index) {
                        p.name = name;
                    }
                    app.ntfy_info("Profile renamed");
                } else {
                    vault.profiles.push(Profile::new(name));
                    app.selected = vault.profiles.len() - 1;
                    app.ntfy_info("Profile created");
                }
                app.dirty = true;
            }
            app.state = State::ProfileList;
        }
        KeyCode::Char(c) => {
            app.profile_name_input.push(c);
        }
        KeyCode::Backspace => {
            app.profile_name_input.pop();
        }
        KeyCode::Esc => {
            app.profile_name_input.clear();
            app.state = State::ProfileList;
        }
        _ => {}
    }

    Ok(())
}
