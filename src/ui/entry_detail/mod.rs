use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::{
    app::{App, entry::FocusedField},
    ui::{
        style::{ACCENT2, BG_SEL, BORDER, DIM, MUTED, TEXT, panel},
        utils::{render_footer, render_header, three_rows},
    },
};

pub fn render(frame: &mut Frame, app: &App, area: Rect, profile_index: usize, entry_index: usize) {
    let entry = match app
        .vault
        .as_ref()
        .and_then(|v| v.profiles.get(profile_index))
        .and_then(|p| p.entries.get(entry_index))
    {
        Some(e) => e.clone(),
        None => return,
    };

    let [header, body, footer] = three_rows(area);
    render_header(frame, &entry.username, Some("Details"), header);

    // Split body: top 3 fields as list, raw data below
    let focused_field = app.entry_form.as_ref().map(|f| f.focused_field);

    let fields: &[(FocusedField, &str, &str, bool)] = &[
        (FocusedField::Username, "Username", &entry.username, false),
        (FocusedField::Password, "Password", &entry.password, true),
        (FocusedField::Website, "Website", &entry.website, false),
    ];

    let raw_selected = focused_field == Some(FocusedField::RawData);

    // Raw data block needs enough height to be useful; give it at least 3 rows.
    let raw_h = (entry.raw_data.lines().count() as u16 + 2)
        .max(3)
        .min(body.height.saturating_sub(fields.len() as u16));
    let [list_area, raw_area] = {
        let c = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(raw_h)])
            .split(body);
        [c[0], c[1]]
    };

    let items: Vec<ListItem> = fields
        .iter()
        .map(|(field, label, value, secret)| {
            let selected = focused_field == Some(*field);
            let disp = if *secret {
                "●".repeat(value.len())
            } else if value.is_empty() {
                "—".to_string()
            } else {
                value.to_string()
            };

            let hint = if selected {
                Span::styled("  ↵ copy", Style::default().fg(DIM))
            } else {
                Span::raw("")
            };
            let lbl_style = if selected {
                Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(MUTED)
            };
            let val_style = if *secret {
                Style::default().fg(Color::Rgb(130, 130, 160))
            } else if selected {
                Style::default().fg(TEXT).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(TEXT)
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("  {:>14}   ", label), lbl_style),
                Span::styled(disp, val_style),
                hint,
            ]))
            .style(if selected {
                Style::default().bg(BG_SEL)
            } else {
                Style::default()
            })
        })
        .collect();

    frame.render_widget(List::new(items).block(panel("Details")), list_area);

    // Raw data as a multi-line paragraph
    let raw_label_style = if raw_selected {
        Style::default().fg(ACCENT2).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(MUTED)
    };
    let raw_val_style = if raw_selected {
        Style::default().fg(TEXT).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(TEXT)
    };
    let raw_hint = if raw_selected { "  ↵ copy" } else { "" };
    let raw_text = if entry.raw_data.is_empty() {
        "—".to_string()
    } else {
        format!("{}{}", entry.raw_data, raw_hint)
    };
    frame.render_widget(
        Paragraph::new(raw_text)
            .style(raw_val_style)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .title(Line::from(vec![Span::styled(
                        format!("  {:>14}   ", "Custom Data"),
                        raw_label_style,
                    )]))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if raw_selected { ACCENT2 } else { BORDER }))
                    .style(Style::default().bg(if raw_selected {
                        BG_SEL
                    } else {
                        Default::default()
                    })),
            ),
        raw_area,
    );

    render_footer(
        frame,
        &[
            ("j/k", "move"),
            ("↵", "copy field"),
            ("i/o/a/e/r", "edit"),
            ("g/G", "go to the first/last"),
            ("h/Esc", "back"),
            ("<Ctrl-c>", "quit"),
        ],
        footer,
    );
}
