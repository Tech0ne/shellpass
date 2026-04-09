use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::style::{s_accent, s_bg_accent, s_border, s_dim, s_muted, s_panel, s_text};

pub fn three_rows(area: Rect) -> [Rect; 3] {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);
    [chunks[0], chunks[1], chunks[2]]
}

pub fn three_equal(area: Rect) -> [Rect; 3] {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(area);
    [chunks[0], chunks[1], chunks[2]]
}

pub fn render_header(frame: &mut Frame, title: &str, section: Option<&str>, area: Rect) {
    let [left, right] = two_cols(area);
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("  ◈ VAULTERM  ", s_accent().add_modifier(Modifier::BOLD)),
            Span::styled("/ ", s_dim()),
            Span::styled(title.to_string(), s_text().add_modifier(Modifier::BOLD)),
        ]))
        .style(s_panel())
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(s_border()),
        ),
        left,
    );
    let sec = section.unwrap_or("");
    frame.render_widget(
        Paragraph::new(format!("  {}  ", sec))
            .style(s_muted())
            .alignment(Alignment::Right)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(s_border()),
            ),
        right,
    );
}

pub fn render_footer(frame: &mut Frame, hints: &[(&str, &str)], area: Rect) {
    let spans: Vec<Span> = hints
        .iter()
        .flat_map(|(k, d)| {
            vec![
                Span::styled(format!(" {} ", k), s_bg_accent()),
                Span::styled(format!(" {}  ", d), s_muted()),
            ]
        })
        .collect();
    frame.render_widget(
        Paragraph::new(Line::from(spans)).style(s_panel()).block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(s_border()),
        ),
        area,
    );
}

fn two_cols(area: Rect) -> [Rect; 2] {
    let c = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(20)])
        .split(area);
    [c[0], c[1]]
}

pub fn centered_rect(pw: u16, ph: u16, r: Rect) -> Rect {
    let vert = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - ph) / 2),
            Constraint::Percentage(ph),
            Constraint::Percentage((100 - ph) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - pw) / 2),
            Constraint::Percentage(pw),
            Constraint::Percentage((100 - pw) / 2),
        ])
        .split(vert[1])[1]
}
