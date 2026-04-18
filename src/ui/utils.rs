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

pub fn render_footer(frame: &mut Frame, hints: &[(&str, &str)], area: Rect) -> Vec<Rect> {
    let mut badge_rects: Vec<Rect> = Vec::new();
    let mut x = area.x;
    let text_y = area.y + 1;

    let spans: Vec<Span> = hints
        .iter()
        .flat_map(|(k, d)| {
            let badge = format!(" {} ", k);
            let desc = format!(" {}  ", d);
            let badge_w = badge.chars().count() as u16;
            let desc_w = desc.chars().count() as u16;

            if text_y < area.y + area.height {
                badge_rects.push(Rect::new(x, text_y, badge_w, 1));
            }
            x += badge_w + desc_w;

            vec![
                Span::styled(
                    badge,
                    Style::default()
                        .fg(BG)
                        .bg(ACCENT)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(desc, Style::default().fg(MUTED)),
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

    badge_rects
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

pub fn clicked_list_row(mouse_col: u16, mouse_row: u16, body: Rect) -> Option<usize> {
    if mouse_col < body.x || mouse_col >= body.x + body.width {
        return None;
    }
    let inner_y = body.y + 1;
    let inner_h = body.height.saturating_sub(2);
    if mouse_row < inner_y || mouse_row >= inner_y + inner_h {
        return None;
    }
    Some((mouse_row - inner_y) as usize)
}

pub fn clicked_hint(mouse_col: u16, mouse_row: u16, hint_rects: &[Rect]) -> Option<usize> {
    hint_rects.iter().position(|r| {
        mouse_col >= r.x && mouse_col < r.x + r.width
            && mouse_row >= r.y && mouse_row < r.y + r.height
    })
}

pub fn popup_hint_rects(hints: &[(&str, &str)], area: Rect) -> Vec<Rect> {
    let total_w: u16 = hints.iter().map(|(k, d)| {
        (k.chars().count() + 2 + d.chars().count() + 3) as u16
    }).sum();

    let start_x = area.x + area.width.saturating_sub(total_w) / 2;
    let y = area.y;

    let mut x = start_x;
    hints.iter().map(|(k, d)| {
        let badge_w = (k.chars().count() + 2) as u16;
        let desc_w  = (d.chars().count() + 3) as u16;
        let r = Rect::new(x, y, badge_w, 1);
        x += badge_w + desc_w;
        r
    }).collect()
}
