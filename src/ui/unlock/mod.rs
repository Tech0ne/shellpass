use ratatui::{Frame, layout::Rect, widgets::Clear};

use crate::{app::App, ui::utils::centered_rect};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let popup = centered_rect(52, 20, area);
    frame.render_widget(Clear, popup);
}
