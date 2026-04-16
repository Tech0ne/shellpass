use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::{
    app::{App, edit_mode::Mode, entry::FocusedField, state::State},
    errors::Result, ui::utils::{clicked_hint, clicked_list_row},
};

pub fn handle(
    app: &mut App,
    mouse: MouseEvent,
    profile_index: usize,
    entry_index: usize,
) -> Result<()> {
    let col = mouse.column;
    let row = mouse.row;

    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            let double = app.is_double_click(col, row);
            let body = app.layout.list_body;
            let raw_area = app.layout.raw_area;

            if col >= raw_area.x
                && col < raw_area.x + raw_area.width
                && row >= raw_area.y
                && row < raw_area.y + raw_area.height
            {
                if let Some(f) = &mut app.entry_form {
                    f.focused_field = FocusedField::RawData;
                }
                if double {
                    super::key::copy_field(
                        app,
                        profile_index,
                        entry_index,
                        FocusedField::RawData,
                    );
                }
                return Ok(());
            }

            if let Some(idx) = clicked_list_row(col, row, body) {
                let field = match idx {
                    0 => Some(FocusedField::Username),
                    1 => Some(FocusedField::Password),
                    2 => Some(FocusedField::Website),
                    _ => None,
                };
                if let Some(field) = field {
                    if let Some(f) = &mut app.entry_form {
                        f.focused_field = field;
                    }
                    if double {
                        super::key::copy_field(app, profile_index, entry_index, field);
                    }
                }
                return Ok(());
            }

            if double {
                let hints = app.layout.footer_hints.clone();
                match clicked_hint(col, row, &hints) {
                    Some(1) => {
                        if let Some(f) = &app.entry_form {
                            super::key::copy_field(
                                app,
                                profile_index,
                                entry_index,
                                f.focused_field,
                            );
                        }
                    }
                    Some(2) => {
                        app.mode = Mode::Insert;
                        app.state = State::EditEntry {
                            profile_index,
                            entry_index: Some(entry_index),
                        };
                    }
                    Some(4) => {
                        app.entry_form = None;
                        app.selected = entry_index;
                        app.state = State::EntryList { profile_index };
                    }
                    Some(5) => {
                        app.quit = true;
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    Ok(())
}
