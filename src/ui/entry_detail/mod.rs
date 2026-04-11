use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem},
};

use crate::{
    app::App,
    ui::{
        style::{ACCENT2, BG_SEL, DIM, MUTED, TEXT, panel},
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

    let mut fields: Vec<(String, String, bool)> = vec![
        ("Username".into(), entry.username.clone(), false),
        ("Password".into(), entry.password.clone(), true),
        ("Website".into(), entry.website.clone(), false),
    ];
    for field in &entry.custom_fields {
        fields.push((field.key.clone(), field.val.clone(), false));
    }

    let items: Vec<ListItem> = fields
        .iter()
        .enumerate()
        .map(|(i, (label, value, secret))| {
            let selected = i == app.detail_selected;
            let disp = if *secret {
                "●●●●●●●●●●".to_string()
            } else if value.is_empty() {
                "—".to_string()
            } else {
                value.clone()
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

    frame.render_widget(List::new(items).block(panel("Details")), body);

    render_footer(
        frame,
        &[
            ("j/k", "move"),
            ("↵", "copy field"),
            ("e", "edit"),
            ("h/Esc", "back"),
            (":w", "save"),
        ],
        footer,
    );
}
