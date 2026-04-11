use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::{
    app::App,
    ui::{
        style::{ACCENT, ACCENT2, BG, DIM, MUTED, TEXT, panel_focused},
        utils::centered_rect,
    },
};

pub fn render(frame: &mut Frame, app: &App, area: Rect, profile_index: Option<usize>) {
    let title = if profile_index.is_some() {
        "Rename Profile"
    } else {
        "New Profile"
    };

    let popup = centered_rect(50, 12, area);
    frame.render_widget(Clear, popup);

    let block = panel_focused(title);
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner);

    frame.render_widget(
        Paragraph::new("Profile name:").style(Style::default().fg(MUTED)),
        chunks[0],
    );
    frame.render_widget(
        Paragraph::new(format!(" {}▌", app.profile_name_input))
            .style(Style::default().fg(TEXT))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(ACCENT2)),
            ),
        chunks[1],
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                " Enter ",
                Style::default()
                    .fg(BG)
                    .bg(ACCENT)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" confirm  ", Style::default().fg(MUTED)),
            Span::styled(
                " Esc ",
                Style::default().fg(BG).bg(DIM).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" cancel", Style::default().fg(MUTED)),
        ]))
        .alignment(Alignment::Center),
        chunks[2],
    );
}
