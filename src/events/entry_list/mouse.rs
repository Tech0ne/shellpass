use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::{app::{App, entry::Entry, state::State}, errors::Result, ui::utils::{clicked_hint, clicked_list_row}};

pub fn handle(app: &mut App, mouse: MouseEvent, profile_index: usize) -> Result<()> {
    let col = mouse.column;
    let row = mouse.row;

    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            let double = app.is_double_click(col, row);
            let body = app.layout.list_body;
            let count = app.current_entry_count();

            if let Some(idx) = clicked_list_row(col, row, body) {
                let idx = idx.min(count.saturating_sub(1));
                if count > 0 {
                    app.selected = idx;
                    if double {
                        let form = app
                            .vault
                            .as_ref()
                            .and_then(|v| v.profiles.get(profile_index))
                            .and_then(|p| p.entries.get(idx))
                            .map(|e| Entry::from(e));
                        if let Some(f) = form {
                            app.entry_form = Some(f);
                            app.state = State::EntryDetail {
                                profile_index,
                                entry_index: idx,
                            };
                        }
                    }
                }
                return Ok(());
            }

            if double {
                let hints = app.layout.footer_hints.clone();
                match clicked_hint(col, row, &hints) {
                    Some(0) => {
                        app.entry_form = Some(Entry::new());
                        app.state = State::EditEntry {
                            profile_index,
                            entry_index: None,
                        };
                    }
                    Some(1) => {
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
                    Some(2) => {
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
                    Some(3) => {
                        let form = app
                            .vault
                            .as_ref()
                            .and_then(|v| v.profiles.get(profile_index))
                            .and_then(|p| p.entries.get(app.selected))
                            .map(|e| Entry::from(e));
                        if let Some(f) = form {
                            app.entry_form = Some(f);
                            app.state = State::EntryDetail {
                                profile_index,
                                entry_index: app.selected,
                            };
                        }
                    }
                    Some(4) => {
                        app.state = State::ProfileList;
                        app.selected = profile_index;
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
