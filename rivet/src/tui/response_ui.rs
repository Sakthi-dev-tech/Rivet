use ratatui::{
    style::{Color, Style},
    symbols::border,
    widgets::{Block, Widget},
};

pub fn response_ui(is_hovered: bool, is_focused: bool) -> impl Widget {
    let border_style = if is_hovered {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    };

    Block::bordered()
        .border_style(border_style)
        .border_set(if is_focused {
            border::DOUBLE
        } else {
            border::ROUNDED
        })
        .title_top(" Response ")
}
