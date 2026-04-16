use ratatui::layout::Rect;

#[derive(Default, Clone)]
pub struct LayoutCache {
    pub list_body: Rect,
    pub footer: Rect,
    pub footer_hints: Vec<Rect>,
    pub raw_area: Rect,
}
