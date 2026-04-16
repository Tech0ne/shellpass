use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    app::{App, edit_mode::Mode, entry::FocusedField, state::State},
    errors::Result,
};

pub fn handle(
    app: &mut App,
    key: KeyEvent,
    profile_index: usize,
    entry_index: Option<usize>,
) -> Result<()> {
    if app.mode == Mode::Insert {
        match key.code {
            KeyCode::Esc => {
                app.mode = Mode::Normal;
            }
            KeyCode::Char(c) => {
                if let Some(f) = &mut app.entry_form {
                    f.active_value_mut().push(c);
                }
            }
            KeyCode::Backspace => {
                if let Some(f) = &mut app.entry_form {
                    f.active_value_mut().pop();
                }
            }
            KeyCode::Tab | KeyCode::Enter => {
                if let Some(f) = &mut app.entry_form {
                    f.focused_field = match f.focused_field {
                        FocusedField::Username => FocusedField::Password,
                        FocusedField::Password => FocusedField::Website,
                        FocusedField::Website => FocusedField::RawData,
                        FocusedField::RawData => {
                            f.raw_data
                                .push(if key.code == KeyCode::Tab { '\t' } else { '\n' });
                            FocusedField::RawData
                        }
                    }
                }
            }
            _ => {}
        }
        return Ok(());
    }

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            if let Some(f) = &mut app.entry_form {
                f.focused_field = match f.focused_field {
                    FocusedField::Username => FocusedField::Password,
                    FocusedField::Password => FocusedField::Website,
                    FocusedField::Website => FocusedField::RawData,
                    FocusedField::RawData => FocusedField::RawData,
                }
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if let Some(f) = &mut app.entry_form {
                f.focused_field = match f.focused_field {
                    FocusedField::Username => FocusedField::Username,
                    FocusedField::Password => FocusedField::Username,
                    FocusedField::Website => FocusedField::Password,
                    FocusedField::RawData => FocusedField::Website,
                }
            }
        }
        KeyCode::Char('g') => {
            if let Some(f) = &mut app.entry_form {
                f.focused_field = FocusedField::Username;
            }
        }
        KeyCode::Char('G') => {
            if let Some(f) = &mut app.entry_form {
                f.focused_field = FocusedField::RawData;
            }
        }
        KeyCode::Char('i') | KeyCode::Char('I') | KeyCode::Char('a') | KeyCode::Char('A') => {
            app.mode = Mode::Insert;
        }
        KeyCode::Char('u') => {
            if let Some(f) = &mut app.entry_form {
                f.active_value_mut().clear();
            }
        }
        KeyCode::Esc => {
            app.mode = Mode::Normal;
            match entry_index {
                Some(index) => {
                    let form = app
                        .vault
                        .as_ref()
                        .and_then(|v| v.profiles.get(profile_index))
                        .and_then(|p| p.entries.get(index))
                        .map(|e| crate::app::entry::Entry::from(e));
                    app.entry_form = form;
                    app.state = State::EntryDetail {
                        profile_index,
                        entry_index: index,
                    };
                }
                None => {
                    app.entry_form = None;
                    app.state = State::EntryList { profile_index };
                }
            }
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Some(vault) = &mut app.vault {
                if let Some(profile) = vault.profiles.get_mut(profile_index) {
                    if let Some(form) = &app.entry_form {
                        match entry_index {
                            Some(index) => {
                                if let Some(existing) = profile.entries.get_mut(index) {
                                    existing.username = form.username.clone();
                                    existing.password = form.password.clone();
                                    existing.website = form.website.clone();
                                    existing.raw_data = form.raw_data.clone();
                                }
                            }
                            None => {
                                profile.entries.push(form.clone().into());
                            }
                        }
                        app.dirty = true;
                        app.ntfy_info("Entry saved");
                        app.state = match entry_index {
                            Some(index) => State::EntryDetail {
                                profile_index,
                                entry_index: index,
                            },
                            None => State::EntryList { profile_index },
                        };
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}
