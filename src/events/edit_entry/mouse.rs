use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

use crate::{
    app::{App, edit_mode::Mode, entry::FocusedField},
    errors::Result,
    ui::utils::clicked_list_row,
};

pub fn handle(
    app: &mut App,
    mouse: MouseEvent,
    _profile_index: usize,
    _entry_index: Option<usize>,
) -> Result<()> {
    let col = mouse.column;
    let row = mouse.row;

    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            app.is_double_click(col, row);
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
                app.mode = Mode::Insert;
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
                    app.mode = Mode::Insert;
                }
            }
        }
        _ => {}
    }
    Ok(())
}
