use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::{
    app::{App, edit_mode::Mode, entry::FocusedField},
    ui::{
        style::{ACCENT, ACCENT2, BG, BG_PANEL, BG_SEL, BORDER, MUTED, TEXT, panel},
        utils::{render_footer, three_rows},
    },
};

pub fn render(
    frame: &mut Frame,
    app: &mut App,
    area: Rect,
    _profile_index: usize,
    entry_index: Option<usize>,
) {
    let title = if entry_index.is_some() {
        "Edit Entry"
    } else {
        "New Entry"
    };
    let form = match &app.entry_form {
        Some(f) => f,
        None => return,
    };

    let [header, body, footer] = three_rows(area);

    app.layout.list_body = body;
    app.layout.footer = footer;

    let mode_span = match app.mode {
        Mode::Insert => Span::styled(
            " INSERT ",
            Style::default()
                .fg(BG)
                .bg(ACCENT2)
                .add_modifier(Modifier::BOLD),
        ),
        Mode::Normal => Span::styled(
            " NORMAL ",
            Style::default()
                .fg(BG)
                .bg(ACCENT)
                .add_modifier(Modifier::BOLD),
        ),
    };
    frame.render_widget(Clear, header);
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                "  ◈ SHELLPASS  / ",
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                title,
                Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            mode_span,
        ]))
        .style(Style::default().bg(BG_PANEL))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(BORDER)),
        ),
        header,
    );

    let in_insert = app.mode == Mode::Insert;
    let focused = &form.focused_field;
    let raw_focused = focused == &FocusedField::RawData;

    let field_row =
        |field: FocusedField, label: &str, val: &str, secret: bool| -> ListItem<'static> {
            let is_focused = focused == &field;
            let shown = if secret && !(in_insert && is_focused) {
                "●".repeat(val.len())
            } else {
                val.to_string()
            };
            let cursor = if is_focused && in_insert { "▌" } else { "" };
            let lbl_style = if is_focused {
                Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(MUTED)
            };
            let val_style = if is_focused {
                Style::default().fg(TEXT).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(TEXT)
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("  {:>14}   ", label), lbl_style),
                Span::styled(format!("{}{}", shown, cursor), val_style),
            ]))
            .style(if is_focused {
                Style::default().bg(BG_SEL)
            } else {
                Style::default().bg(BG_PANEL)
            })
        };

    let rows: Vec<ListItem> = vec![
        field_row(FocusedField::Username, "Username", &form.username, false),
        field_row(FocusedField::Password, "Password", &form.password, true),
        field_row(FocusedField::Website, "Website", &form.website, false),
    ];

    // Raw data block height: count lines in content, minimum 3, leave room for the list above.
    let raw_line_count = form.raw_data.lines().count().max(1) as u16;
    let raw_h = (raw_line_count + 2)
        .max(3)
        .min(body.height.saturating_sub(rows.len() as u16));
    let [list_area, raw_area] = {
        let c = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(raw_h)])
            .split(body);
        [c[0], c[1]]
    };

    app.layout.raw_area = raw_area;

    frame.render_widget(List::new(rows).block(panel(title)), list_area);

    // Raw data: multi-line paragraph with cursor appended at end when focused+insert
    let raw_lbl_style = if raw_focused {
        Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(MUTED)
    };
    let raw_val_style = if raw_focused {
        Style::default().fg(TEXT).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(TEXT)
    };
    let raw_display = if in_insert && raw_focused {
        format!("{}▌", form.raw_data)
    } else {
        form.raw_data.clone()
    };
    frame.render_widget(
        Paragraph::new(raw_display)
            .style(raw_val_style)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .title(Line::from(vec![Span::styled(
                        format!("  {:>14}   ", "Custom Data"),
                        raw_lbl_style,
                    )]))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if raw_focused { ACCENT2 } else { BORDER }))
                    .style(Style::default().bg(if raw_focused { BG_SEL } else { BG_PANEL })),
            ),
        raw_area,
    );

    let hints: Vec<(&str, &str)> = if in_insert {
        if raw_focused {
            vec![("Esc", "normal mode")]
        } else {
            vec![("Esc", "normal mode"), ("Tab", "next field")]
        }
    } else {
        vec![
            ("j/k", "move"),
            ("g/G", "move to first/last"),
            ("i/↵", "edit"),
            ("u", "clear field"),
            ("Esc", "cancel"),
            ("<Ctrl-s>", "save changes"),
            ("<Ctrl-c>", "quit"),
        ]
    };
    app.layout.footer_hints = render_footer(frame, &hints, footer);
}
