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
        style::{ACCENT, ACCENT2, BG, BG_PANEL, DIM, MUTED, TEXT, panel_focused},
        utils::centered_rect,
    },
};

pub fn render(frame: &mut Frame, app: &mut App, area: Rect, profile_index: Option<usize>) {
    app.layout = Default::default();

    let title = if profile_index.is_some() {
        "Rename Profile"
    } else {
        "New Profile"
    };
    let h = area.height;

    if h < 3 {
        return;
    }

    let input_widget = || {
        Paragraph::new(format!(" {}▌", app.profile_name_input))
            .style(Style::default().fg(TEXT))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(ACCENT2)),
            )
    };

    if h < 7 {
        let popup = centered_rect(50, 100, area);
        let field_rect = Rect::new(popup.x, area.y + h.saturating_sub(3) / 2, popup.width, 3);
        frame.render_widget(Clear, field_rect);
        frame.render_widget(
            Paragraph::new(format!(" {}▌", app.profile_name_input))
                .style(Style::default().fg(TEXT).bg(BG_PANEL))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(ACCENT2)),
                ),
            field_rect,
        );
        return;
    }

    let show_label = h >= 11;
    let show_hints = h >= 9;

    let content_h: u16 = 3 + if show_label { 2 } else { 0 } + if show_hints { 2 } else { 0 };

    let popup_h = content_h + 4;
    let popup_w = 50_u16.min(area.width);
    let popup_x = area.x + area.width.saturating_sub(popup_w) / 2;
    let popup_y = area.y + area.height.saturating_sub(popup_h) / 2;
    let popup = Rect::new(popup_x, popup_y, popup_w, popup_h);

    frame.render_widget(Clear, popup);

    let block = panel_focused(title);
    let inner = block.inner(popup);
    frame.render_widget(block, popup);

    let mut constraints: Vec<Constraint> = Vec::new();
    if show_label {
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

    if show_label {
        frame.render_widget(
            Paragraph::new("Profile name:").style(Style::default().fg(MUTED)),
            chunks[i],
        );
        i += 2;
    }

    frame.render_widget(input_widget(), chunks[i]);
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
                Span::styled(" confirm  ", Style::default().fg(MUTED)),
                Span::styled(
                    " Esc ",
                    Style::default().fg(BG).bg(DIM).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" cancel", Style::default().fg(MUTED)),
            ]))
            .alignment(Alignment::Center),
            chunks[i],
        );
    }
}
