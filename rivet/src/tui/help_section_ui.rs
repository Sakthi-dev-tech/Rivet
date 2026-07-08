use ratatui::{
    symbols::border,
    widgets::{Block, Widget},
};

pub fn help_section_ui() -> impl Widget {
    Block::bordered()
        .border_set(border::ROUNDED)
        .title_top(" Help ")
}
