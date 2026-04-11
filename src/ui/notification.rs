use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::{app::App, ui::style::BG_PANEL};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let n = match &app.notification {
        Some(n) => n,
        None => return,
    };

    let msg = &n.message;
    let w = (msg.len() as u16 + 6).min(area.width.saturating_sub(4));
    let popup = Rect {
        x: (area.width.saturating_sub(w)) / 2,
        y: area.height.saturating_sub(4),
        width: w,
        height: 3,
    };
    frame.render_widget(Clear, popup);
    let (bc, tc) = if n.error {
        (Color::Rgb(255, 85, 85), Color::Rgb(255, 85, 85))
    } else {
        (Color::Rgb(80, 220, 140), Color::Rgb(80, 220, 140))
    };
    let prefix = if n.error { "✗ " } else { "✓ " };
    frame.render_widget(
        Paragraph::new(format!("{}{}", prefix, msg))
            .style(Style::default().fg(tc))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(bc))
                    .style(Style::default().bg(BG_PANEL)),
            ),
        popup,
    );
}
