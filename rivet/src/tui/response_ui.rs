use ratatui::{
    Frame, layout::Rect, style::{Color, Style}, symbols::border, widgets::Block,
};

pub fn response_ui(
    frame: &mut Frame,
    area: Rect,
    is_hovered: bool,
    is_focused: bool
    ) {
    let border_style = if is_hovered {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    };

    let response_widget = Block::bordered()
        .border_style(border_style)
        .border_set(if is_focused {
            border::DOUBLE
        } else {
            border::ROUNDED
        })
        .title_top(" Response ");

    frame.render_widget(response_widget, area);
}
