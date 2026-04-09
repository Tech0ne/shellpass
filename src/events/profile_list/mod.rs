use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{App, state::State},
    errors::Result,
};

fn take_count(app: &mut App) -> usize {
    let n: usize = app.count_buf.parse().unwrap_or(1).max(1);
    app.count_buf.clear();
    n
}

fn feed_digit(app: &mut App, c: char) -> bool {
    if c.is_ascii_digit() {
        app.count_buf.push(c);
        true
    } else {
        false
    }
}

pub fn handle(app: &mut App, key: KeyEvent) -> Result<()> {
    let count = app.current_profile_count();

    match key.code {
        KeyCode::Char('j') | KeyCode::Down => {
            app.selected = (app.selected + 1).min(count - 1);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.selected += count;
            app.selected -= 1;
            app.selected %= count;
        }
        KeyCode::Char('g') => {
            app.selected = 0;
        }
        KeyCode::Char('G') => {
            app.selected = count - 1;
        }
        KeyCode::Enter | KeyCode::Char('l') | KeyCode::Right => {
            let index = app.selected;
            app.state = State::EntryList {
                profile_index: index,
            };
            app.selected = 0;
            app.scroll = 0;
        }
        KeyCode::Char('n') => {
            app.profile_name_input.clear();
            app.state = State::EditProfile {
                profile_index: None,
            };
        }
        KeyCode::Char('r') | KeyCode::Char('e') => {
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
        _ => {}
    }

    Ok(())
}
