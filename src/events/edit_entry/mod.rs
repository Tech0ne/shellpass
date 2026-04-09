use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, state::State, vim_mode::Mode},
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
                    if let Some(v) = f.active_value_mut() {
                        v.push(c);
                    }
                }
            }
            KeyCode::Backspace => {
                if let Some(f) = &mut app.entry_form {
                    if let Some(v) = f.active_value_mut() {
                        v.pop();
                    }
                }
            }
            KeyCode::Tab | KeyCode::Enter => {
                if let Some(f) = &mut app.entry_form {
                    let max = f.field_count() - 1;
                    f.focused_field = (f.focused_field + 1).min(max);
                }
            }
            _ => {}
        }
        return Ok(());
    }

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            if let Some(f) = &mut app.entry_form {
                let max = f.field_count() - 1;
                f.focused_field = (f.focused_field + 1).min(max);
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if let Some(f) = &mut app.entry_form {
                if f.focused_field > 0 {
                    f.focused_field -= 1;
                }
            }
        }
        KeyCode::Char('i') | KeyCode::Char('I') | KeyCode::Char('a') | KeyCode::Char('A') => {
            app.mode = Mode::Insert;
        }
        KeyCode::Char('o') => {
            if let Some(f) = &mut app.entry_form {
                f.custom_fields.push((String::new(), String::new()));
                let new_pos = f.field_count() - 3;
                f.focused_field = new_pos;
            }
            app.mode = Mode::Insert;
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if let Some(f) = &mut app.entry_form {
                if f.focused_field >= 3 {
                    let field_index = (f.focused_field - 3) / 2;
                    if field_index < f.custom_fields.len() {
                        f.custom_fields.remove(field_index);
                        if f.focused_field > 0 {
                            f.focused_field -= 1;
                        }
                    }
                }
            }
        }
        KeyCode::Char('u') => {
            if let Some(f) = &mut app.entry_form {
                if let Some(v) = f.active_value_mut() {
                    v.clear();
                }
            }
        }
        KeyCode::Esc => {
            app.entry_form = None;
            app.state = match entry_index {
                Some(index) => State::EntryDetail {
                    profile_index,
                    entry_index: index,
                },
                None => State::EntryList { profile_index },
            };
            app.mode = Mode::Normal;
        }
        _ => {}
    }

    Ok(())
}
