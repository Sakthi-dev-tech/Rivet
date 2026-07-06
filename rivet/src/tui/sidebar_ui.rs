use ratatui::{
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};

pub fn sidebar_ui() -> impl Widget {
    Block::bordered()
        .title_top(Line::from(" Your APIs ").bold().left_aligned())
        .border_set(border::ROUNDED)
}
