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
        style::{ACCENT, BG_SEL, MUTED, TEXT, panel},
        utils::{render_footer, render_header, three_rows},
    },
};

pub fn render(frame: &mut Frame, app: &mut App, area: Rect) {
    let [header, body, footer] = three_rows(area);
    render_header(frame, "Profiles", None, header);

    app.layout.list_body = body;
    app.layout.footer = footer;

    let vault = match &app.vault {
        Some(v) => v,
        None => return,
    };

    let items: Vec<ListItem> = vault
        .profiles
        .iter()
        .map(|p| {
            let n = p.entries.len();
            let sub = match n {
                0 => "empty".into(),
                1 => "1 entry".into(),
                n => format!("{} entries", n),
            };
            ListItem::new(Line::from(vec![
                Span::styled("  ⊞  ", Style::default().fg(ACCENT)),
                Span::styled(
                    p.name.clone(),
                    Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("  ·  {}", sub), Style::default().fg(MUTED)),
            ]))
        })
        .collect();

    let mut state = ListState::default();
    if !items.is_empty() {
        state.select(Some(app.selected));
    }

    frame.render_stateful_widget(
        List::new(items)
            .block(panel("Profiles"))
            .highlight_style(Style::default().bg(BG_SEL))
            .highlight_symbol("▶ "),
        body,
        &mut state,
    );

    app.layout.footer_hints = render_footer(
        frame,
        &[
            ("n", "new"),
            ("r", "rename"),
            ("d", "del"),
            ("↵", "open"),
            ("<Ctrl-s>", "save"),
            ("<Ctrl-c>", "quit"),
            ("<Ctrl-x>", "save and quit"),
        ],
        footer,
    );
}
