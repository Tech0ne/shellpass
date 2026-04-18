use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::{
    app::{App, edit_mode::Mode, state::State},
    errors::Result,
    ui::utils::clicked_hint,
    vault::vault_data::VaultData,
};

pub fn handle(app: &mut App, mouse: MouseEvent) -> Result<()> {
    let col = mouse.column;
    let row = mouse.row;

    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            let double = app.is_double_click(col, row);
            if !double {
                return Ok(());
            }

            let hints = app.layout.footer_hints.clone();

            match clicked_hint(col, row, &hints) {
                Some(0) => {
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
                Some(1) => {
                    app.show_input = !app.show_input;
                }
                Some(2) => {
                    app.quit = true;
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
