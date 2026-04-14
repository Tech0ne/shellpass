use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Clear, Gauge, Paragraph},
};

use crate::{
    app::App,
    ui::{
        style::{ACCENT2, BG, BG_PANEL, MUTED},
        utils::three_equal,
    },
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let timer = match &app.clip_timer {
        Some(t) => t,
        None => return,
    };

    let w = 34u16;
    let popup = Rect {
        x: area.width.saturating_sub(w + 1),
        y: 1,
        width: w,
        height: 5,
    };

    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(Span::styled(
            " 🔒 Clipboard ",
            Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(ACCENT2))
        .style(Style::default().bg(BG_PANEL));
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let [row0, row1, _] = three_equal(inner);
    let secs = timer.remaining_secs();
    frame.render_widget(
        Paragraph::new(format!("  Clears in {}s", secs)).style(Style::default().fg(MUTED)),
        row0,
    );
    let frac = timer.remaining_frac();
    let color = if frac > 0.5 {
        Color::Rgb(255, 185, 80)
    } else if frac > 0.2 {
        Color::Rgb(255, 140, 0)
    } else {
        Color::Rgb(255, 85, 85)
    };
    frame.render_widget(
        Gauge::default()
            .gauge_style(Style::default().fg(color).bg(BG))
            .ratio(frac),
        row1,
    );
}
