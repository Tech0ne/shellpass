use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::{app::{App, state::State}, errors::Result, ui::utils::{clicked_hint, clicked_list_row}};

pub fn handle(app: &mut App, mouse: MouseEvent) -> Result<()> {
    let col = mouse.column;
    let row = mouse.row;

    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            let double = app.is_double_click(col, row);
            let body = app.layout.list_body;
            let count = app.current_profile_count();

            if let Some(idx) = clicked_list_row(col, row, body) {
                let idx = idx.min(count.saturating_sub(1));
                if count > 0 {
                    app.selected = idx;
                    if double {
                        // double-click = open
                        app.state = State::EntryList { profile_index: idx };
                        app.selected = 0;
                        app.scroll = 0;
                    }
                }
                return Ok(());
            }

            if double {
                let hints = app.layout.footer_hints.clone();
                match clicked_hint(col, row, &hints) {
                    Some(0) => {
                        app.profile_name_input.clear();
                        app.state = State::EditProfile {
                            profile_index: None,
                        };
                    }
                    Some(1) => {
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
                    Some(2) => {
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
                    Some(3) => {
                        if count > 0 {
                            let index = app.selected;
                            app.state = State::EntryList {
                                profile_index: index,
                            };
                            app.selected = 0;
                            app.scroll = 0;
                        }
                    }
                    Some(4) => {
                        if let Some(vault) = &app.vault {
                            vault.save(&app.vault_path, &app.vault_pass)?;
                            app.ntfy_info("Saved vault successfully");
                            app.dirty = false;
                        }
                    }
                    Some(5) => {
                        app.quit = true;
                    }
                    Some(6) => {
                        if let Some(vault) = &app.vault {
                            vault.save(&app.vault_path, &app.vault_pass)?;
                            app.dirty = false;
                            app.quit = true;
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    Ok(())
}
