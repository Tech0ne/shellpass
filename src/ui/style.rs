use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
};

pub const BG: Color = Color::Rgb(10, 12, 18);
pub const BG_PANEL: Color = Color::Rgb(16, 20, 30);
pub const BG_SEL: Color = Color::Rgb(30, 38, 58);
pub const ACCENT: Color = Color::Rgb(255, 185, 80);
pub const ACCENT2: Color = Color::Rgb(90, 200, 255);
pub const DIM: Color = Color::Rgb(70, 80, 100);
pub const TEXT: Color = Color::Rgb(210, 215, 230);
pub const MUTED: Color = Color::Rgb(110, 120, 145);
pub const SUCCESS: Color = Color::Rgb(80, 220, 140);
pub const DANGER: Color = Color::Rgb(255, 85, 85);
pub const BORDER: Color = Color::Rgb(40, 50, 75);

pub fn panel(title: &str) -> Block<'_> {
    Block::default()
        .title(Span::styled(
            format!(" {} ", title),
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER))
        .style(Style::default().fg(BG_PANEL))
}

pub fn panel_focused(title: &str) -> Block<'_> {
    Block::default()
        .title(Span::styled(
            format!(" {} ", title),
            Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(ACCENT2))
        .style(Style::default().bg(BG_PANEL))
}
