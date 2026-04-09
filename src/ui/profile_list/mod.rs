use ratatui::{
    Frame,
    layout::Rect,
    style::Modifier,
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
};

use crate::{
    app::App,
    ui::{
        style::{panel, s_accent, s_muted, s_sel, s_text},
        utils::{render_footer, render_header, three_rows},
    },
};

pub fn render(frame: &mut Frame, app: &App, area: Rect) {
    let [header, body, footer] = three_rows(area);
    render_header(frame, "Profiles", None, area);

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
                Span::styled("  ⊞  ", s_accent()),
                Span::styled(p.name.clone(), s_text().add_modifier(Modifier::BOLD)),
                Span::styled(format!("  ·  {}", sub), s_muted()),
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
            .highlight_style(s_sel())
            .highlight_symbol("▶ "),
        body,
        &mut state,
    );

    render_footer(
        frame,
        &[
            ("n", "new"),
            ("r", "rename"),
            ("d", "del"),
            ("↵", "open"),
            (":w", "save"),
            (":q", "quit"),
        ],
        footer,
    );
}
