use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
};

use crate::{
    app::App,
    ui::{
        style::{ACCENT2, BG_SEL, MUTED, TEXT, panel},
        utils::{render_footer, render_header, three_rows},
    },
};

pub fn render(frame: &mut Frame, app: &mut App, area: Rect, profile_index: usize) {
    let profile = app
        .vault
        .as_ref()
        .and_then(|v| v.profiles.get(profile_index));
    let pname = profile.map(|p| p.name.as_str()).unwrap_or("?");

    let [header, body, footer] = three_rows(area);
    render_header(frame, pname, Some("Entries"), header);

    app.layout.list_body = body;
    app.layout.footer = footer;

    let entries = profile.map(|p| p.entries.as_slice()).unwrap_or(&[]);
    let items: Vec<ListItem> = entries
        .iter()
        .map(|e| {
            let site = if e.website.is_empty() {
                "—".into()
            } else {
                e.website.clone()
            };
            ListItem::new(Line::from(vec![
                Span::styled("  🔑  ", Style::default().fg(ACCENT2)),
                Span::styled(
                    e.username.clone(),
                    Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("  ·  {}", site), Style::default().fg(MUTED)),
            ]))
        })
        .collect();

    let mut state = ListState::default();
    if !items.is_empty() {
        state.select(Some(app.selected));
    }

    frame.render_stateful_widget(
        List::new(items)
            .block(panel(&format!("Entries - {}", pname)))
            .highlight_style(Style::default().bg(BG_SEL))
            .highlight_symbol("▶ "),
        body,
        &mut state,
    );

    app.layout.footer_hints = render_footer(
        frame,
        &[
            ("n", "new"),
            ("e", "edit"),
            ("d", "del"),
            ("↵", "view"),
            ("h/Esc", "back"),
            ("<Ctrl-c>", "quit"),
            ("<Ctrl-x>", "save and quit"),
            // (":w", "save"),
        ],
        footer,
    );
}
