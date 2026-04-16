mod clip_timer;
mod edit_entry;
mod edit_profile;
mod entry_detail;
mod entry_list;
mod notification;
mod profile_list;
mod style;
mod unlock;
pub mod utils;

use ratatui::{Frame, style::Style, widgets::Block};

use crate::{
    app::{App, state::State},
    ui::style::BG,
};

pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    frame.render_widget(Block::default().style(Style::default().bg(BG)), area);

    match &app.state {
        State::Unlock => unlock::render(frame, app, area),
        State::ProfileList => profile_list::render(frame, app, area),
        State::EntryList { profile_index } => entry_list::render(frame, app, area, *profile_index),
        State::EntryDetail {
            profile_index,
            entry_index,
        } => entry_detail::render(frame, app, area, *profile_index, *entry_index),
        State::EditEntry {
            profile_index,
            entry_index,
        } => edit_entry::render(frame, app, area, *profile_index, *entry_index),
        State::EditProfile { profile_index } => {
            edit_profile::render(frame, app, area, *profile_index)
        }
    }

    clip_timer::render(frame, app, area);
    notification::render(frame, app, area);
}
