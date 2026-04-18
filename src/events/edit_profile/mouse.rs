use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::{app::{App, state::State}, errors::Result, ui::utils::clicked_hint, vault::vault_data::profile::Profile};

pub fn handle(app: &mut App, mouse: MouseEvent, profile_index: Option<usize>) -> Result<()> {
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
                Some(1) => {
                    app.profile_name_input.clear();
                    app.state = State::ProfileList;
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
