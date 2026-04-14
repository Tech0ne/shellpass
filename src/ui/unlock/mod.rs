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
    let h = area.height;

    if h < 3 {
        return;
    }

    let stars: String = "●".repeat(app.vault_pass.len());
    let display = if app.show_input {
        &app.vault_pass
    } else {
        &stars
    };

    if h < 7 {
        let popup = centered_rect(52, 100, area);
        let field_rect = Rect::new(popup.x, area.y + (h.saturating_sub(3)) / 2, popup.width, 3);
        frame.render_widget(Clear, field_rect);
        frame.render_widget(
            Paragraph::new(format!(" {}▌", display))
                .style(Style::default().fg(TEXT))
                .block(panel_focused("Password")),
            field_rect,
        );
        return;
    }

    let show_tagline = h >= 11;
    let show_hints = h >= 9;

    let content_h: u16 = 3 + if show_tagline { 2 } else { 0 } + if show_hints { 2 } else { 0 };

    let popup_h = content_h + 4;
    let popup_w = 52_u16.min(area.width);
    let popup_x = area.x + area.width.saturating_sub(popup_w) / 2;
    let popup_y = area.y + area.height.saturating_sub(popup_h) / 2;
    let popup = Rect::new(popup_x, popup_y, popup_w, popup_h);

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

    let mut constraints: Vec<Constraint> = Vec::new();
    if show_tagline {
        constraints.push(Constraint::Length(1));
        constraints.push(Constraint::Length(1));
    }
    constraints.push(Constraint::Length(3));
    if show_hints {
        constraints.push(Constraint::Length(1));
        constraints.push(Constraint::Length(1));
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(inner);

    let mut i = 0;

    if show_tagline {
        let tagline = if app.is_new_vault {
            "Choose a strong master password"
        } else {
            "Enter master password to unlock"
        };
        frame.render_widget(
            Paragraph::new(tagline)
                .style(Style::default().fg(MUTED))
                .alignment(Alignment::Center),
            chunks[i],
        );
        i += 2;
    }

    frame.render_widget(
        Paragraph::new(format!(" {}▌", display))
            .style(Style::default().fg(TEXT))
            .block(panel_focused("Password")),
        chunks[i],
    );
    i += 1;

    if show_hints {
        i += 1;
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
            chunks[i],
        );
    }
}
