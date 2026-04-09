use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders},
};

const BG: Color = Color::Rgb(10, 12, 18);
const BG_PANEL: Color = Color::Rgb(16, 20, 30);
const BG_SEL: Color = Color::Rgb(30, 38, 58);
const ACCENT: Color = Color::Rgb(255, 185, 80);
const ACCENT2: Color = Color::Rgb(90, 200, 255);
const DIM: Color = Color::Rgb(70, 80, 100);
const TEXT: Color = Color::Rgb(210, 215, 230);
const MUTED: Color = Color::Rgb(110, 120, 145);
const SUCCESS: Color = Color::Rgb(80, 220, 140);
const DANGER: Color = Color::Rgb(255, 85, 85);
const BORDER: Color = Color::Rgb(40, 50, 75);

pub fn s_bg() -> Style {
    Style::default().bg(BG)
}

pub fn s_dim() -> Style {
    Style::default().fg(DIM)
}

pub fn s_panel() -> Style {
    Style::default().bg(BG_PANEL)
}

pub fn s_border() -> Style {
    Style::default().fg(BORDER)
}

pub fn s_bg_accent() -> Style {
    Style::default()
        .fg(BG)
        .bg(ACCENT)
        .add_modifier(Modifier::BOLD)
}

pub fn s_accent() -> Style {
    Style::default().fg(ACCENT)
}

pub fn s_text() -> Style {
    Style::default().fg(TEXT)
}

pub fn s_muted() -> Style {
    Style::default().fg(MUTED)
}

pub fn s_sel() -> Style {
    Style::default()
        .fg(ACCENT)
        .bg(BG_SEL)
        .add_modifier(Modifier::BOLD)
}

pub fn s_label() -> Style {
    Style::default().fg(MUTED)
}

pub fn panel(title: &str) -> Block<'_> {
    Block::default()
        .title(Span::styled(
            format!(" {} ", title),
            s_accent().add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER))
        .style(Style::default().bg(BG_PANEL))
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
