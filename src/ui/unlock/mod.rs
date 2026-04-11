use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::{
    app::App,
    ui::{
        style::{ACCENT, BG, BG_PANEL, DIM, MUTED, TEXT, panel_focused},
        utils::centered_rect,
    },
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let popup = centered_rect(52, 20, area);
    frame.render_widget(Clear, popup);

    let title = if app.is_new_vault {
        "✦  Create Vault"
    } else {
        "✦  Unlock Vault"
    };
    let block = Block::default()
        .title(Span::styled(
            format!(" {} ", title),
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::default().fg(ACCENT))
        .style(Style::default().bg(BG_PANEL));
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(inner);

    let tagline = if app.is_new_vault {
        "Choose a strong master password"
    } else {
        "Enter master password to unlock"
    };
    frame.render_widget(
        Paragraph::new(tagline)
            .style(Style::default().fg(MUTED))
            .alignment(Alignment::Center),
        chunks[0],
    );

    let stars: String = "●".repeat(app.password_input.len());
    let display = if app.show_input {
        app.password_input.as_str()
    } else {
        stars.as_str()
    };
    let label = "Password";
    frame.render_widget(
        Paragraph::new(format!(" {}▌", display))
            .style(Style::default().fg(TEXT))
            .block(panel_focused(label)),
        chunks[2],
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
            Span::styled("  confirm  ", Style::default().fg(MUTED)),
            Span::styled(
                " Tab ",
                Style::default().fg(BG).bg(DIM).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" show/hide ", Style::default().fg(MUTED)),
            Span::styled(
                " ^C ",
                Style::default().fg(BG).bg(DIM).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" quit ", Style::default().fg(MUTED)),
        ]))
        .alignment(Alignment::Center),
        chunks[4],
    );
}
