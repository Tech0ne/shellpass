use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

use crate::{
    app::App,
    ui::{
        style::{panel_focused, s_accent, s_bg_accent, s_dim_bg, s_muted, s_panel, s_text},
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
            s_accent().add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(s_accent())
        .style(s_panel());
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
            .style(s_muted())
            .alignment(Alignment::Center),
        chunks[0],
    );

    let stars: String = "●".repeat(app.password_input.len());
    let display = if app.show_input {
        &app.password_input
    } else {
        &stars
    };
    let label = "Password";
    frame.render_widget(
        Paragraph::new(format!(" {}▌", display))
            .style(s_text())
            .block(panel_focused(label)),
        chunks[2],
    );

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(" Enter ", s_bg_accent()),
            Span::styled("  confirm  ", s_muted()),
            Span::styled(" Tab ", s_dim_bg()),
            Span::styled(" show/hide ", s_muted()),
            Span::styled(" ^C ", s_dim_bg()),
            Span::styled(" quit ", s_muted()),
        ]))
        .alignment(Alignment::Center),
        chunks[4],
    );
}
