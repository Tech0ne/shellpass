use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::ui::style::{ACCENT, BG, BG_PANEL, BORDER, DIM, MUTED, TEXT};

pub fn three_rows(area: Rect) -> [Rect; 3] {
    let (body_min, footer_len) = if area.height >= 8 { (0, 3) } else { (0, 0) };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(body_min),
            Constraint::Length(footer_len),
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
            Span::styled(
                "  ◈ SHELLPASS  ",
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::styled("/ ", Style::default().fg(DIM)),
            Span::styled(
                title.to_string(),
                Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
            ),
        ]))
        .style(Style::default().bg(BG_PANEL))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(BORDER)),
        ),
        left,
    );
    let sec = section.unwrap_or("");
    frame.render_widget(
        Paragraph::new(format!("  {}  ", sec))
            .style(Style::default().fg(MUTED))
            .alignment(Alignment::Right)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default().fg(BORDER)),
            ),
        right,
    );
}

pub fn render_footer(frame: &mut Frame, hints: &[(&str, &str)], area: Rect) {
    let spans: Vec<Span> = hints
        .iter()
        .flat_map(|(k, d)| {
            vec![
                Span::styled(
                    format!(" {} ", k),
                    Style::default()
                        .fg(BG)
                        .bg(ACCENT)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!(" {}  ", d), Style::default().fg(MUTED)),
            ]
        })
        .collect();
    frame.render_widget(
        Paragraph::new(Line::from(spans))
            .style(Style::default().bg(BG_PANEL))
            .block(
                Block::default()
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(BORDER)),
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
