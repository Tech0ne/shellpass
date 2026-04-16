use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, state::State, edit_mode::Mode},
    errors::Result,
    vault::vault_data::VaultData,
};

pub fn handle(app: &mut App, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Enter => {
            if app.is_new_vault {
                app.vault = Some(VaultData::new());
                app.state = State::ProfileList;
                app.mode = Mode::Insert;
                app.ntfy_info("New vault created !");
            } else {
                let path = app.vault_path.clone();
                match VaultData::load(&path, &app.vault_pass) {
                    Ok(vault) => {
                        app.vault = Some(vault);
                        app.state = State::ProfileList;
                        app.mode = Mode::Insert;
                        app.ntfy_info("Vault unlocked");
                    }
                    Err(e) => {
                        app.ntfy_error(e.to_string());
                        app.vault_pass.clear();
                    }
                }
            }
        }
        KeyCode::Char(c) => {
            app.vault_pass.push(c);
        }
        KeyCode::Backspace => {
            app.vault_pass.pop();
        }
        KeyCode::Tab => {
            app.show_input = !app.show_input;
        }
        _ => {}
    }

    Ok(())
}
