use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, state::State, vim_mode::Mode},
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
                match VaultData::load(&path, &app.password_input) {
                    Ok(vault) => {
                        app.vault = Some(vault);
                        app.state = State::ProfileList;
                        app.mode = Mode::Insert;
                        app.ntfy_info("Vault unlocked");
                    }
                    Err(e) => {
                        app.ntfy_error(e.to_string());
                        app.password_input.clear();
                    }
                }
            }
        }
        KeyCode::Char(c) => {
            app.password_input.push(c);
        }
        KeyCode::Backspace => {
            app.password_input.pop();
        }
        KeyCode::Tab => {
            app.show_input = !app.show_input;
        }
        _ => {}
    }

    Ok(())
}
