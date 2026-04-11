use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

use crate::{
    app::{App, vim_mode::Mode},
    ui::{
        style::{ACCENT, ACCENT2, BG, BG_PANEL, BG_SEL, BORDER, DIM, MUTED, TEXT, panel},
        utils::{render_footer, three_rows},
    },
};

pub fn render(
    frame: &mut Frame,
    app: &App,
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
        Mode::Command => Span::styled(
            " COMMAND",
            Style::default().fg(BG).bg(DIM).add_modifier(Modifier::BOLD),
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
    let focused = form.focused_field;

    let field_row = |index: usize, label: &str, val: &str, secret: bool| -> ListItem<'static> {
        let is_focused = index == focused;
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

    let mut rows: Vec<ListItem> = vec![
        field_row(0, "Username", &form.username, false),
        field_row(1, "Password", &form.password, true),
        field_row(2, "Website", &form.website, false),
    ];

    for (i, (k, v)) in form.custom_fields.iter().enumerate() {
        rows.push(field_row(3 + i * 2, "Field name", k, false));
        rows.push(field_row(3 + i * 2 + 1, "Field value", v, false));
    }

    // let add_index = 3 + form.custom_fields.len() * 2;
    // let add_style = if add_index == focused {
    //     CustomStyle::new().fg_suc().bg_sel().mod_bold().style
    // } else {
    //     CustomStyle::new().fg_dim().style
    // };
    // rows.push(ListItem::new(Line::from(vec![
    //     Span::raw("                   "),
    //     Span::styled("[ + Add custom field ]", add_style),
    // ])));

    frame.render_widget(List::new(rows).block(panel(title)), body);

    let hints: Vec<(&str, &str)> = if in_insert {
        vec![("Esc", "normal mode"), ("Tab", "next field")]
    } else {
        vec![
            ("i/↵", "edit"),
            ("j/k", "move"),
            ("D", "del field"),
            // (":", "save"),
            ("Esc", "cancel"),
        ]
    };
    render_footer(frame, &hints, footer);
}
